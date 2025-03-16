implementing cga3d;
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float wedge_g2_x = (other.e15 * this.scalar) + (other.scalar * this.e15);
        float wedge_g2_y = (other.e25 * this.scalar) + (other.scalar * this.e25);
        float wedge_g2_z = (other.e35 * this.scalar) + (other.scalar * this.e35);
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0_xyz[0] * other.scalar) + (wedge_g3[1] * other.e43), (wedge_g0_xyz[1] * other.scalar) + (wedge_g3[2] * other.e41), (wedge_g0_xyz[2] * other.scalar) + (wedge_g3[0] * other.e42), -(wedge_g2_y * other.e42) - (wedge_g2_z * other.e43) - (wedge_g0_xyz[0] * other.e15) - (wedge_g0_xyz[1] * other.e25) - (wedge_g0_xyz[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((float2(wedge_g2_w) * other.group1.xy), wedge_g2_w * other.e12, other.scalar * other.scalar * this.scalar) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_x * other.e41), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e31 * other.e4) + (this.scalar * other.e431), (this.e41 * other.e2) + (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group3.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], right_dual_g3[0] * wedge_g3[0]) + float4(float3(0.0), (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * this.scalar * other.e5) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[2] * wedge_g3[2]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar * other.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar * other.scalar) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e3215 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiFlatPoint other) {
        float wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321);
        float right_dual_g0_w = other.e321 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g0_w * wedge_g2_w), 
            /* e15, e25, e35, scalar */
            other.group0 * float4(float3(wedge_g2_w), right_dual_g0_w * this.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.e42 * other.e3, this.e43 * other.e1, this.e41 * other.e2, -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx);
        float4 wedge_g2 = float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy)), this.scalar * other.e5);
        float3 wedge_g3_xyz = float3(this.scalar) * other.group1.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * right_dual_g1[1]) + (wedge_g3_xyz[2] * right_dual_g1[2]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - float4((right_dual_g1.zxy * wedge_g0.yzx), right_dual_g0_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(right_dual_g1.www, right_dual_g0_w)) + float4(float3(0.0), -(right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])) - (right_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((right_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e45 * other.e23), (this.e43 * other.e15) + (this.e45 * other.e31), (this.e41 * other.e25) + (this.e45 * other.e12), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e15);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_dual_g0 * float3(wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4((float2(wedge_g2_w) * other.group1.xyz.xy), wedge_g2_w * other.e35, -(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g0[0] * wedge_g3[3]) + (wedge_g3[2] * other.e25), (right_dual_g0[1] * wedge_g3[3]) + (wedge_g3[0] * other.e35), (right_dual_g0[2] * wedge_g3[3]) + (wedge_g3[1] * other.e15), -(right_dual_g0[1] * wedge_g1_xyz[1]) - (right_dual_g0[2] * wedge_g1_xyz[2])) - float4((other.group1.zxy * wedge_g3.yzx), right_dual_g0[0] * wedge_g1_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * float4(this.group0.xyz, this.scalar);
        float3 wedge_g1_xyz = (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group1.xyz);
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12);
        float4 wedge_g3 = float4(this.e45 * other.e23, this.e45 * other.e31, this.e45 * other.e12, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group1.zxyw * float4(this.group0.yzx, this.scalar)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.scalar)) + (float4(wedge_g2_w) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)), 
            /* e23, e31, e12, e45 */
            float4(wedge_g1_xyz[0] * other.scalar, wedge_g1_xyz[1] * other.scalar, wedge_g1_xyz[2] * other.scalar, -(wedge_g3[0] * other.e23) - (wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((float2(wedge_g2_w) * other.group1.xy), wedge_g2_w * other.e35, other.scalar * other.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.scalar * other.scalar) * this.group2.xyz) + (float3(this.scalar * other.scalar) * other.group1.xyz) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4(this.e41 * other.e5, this.e42 * other.e5, this.e43 * other.e5, -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float4 wedge_g2 = float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float right_dual_g0_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0_w * wedge_g0[0], right_dual_g0_w * wedge_g0[1], right_dual_g0_w * wedge_g0[2], -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g2[2] * other.e2 * -1.0, wedge_g2[0] * other.e3 * -1.0, wedge_g2[1] * other.e1 * -1.0, (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3)) + (float4(right_dual_g0_w) * float4(wedge_g1.xyz, wedge_g2[3])) + (other.group0.zxyx * float4(wedge_g2.yzx, wedge_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xyz.xy), wedge_g2_w * other.e125, -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xy), wedge_g2_w * other.e125, -(wedge_g2_w * other.e12345) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group2.yzx), this.e23 * other.e15) - float4((other.group0.zxy * this.group2.yzx), this.e31 * other.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2_w * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2_w * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e25) + (wedge_g3[3] * other.e23), (wedge_g3[0] * other.e35) + (wedge_g3[3] * other.e31), (wedge_g3[1] * other.e15) + (wedge_g3[3] * other.e12), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group0.zxy * this.group2.yzx), this.e31 * other.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2_w * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2_w * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((wedge_g3.zxy * other.group2.yzx), wedge_g2_w * other.e3215) - (other.group2.zxyx * float4(wedge_g3.yzx, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_via_horizon_from(DualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e5, this.scalar * other.e12345) * float4(this.group0.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.e12345), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g1 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, other.e45 * other.e45 * this.scalar) * float4((wedge_g1.xyz * float3(-1.0)), 1.0) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((this.group0.yzx * other.group0.zxy), this.scalar * other.e3215) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.scalar)) * float4((wedge_g1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_via_horizon_from(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xyz.xy), wedge_g0_w * other.e435, -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar * other.e415, this.scalar * other.e425, this.scalar * other.e435, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e12345);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = this.scalar * other.e5;
        float4 wedge_g3 = (float4(this.scalar) * other.group3) + (float4(other.scalar) * float4(this.group2.xyz, this.e45));
        float3 wedge_g4 = (float3(this.scalar) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz);
        float3 wedge_g5 = (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(other.e5, other.e5, other.e5, this.scalar * other.e321) * float4(this.group0.xyz, 1.0)) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group3.zxy), this.scalar * other.e3215) - (other.group3.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group4.zxy * this.group2.yzx), this.e31 * other.e25);
        float wedge_g10 = (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) + (right_dual_g0[1] * this.scalar * other.scalar) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group0.xyz, other.e5);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_w * wedge_g0[0]) + (right_dual_g1 * wedge_g2_xyz[0]), (right_dual_g0_w * wedge_g0[1]) + (right_dual_g1 * wedge_g2_xyz[1]), (right_dual_g0_w * wedge_g0[2]) + (right_dual_g1 * wedge_g2_xyz[2]), -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g2_xyz[2] * other.e2 * -1.0, wedge_g2_xyz[0] * other.e3 * -1.0, wedge_g2_xyz[1] * other.e1 * -1.0, (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (right_dual_g0_w * this.scalar * other.e4)) + (other.group0.zxyx * float4(wedge_g2_xyz.yzx, wedge_g3[0])) + float4((float2(right_dual_g0_w) * wedge_g1.xy), right_dual_g0_w * wedge_g1[2], right_dual_g1 * wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_via_horizon_from(Scalar other) {
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
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_via_horizon_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.yzx * other.group3.zxy), this.scalar * other.e12345) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group2.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], wedge_g0[3] * other.e12345 * -1.0) + float4(float3(0.0), (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * this.scalar * other.e5) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[1] * wedge_g3[1]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0_xyz = (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group0.xyz);
        float4 wedge_g1 = (float4(this.scalar) * other.group1) + (float4(other.scalar) * this.group1);
        float4 wedge_g2 = float4(this.scalar * other.e15, this.scalar * other.e25, this.scalar * other.e35, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (this.group2 * float4(other.group0.www, other.e1234));
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) + (other.scalar * other.scalar * this.scalar) - (right_dual_g2_xyz[1] * wedge_g0_xyz[1]) - (right_dual_g2_xyz[2] * wedge_g0_xyz[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0_xyz * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4((other.e42 * this.e3) + (other.e23 * this.e4) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e31 * this.e4) + (other.scalar * this.e431), (other.e41 * this.e2) + (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group3.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(wedge_g0[3])) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0[3] * other.e15, wedge_g0[3] * other.e25, wedge_g0[3] * other.e35, -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (float4(other.scalar, other.scalar, other.scalar, other.scalar * other.scalar * this.e5) * float4(wedge_g2_xyz, 1.0)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g0[2] * other.e25) + (wedge_g3[0] * other.scalar), (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g0[0] * other.e35) + (wedge_g3[1] * other.scalar), (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g0[1] * other.e15) + (wedge_g3[2] * other.scalar), -(right_dual_g0[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g0[0]) - (right_dual_g1[1] * wedge_g0[1]) - (right_dual_g1[2] * wedge_g0[2])) + float4((right_dual_g0.zxy * wedge_g2_xyz.yzx), wedge_g3[3] * other.scalar) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), right_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (other.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(other.group3.yzx, other.e4));
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((other.group3.xyz * this.group3.www), other.e4 * this.e321);
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + (this.group2.wwwy * float4(other.group2.xyz, other.e2)) + float4((other.group1.yzx * this.group3.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.yy, other.scalar, this.e4 * other.e3215) * float4(this.group0.xyz, 1.0);
        float4 wedge_g2 = float4(other.scalar) * float4(this.group2.xyz, this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g2[3] * other.scalar)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4) * other.group0.xyz;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e4 * other.e321), 
            /* e5 */
            (right_dual_g0[3] * this.e1 * other.e235) + (right_dual_g0[3] * this.e2 * other.e315) + (right_dual_g0[3] * this.e3 * other.e125) + (right_dual_g0[3] * this.e5 * other.e321) - (wedge_g0_xyz[0] * right_dual_g0[0]) - (wedge_g0_xyz[1] * right_dual_g0[1]) - (wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx);
        float4 wedge_g2 = float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((this.group3.xyz * other.group1.www), this.e4 * other.e321) - (other.group1.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e235);
        float right_dual_g0_w = other.e321 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), right_dual_g1_w * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g1_w) * wedge_g3.xy), right_dual_g1_w * wedge_g3[2], right_dual_g0_w * wedge_g2[3]) + float4(float3(0.0), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group1.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (wedge_g2.wwwz * float4(other.group0.xyz, other.e3)) + (other.group1.zxyy * float4(wedge_g1_xyz.yzx, wedge_g2[1])) + float4((float2(right_dual_g1_w) * wedge_g0.xy), right_dual_g1_w * wedge_g0[2], wedge_g2[0] * other.e1) + float4(float3(0.0), (right_dual_g1_w * this.e4 * other.e5) - (wedge_g3[0] * other.e235) - (wedge_g3[1] * other.e315) - (wedge_g3[2] * other.e125)) - float4((wedge_g1_xyz.zxy * other.group1.yzx), right_dual_g0_w * wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e15);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * float3(wedge_g2[3])), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g1 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_w * right_dual_g0[0]) + (right_dual_g1[1] * wedge_g0[2]), (wedge_g1_w * right_dual_g0[1]) + (right_dual_g1[2] * wedge_g0[0]), (wedge_g1_w * right_dual_g0[2]) + (right_dual_g1[0] * wedge_g0[1]), -(right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (right_dual_g1[0] * wedge_g1_xyz[0]) - (right_dual_g1[1] * wedge_g1_xyz[1]) - (right_dual_g1[2] * wedge_g1_xyz[2])) - float4((right_dual_g1.zxy * wedge_g0.yzx), right_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.e4 * other.e23, this.e4 * other.e31, this.e4 * other.e12, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) + float4((this.group0.xyz * other.group0.www), this.e4 * other.e3215);
        float4 wedge_g1 = float4(this.e4 * other.e15, this.e4 * other.e25, this.e4 * other.e35, -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group0.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g2[0]) - (right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[3] * wedge_g3[0]) + (right_dual_g1[1] * wedge_g0[2]), (right_dual_g0[3] * wedge_g3[1]) + (right_dual_g1[2] * wedge_g0[0]), (right_dual_g0[3] * wedge_g3[2]) + (right_dual_g1[0] * wedge_g0[1]), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (right_dual_g0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx);
        float4 wedge_g2 = float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = float4(this.e425 * other.e3 * -1.0, this.e435 * other.e1 * -1.0, this.e415 * other.e2 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float right_dual_g0_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.zxy * other.group0.yzx) - (wedge_g3.yzx * other.group0.zxy)), right_dual_g0_w * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0_w * wedge_g3[0], right_dual_g0_w * wedge_g3[1], right_dual_g0_w * wedge_g3[2], -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(wedge_g1_xyz[2] * other.e2 * -1.0, wedge_g1_xyz[0] * other.e3 * -1.0, wedge_g1_xyz[1] * other.e1 * -1.0, (wedge_g2[2] * other.e3) + (right_dual_g0_w * this.e4 * other.e5)) + (other.group0.zxyy * float4(wedge_g1_xyz.yzx, wedge_g2[1])) + float4((float2(right_dual_g0_w) * wedge_g0.xy), right_dual_g0_w * wedge_g0[2], wedge_g2[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - (this.group3.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group3.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + float4((other.group2.xyz * other.group0.xxx * this.group3.xxx), wedge_g0[2] * other.e412) + float4((other.group2.xyz * other.group0.yyy * this.group3.yyy), right_dual_g1_w * this.e4 * other.e321) + float4((other.group2.xyz * this.group2.www * other.group1.www), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - (this.group3.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group3.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + float4((other.group0.xxx * this.group3.xxx * other.group2.xyz), wedge_g0[2] * other.e412) + float4((other.group0.yyy * this.group3.yyy * other.group2.xyz), right_dual_g1_w * this.e4 * other.e321) + float4((other.group2.xyz * this.group2.www * other.group1.www), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((other.group2.yzx * this.group3.zxy), this.e423 * other.e15);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * wedge_g2[1]) + (right_dual_g2[1] * wedge_g0[2]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2[2]) + (right_dual_g2[2] * wedge_g0[0]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2[0]) + (right_dual_g2[0] * wedge_g0[1]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g2[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) - float4((right_dual_g0.yzx * wedge_g2.zxy), right_dual_g2[0] * wedge_g1[0]) - float4((right_dual_g2.zxy * wedge_g0.yzx), right_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.yzx * other.group2.zxy), this.e4 * other.e3215) - (other.group2.yzxx * float4(this.group3.zxy, this.e423));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g2.www), (wedge_g2[3] * other.e1234) - (right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[3] * other.e4235), (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[3] * other.e4315), (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[3] * other.e4125), -(right_dual_g1[2] * wedge_g2[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (wedge_g2.yzxw * float4(right_dual_g0.zxy, other.e3215)) - (wedge_g2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_via_horizon_from(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e4 * other.e5 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4(this.e4 * other.e15, this.e4 * other.e25, this.e4 * other.e35, -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) * float4(right_dual_g0.www, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[3] * other.e4235, wedge_g0[3] * other.e4315, wedge_g0[3] * other.e4125, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_dual_g0.www, other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(right_dual_g1_w) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((right_dual_g0.zxy * wedge_g1.yzx), right_dual_g1_w * this.e4 * other.e5) - (wedge_g1.zxyx * float4(right_dual_g0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g1 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = (float4(other.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz);
        float3 wedge_g5 = (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx);
        float4 wedge_g6 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (float3(this.e4) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group3.zxy) - (this.group3.zxy * other.group3.yzx);
        float4 wedge_g9 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group8.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group6.yzx), this.e125 * other.e3) - (float4(this.e5) * float4(other.group7.xyz, other.e321)) - (this.group3.yzxz * float4(other.group6.zxy, other.e125)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e235) - float4((this.group2.xyz * other.group1.www), this.e2 * other.e315);
        float wedge_g10 = (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_reject_via_horizon_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(other.e4235 * other.e4235 * this.e1) - (this.e4 * other.e4235 * other.e3215) - (this.e2 * other.e4235 * other.e4315) - (this.e3 * other.e4235 * other.e4125), -(other.e4315 * other.e4315 * this.e2) - (this.e4 * other.e4315 * other.e3215) - (this.e1 * other.e4235 * other.e4315) - (this.e3 * other.e4315 * other.e4125), -(other.e4125 * other.e4125 * this.e3) - (this.e4 * other.e4125 * other.e3215) - (this.e1 * other.e4235 * other.e4125) - (this.e2 * other.e4315 * other.e4125), (other.e3215 * other.e3215 * this.e4) + (this.e1 * other.e4235 * other.e3215) + (this.e2 * other.e4315 * other.e3215) + (this.e3 * other.e4125 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = float4((this.group3.yzx * other.group0.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group0.yzxw);
        float4 wedge_g2 = float4(this.e1 * other.e5, this.e2 * other.e5, this.e3 * other.e5, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group0.yzx), right_dual_g0_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0_w * wedge_g3[0], right_dual_g0_w * wedge_g3[1], right_dual_g0_w * wedge_g3[2], -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group0.xyz * wedge_g3.www), right_dual_g1 * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * wedge_g2[0]) - (wedge_g1[2] * other.e2), -(right_dual_g1 * wedge_g2[1]) - (wedge_g1[0] * other.e3), -(right_dual_g1 * wedge_g2[2]) - (wedge_g1[1] * other.e1), (wedge_g2[1] * other.e2) + (wedge_g2[2] * other.e3)) + (float4(right_dual_g0_w) * float4(wedge_g0, wedge_g1[3])) + (other.group0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(Scalar other) {
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
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = float4((this.group3.yzx * other.group3.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group3.yzxw);
        float4 wedge_g2 = float4(float3(0.0), (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + float4((this.group3.xyz * other.group2.www), this.e4 * other.e321) - (other.group3.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group1.yzx), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxz * float4(other.group1.zxy, other.e125)) - float4((this.group1.yzx * other.group3.zxy), this.e1 * other.e235) - float4((this.group2.xyz * other.group3.www), this.e2 * other.e315);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = (this.group3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.xyz * other.group0.www), this.e4 * other.e3215) + float4((other.group1.xyz * this.group2.www), this.e1 * other.e4235) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2]) - (right_dual_g2[0] * wedge_g1[0]) - (right_dual_g2[1] * wedge_g1[1]) - (right_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[3] * wedge_g1[0]) + (right_dual_g2[1] * wedge_g0[2]) + (wedge_g2[1] * other.e43) + (wedge_g3[0] * other.scalar), (right_dual_g1[3] * wedge_g1[1]) + (right_dual_g2[2] * wedge_g0[0]) + (wedge_g2[2] * other.e41) + (wedge_g3[1] * other.scalar), (right_dual_g1[3] * wedge_g1[2]) + (right_dual_g2[0] * wedge_g0[1]) + (wedge_g2[0] * other.e42) + (wedge_g3[2] * other.scalar), -(right_dual_g1[2] * wedge_g0[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) + float4((right_dual_g1.xyz * wedge_g1.www), wedge_g3[3] * other.scalar) - (wedge_g0.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])) - float4((wedge_g2.zxy * other.group0.yzx), right_dual_g1[1] * wedge_g0[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g3_w = other.scalar * this.e3215;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.scalar) - (right_dual_g0[0] * wedge_g2_xyz[0]) - (right_dual_g0[1] * wedge_g2_xyz[1]) - (right_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0 * float3(wedge_g3_w)) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.scalar)) + (float3(wedge_g3_w) * other.group1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g3_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, other.e4 * this.e3215) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e5);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_dual_g2_w) * float4(wedge_g1.xyz, wedge_g2[3])) + (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4(float3(0.0), (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[1] * wedge_g3[1]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.scalar * other.scalar * this.e3215) + (other.e3215 * other.scalar * this.scalar), other.scalar * other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g0[3] * other.e321) + (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.group0.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group1.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) * float4(wedge_g0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            (wedge_g1[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * wedge_g2.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2[3] * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2[3] * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g3.zxy * other.group2.yzx), wedge_g2[3] * other.e3215) - (other.group2.zxyx * float4(wedge_g3.yzx, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_via_horizon_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * -1.0) * float2(other.e5 * other.e12345, other.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.scalar)) * float4((wedge_g1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e3215 * other.e4) + (this.scalar * other.e12345);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = this.scalar * other.e5;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float3 wedge_g4 = float3(this.scalar) * other.group4.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group5.xyz;
        float4 wedge_g6 = float4(this.scalar) * other.group6;
        float3 wedge_g7 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group8.xyz;
        float4 wedge_g9 = float4(this.group0.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group9.xyz, 1.0);
        float wedge_g10 = this.scalar * other.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) + (right_dual_g0[1] * this.scalar * other.scalar) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_via_horizon_from(RoundPoint other) {
        float wedge_g0_w = this.e3215 * other.e4;
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (right_dual_g0[0] * wedge_g3[0]) + (right_dual_g0[1] * wedge_g3[1]) + (right_dual_g0[2] * wedge_g3[2]) + (right_dual_g0[3] * wedge_g3[3]) + (right_dual_g1 * this.scalar * other.e5)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_dual_g1 * wedge_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_via_horizon_from(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_via_horizon_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0 * float4(wedge_g0[3])) + (float4(right_dual_g2_w) * float4(wedge_g1.xyz, wedge_g2[3])) + (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * wedge_g3[1]) + (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) - (right_dual_g0[0] * wedge_g2[0]) - (right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.group0.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group3.xyz, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g0[3] * other.scalar) + (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (right_dual_g2_xyz[1] * wedge_g0[1]) - (right_dual_g2_xyz[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.xyz * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g1 = float4(float3(0.0), other.scalar * this.e321);
        float3 wedge_g2_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g2_w) * float4(right_dual_g0, other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e15) + (wedge_g2_xyz[0] * other.scalar), (wedge_g2_w * other.e25) + (wedge_g2_xyz[1] * other.scalar), (wedge_g2_w * other.e35) + (wedge_g2_xyz[2] * other.scalar), -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * wedge_g2_xyz.yzx), 0.0) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0_xyz = float3(other.e4 * -1.0) * this.group0.xyz;
        float wedge_g0_w = (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321);
        float wedge_g1 = other.e4 * this.e321 * -1.0;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (wedge_g0_xyz * float3(right_dual_g2_w)), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz.zxy * other.group3.yzx) - (wedge_g0_xyz.yzx * other.group3.zxy)), (right_dual_g3_w * wedge_g1) - (right_dual_g2_w * wedge_g0_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0_w * other.e1 * -1.0, wedge_g0_w * other.e2 * -1.0, wedge_g0_w * other.e3 * -1.0, (wedge_g0_xyz[0] * other.e423) + (wedge_g0_xyz[1] * other.e431) + (wedge_g0_xyz[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0_xyz.xy), right_dual_g3_w * wedge_g0_xyz[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0_xyz[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0_xyz[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0_xyz[0] * other.e425), -(wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125)) - (float4(wedge_g0_w) * float4(other.group0.xyz, right_dual_g1_w)) - float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g0_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_via_horizon_from(AntiFlector other) {
        float wedge_g0_x = (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g0_x * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), wedge_g0_x * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.scalar), 
            /* e1, e2, e3, e5 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_via_horizon_from(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            -float4((float3(this.e321 * other.e5) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group0.xyz), 0.0) - float4((float3(other.e1) * this.group0.yxx * other.group0.yyz), 0.0) - float4((float3(other.e3) * this.group0.zzy * other.group0.xyy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_reject_via_horizon_from(Dipole other) {
        float wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_via_horizon_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e321) * float4(other.e15 * other.e45, other.e25 * other.e45, other.e35 * other.e45, other.e45 * other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0 = this.e321 * other.e45 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float wedge_g6_w = this.e321 * other.scalar;
        float3 wedge_g8 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g9 = float4(other.e4, other.e4, other.e4, (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float wedge_g10 = this.e321 * other.e4 * -1.0;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (wedge_g6_w * right_dual_g3[3]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g7[1] * wedge_g8[2]) - (wedge_g9[2] * other.e425) - (wedge_g9[3] * other.e423), -(right_dual_g7[2] * wedge_g8[0]) - (wedge_g9[0] * other.e435) - (wedge_g9[3] * other.e431), -(right_dual_g7[0] * wedge_g8[1]) - (wedge_g9[1] * other.e415) - (wedge_g9[3] * other.e412), 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g9.yzxz * float4(other.group6.zxy, other.e412)) + float4((float2(wedge_g6_w) * right_dual_g6_xyz.xy), wedge_g6_w * right_dual_g6_xyz[2], wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]), right_dual_g6_xyz[2] * wedge_g9[2] * -1.0) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6_w) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (right_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) + float4(float3(0.0), wedge_g6_w * right_dual_g0[1]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(other.e4, other.e4, other.e4, (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float wedge_g1 = this.e321 * other.e4 * -1.0;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group0.yzx), right_dual_g0_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_via_horizon_from(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(other.e4, other.e4, other.e4, (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float wedge_g1 = this.e321 * other.e4 * -1.0;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group3.yzx), right_dual_g3_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[0] * other.e423) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0.xy), right_dual_g3_w * wedge_g0[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0[0] * other.e425), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyw * float4(other.group1.yzx, right_dual_g1_w)) - (wedge_g0.wwwx * float4(other.group0.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g1 = float4(float3(0.0), this.e321 * other.scalar);
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) * float4(this.group0.xyz, 1.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(right_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0[3] * wedge_g2[0], right_dual_g0[3] * wedge_g2[1], right_dual_g0[3] * wedge_g2[2], -(right_dual_g2_xyz[0] * wedge_g1[0]) - (right_dual_g2_xyz[1] * wedge_g1[1]) - (right_dual_g2_xyz[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) + (float4(wedge_g2[3]) * float4(right_dual_g2_xyz, other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g2[1]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2[2]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2[0]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) + (float4(wedge_g2[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (right_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2, -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float3 wedge_g3_xyz = float3(other.scalar) * this.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(wedge_g0[3])) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0[3] * other.e15, wedge_g0[3] * other.e25, wedge_g0[3] * other.e35, -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (float4(other.scalar, other.scalar, other.scalar, other.scalar * other.scalar * this.e5) * float4(wedge_g2_xyz, 1.0)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g2_xyz[1]) + (wedge_g3_xyz[0] * other.scalar) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g0[2] * other.e25), (right_dual_g0[0] * wedge_g2_xyz[2]) + (wedge_g3_xyz[1] * other.scalar) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g0[0] * other.e35), (right_dual_g0[1] * wedge_g2_xyz[0]) + (wedge_g3_xyz[2] * other.scalar) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g0[1] * other.e15), -(right_dual_g0[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g0[0]) - (right_dual_g1[1] * wedge_g0[1]) - (right_dual_g1[2] * wedge_g0[2])) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), right_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy);
        float wedge_g1_w = other.e4 * this.e5 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((other.group3.xyz * this.group1.www), other.e4 * this.e321);
        float4 wedge_g3 = float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1_w), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1_w)) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public DualNum anti_reject_via_horizon_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e321 * other.e321 * this.e5) - (other.e235 * other.e321 * this.e1) - (other.e315 * other.e321 * this.e2) - (other.e125 * other.e321 * this.e3), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0_xyz = (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g1 = (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group1.wwwx * float4(other.group1.xyz, other.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3] * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0_xyz[2] * other.e2 * -1.0, wedge_g0_xyz[0] * other.e3 * -1.0, wedge_g0_xyz[1] * other.e1 * -1.0, (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e321)) + (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.zxyx * float4(other.group1.yzx, other.e23));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0 * float3(wedge_g0[3])), -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23));
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(right_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[3] * wedge_g1[0], right_dual_g0[3] * wedge_g1[1], right_dual_g0[3] * wedge_g1[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (right_dual_g0 * float4(wedge_g0.www, wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0_xyz = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 wedge_g1 = float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) + (other.group0.wwwx * float4(this.group1.xyz, this.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3] * -1.0) * other.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0_xyz[2] * other.e2 * -1.0, wedge_g0_xyz[0] * other.e3 * -1.0, wedge_g0_xyz[1] * other.e1 * -1.0, (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)) + (other.group0.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - (this.group1.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group1.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + float4((other.group2.xyz * other.group0.xxx * this.group1.xxx), wedge_g0[1] * other.e431) + float4((other.group2.xyz * other.group0.yyy * this.group1.yyy), wedge_g0[2] * other.e412), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - (this.group1.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group1.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + float4((other.group0.xxx * this.group1.xxx * other.group2.xyz), wedge_g0[1] * other.e431) + float4((other.group0.yyy * this.group1.yyy * other.group2.xyz), wedge_g0[2] * other.e412), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((other.group2.yzx * this.group1.zxy), this.e235 * other.e41);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * wedge_g2[1]) + (right_dual_g2[1] * wedge_g0[2]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2[2]) + (right_dual_g2[2] * wedge_g0[0]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2[0]) + (right_dual_g2[0] * wedge_g0[1]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g2[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) - float4((right_dual_g0.yzx * wedge_g2.zxy), right_dual_g2[0] * wedge_g1[0]) - float4((right_dual_g2.zxy * wedge_g0.yzx), right_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = (this.group1.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group1.wwwy * float4(other.group1.xyz, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group1.zxy * other.group2.yzx), this.e235 * other.e41);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g2.www), (wedge_g2[3] * other.e1234) - (right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[3] * other.e4235), (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[3] * other.e4315), (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[3] * other.e4125), -(right_dual_g1[2] * wedge_g2[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (wedge_g2.yzxw * float4(right_dual_g0.zxy, other.e3215)) - (wedge_g2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4(other.e45 * -1.0) * float4(this.group1.xyz, this.e321);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) * float4(right_dual_g0.www, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(other.e45, other.e45, other.e45, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)) * float4((this.group1.xyz * float3(-1.0)), 1.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[3] * other.e4235, wedge_g0[3] * other.e4315, wedge_g0[3] * other.e4125, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_dual_g0.www, other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group0.yzx), this.e321 * other.e5) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g1[1] * other.e435, wedge_g1[2] * other.e415, wedge_g1[0] * other.e425, -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) - (wedge_g1.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float4 wedge_g1 = float4((float3(other.scalar) * this.group1.xyz), 0.0);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0);
        float3 wedge_g4 = float3(other.e4 * -1.0) * this.group1.xyz;
        float3 wedge_g5 = (this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx);
        float4 wedge_g6 = float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group4.xyz * this.group1.www), this.e321 * other.scalar) - (this.group1.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (other.group4.yzx * this.group1.zxy) - (other.group4.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float4 wedge_g9 = float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group6.yzx), this.e235 * other.e1) - (this.group1.yzxy * float4(other.group6.zxy, other.e315)) - (this.group1.wwwz * float4(other.group7.xyz, other.e125)) - float4((this.group0.xyz * other.group1.www), this.e1 * other.e235);
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e321 * other.e4);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_via_horizon_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(other.e4235 * other.e4235 * this.e1) - (this.e2 * other.e4235 * other.e4315) - (this.e3 * other.e4235 * other.e4125), -(other.e4315 * other.e4315 * this.e2) - (this.e1 * other.e4235 * other.e4315) - (this.e3 * other.e4315 * other.e4125), -(other.e4125 * other.e4125 * this.e3) - (this.e1 * other.e4235 * other.e4125) - (this.e2 * other.e4315 * other.e4125), (this.e1 * other.e4235 * other.e3215) + (this.e2 * other.e4315 * other.e3215) + (this.e3 * other.e4125 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz);
        float wedge_g2_w = this.e321 * other.e4 * -1.0;
        float4 wedge_g3 = float4(other.e4, other.e4, other.e4, (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2_w) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group0.yzx), right_dual_g0_w * wedge_g2_w) - (wedge_g3.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0_w * wedge_g3[0], right_dual_g0_w * wedge_g3[1], right_dual_g0_w * wedge_g3[2], -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group0.xyz * wedge_g3.www), right_dual_g1 * wedge_g1_w), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * wedge_g2_xyz[0]) - (wedge_g1_xyz[2] * other.e2), -(right_dual_g1 * wedge_g2_xyz[1]) - (wedge_g1_xyz[0] * other.e3), -(right_dual_g1 * wedge_g2_xyz[2]) - (wedge_g1_xyz[1] * other.e1), (wedge_g2_xyz[1] * other.e2) + (wedge_g2_xyz[2] * other.e3)) + (float4(right_dual_g0_w) * float4(wedge_g0, wedge_g1_w)) + (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_via_horizon_from(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - (other.group3 * float4(this.group1.www, this.e321));
        float4 wedge_g3 = float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e1) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - (this.group1.wwwz * float4(other.group0.xyz, other.e125)) - float4((this.group0.xyz * other.group3.www), this.e1 * other.e235);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1_w), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1_w)) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiFlector {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = (this.group1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (other.group0.zxyx * float4(this.group1.yzx, this.e235));
        float4 wedge_g1 = (other.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float3 wedge_g3_xyz = float3(other.scalar) * this.group1.xyz;
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2]) - (right_dual_g2[0] * wedge_g1[0]) - (right_dual_g2[1] * wedge_g1[1]) - (right_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g3_xyz[0] * other.scalar) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (right_dual_g2[1] * wedge_g0[2]) + (wedge_g2[1] * other.e43), (wedge_g3_xyz[1] * other.scalar) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (right_dual_g2[2] * wedge_g0[0]) + (wedge_g2[2] * other.e41), (wedge_g3_xyz[2] * other.scalar) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (right_dual_g2[0] * wedge_g0[1]) + (wedge_g2[0] * other.e42), -(right_dual_g1[2] * wedge_g0[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (wedge_g0.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])) - float4((wedge_g2.zxy * other.group0.yzx), right_dual_g1[1] * wedge_g0[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiLine> operator /(AntiLine lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g1_xyz = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group1.xyz;
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e45 * this.e23), (other.e43 * this.e15) + (other.e45 * this.e31), (other.e41 * this.e25) + (other.e45 * this.e12), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group1.yzx), other.e23 * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e23) + (wedge_g3[1] * other.e43), (wedge_g2_w * other.e31) + (wedge_g3[2] * other.e41), (wedge_g2_w * other.e12) + (wedge_g3[0] * other.e42), -(wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g1_xyz[0] * other.e23), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e15) + (wedge_g1_xyz[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g2_w * other.e25) + (wedge_g1_xyz[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g2_w * other.e35) + (wedge_g1_xyz[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.scalar)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e4) * this.group1.xyz;
        float wedge_g1_w = -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) - float4((this.group1.yzx * other.group3.zxy), other.e423 * this.e15);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1_xyz[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1_xyz[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1_xyz[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1_w), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(wedge_g1_xyz[1] * right_dual_g3[1]) - (wedge_g1_xyz[2] * right_dual_g3[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * float4(float3(wedge_g1_w), wedge_g1_xyz[0])), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(right_dual_g3[3])) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23));
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[3]) * float4((right_dual_g1.xyz * float3(-1.0)), other.e321), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g1.zxy * wedge_g0.yzx) - (right_dual_g1.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public FlatPoint anti_reject_via_horizon_from(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e15 * this.e23) * other.group0.xyz), 0.0) + float4((float3(other.e25 * this.e31) * other.group0.xyz), 0.0) + float4((float3(other.e35 * this.e12) * other.group0.xyz), 0.0) + float4((other.group0.xyz * other.group0.xyz * this.group1.xyz), 0.0) + float4((other.group0.xyz * other.group0.yxx * this.group1.yxx), 0.0) + float4((other.group0.xyz * other.group0.zzy * this.group1.zzy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar, other.scalar, other.scalar, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) * float4(this.group1.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar, other.scalar, other.scalar, -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)) * float4(wedge_g0_xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0[3] * -1.0) * right_dual_g0.xyz, 
            /* e15, e25, e35 */
            (right_dual_g0.zxy * wedge_g0.yzx) - (right_dual_g0.yzx * wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_via_horizon_from(Circle other) {
        float wedge_g0 = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0 = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float wedge_g1 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float wedge_g1 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0[1] * other.e35 * -1.0, wedge_g0[2] * other.e15 * -1.0, wedge_g0[0] * other.e25 * -1.0, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e1234)) + (wedge_g0.wwwx * float4(other.group1.xyz, other.e4235)) + float4((wedge_g0.zxy * other.group2.yzx), wedge_g1 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4(other.e45, other.e45, other.e45, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) * float4(this.group0.xyz, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * wedge_g0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(other.e45, other.e45, other.e45, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) * float4(this.group0.xyz, 1.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4((right_dual_g0.www * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * wedge_g0.zxy) - (right_dual_g0.zxy * wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_via_horizon_from(Line other) {
        float wedge_g0 = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(wedge_g0_w) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4((float3(other.scalar) * this.group1.xyz), 0.0);
        float3 wedge_g5 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g6 = float4(other.e4, other.e4, other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) * float4(this.group1.xyz, 1.0);
        float3 wedge_g7 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g8 = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group4.zxy), this.e23 * other.e15);
        float wedge_g10 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), right_dual_g7[2] * wedge_g6[2] * -1.0) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float4 wedge_g1 = float4(other.e4, other.e4, other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) * float4(this.group1.xyz, 1.0);
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_w * wedge_g0[0]) + (right_dual_g1 * wedge_g2[0]), (right_dual_g0_w * wedge_g0[1]) + (right_dual_g1 * wedge_g2[1]), (right_dual_g0_w * wedge_g0[2]) + (right_dual_g1 * wedge_g2[2]), -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_dual_g0_w) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_via_horizon_from(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float4 wedge_g1 = float4(other.e4, other.e4, other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) * float4(this.group1.xyz, 1.0);
        float4 wedge_g2 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group1.yzx * other.group3.zxy), this.e23 * other.e415);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g1_xyz = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) * float4(this.group1.xyz, 1.0);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(other.group0.zxy, right_dual_g3_xyz[1])) + float4(float3(0.0), (right_dual_g3_xyz[2] * wedge_g3[2]) + (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[0] * wedge_g3[0]) - float4((wedge_g3.zxy * other.group0.yzx), wedge_g1_xyz[0] * other.e23), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_xyz[0] * wedge_g2[3]) + (wedge_g1_xyz[0] * other.scalar) + (wedge_g3[3] * other.e41), (right_dual_g2_xyz[1] * wedge_g2[3]) + (wedge_g1_xyz[1] * other.scalar) + (wedge_g3[3] * other.e42), (right_dual_g2_xyz[2] * wedge_g2[3]) + (wedge_g1_xyz[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.scalar);
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz);
        float wedge_g1_w = other.e45 * this.scalar;
        float wedge_g2_x = (other.e15 * this.scalar) + (other.scalar * this.e15);
        float wedge_g2_y = (other.e25 * this.scalar) + (other.scalar * this.e25);
        float wedge_g2_z = (other.e35 * this.scalar) + (other.scalar * this.e35);
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12);
        float4 wedge_g3 = float4(other.e45 * this.e23, other.e45 * this.e31, other.e45 * this.e12, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group1.zxyw * float4(other.group0.yzx, other.scalar)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[0] * other.scalar) + (wedge_g3[1] * other.e43), (wedge_g0[1] * other.scalar) + (wedge_g3[2] * other.e41), (wedge_g0[2] * other.scalar) + (wedge_g3[0] * other.e42), -(wedge_g2_x * other.e41) - (wedge_g2_y * other.e42) - (wedge_g2_z * other.e43) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) + float4((float2(wedge_g2_w) * other.group1.xy), wedge_g2_w * other.e12, wedge_g0[3] * other.scalar) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g1_w * other.e45), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1_xyz[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1_xyz[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + (other.group2 * float4(float3(wedge_g2_w), wedge_g1_w)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(other.e4 * this.e23, other.e4 * this.e31, other.e4 * this.e12, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) + float4((other.group0.xyz * this.group0.www), other.e4 * this.e3215);
        float4 wedge_g1 = float4(other.e4 * this.e15, other.e4 * this.e25, other.e4 * this.e35, -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], right_dual_g3[0] * wedge_g3[0]) + float4(float3(0.0), (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * other.e5 * this.scalar) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[2] * wedge_g3[2]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar * other.scalar) * this.group1.xyz), (other.scalar * other.scalar * this.e3215) + (other.e3215 * other.scalar * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_via_horizon_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g0[3] * other.e321) + (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual_g0[0] * wedge_g0_xyz[0]) - (right_dual_g0[1] * wedge_g0_xyz[1]) - (right_dual_g0[2] * wedge_g0_xyz[2])), 
            /* e15, e25, e35, e3215 */
            -float4((right_dual_g0 * float3(other.e23 * this.e15)), 0.0) - float4((right_dual_g0 * float3(other.e31 * this.e25)), 0.0) - float4((right_dual_g0 * float3(other.e12 * this.e35)), 0.0) - float4((right_dual_g0 * float3(other.e15 * this.e23)), 0.0) - float4((right_dual_g0 * float3(other.e25 * this.e31)), 0.0) - float4((right_dual_g0 * float3(other.e35 * this.e12)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar, other.scalar, other.scalar, (other.scalar * other.scalar * this.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)) * float4(wedge_g0_xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_via_horizon_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xyz.xy), wedge_g2_w * other.e125, -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xy), wedge_g2_w * other.e125, -(wedge_g2_w * other.e12345) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e15);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2_w * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2_w * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e25) + (wedge_g3[3] * other.e23), (wedge_g3[0] * other.e35) + (wedge_g3[3] * other.e31), (wedge_g3[1] * other.e15) + (wedge_g3[3] * other.e12), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.scalar * other.e4235) + (this.e35 * other.e42), (this.scalar * other.e4315) + (this.e15 * other.e43), (this.scalar * other.e4125) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group1.www, other.e3215)) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e15);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2_w * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2_w * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((wedge_g3.zxy * other.group2.yzx), wedge_g2_w * other.e3215) - (other.group2.zxyx * float4(wedge_g3.yzx, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiDualNum anti_reject_via_horizon_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * -1.0) * float2(other.e5 * other.e12345, other.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g1 = float4(other.e45, other.e45, other.e45, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) * float4(this.group0.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, other.e45 * other.e45 * this.scalar) * float4((wedge_g1.xyz * float3(-1.0)), 1.0) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar * other.e4235, this.scalar * other.e4315, this.scalar * other.e4125, -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) + (this.group0 * float4(other.group0.www, other.e3215));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.scalar)) * float4((wedge_g1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xyz.xy), wedge_g0_w * other.e435, -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xy), wedge_g0_w * other.e435, -(wedge_g0_w * other.e12345) - (wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(right_dual_g1_w)) + (float3(wedge_g0_w) * other.group1.xyz)), right_dual_g1_w * wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = this.scalar * other.e5;
        float4 wedge_g3 = float4(((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz)), this.scalar * other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group4.xyz;
        float3 wedge_g5 = (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz);
        float4 wedge_g6 = float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group6);
        float3 wedge_g7 = (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group0.xyz);
        float3 wedge_g8 = (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy);
        float4 wedge_g9 = float4(this.e35 * other.e42, this.e15 * other.e43, this.e25 * other.e41, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group3.www, other.e3215)) + float4((other.group9.xyz * this.group0.www), this.e3215 * other.scalar) - float4((other.group4.zxy * this.group1.yzx), this.e23 * other.e15);
        float wedge_g10 = (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) + (right_dual_g0[1] * this.scalar * other.scalar) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(other.e4) * float4(this.group0.xyz, this.e3215);
        float4 wedge_g1 = float4(other.e4, other.e4, other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) * float4(this.group1.xyz, 1.0);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[2] * wedge_g0[1] * -1.0, right_dual_g0[0] * wedge_g0[2] * -1.0, right_dual_g0[1] * wedge_g0[0] * -1.0, (right_dual_g0[2] * wedge_g3[2]) + (right_dual_g0[3] * wedge_g3[3]) + (right_dual_g1 * this.scalar * other.e5)) + (right_dual_g0.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4((float2(right_dual_g1) * wedge_g1.xy), right_dual_g1 * wedge_g1[2], right_dual_g0[0] * wedge_g3[0]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * wedge_g2_xyz[0]) + (right_dual_g0[3] * wedge_g0[0]), (right_dual_g1 * wedge_g2_xyz[1]) + (right_dual_g0[3] * wedge_g0[1]), (right_dual_g1 * wedge_g2_xyz[2]) + (right_dual_g0[3] * wedge_g0[2]), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (right_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * right_dual_g0.zxy) - (wedge_g2_xyz.zxy * right_dual_g0.yzx)), right_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_via_horizon_from(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_via_horizon_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = (this.group0 * float4(other.group3.www, other.e12345)) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((other.group0.xyz * this.group0.www), this.e3215 * other.e4);
        float4 wedge_g1 = float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], wedge_g0[3] * other.e12345 * -1.0) + float4(float3(0.0), (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * this.scalar * other.e5) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[1] * wedge_g3[1]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 wedge_g1_xyz = (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz);
        float wedge_g1_w = this.scalar * other.e45;
        float4 wedge_g2 = float4(this.e15 * other.scalar, this.e25 * other.scalar, this.e35 * other.scalar, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (float4(this.scalar) * other.group2);
        float4 wedge_g3 = float4(this.e35 * other.e42, this.e15 * other.e43, this.e25 * other.e41, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group1.www, other.e3215)) + float4((other.group3.xyz * this.group0.www), this.e3215 * other.scalar) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g0[3] * other.scalar) + (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (right_dual_g2_xyz[0] * wedge_g0[0]) - (right_dual_g2_xyz[1] * wedge_g0[1]) - (right_dual_g2_xyz[2] * wedge_g0[2]) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.xyz * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), wedge_g1_w * other.e45), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1_xyz[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1_xyz[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1_w * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4(other.e41 * this.e5, other.e42 * this.e5, other.e43 * this.e5, -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.scalar), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.scalar), 
            /* e235, e315, e125, e4 */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g2[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1[0] * other.e45) + (wedge_g1[3] * other.e23) + (wedge_g2[1] * other.e43) + (wedge_g3[0] * other.scalar), (wedge_g1[1] * other.e45) + (wedge_g1[3] * other.e31) + (wedge_g2[2] * other.e41) + (wedge_g3[1] * other.scalar), (wedge_g1[2] * other.e45) + (wedge_g1[3] * other.e12) + (wedge_g2[0] * other.e42) + (wedge_g3[2] * other.scalar), -(wedge_g1[2] * other.e35) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12)) + (other.group2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((other.group0.yzx * wedge_g2.zxy), wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy);
        float wedge_g1_w = other.e4 * this.e5 * -1.0;
        float4 wedge_g2 = float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3)) + (this.group0.xyzx * float4(other.group3.www, other.e423));
        float4 wedge_g3 = float4(other.e425 * this.e3, other.e435 * this.e1, other.e415 * this.e2, -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1_w), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1_w)) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public DualNum anti_reject_via_horizon_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e321 * other.e321 * this.e5) - (other.e235 * other.e321 * this.e1) - (other.e315 * other.e321 * this.e2) - (other.e125 * other.e321 * this.e3), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0_xyz = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group0.wwwx * float4(other.group1.xyz, other.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3] * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0_xyz[2] * other.e2 * -1.0, wedge_g0_xyz[0] * other.e3 * -1.0, wedge_g0_xyz[1] * other.e1 * -1.0, (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e321)) + (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0 * float3(wedge_g0[3])), -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23));
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(right_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[3] * wedge_g1[0], right_dual_g0[3] * wedge_g1[1], right_dual_g0[3] * wedge_g1[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (right_dual_g0 * float4(wedge_g0.www, wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 wedge_g1 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[2] * other.e2 * -1.0, wedge_g0[0] * other.e3 * -1.0, wedge_g0[1] * other.e1 * -1.0, (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)) + (other.group0.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group0.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + float4((other.group2.xyz * other.group0.xxx * this.group0.xxx), wedge_g0[1] * other.e431) + float4((other.group2.xyz * other.group0.yyy * this.group0.yyy), wedge_g0[2] * other.e412), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * this.e2 * other.e431) + (right_dual_g1_w * this.e3 * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group0.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + float4((other.group0.xxx * this.group0.xxx * other.group2.xyz), wedge_g0[1] * other.e431) + float4((other.group0.yyy * this.group0.yyy * other.group2.xyz), wedge_g0[2] * other.e412), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float3 wedge_g2 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e25) + (wedge_g2[1] * other.e43) + (wedge_g1[0] * other.e45) + (wedge_g1[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g2[2] * other.e41) + (wedge_g1[1] * other.e45) + (wedge_g1[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g2[0] * other.e42) + (wedge_g1[2] * other.e45) + (wedge_g1[3] * other.e12), -(wedge_g0[2] * other.e12) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e23) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * other.e31), 
            /* e5 */
            -(wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(this.e3 * other.e25 * -1.0, this.e1 * other.e35 * -1.0, this.e2 * other.e15 * -1.0, (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g2.www), (wedge_g2[3] * other.e1234) - (right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[3] * other.e4235), (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[3] * other.e4315), (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[3] * other.e4125), -(right_dual_g1[2] * wedge_g2[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (wedge_g2.yzxw * float4(right_dual_g0.zxy, other.e3215)) - (wedge_g2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(FlatPoint other) {
        float3 wedge_g0 = float3(other.e45 * -1.0) * this.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e45, other.e45, other.e45, -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) * float4(wedge_g0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(other.e45, other.e45, other.e45, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[3] * other.e4235, wedge_g0[3] * other.e4315, wedge_g0[3] * other.e4125, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_dual_g0.www, other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g1[1] * other.e435, wedge_g1[2] * other.e415, wedge_g1[0] * other.e425, -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) - (wedge_g1.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        float4 wedge_g1 = float4((float3(other.scalar) * this.group0.xyz), 0.0);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0);
        float3 wedge_g4 = float3(other.e4 * -1.0) * this.group0.xyz;
        float3 wedge_g5 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g6 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g9 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group6.zxy, other.e235)) - (this.group0.wwwy * float4(other.group7.xyz, other.e315));
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(other.e4235 * other.e4235 * this.e1) - (this.e2 * other.e4235 * other.e4315) - (this.e3 * other.e4235 * other.e4125), -(other.e4315 * other.e4315 * this.e2) - (this.e1 * other.e4235 * other.e4315) - (this.e3 * other.e4315 * other.e4125), -(other.e4125 * other.e4125 * this.e3) - (this.e1 * other.e4235 * other.e4125) - (this.e2 * other.e4315 * other.e4125), (this.e1 * other.e4235 * other.e3215) + (this.e2 * other.e4315 * other.e3215) + (this.e3 * other.e4125 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float right_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g1_xyz[1] * other.e3) + (wedge_g2[0] * other.e4), (wedge_g1_xyz[2] * other.e1) + (wedge_g2[1] * other.e4), (wedge_g1_xyz[0] * other.e2) + (wedge_g2[2] * other.e4), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) + float4((float2(right_dual_g0_w) * wedge_g0.xy), right_dual_g0_w * wedge_g0[2], wedge_g1_w * other.e4) - (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e5 */
            (right_dual_g0_w * wedge_g1_w) + (wedge_g2[0] * other.e1) + (wedge_g2[1] * other.e2) + (wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_via_horizon_from(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float4 wedge_g2 = float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412)) + (this.group0.xyzx * float4(other.group2.www, other.e423));
        float4 wedge_g3 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1_w), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1_w)) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiPlane {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.e2 * other.e43 * -1.0, this.e3 * other.e41 * -1.0, this.e1 * other.e42 * -1.0, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235));
        float4 wedge_g1 = float4(this.e5 * other.e41, this.e5 * other.e42, this.e5 * other.e43, -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float3 wedge_g3_xyz = float3(other.scalar) * this.group0.xyz;
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2]) - (right_dual_g2[0] * wedge_g1[0]) - (right_dual_g2[1] * wedge_g1[1]) - (right_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g3_xyz[0] * other.scalar) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (right_dual_g2[1] * wedge_g0[2]) + (wedge_g2[1] * other.e43), (wedge_g3_xyz[1] * other.scalar) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (right_dual_g2[2] * wedge_g0[0]) + (wedge_g2[2] * other.e41), (wedge_g3_xyz[2] * other.scalar) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (right_dual_g2[0] * wedge_g0[1]) + (wedge_g2[0] * other.e42), -(right_dual_g1[2] * wedge_g0[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (wedge_g0.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])) - float4((wedge_g2.zxy * other.group0.yzx), right_dual_g1[1] * wedge_g0[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_reject_via_horizon_from(AntiCircleRotor other) {
        float wedge_g0 = other.scalar * this.e12345;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public DualNum anti_reject_via_horizon_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * float2(other.e3215 * other.scalar, other.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public Motor anti_reject_via_horizon_from(AntiMotor other) {
        float wedge_g0 = other.scalar * this.e12345;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0 = this.e12345 * other.scalar;
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group9.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            wedge_g0 * other.e4 * -1.0
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float wedge_g0 = this.e12345 * other.scalar;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Circle> operator /(Circle lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(wedge_g2_w)) + (wedge_g0 * float3(other.scalar))), wedge_g2_w * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e15) + (wedge_g2_xyz[0] * other.scalar), (wedge_g2_w * other.e25) + (wedge_g2_xyz[1] * other.scalar), (wedge_g2_w * other.e35) + (wedge_g2_xyz[2] * other.scalar), -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g2_xyz[1]) + (wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2_xyz[2]) + (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2_xyz[0]) + (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), right_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + (other.group3.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group3.yzx), right_dual_g3_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[0] * other.e423) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0.xy), right_dual_g3_w * wedge_g0[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0[0] * other.e425), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyw * float4(other.group1.yzx, right_dual_g1_w)) - (wedge_g0.wwwx * float4(other.group0.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Circle anti_reject_via_horizon_from(AntiDualNum other) {
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
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + (other.group1.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g0_w = other.e321 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), right_dual_g1_w * wedge_g1), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g0_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((float2(wedge_g1) * other.group0.xy), wedge_g1 * other.e125, -(right_dual_g0_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Line anti_reject_via_horizon_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((this.group2.xyz * other.group0.www), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            float4(wedge_g2[3] * other.e15, wedge_g2[3] * other.e25, wedge_g2[3] * other.e35, -(wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12)) + (wedge_g2 * float4(other.group0.www, other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e25) + (wedge_g1[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g1[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g1[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Circle anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g0_w = other.e5 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), right_dual_g0_w * wedge_g1), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Circle anti_reject_via_horizon_from(Dipole other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_reject_via_horizon_from(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_reject_via_horizon_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((other.e15 * other.e15 * this.e423) + (this.e431 * other.e15 * other.e25) + (this.e412 * other.e15 * other.e35) + (this.e321 * other.e15 * other.e45), (other.e25 * other.e25 * this.e431) + (this.e423 * other.e15 * other.e25) + (this.e412 * other.e25 * other.e35) + (this.e321 * other.e25 * other.e45), (other.e35 * other.e35 * this.e412) + (this.e423 * other.e15 * other.e35) + (this.e431 * other.e25 * other.e35) + (this.e321 * other.e35 * other.e45), -(other.e45 * other.e45 * this.e321) - (this.e423 * other.e15 * other.e45) - (this.e431 * other.e25 * other.e45) - (this.e412 * other.e35 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * wedge_g0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4(other.scalar) * this.group1;
        float3 wedge_g7 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g8 = float3(other.scalar) * this.group2.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g10 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[1] * wedge_g6[1]) - (right_dual_g7[2] * wedge_g6[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[0] * wedge_g6[0]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g6_xyz[2] * wedge_g7[2]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Circle anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group0.yzx), right_dual_g0_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public Circle anti_reject_via_horizon_from(Scalar other) {
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
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group3.yzx), right_dual_g3_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[0] * other.e423) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0.xy), right_dual_g3_w * wedge_g0[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0[0] * other.e425), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyw * float4(other.group1.yzx, right_dual_g1_w)) - (wedge_g0.wwwx * float4(other.group0.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) * float4(this.group2.xyz, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.scalar)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(wedge_g2[0] * other.scalar, wedge_g2[1] * other.scalar, wedge_g2[2] * other.scalar, -(right_dual_g2_xyz[0] * wedge_g1[0]) - (right_dual_g2_xyz[1] * wedge_g1[1]) - (right_dual_g2_xyz[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) + (float4(wedge_g2[3]) * float4(right_dual_g2_xyz, other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g3_xyz[0] * wedge_g2[3]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[1] * other.e43), (right_dual_g3_xyz[1] * wedge_g2[3]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[2] * other.e41), (right_dual_g3_xyz[2] * wedge_g2[3]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[0] * other.e42), -(wedge_g0[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((right_dual_g2_xyz.yzx * wedge_g0.zxy), wedge_g2[3] * other.e1234) - float4((right_dual_g2_xyz.zxy * wedge_g0.yzx), wedge_g0[0] * right_dual_g1[0]) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * right_dual_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(wedge_g2_w)) + (wedge_g0 * float3(other.scalar))), wedge_g2_w * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e15) + (wedge_g2_xyz[0] * other.scalar), (wedge_g2_w * other.e25) + (wedge_g2_xyz[1] * other.scalar), (wedge_g2_w * other.e35) + (wedge_g2_xyz[2] * other.scalar), -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g2_xyz[1]) + (wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2_xyz[2]) + (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2_xyz[0]) + (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), right_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + (other.group3.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group3.yzx), right_dual_g3_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[0] * other.e423) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0.xy), right_dual_g3_w * wedge_g0[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0[0] * other.e425), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyw * float4(other.group1.yzx, right_dual_g1_w)) - (wedge_g0.wwwx * float4(other.group0.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(other.scalar * other.scalar) * this.group0.xyz), wedge_g2[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            wedge_g2 * float4(other.group0.yy, other.scalar, other.e3215), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + (other.group1.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g0_w = other.e321 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), right_dual_g1_w * wedge_g1), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g0_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((float2(wedge_g1) * other.group0.xy), wedge_g1 * other.e125, -(right_dual_g0_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public Line anti_reject_via_horizon_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((this.group2.xyz * other.group0.www), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            float4(wedge_g2[3] * other.e15, wedge_g2[3] * other.e25, wedge_g2[3] * other.e35, -(wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12)) + (wedge_g2 * float4(other.group0.www, other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e25) + (wedge_g1[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g1[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g1[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315));
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g0_w = other.e5 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), right_dual_g0_w * wedge_g1), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_via_horizon_from(Dipole other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint anti_reject_via_horizon_from(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint anti_reject_via_horizon_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((other.e15 * other.e15 * this.e423) + (this.e431 * other.e15 * other.e25) + (this.e412 * other.e15 * other.e35) + (this.e321 * other.e15 * other.e45), (other.e25 * other.e25 * this.e431) + (this.e423 * other.e15 * other.e25) + (this.e412 * other.e25 * other.e35) + (this.e321 * other.e25 * other.e45), (other.e35 * other.e35 * this.e412) + (this.e423 * other.e15 * other.e35) + (this.e431 * other.e25 * other.e35) + (this.e321 * other.e35 * other.e45), -(other.e45 * other.e45 * this.e321) - (this.e423 * other.e15 * other.e45) - (this.e431 * other.e25 * other.e45) - (this.e412 * other.e35 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * wedge_g0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4(other.scalar) * this.group1;
        float3 wedge_g7 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g8 = float3(other.scalar) * this.group2.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g10 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[1] * wedge_g6[1]) - (right_dual_g7[2] * wedge_g6[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[0] * wedge_g6[0]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g6_xyz[2] * wedge_g7[2]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group0.yzx), right_dual_g0_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_reject_via_horizon_from(Scalar other) {
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
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.zxy * other.group3.yzx), right_dual_g3_w * wedge_g1) - (wedge_g0.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[0] * other.e423) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + float4((float2(right_dual_g3_w) * wedge_g0.xy), right_dual_g3_w * wedge_g0[2], right_dual_g1_w * wedge_g1), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * other.e235) + (wedge_g0[1] * other.e435), (wedge_g1 * other.e315) + (wedge_g0[2] * other.e415), (wedge_g1 * other.e125) + (wedge_g0[0] * other.e425), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyw * float4(other.group1.yzx, right_dual_g1_w)) - (wedge_g0.wwwx * float4(other.group0.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) * float4(this.group2.xyz, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.scalar)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(wedge_g2[0] * other.scalar, wedge_g2[1] * other.scalar, wedge_g2[2] * other.scalar, -(right_dual_g2_xyz[0] * wedge_g1[0]) - (right_dual_g2_xyz[1] * wedge_g1[1]) - (right_dual_g2_xyz[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) + (float4(wedge_g2[3]) * float4(right_dual_g2_xyz, other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g3_xyz[0] * wedge_g2[3]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[1] * other.e43), (right_dual_g3_xyz[1] * wedge_g2[3]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[2] * other.e41), (right_dual_g3_xyz[2] * wedge_g2[3]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[0] * other.e42), -(wedge_g0[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((right_dual_g2_xyz.yzx * wedge_g0.zxy), wedge_g2[3] * other.e1234) - float4((right_dual_g2_xyz.zxy * wedge_g0.yzx), wedge_g0[0] * right_dual_g1[0]) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * right_dual_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Dipole> operator /(Dipole lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e23 * this.e15) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e23) + (wedge_g0[0] * other.scalar) + (wedge_g3[1] * other.e43), (wedge_g2_w * other.e31) + (wedge_g0[1] * other.scalar) + (wedge_g3[2] * other.e41), (wedge_g2_w * other.e12) + (wedge_g0[2] * other.scalar) + (wedge_g3[0] * other.e42), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.scalar)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((this.group2.yzx * other.group3.zxy), other.e423 * this.e15);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(AntiDualNum other) {
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
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public FlatPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(other.e235 * other.e235 * this.e41) - (other.e235 * other.e315 * this.e42) - (other.e235 * other.e125 * this.e43) - (other.e235 * other.e321 * this.e45), -(other.e315 * other.e315 * this.e42) - (other.e235 * other.e315 * this.e41) - (other.e315 * other.e125 * this.e43) - (other.e315 * other.e321 * this.e45), -(other.e125 * other.e125 * this.e43) - (other.e235 * other.e125 * this.e41) - (other.e315 * other.e125 * this.e42) - (other.e125 * other.e321 * this.e45), (other.e321 * other.e321 * this.e45) + (other.e235 * other.e321 * this.e41) + (other.e315 * other.e321 * this.e42) + (other.e125 * other.e321 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g1 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((this.group2.yzx * other.group1.zxy), other.e235 * this.e41);
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[2] * right_dual_g1[1], wedge_g0[0] * right_dual_g1[2], wedge_g0[1] * right_dual_g1[0], -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - float4((wedge_g0.yzx * right_dual_g1.zxy), right_dual_g0_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])) + float4((wedge_g0 * right_dual_g1.www), right_dual_g0_w * wedge_g2[3]) - (right_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((right_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float wedge_g1 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_dual_g0 * float3(wedge_g1), 
            /* e23, e31, e12, e45 */
            float4((float2(wedge_g1) * other.group1.xyz.xy), wedge_g1 * other.e35, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35 */
            (right_dual_g0 * float3(wedge_g0[3])) + (other.group1.yzx * wedge_g0.zxy) - (other.group1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float wedge_g2_w = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((this.group0.zxy * other.group1.yzx), other.e23 * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[0] * other.scalar, wedge_g0[1] * other.scalar, wedge_g0[2] * other.scalar, -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) + (float4(wedge_g2_w) * float4(other.group0.xyz, other.e3215)), 
            /* e23, e31, e12, e45 */
            float4(wedge_g1[0] * other.scalar, wedge_g1[1] * other.scalar, wedge_g1[2] * other.scalar, -(wedge_g3[0] * other.e23) - (wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((float2(wedge_g2_w) * other.group1.xy), wedge_g2_w * other.e35, wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.scalar * other.scalar) * this.group2.xyz) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float3 wedge_g2 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float right_dual_g0_w = other.e5 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0_w * wedge_g0[0], right_dual_g0_w * wedge_g0[1], right_dual_g0_w * wedge_g0[2], -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_dual_g0_w) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(Circle other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e23 * this.e15) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float wedge_g1 = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0[1] * other.e35 * -1.0, wedge_g0[2] * other.e15 * -1.0, wedge_g0[0] * other.e25 * -1.0, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e1234)) + (wedge_g0.wwwx * float4(other.group1.xyz, other.e4235)) + float4((wedge_g0.zxy * other.group2.yzx), wedge_g1 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public FlatPoint anti_reject_via_horizon_from(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiLine anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * wedge_g0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4((right_dual_g0.www * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * wedge_g0.zxy) - (right_dual_g0.zxy * wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiLine anti_reject_via_horizon_from(Line other) {
        float wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.e5, other.e5, other.e5, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) * float4(this.group0.xyz, 1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g5 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g6 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - (other.group3.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group4.zxy), this.e31 * other.e25);
        float wedge_g10 = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float3 wedge_g2 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_w * wedge_g0[0]) + (right_dual_g1 * wedge_g2[0]), (right_dual_g0_w * wedge_g0[1]) + (right_dual_g1 * wedge_g2[1]), (right_dual_g0_w * wedge_g0[2]) + (right_dual_g1 * wedge_g2[2]), -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_dual_g0_w) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_via_horizon_from(Scalar other) {
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
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx);
        float4 wedge_g2 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) * float4(this.group2.xyz, 1.0);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (right_dual_g2_xyz[1] * wedge_g0[1]) - (right_dual_g2_xyz[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0 * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e23) + (wedge_g0[0] * other.scalar) + (wedge_g3[1] * other.e43), (wedge_g2_w * other.e31) + (wedge_g0[1] * other.scalar) + (wedge_g3[2] * other.e41), (wedge_g2_w * other.e12) + (wedge_g0[2] * other.scalar) + (wedge_g3[0] * other.e42), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.scalar)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.yzx * this.group2.zxy), other.e4 * this.e3215) - (this.group2.yzxx * float4(other.group3.zxy, other.e423));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(other.scalar * other.scalar) * this.group0.xyz), wedge_g2[3] * other.e3215), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public FlatPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(other.e235 * other.e235 * this.e41) - (other.e235 * other.e315 * this.e42) - (other.e235 * other.e125 * this.e43) - (other.e235 * other.e321 * this.e45), -(other.e315 * other.e315 * this.e42) - (other.e235 * other.e315 * this.e41) - (other.e315 * other.e125 * this.e43) - (other.e315 * other.e321 * this.e45), -(other.e125 * other.e125 * this.e43) - (other.e235 * other.e125 * this.e41) - (other.e315 * other.e125 * this.e42) - (other.e125 * other.e321 * this.e45), (other.e321 * other.e321 * this.e45) + (other.e235 * other.e321 * this.e41) + (other.e315 * other.e321 * this.e42) + (other.e125 * other.e321 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g1 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group1.zxy * this.group2.yzx), other.e235 * this.e41);
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[2] * right_dual_g1[1], wedge_g0[0] * right_dual_g1[2], wedge_g0[1] * right_dual_g1[0], -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - float4((wedge_g0.yzx * right_dual_g1.zxy), right_dual_g0_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])) + float4((wedge_g0 * right_dual_g1.www), right_dual_g0_w * wedge_g2[3]) - (right_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((right_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float wedge_g1 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_dual_g0 * float3(wedge_g1), 
            /* e23, e31, e12, e45 */
            float4((float2(wedge_g1) * other.group1.xyz.xy), wedge_g1 * other.e35, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35 */
            (right_dual_g0 * float3(wedge_g0[3])) + (other.group1.yzx * wedge_g0.zxy) - (other.group1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float wedge_g2_w = (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.scalar * this.e4235) + (other.e35 * this.e42), (other.scalar * this.e4315) + (other.e15 * this.e43), (other.scalar * this.e4125) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group1.www, this.e3215)) - float4((this.group0.zxy * other.group1.yzx), other.e23 * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[0] * other.scalar, wedge_g0[1] * other.scalar, wedge_g0[2] * other.scalar, -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) + (float4(wedge_g2_w) * float4(other.group0.xyz, other.e3215)), 
            /* e23, e31, e12, e45 */
            float4(wedge_g1[0] * other.scalar, wedge_g1[1] * other.scalar, wedge_g1[2] * other.scalar, -(wedge_g3[0] * other.e23) - (wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((float2(wedge_g2_w) * other.group1.xy), wedge_g2_w * other.e35, wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.scalar * other.scalar) * this.group2.xyz) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4(other.e3 * this.e25 * -1.0, other.e1 * this.e35 * -1.0, other.e2 * this.e15 * -1.0, (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (wedge_g0.zxy * right_dual_g0.yzx) - (wedge_g0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[0] * right_dual_g0[3], wedge_g0[1] * right_dual_g0[3], wedge_g0[2] * right_dual_g0[3], -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (right_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * wedge_g1.xyz), 0.0) + float4((right_dual_g0.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g0.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_via_horizon_from(Circle other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0[1] * other.e35 * -1.0, wedge_g0[2] * other.e15 * -1.0, wedge_g0[0] * other.e25 * -1.0, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e1234)) + (wedge_g0.wwwx * float4(other.group1.xyz, other.e4235)) + float4((wedge_g0.zxy * other.group2.yzx), wedge_g1 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_via_horizon_from(DualNum other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e1234);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.e12345), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiLine anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * wedge_g0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4((right_dual_g0.www * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * wedge_g0.zxy) - (right_dual_g0.zxy * wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiLine anti_reject_via_horizon_from(Line other) {
        float wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.e5, other.e5, other.e5, (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) * float4(this.group0.xyz, 1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g5 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g6 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.e4235 * other.scalar), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.e4315 * other.scalar), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.e4125 * other.scalar), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group3.zxy), this.e3215 * other.scalar) - (other.group3.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group4.zxy * this.group2.yzx), this.e31 * other.e25);
        float wedge_g10 = (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float4 wedge_g2 = float4(this.e25 * other.e3 * -1.0, this.e35 * other.e1 * -1.0, this.e15 * other.e2 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * right_dual_g0.yzx) - (wedge_g0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * wedge_g2[0]) + (wedge_g0[0] * right_dual_g0[3]), (right_dual_g1 * wedge_g2[1]) + (wedge_g0[1] * right_dual_g0[3]), (right_dual_g1 * wedge_g2[2]) + (wedge_g0[2] * right_dual_g0[3]), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (right_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (right_dual_g0.zxy * wedge_g2.yzx) - (right_dual_g0.yzx * wedge_g2.zxy)), right_dual_g1 * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_via_horizon_from(Scalar other) {
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
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx);
        float4 wedge_g2 = (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g1[0]) + (wedge_g0[2] * right_dual_g3[1]) + (wedge_g2[3] * other.e423), (right_dual_g2_w * wedge_g1[1]) + (wedge_g0[0] * right_dual_g3[2]) + (wedge_g2[3] * other.e431), (right_dual_g2_w * wedge_g1[2]) + (wedge_g0[1] * right_dual_g3[0]) + (wedge_g2[3] * other.e412), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345)) - float4((wedge_g0.yzx * right_dual_g3.zxy), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_dual_g3[3]) + (wedge_g2[3] * other.e415), (wedge_g0[1] * right_dual_g3[3]) + (wedge_g2[3] * other.e425), (wedge_g0[2] * right_dual_g3[3]) + (wedge_g2[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + (wedge_g2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4(other.scalar, other.scalar, other.scalar, (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) * float4(this.group2.xyz, 1.0);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.e4235 * other.scalar), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.e4315 * other.scalar), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.e4125 * other.scalar), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (right_dual_g2_xyz[1] * wedge_g0[1]) - (right_dual_g2_xyz[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0 * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<DualNum> operator /(DualNum lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.group0.xx, this.e5, other.scalar * this.e12345) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, other.scalar);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * float4(other.group0.xyz, right_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g2[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4(right_dual_g2[3] * wedge_g1[0], right_dual_g2[3] * wedge_g1[1], right_dual_g2[3] * wedge_g1[2], -(right_dual_g2[0] * wedge_g0[0]) - (right_dual_g2[1] * wedge_g0[1]) - (right_dual_g2[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) + (right_dual_g2 * float4(wedge_g0.www, wedge_g1[3])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * other.e45) + (wedge_g1[1] * other.e43), (wedge_g0[1] * other.e45) + (wedge_g1[2] * other.e41), (wedge_g0[2] * other.e45) + (wedge_g1[0] * other.e42), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - float4((other.group0.yzx * wedge_g1.zxy), wedge_g0[0] * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group3.xyz, other.e4);
        float4 wedge_g1 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.zxy * other.group3.yzx) - (wedge_g1.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((other.group3.xyz * wedge_g1.www), right_dual_g2_w * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g1[3] * other.e321)) + float4((wedge_g1.yzx * other.group1.zxy), right_dual_g3_w * wedge_g0[3]) - (wedge_g1.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g0.xy), right_dual_g2_w * wedge_g0[2], wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(AntiDualNum other) {
        float2 wedge_g0 = float2(other.scalar) * this.group0.xy;
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0[0] * other.scalar) + (wedge_g0[1] * other.e3215), wedge_g0[1] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * other.e321 * this.e5, 0.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public AntiFlector anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0_xyz = float3(this.e5 * -1.0) * other.group1.xyz;
        float wedge_g1_w = other.e321 * this.e5 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1_w * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g1_w * other.e321) + (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_via_horizon_from(AntiMotor other) {
        float wedge_g0_w = other.scalar * this.e12345;
        float4 wedge_g1 = float4(this.e5) * other.group0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(wedge_g0_w) * other.group0, 
            /* e235, e315, e125, e5 */
            (float4(wedge_g0_w) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0_xyz = float3(this.e5 * -1.0) * other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)), 
            /* e5 */
            (wedge_g0[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)), 
            /* e5 */
            (wedge_g0[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.e5) * other.group0.xyz;
        float3 wedge_g1 = float3(this.e5) * other.group1.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * other.e45) + (wedge_g1[1] * other.e43), (wedge_g0[1] * other.e45) + (wedge_g1[2] * other.e41), (wedge_g0[2] * other.e45) + (wedge_g1[0] * other.e42), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - float4((wedge_g1.zxy * other.group0.yzx), wedge_g0[0] * other.e41), 
            /* e5 */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e1234);
        float3 wedge_g1_xyz = float3(this.e5) * other.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g0[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g0.www), (wedge_g0[3] * other.e1234) - (right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[3] * wedge_g0[0]) + (wedge_g0[3] * other.e4235), (right_dual_g1[3] * wedge_g0[1]) + (wedge_g0[3] * other.e4315), (right_dual_g1[3] * wedge_g0[2]) + (wedge_g0[3] * other.e4125), -(wedge_g1_xyz[1] * right_dual_g1[1]) - (wedge_g1_xyz[2] * right_dual_g1[2]) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) + float4((right_dual_g0.zxy * wedge_g1_xyz.yzx), wedge_g0[3] * other.e3215) - float4((right_dual_g0.yzx * wedge_g1_xyz.zxy), wedge_g1_xyz[0] * right_dual_g1[0])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e5 * other.e1234) + (this.e12345 * other.scalar);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(this.e5 * -1.0) * other.group1;
        float4 wedge_g6 = float4((float3(this.e5) * other.group4.xyz), 0.0);
        float3 wedge_g8 = float3(this.e5) * other.group5.xyz;
        float4 wedge_g9 = float4(this.e5 * -1.0) * float4(other.group7.xyz, other.e321);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (right_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(wedge_g9[1] * other.e435, wedge_g9[2] * other.e415, wedge_g9[0] * other.e425, 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[0] * other.e423) + float4((right_dual_g6.xyz * wedge_g6.www), wedge_g9[1] * other.e431) + float4((wedge_g6.xyz * right_dual_g6.www), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[2] * wedge_g6[2]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g7[0] * wedge_g6[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g6[0]) - (wedge_g8[1] * right_dual_g6[1]) - (wedge_g8[2] * right_dual_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g6[0] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g6[1] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g6[2] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_dual_g0[1])) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * other.group0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            wedge_g0 * float4(other.e4), 
            /* e5 */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) - (wedge_g0[3] * other.e5)
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
            /* e5, e12345 */
            float2(other.scalar * other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = this.e5 * other.e1234;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * other.group3;
        float4 wedge_g1 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.zxy * other.group3.yzx) - (wedge_g1.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((other.group3.xyz * wedge_g1.www), right_dual_g2_w * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g1[3] * other.e321)) + float4((wedge_g1.yzx * other.group1.zxy), right_dual_g3_w * wedge_g0[3]) - (wedge_g1.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g0.xy), right_dual_g2_w * wedge_g0[2], wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.group0.xx, this.e5, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, other.scalar);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g2[0] * wedge_g0[0]) - (right_dual_g2[1] * wedge_g0[1]) - (right_dual_g2[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g1[1]) + (wedge_g0[0] * other.e45), (right_dual_g0[0] * wedge_g1[2]) + (wedge_g0[1] * other.e45), (right_dual_g0[1] * wedge_g1[0]) + (wedge_g0[2] * other.e45), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (right_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[2] * wedge_g1[1], right_dual_g0[0] * wedge_g1[2], right_dual_g0[1] * wedge_g1[0], -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (wedge_g0[3] * other.e45)) - float4((right_dual_g0.yzx * wedge_g1.zxy), right_dual_g0[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) + float4((right_dual_g0 * wedge_g1.www), wedge_g0[3] * other.scalar) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.scalar) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(other.e4 * this.e15, other.e4 * this.e25, other.e4 * this.e35, -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g0[0]) + (wedge_g0[3] * other.e423), (right_dual_g2_w * wedge_g0[1]) + (wedge_g0[3] * other.e431), (right_dual_g2_w * wedge_g0[2]) + (wedge_g0[3] * other.e412), -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_w * wedge_g1_xyz[0]) + (wedge_g0[3] * other.e415), (right_dual_g2_w * wedge_g1_xyz[1]) + (wedge_g0[3] * other.e425), (right_dual_g2_w * wedge_g1_xyz[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[0] * wedge_g0[0]) - (right_dual_g3[1] * wedge_g0[1]) - (right_dual_g3[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g1_xyz.yzx * right_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_via_horizon_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e45) * float4(other.e235 * other.e321, other.e315 * other.e321, other.e125 * other.e321, other.e321 * other.e321) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public Flector anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.e45 * -1.0) * float4(other.group1.xyz, other.e321);
        float3 wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1_xyz[1] * right_dual_g1[2]) + (right_dual_g1[3] * wedge_g0[0]) + (wedge_g0[3] * other.e235), (wedge_g1_xyz[2] * right_dual_g1[0]) + (right_dual_g1[3] * wedge_g0[1]) + (wedge_g0[3] * other.e315), (wedge_g1_xyz[0] * right_dual_g1[1]) + (right_dual_g1[3] * wedge_g0[2]) + (wedge_g0[3] * other.e125), -(right_dual_g1[1] * wedge_g0[1]) - (right_dual_g1[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)) - (right_dual_g1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4(this.e45, this.e45, this.e45, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) * float4(other.group0.xyz, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[2] * other.e25) + (wedge_g0[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g0[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g0[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public Flector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(this.e45, this.e45, this.e45, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) * float4(other.group0.xyz, 1.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[3] * wedge_g0[0]) + (wedge_g1[2] * other.e25), (right_dual_g0[3] * wedge_g0[1]) + (wedge_g1[0] * other.e35), (right_dual_g0[3] * wedge_g0[2]) + (wedge_g1[1] * other.e15), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) + (right_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) - (wedge_g1.yzxx * float4(other.group1.zxy, right_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e45 * -1.0) * other.group0.xyz;
        float3 wedge_g1 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float right_dual_g0_w = other.e5 * -1.0;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0_w * wedge_g0[0]) + (wedge_g1[1] * other.e3), (right_dual_g0_w * wedge_g0[1]) + (wedge_g1[2] * other.e1), (right_dual_g0_w * wedge_g0[2]) + (wedge_g1[0] * other.e2), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_via_horizon_from(Circle other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (right_dual_g2.yzx * wedge_g0.zxy) - (right_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0[1] * other.e35 * -1.0, wedge_g0[2] * other.e15 * -1.0, wedge_g0[0] * other.e25 * -1.0, (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e1234)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e4235)) + (wedge_g0.wwwy * float4(other.group1.xyz, other.e4315))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0);
        float3 wedge_g8 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group4.zxy, other.e23));
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (right_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(wedge_g9[1] * other.e435, wedge_g9[2] * other.e415, wedge_g9[0] * other.e425, 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[0] * other.e423) + float4((right_dual_g6.xyz * wedge_g6.www), wedge_g9[1] * other.e431) + float4((wedge_g6.xyz * right_dual_g6.www), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[2] * wedge_g6[2]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g7[0] * wedge_g6[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g6[0]) - (wedge_g8[1] * right_dual_g6[1]) - (wedge_g8[2] * right_dual_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g6[0] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g6[1] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g6[2] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_dual_g0[1])) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group0.xyz);
        float3 wedge_g1 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float right_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            wedge_g0 * float3(right_dual_g1), 
            /* e23, e31, e12, e45 */
            float4((float2(right_dual_g1) * wedge_g1.xy), right_dual_g1 * wedge_g1[2], -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)), 
            /* e15, e25, e35 */
            (wedge_g0 * float3(other.e5 * -1.0)) + (wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_via_horizon_from(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(this.e15 * other.e4, this.e25 * other.e4, this.e35 * other.e4, -(this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g0[0]) + (wedge_g0[3] * other.e423), (right_dual_g2_w * wedge_g0[1]) + (wedge_g0[3] * other.e431), (right_dual_g2_w * wedge_g0[2]) + (wedge_g0[3] * other.e412), -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_w * wedge_g1_xyz[0]) + (wedge_g0[3] * other.e415), (right_dual_g2_w * wedge_g1_xyz[1]) + (wedge_g0[3] * other.e425), (right_dual_g2_w * wedge_g1_xyz[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[0] * wedge_g0[0]) - (right_dual_g3[1] * wedge_g0[1]) - (right_dual_g3[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g1_xyz.yzx * right_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(right_dual_g0.zxy, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g3_xyz[1] * wedge_g1[1]) + (right_dual_g3_xyz[2] * wedge_g1[2]) + (wedge_g1[3] * other.e1234) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (wedge_g0[3] * other.e45)) - (right_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (right_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((right_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Flector> operator /(Flector lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((other.group0.yzx * this.group0.zxy), other.scalar * this.e3215) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[2] * wedge_g1[1], right_dual_g0[0] * wedge_g1[2], right_dual_g0[1] * wedge_g1[0], -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (wedge_g0[3] * other.e45)) - float4((right_dual_g0.yzx * wedge_g1.zxy), right_dual_g0[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) + float4((right_dual_g0 * wedge_g1.www), wedge_g0[3] * other.scalar) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.scalar) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g0[0]) + (wedge_g0[3] * other.e423), (right_dual_g2_w * wedge_g0[1]) + (wedge_g0[3] * other.e431), (right_dual_g2_w * wedge_g0[2]) + (wedge_g0[3] * other.e412), -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_w * wedge_g1_xyz[0]) + (wedge_g0[3] * other.e415), (right_dual_g2_w * wedge_g1_xyz[1]) + (wedge_g0[3] * other.e425), (right_dual_g2_w * wedge_g1_xyz[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[0] * wedge_g0[0]) - (right_dual_g3[1] * wedge_g0[1]) - (right_dual_g3[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g1_xyz.yzx * right_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public FlatPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e45) * float4(other.e235 * other.e321, other.e315 * other.e321, other.e125 * other.e321, other.e321 * other.e321) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.e45, this.e45, this.e45, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)) * float4((other.group1.xyz * float3(-1.0)), 1.0);
        float3 wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1_xyz[1] * right_dual_g1[2]) + (right_dual_g1[3] * wedge_g0[0]) + (wedge_g0[3] * other.e235), (wedge_g1_xyz[2] * right_dual_g1[0]) + (right_dual_g1[3] * wedge_g0[1]) + (wedge_g0[3] * other.e315), (wedge_g1_xyz[0] * right_dual_g1[1]) + (right_dual_g1[3] * wedge_g0[2]) + (wedge_g0[3] * other.e125), -(right_dual_g1[1] * wedge_g0[1]) - (right_dual_g1[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)) - (right_dual_g1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public FlatPoint anti_reject_via_horizon_from(AntiLine other) {
        float4 wedge_g0 = float4(this.e45, this.e45, this.e45, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) * float4(other.group0.xyz, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[2] * other.e25) + (wedge_g0[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g0[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g0[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) + (other.group0 * float4(this.group0.www, this.e3215));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[3] * wedge_g0[0]) + (wedge_g1[2] * other.e25), (right_dual_g0[3] * wedge_g0[1]) + (wedge_g1[0] * other.e35), (right_dual_g0[3] * wedge_g0[2]) + (wedge_g1[1] * other.e15), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) + (right_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) - (wedge_g1.yzxx * float4(other.group1.zxy, right_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(this.e45, this.e45, this.e45, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)) * float4((other.group0.xyz * float3(-1.0)), 1.0);
        float3 wedge_g1_xyz = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1_xyz[1] * right_dual_g0[2]) + (right_dual_g0[3] * wedge_g0[0]), (wedge_g1_xyz[2] * right_dual_g0[0]) + (right_dual_g0[3] * wedge_g0[1]), (wedge_g1_xyz[0] * right_dual_g0[1]) + (right_dual_g0[3] * wedge_g0[2]), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) - (right_dual_g0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Dipole anti_reject_via_horizon_from(Circle other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Dipole anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (right_dual_g2.yzx * wedge_g0.zxy) - (right_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(wedge_g0[1] * other.e35 * -1.0, wedge_g0[2] * other.e15 * -1.0, wedge_g0[0] * other.e25 * -1.0, (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e1234)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e4235)) + (wedge_g0.wwwy * float4(other.group1.xyz, other.e4315))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0);
        float3 wedge_g8 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e45 * other.e23) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e4125 * other.scalar), -(this.e25 * other.e31) - (this.e35 * other.e12)) + float4((other.group4.yzx * this.group0.zxy), this.e3215 * other.scalar) - (this.group0.yzxx * float4(other.group4.zxy, other.e23));
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (right_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(wedge_g9[1] * other.e435, wedge_g9[2] * other.e415, wedge_g9[0] * other.e425, 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[0] * other.e423) + float4((right_dual_g6.xyz * wedge_g6.www), wedge_g9[1] * other.e431) + float4((wedge_g6.xyz * right_dual_g6.www), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[2] * wedge_g6[2]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g7[0] * wedge_g6[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g6[0]) - (wedge_g8[1] * right_dual_g6[1]) - (wedge_g8[2] * right_dual_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g6[0] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g6[1] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g6[2] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_dual_g0[1])) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public DipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.e45 * other.e1 * -1.0, this.e45 * other.e2 * -1.0, this.e45 * other.e3 * -1.0, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) + (other.group0.wwwx * float4(this.group0.xyz, this.e4235));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g1) * wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float2(right_dual_g1) * wedge_g1_xyz.xy), right_dual_g1 * wedge_g1_xyz[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * right_dual_g0.zxy) - (wedge_g1_xyz.zxy * right_dual_g0.yzx)), right_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0[3])
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
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = (other.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2_w * wedge_g0[0]) + (wedge_g0[3] * other.e423), (right_dual_g2_w * wedge_g0[1]) + (wedge_g0[3] * other.e431), (right_dual_g2_w * wedge_g0[2]) + (wedge_g0[3] * other.e412), -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_w * wedge_g1_xyz[0]) + (wedge_g0[3] * other.e415), (right_dual_g2_w * wedge_g1_xyz[1]) + (wedge_g0[3] * other.e425), (right_dual_g2_w * wedge_g1_xyz[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[0] * wedge_g0[0]) - (right_dual_g3[1] * wedge_g0[1]) - (right_dual_g3[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g1_xyz.yzx * right_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((this.e45 * other.e23) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e4125 * other.scalar), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(right_dual_g0.zxy, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g3_xyz[1] * wedge_g1[1]) + (right_dual_g3_xyz[2] * wedge_g1[2]) + (wedge_g1[3] * other.e1234) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (wedge_g0[3] * other.e45)) - (right_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (right_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((right_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Line> operator /(Line lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g0_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = float3(other.scalar) * this.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.scalar), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(other.scalar)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4((wedge_g0_w * other.e15) + (wedge_g1_xyz[0] * other.scalar), (wedge_g0_w * other.e25) + (wedge_g1_xyz[1] * other.scalar), (wedge_g0_w * other.e35) + (wedge_g1_xyz[2] * other.scalar), -(wedge_g0_xyz[0] * other.e15) - (wedge_g0_xyz[1] * other.e25) - (wedge_g0_xyz[2] * other.e35) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0_xyz[0] * other.e45) + (wedge_g1_xyz[1] * other.e43), (wedge_g0_xyz[1] * other.e45) + (wedge_g1_xyz[2] * other.e41), (wedge_g0_xyz[2] * other.e45) + (wedge_g1_xyz[0] * other.e42), -(wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g2_w = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group3.yzx) - (wedge_g0.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g0[3] * -1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.xyzx * float4(float3(other.e5 * -1.0), other.e423)), 
            /* e1, e2, e3, e5 */
            (wedge_g0.yzxw * other.group1.zxyw) + float4(float3(0.0), wedge_g0[2] * other.e125 * -1.0) - (wedge_g0.zxyx * float4(other.group1.yzx, other.e235)) - (wedge_g0.wwwy * float4(other.group0.xyz, other.e315))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Motor anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g1_w = other.e5 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual_g1_w * wedge_g0[0], right_dual_g1_w * wedge_g0[1], right_dual_g1_w * wedge_g0[2], -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e321 * -1.0))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Motor anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4((float3(other.scalar) * this.group1.xyz), 0.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.scalar)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.scalar), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(wedge_g0_w)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(wedge_g0_xyz[0] * right_dual_g1[0]) - (wedge_g0_xyz[1] * right_dual_g1[1]) - (wedge_g0_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        return Line(
            /* e415, e425, e435 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e5 * -1.0) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Circle anti_reject_via_horizon_from(Dipole other) {
        float wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4((float3(other.scalar) * this.group0.xyz), 0.0);
        float3 wedge_g8 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235));
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(wedge_g9[1] * other.e435, wedge_g9[2] * other.e415, wedge_g9[0] * other.e425, 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[0] * other.e423) + float4((right_dual_g6.xyz * wedge_g6.www), wedge_g9[1] * other.e431) + float4((wedge_g6.xyz * right_dual_g6.www), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[2] * wedge_g6[2]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g7[0] * wedge_g6[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g6[0]) - (wedge_g8[1] * right_dual_g6[1]) - (wedge_g8[2] * right_dual_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g6[0] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g6[1] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g6[2] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_dual_g0[1])) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Circle anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g1 * -1.0) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), right_dual_g1 * wedge_g0[3] * -1.0), 
            /* e235, e315, e125 */
            (float3(other.e5 * -1.0) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group0.xyz)
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
            /* e415, e425, e435 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g2_w = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.zxy * other.group3.yzx) - (wedge_g0.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g0[3] * -1.0), 
            /* e235, e315, e125, e4 */
            float4(wedge_g0[3] * other.e1 * -1.0, wedge_g0[3] * other.e2 * -1.0, wedge_g0[3] * other.e3 * -1.0, (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.xyzx * float4(float3(other.e5 * -1.0), other.e423)), 
            /* e1, e2, e3, e5 */
            (wedge_g0.yzxw * other.group1.zxyw) + float4(float3(0.0), wedge_g0[2] * other.e125 * -1.0) - (wedge_g0.zxyx * float4(other.group1.yzx, other.e235)) - (wedge_g0.wwwy * float4(other.group0.xyz, other.e315))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar, other.scalar, other.scalar, -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) * float4(this.group0.xyz, 1.0);
        float4 wedge_g1 = float4((float3(other.scalar) * this.group1.xyz), 0.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g2[0] * wedge_g0[0]) - (right_dual_g2[1] * wedge_g0[1]) - (right_dual_g2[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g1[1]) + (wedge_g0[0] * other.e45), (right_dual_g0[0] * wedge_g1[2]) + (wedge_g0[1] * other.e45), (right_dual_g0[1] * wedge_g1[0]) + (wedge_g0[2] * other.e45), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (right_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Motor> operator /(Motor lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar * this.e415, other.scalar * this.e425, other.scalar * this.e435, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e12345);
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * float4(other.group0.xyz, other.scalar), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0[3] * other.e15, wedge_g0[3] * other.e25, wedge_g0[3] * other.e35, -(wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) + (float4(other.scalar, other.scalar, other.scalar, other.scalar * other.scalar * this.e5) * float4(wedge_g1_xyz, 1.0)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g1_xyz[1] * other.e43) + (wedge_g0[0] * other.e45), (wedge_g1_xyz[2] * other.e41) + (wedge_g0[1] * other.e45), (wedge_g1_xyz[0] * other.e42) + (wedge_g0[2] * other.e45), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0[0] * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group3.xyz, other.e4);
        float4 wedge_g1 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(other.group0.xyz, other.e321));
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.zxy * other.group3.yzx) - (wedge_g1.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((other.group3.xyz * wedge_g1.www), right_dual_g2_w * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g1[3] * other.e321)) + float4((wedge_g1.yzx * other.group1.zxy), right_dual_g3_w * wedge_g0[3]) - (wedge_g1.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g0.xy), right_dual_g2_w * wedge_g0[2], wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g1[3] * other.scalar)) * float4(wedge_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public DualNum anti_reject_via_horizon_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * other.e321 * this.e5, 0.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0_xyz = float3(this.e5 * -1.0) * other.group1.xyz;
        float4 wedge_g1 = (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group0.yzx), other.e321 * this.e5);
        float right_dual_g1_w = other.e5 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.zxy * other.group1.yzx) - (wedge_g1.yzx * other.group1.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float2(right_dual_g1_w) * wedge_g1.xy), right_dual_g1_w * wedge_g1[2], wedge_g0_xyz[0] * other.e1) + float4(float3(0.0), (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) - (float4(wedge_g1[3]) * float4(other.group1.xyz, other.e321 * -1.0))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(AntiLine other) {
        float wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = float3(this.e5) * other.group0.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual_g0 * float3(wedge_g0_w)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float2(wedge_g0_w) * other.group1.xyz.xy), wedge_g0_w * other.e35, -(right_dual_g0[0] * wedge_g1_xyz[0]) - (right_dual_g0[1] * wedge_g1_xyz[1]) - (right_dual_g0[2] * wedge_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g0_w = (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), other.scalar * this.e5);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.scalar)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.scalar), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(wedge_g0_w)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(wedge_g0_xyz[0] * right_dual_g1[0]) - (wedge_g0_xyz[1] * right_dual_g1[1]) - (wedge_g0_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0_xyz = float3(this.e5 * -1.0) * other.group0.xyz;
        float4 wedge_g1 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g0_w = other.e5 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(wedge_g1[3] * other.e1 * -1.0, wedge_g1[3] * other.e2 * -1.0, wedge_g1[3] * other.e3 * -1.0, (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)) + float4((float2(right_dual_g0_w) * wedge_g1.xy), right_dual_g0_w * wedge_g1[2], wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)), 
            /* e5 */
            (wedge_g0[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)), 
            /* e5 */
            (wedge_g0[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_via_horizon_from(Dipole other) {
        float4 wedge_g0 = float4(this.e5, this.e5, this.e5, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) * float4(other.group0.xyz, 1.0);
        float3 wedge_g1_xyz = float3(this.e5) * other.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g0[3]), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(wedge_g0[3])), -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * wedge_g1_xyz[1]) + (right_dual_g1[3] * wedge_g0[0]), (right_dual_g0[0] * wedge_g1_xyz[2]) + (right_dual_g1[3] * wedge_g0[1]), (right_dual_g0[1] * wedge_g1_xyz[0]) + (right_dual_g1[3] * wedge_g0[2]), -(right_dual_g2[1] * wedge_g0[1]) - (right_dual_g2[2] * wedge_g0[2]) - (wedge_g1_xyz[0] * right_dual_g1[0]) - (wedge_g1_xyz[1] * right_dual_g1[1]) - (wedge_g1_xyz[2] * right_dual_g1[2])) - float4((right_dual_g0.yzx * wedge_g1_xyz.zxy), right_dual_g2[0] * wedge_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5, this.e5, this.e5, (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) * float4(other.group0.xyz, 1.0);
        float3 wedge_g1_xyz = float3(this.e5) * other.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g0[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g0.www), (wedge_g0[3] * other.e1234) - (right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[3] * wedge_g0[0]) + (wedge_g0[3] * other.e4235), (right_dual_g1[3] * wedge_g0[1]) + (wedge_g0[3] * other.e4315), (right_dual_g1[3] * wedge_g0[2]) + (wedge_g0[3] * other.e4125), -(wedge_g1_xyz[1] * right_dual_g1[1]) - (wedge_g1_xyz[2] * right_dual_g1[2]) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) + float4((right_dual_g0.zxy * wedge_g1_xyz.yzx), wedge_g0[3] * other.e3215) - float4((right_dual_g0.yzx * wedge_g1_xyz.zxy), wedge_g1_xyz[0] * right_dual_g1[0])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(this.e5 * -1.0) * other.group1;
        float4 wedge_g6 = float4(((float3(this.e5) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz)), 0.0);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g9 = float4(-(this.e235 * other.e4) - (this.e5 * other.e423), -(this.e315 * other.e4) - (this.e5 * other.e431), -(this.e125 * other.e4) - (this.e5 * other.e412), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * other.group1.zxy), this.e5 * other.e321);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (right_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(wedge_g9[1] * other.e435, wedge_g9[2] * other.e415, wedge_g9[0] * other.e425, 0.0) + (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[0] * other.e423) + float4((right_dual_g6.xyz * wedge_g6.www), wedge_g9[1] * other.e431) + float4((wedge_g6.xyz * right_dual_g6.www), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[2] * wedge_g6[2]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g7[0] * wedge_g6[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g6[0]) - (wedge_g8[1] * right_dual_g6[1]) - (wedge_g8[2] * right_dual_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g6[0] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g6[1] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g6[2] * wedge_g9[3]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_dual_g0[1])) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * other.group0;
        float4 wedge_g1 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g1 * -1.0) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), right_dual_g1 * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual_g0_w) * wedge_g1.xyz) - (float3(wedge_g1[3]) * other.group0.xyz)), right_dual_g1 * wedge_g0[3] * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float2(right_dual_g1 * -1.0) * wedge_g0.xy), right_dual_g1 * wedge_g0[2] * -1.0, (right_dual_g0_w * wedge_g0[3]) + (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
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
            /* e415, e425, e435, e12345 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = this.e5 * other.e1234;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * other.group3;
        float4 wedge_g1 = float4(-(this.e235 * other.e4) - (this.e5 * other.e423), -(this.e315 * other.e4) - (this.e5 * other.e431), -(this.e125 * other.e4) - (this.e5 * other.e412), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * other.group3.zxy), this.e5 * other.e321);
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2_w * -1.0) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.zxy * other.group3.yzx) - (wedge_g1.yzx * other.group3.zxy)), right_dual_g2_w * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((other.group3.xyz * wedge_g1.www), right_dual_g2_w * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g1[3] * other.e321)) + float4((wedge_g1.yzx * other.group1.zxy), right_dual_g3_w * wedge_g0[3]) - (wedge_g1.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g0.xy), right_dual_g2_w * wedge_g0[2], wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = (float4(this.e5) * float4(other.group0.xyz, other.e1234)) + (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 wedge_g1 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e5 * other.scalar);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g2[0] * wedge_g0[0]) - (right_dual_g2[1] * wedge_g0[1]) - (right_dual_g2[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g1[1]) + (wedge_g0[0] * other.e45), (right_dual_g0[0] * wedge_g1[2]) + (wedge_g0[1] * other.e45), (right_dual_g0[1] * wedge_g1[0]) + (wedge_g0[2] * other.e45), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) - (right_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<MultiVector> operator /(MultiVector lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiCircleRotor other) {
        float wedge_g0_y = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g3 = (float4(other.scalar) * this.group3) + (float4(this.scalar) * float4(other.group2.xyz, other.e45));
        float3 wedge_g4 = (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(this.e5, this.e5, this.e5, other.scalar * this.e321) * float4(other.group0.xyz, 1.0)) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group3.zxy), other.scalar * this.e3215) - (this.group3.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group4.zxy * other.group2.yzx), other.e31 * this.e25);
        float wedge_g10 = (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * other.scalar * this.scalar) - (right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]) - (wedge_g4[0] * other.e15) - (wedge_g4[1] * other.e25) - (wedge_g4[2] * other.e35) - (wedge_g5[0] * right_dual_g1[0]) - (wedge_g5[1] * right_dual_g1[1]) - (wedge_g5[2] * right_dual_g1[2]) - (right_dual_g1[3] * wedge_g3[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7[2] * other.e25) + (right_dual_g1[0] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[0]) + (wedge_g1[0] * other.scalar), (wedge_g7[0] * other.e35) + (right_dual_g1[1] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[1]) + (wedge_g1[1] * other.scalar), (wedge_g7[1] * other.e15) + (right_dual_g1[2] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[2]) + (wedge_g1[2] * other.scalar), -(right_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_dual_g1[0]) - (wedge_g7[1] * right_dual_g1[1]) - (wedge_g7[2] * right_dual_g1[2])) + float4((right_dual_g0.zxy * wedge_g8.yzx), wedge_g1[3] * other.scalar) - float4((right_dual_g0.yzx * wedge_g8.zxy), right_dual_g0[0] * wedge_g6[0]) - float4((wedge_g7.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            (other.scalar * other.scalar * this.e5) - (wedge_g8[0] * right_dual_g1[0]) - (wedge_g8[1] * right_dual_g1[1]) - (wedge_g8[2] * right_dual_g1[2]) - (wedge_g6[0] * other.e15) - (wedge_g6[1] * other.e25) - (wedge_g6[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4((wedge_g3[0] * other.scalar) + (wedge_g9[2] * other.e25), (wedge_g3[1] * other.scalar) + (wedge_g9[0] * other.e35), (wedge_g3[2] * other.scalar) + (wedge_g9[1] * other.e15), -(right_dual_g1[1] * wedge_g9[1]) - (right_dual_g1[2] * wedge_g9[2])) + float4((right_dual_g1.xyz * wedge_g9.www), wedge_g3[3] * other.scalar) - (wedge_g9.yzxx * float4(other.group2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.scalar)) + (float3(wedge_g10) * right_dual_g1.xyz) + (right_dual_g0.zxy * wedge_g9.yzx) - (right_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(wedge_g9[3])) + (wedge_g5 * float3(other.scalar)) + (float3(wedge_g10) * other.group2.xyz) - (float3(right_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g6 * float4(other.scalar)), 
            /* e423, e431, e412 */
            (right_dual_g0 * float3(wedge_g0_y)) + (wedge_g7 * float3(other.scalar)), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.scalar)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(other.scalar), 
            /* e1234 */
            wedge_g10 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float wedge_g0_y = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * float4(other.group3.xyz, other.e4));
        float3 wedge_g4 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group3.zxy) - (other.group3.zxy * this.group3.yzx);
        float4 wedge_g9 = (float4(other.e5) * float4(this.group7.xyz, this.e321)) + (other.group3.yzxz * float4(this.group6.zxy, this.e125)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) + float4((other.group2.xyz * this.group1.www), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group8.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group6.yzx), other.e125 * this.e3);
        float wedge_g10 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g3[0] * wedge_g1[0]) + (right_dual_g3[1] * wedge_g1[1]) + (right_dual_g3[2] * wedge_g1[2]) + (right_dual_g3[3] * wedge_g1[3]) + (right_dual_g2_w * other.e5 * this.scalar) - (right_dual_g1_w * wedge_g6[3]) - (wedge_g7[0] * other.e235) - (wedge_g7[1] * other.e315) - (wedge_g7[2] * other.e125) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g10) * float4(other.group2.xyz, right_dual_g1_w)) + (wedge_g9.yzxz * float4(other.group1.zxy, other.e412)) + float4((wedge_g4 * right_dual_g3.www), wedge_g9[0] * other.e423) + float4((wedge_g5.yzx * right_dual_g3.zxy), wedge_g9[1] * other.e431) - (wedge_g3 * float4(right_dual_g2_w)) - (right_dual_g3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) - float4((other.group0.xyz * wedge_g9.www), wedge_g4[2] * right_dual_g3[2]) - float4((wedge_g9.zxy * other.group1.yzx), wedge_g4[1] * right_dual_g3[1]), 
            /* e5 */
            (right_dual_g3[0] * wedge_g3[0]) + (right_dual_g3[1] * wedge_g3[1]) + (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) - (right_dual_g1_w * wedge_g9[3]) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_dual_g3[2]) + (right_dual_g3[3] * wedge_g6[0]), (wedge_g8[2] * right_dual_g3[0]) + (right_dual_g3[3] * wedge_g6[1]), (wedge_g8[0] * right_dual_g3[1]) + (right_dual_g3[3] * wedge_g6[2]), -(right_dual_g3[1] * wedge_g6[1]) - (right_dual_g3[2] * wedge_g6[2])) + (float4(wedge_g0_y) * float4(other.group2.xyz, right_dual_g1_w)) - (right_dual_g3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g2_w) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group0.xyz) + (wedge_g7.zxy * right_dual_g3.yzx) - (wedge_g7.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_dual_g3[3])) + (wedge_g8 * float3(right_dual_g2_w)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * right_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(wedge_g9.zxy, wedge_g10)) - (wedge_g9.yzxw * float4(right_dual_g3.zxy, right_dual_g2_w)), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_dual_g3.xyz) - (float3(right_dual_g2_w) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g3[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0_y), 
            /* e1234 */
            right_dual_g2_w * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiDualNum other) {
        float wedge_g0_y = (other.e3215 * this.e4) + (other.scalar * this.e12345);
        float wedge_g10 = other.scalar * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e3215) + (other.scalar * other.scalar * this.scalar), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e5 */
            (wedge_g0_y * other.e3215) + (other.scalar * other.scalar * this.e5), 
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
            float4((float3(other.scalar * other.scalar) * this.group9.xyz), (other.scalar * other.scalar * this.e3215) + (other.e3215 * other.scalar * this.scalar)), 
            /* e1234 */
            wedge_g10 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiFlatPoint other) {
        float4 wedge_g9 = float4(this.e4, this.e4, this.e4, -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) * float4(other.group0.xyz, 1.0);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[3] * other.e321 * this.scalar, 0.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            -(right_dual_g0[0] * wedge_g9[0]) - (right_dual_g0[1] * wedge_g9[1]) - (right_dual_g0[2] * wedge_g9[2]) - (right_dual_g0[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            -(right_dual_g0 * float4(other.e235 * this.e41)) - (right_dual_g0 * float4(other.e315 * this.e42)) - (right_dual_g0 * float4(other.e125 * this.e43)) - (right_dual_g0 * float4(other.e321 * this.e45)), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiFlector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45);
        float3 wedge_g1_xyz = float3(this.scalar) * other.group1.xyz;
        float3 wedge_g3_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz);
        float3 wedge_g4 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g5 = (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group4.xyz * other.group1.www), other.e321 * this.scalar) - (other.group1.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (this.group4.yzx * other.group1.zxy) - (this.group4.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx);
        float4 wedge_g9 = (other.group1.yzxy * float4(this.group6.zxy, this.e315)) + (other.group1.wwwz * float4(this.group7.xyz, this.e125)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group0.xyz * this.group1.www), other.e1 * this.e235) - float4((other.group1.zxy * this.group6.yzx), other.e235 * this.e1);
        float wedge_g10 = (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * right_dual_g1[0]) + (wedge_g1_xyz[1] * right_dual_g1[1]) + (wedge_g1_xyz[2] * right_dual_g1[2]) - (wedge_g7[0] * right_dual_g0[0]) - (wedge_g7[1] * right_dual_g0[1]) - (wedge_g7[2] * right_dual_g0[2]) - (right_dual_g0[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g1[3]) + (wedge_g5[1] * right_dual_g1[2]), (wedge_g4[1] * right_dual_g1[3]) + (wedge_g5[2] * right_dual_g1[0]), (wedge_g4[2] * right_dual_g1[3]) + (wedge_g5[0] * right_dual_g1[1]), -(wedge_g4[1] * right_dual_g1[1]) - (wedge_g4[2] * right_dual_g1[2])) + (right_dual_g0 * float4(wedge_g10)) - (right_dual_g1.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            (wedge_g3_xyz[0] * right_dual_g1[0]) + (wedge_g3_xyz[1] * right_dual_g1[1]) + (wedge_g3_xyz[2] * right_dual_g1[2]) + (right_dual_g1[3] * other.e5 * this.e4) - (right_dual_g0[0] * wedge_g9[0]) - (right_dual_g0[1] * wedge_g9[1]) - (right_dual_g0[2] * wedge_g9[2]) - (right_dual_g0[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_dual_g1[2]) + (right_dual_g1[3] * wedge_g6[0]), (wedge_g8[2] * right_dual_g1[0]) + (right_dual_g1[3] * wedge_g6[1]), (wedge_g8[0] * right_dual_g1[1]) + (right_dual_g1[3] * wedge_g6[2]), -(right_dual_g1[1] * wedge_g6[1]) - (right_dual_g1[2] * wedge_g6[2])) + (right_dual_g0 * float4(wedge_g0_y)) - (right_dual_g1.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * right_dual_g1.yzx) - (wedge_g7.yzx * right_dual_g1.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_dual_g1[3])) - (float3(wedge_g6[3]) * right_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1.yzx * wedge_g9.zxy) - (right_dual_g1.zxy * wedge_g9.yzx)), wedge_g10 * right_dual_g1[3]), 
            /* e423, e431, e412 */
            float3(wedge_g10) * right_dual_g1.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual_g1[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g0_y), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiLine other) {
        float wedge_g0_y = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 wedge_g5 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g6 = float4(this.e4, this.e4, this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) * float4(other.group1.xyz, 1.0);
        float3 wedge_g7 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g8 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group4.zxy), other.e23 * this.e15);
        float wedge_g10 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * wedge_g5[0]) - (right_dual_g0[1] * wedge_g5[1]) - (right_dual_g0[2] * wedge_g5[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * wedge_g6[3]) + (right_dual_g1[1] * wedge_g7[2]), (right_dual_g0[1] * wedge_g6[3]) + (right_dual_g1[2] * wedge_g7[0]), (right_dual_g0[2] * wedge_g6[3]) + (right_dual_g1[0] * wedge_g7[1]), -(right_dual_g0[1] * wedge_g7[1]) - (right_dual_g0[2] * wedge_g7[2])) - float4((right_dual_g1.zxy * wedge_g7.yzx), right_dual_g0[0] * wedge_g7[0]), 
            /* e5 */
            -(right_dual_g0[0] * wedge_g8[0]) - (right_dual_g0[1] * wedge_g8[1]) - (right_dual_g0[2] * wedge_g8[2]) - (right_dual_g1[0] * wedge_g6[0]) - (right_dual_g1[1] * wedge_g6[1]) - (right_dual_g1[2] * wedge_g6[2]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * wedge_g9[3]) + (right_dual_g1[1] * wedge_g9[2]), (right_dual_g0[1] * wedge_g9[3]) + (right_dual_g1[2] * wedge_g9[0]), (right_dual_g0[2] * wedge_g9[3]) + (right_dual_g1[0] * wedge_g9[1]), -(right_dual_g0[1] * wedge_g9[1]) - (right_dual_g0[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e41, e42, e43 */
            right_dual_g0 * float3(wedge_g10), 
            /* e23, e31, e12 */
            right_dual_g1 * float3(wedge_g10), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * float3(wedge_g0_y)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_dual_g1 * float3(wedge_g0_y), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiMotor other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g4 = float3(other.scalar) * this.group4.xyz;
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g6 = float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6);
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = float4(other.e35 * this.e42, other.e15 * this.e43, other.e25 * this.e41, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group3.www, this.e3215)) + float4((this.group9.xyz * other.group0.www), other.e3215 * this.scalar) - float4((this.group4.zxy * other.group1.yzx), other.e23 * this.e15);
        float wedge_g10 = (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e3215) + (right_dual_g0[3] * other.scalar * this.scalar) - (wedge_g4[0] * other.e15) - (wedge_g4[1] * other.e25) - (wedge_g4[2] * other.e35) - (wedge_g5[0] * right_dual_g0[0]) - (wedge_g5[1] * right_dual_g0[1]) - (wedge_g5[2] * right_dual_g0[2]), wedge_g0_y * right_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * wedge_g6[3]) + (right_dual_g0[3] * wedge_g1[0]), (right_dual_g0[1] * wedge_g6[3]) + (right_dual_g0[3] * wedge_g1[1]), (right_dual_g0[2] * wedge_g6[3]) + (right_dual_g0[3] * wedge_g1[2]), -(wedge_g7[1] * right_dual_g0[1]) - (wedge_g7[2] * right_dual_g0[2])) + float4((wedge_g7.zxy * other.group1.yzx), right_dual_g0[3] * wedge_g1[3]) - float4((wedge_g7.yzx * other.group1.zxy), wedge_g7[0] * right_dual_g0[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_dual_g0[3] * other.scalar * this.e5) - (wedge_g8[0] * right_dual_g0[0]) - (wedge_g8[1] * right_dual_g0[1]) - (wedge_g8[2] * right_dual_g0[2]) - (wedge_g6[0] * other.e15) - (wedge_g6[1] * other.e25) - (wedge_g6[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4((wedge_g9[2] * other.e25) + (right_dual_g0[3] * other.scalar * this.e15) + (right_dual_g0[3] * other.e15 * this.scalar), (wedge_g9[0] * other.e35) + (right_dual_g0[3] * other.scalar * this.e25) + (right_dual_g0[3] * other.e25 * this.scalar), (wedge_g9[1] * other.e15) + (right_dual_g0[3] * other.scalar * this.e35) + (right_dual_g0[3] * other.e35 * this.scalar), -(right_dual_g0[1] * wedge_g9[1]) - (right_dual_g0[2] * wedge_g9[2])) + (right_dual_g0 * float4(wedge_g9.www, other.scalar * this.e45)) - (wedge_g9.yzxx * float4(other.group1.zxy, right_dual_g0[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_dual_g0[3])) + (float3(wedge_g10) * right_dual_g0.xyz), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(right_dual_g0[3])) + (float3(wedge_g10) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0_y) * right_dual_g0.xyz) + (float3(right_dual_g0[3]) * wedge_g6.xyz)), right_dual_g0[3] * wedge_g6[3]), 
            /* e423, e431, e412 */
            wedge_g7 * float3(right_dual_g0[3]), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(right_dual_g0[3])) + (float3(wedge_g0_y) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(right_dual_g0[3]), 
            /* e1234 */
            wedge_g10 * right_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g3_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz);
        float3 wedge_g4 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g5 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g9 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group6.zxy, this.e235)) + (other.group0.wwwy * float4(this.group7.xyz, this.e315));
        float wedge_g10 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * right_dual_g0[0]) + (wedge_g1_xyz[1] * right_dual_g0[1]) + (wedge_g1_xyz[2] * right_dual_g0[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g0[3]) + (wedge_g5[1] * right_dual_g0[2]), (wedge_g4[1] * right_dual_g0[3]) + (wedge_g5[2] * right_dual_g0[0]), (wedge_g4[2] * right_dual_g0[3]) + (wedge_g5[0] * right_dual_g0[1]), -(wedge_g4[1] * right_dual_g0[1]) - (wedge_g4[2] * right_dual_g0[2])) - (right_dual_g0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            (wedge_g3_xyz[0] * right_dual_g0[0]) + (wedge_g3_xyz[1] * right_dual_g0[1]) + (wedge_g3_xyz[2] * right_dual_g0[2]) + (right_dual_g0[3] * other.e5 * this.e4), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_dual_g0[2]) + (right_dual_g0[3] * wedge_g6[0]), (wedge_g8[2] * right_dual_g0[0]) + (right_dual_g0[3] * wedge_g6[1]), (wedge_g8[0] * right_dual_g0[1]) + (right_dual_g0[3] * wedge_g6[2]), -(right_dual_g0[1] * wedge_g6[1]) - (right_dual_g0[2] * wedge_g6[2])) - (right_dual_g0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * right_dual_g0.yzx) - (wedge_g7.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_dual_g0[3])) - (float3(wedge_g6[3]) * right_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * wedge_g9.zxy) - (right_dual_g0.zxy * wedge_g9.yzx)), wedge_g10 * right_dual_g0[3]), 
            /* e423, e431, e412 */
            float3(wedge_g10) * right_dual_g0.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g0 * float4(other.e1 * this.e4235)) + (right_dual_g0 * float4(other.e2 * this.e4315)) + (right_dual_g0 * float4(other.e3 * this.e4125)) + (right_dual_g0 * float4(other.e5 * this.e1234)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Circle other) {
        float wedge_g0_y = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g6 = float4(this.scalar) * other.group1;
        float3 wedge_g7 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group2.xyz;
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g1_w * wedge_g6[3]) - (wedge_g7[0] * other.e235) - (wedge_g7[1] * other.e315) - (wedge_g7[2] * other.e125) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g9[2] * other.e425) - (wedge_g9[3] * other.e423), -(wedge_g9[0] * other.e435) - (wedge_g9[3] * other.e431), -(wedge_g9[1] * other.e415) - (wedge_g9[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g9[2] * other.e412) * float4((other.group2.xyz * other.group0.yyy), 1.0)) + (wedge_g9.yzxx * float4(other.group1.zxy, other.e423)) + (this.group1.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + (this.group1.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group2.xyz * other.group0.xxx * this.group1.xxx), wedge_g9[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g9[3]) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(other.group2.xyz, right_dual_g1_w), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(CircleRotor other) {
        float wedge_g0_y = (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g6 = float4(this.scalar) * other.group1;
        float3 wedge_g7 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group2.xyz;
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g1_w * wedge_g6[3]) - (wedge_g0_y * other.e12345) - (wedge_g7[0] * other.e235) - (wedge_g7[1] * other.e315) - (wedge_g7[2] * other.e125) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g9[2] * other.e425) - (wedge_g9[3] * other.e423), -(wedge_g9[0] * other.e435) - (wedge_g9[3] * other.e431), -(wedge_g9[1] * other.e415) - (wedge_g9[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g9[2] * other.e412) * float4((other.group0.yyy * other.group2.xyz), 1.0)) + (wedge_g9.yzxx * float4(other.group1.zxy, other.e423)) + (this.group1.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + (this.group1.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group0.xxx * this.group1.xxx * other.group2.xyz), wedge_g9[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g9[3]) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(other.group2.xyz, right_dual_g1_w), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Dipole other) {
        float wedge_g0_y = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g6 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group3.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group4.zxy), other.e31 * this.e25);
        float wedge_g10 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]) - (right_dual_g2[0] * wedge_g4[0]) - (right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g5[0] * right_dual_g1[0]) - (wedge_g5[1] * right_dual_g1[1]) - (wedge_g5[2] * right_dual_g1[2]) - (right_dual_g1[3] * wedge_g3[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g8[1]) + (right_dual_g2[1] * wedge_g7[2]) + (right_dual_g1[0] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[0]), (right_dual_g0[0] * wedge_g8[2]) + (right_dual_g2[2] * wedge_g7[0]) + (right_dual_g1[1] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[1]), (right_dual_g0[1] * wedge_g8[0]) + (right_dual_g2[0] * wedge_g7[1]) + (right_dual_g1[2] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[2]), -(right_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_dual_g1[0]) - (wedge_g7[1] * right_dual_g1[1]) - (wedge_g7[2] * right_dual_g1[2])) - float4((right_dual_g0.yzx * wedge_g8.zxy), right_dual_g0[0] * wedge_g6[0]) - float4((right_dual_g2.zxy * wedge_g7.yzx), right_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            -(right_dual_g2[0] * wedge_g6[0]) - (right_dual_g2[1] * wedge_g6[1]) - (right_dual_g2[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g1[0]) - (wedge_g8[1] * right_dual_g1[1]) - (wedge_g8[2] * right_dual_g1[2]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g2[1] * wedge_g9[2]) + (right_dual_g1[0] * wedge_g9[3]), (right_dual_g2[2] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[3]), (right_dual_g2[0] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[3]), -(right_dual_g1[1] * wedge_g9[1]) - (right_dual_g1[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * right_dual_g1.xyz) + (right_dual_g0.zxy * wedge_g9.yzx) - (right_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(wedge_g9[3])) + (right_dual_g2 * float3(wedge_g10)) - (float3(right_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g0_y), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g0_y), 
            /* e235, e315, e125 */
            right_dual_g2 * float3(wedge_g0_y), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(DipoleInversion other) {
        float wedge_g0_y = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g6 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group3.zxy), other.e3215 * this.scalar) - (this.group3.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group4.zxy * other.group2.yzx), other.e31 * this.e25);
        float wedge_g10 = (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e3215) + (wedge_g9[0] * other.e4235) + (wedge_g9[1] * other.e4315) + (wedge_g9[2] * other.e4125) + (wedge_g9[3] * other.e1234) - (right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]) - (wedge_g4[0] * other.e15) - (wedge_g4[1] * other.e25) - (wedge_g4[2] * other.e35) - (wedge_g5[0] * right_dual_g1[0]) - (wedge_g5[1] * right_dual_g1[1]) - (wedge_g5[2] * right_dual_g1[2]) - (right_dual_g1[3] * wedge_g3[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * wedge_g8[1]) + (wedge_g7[2] * other.e25) + (right_dual_g1[0] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[0]), (right_dual_g0[0] * wedge_g8[2]) + (wedge_g7[0] * other.e35) + (right_dual_g1[1] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[1]), (right_dual_g0[1] * wedge_g8[0]) + (wedge_g7[1] * other.e15) + (right_dual_g1[2] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[2]), -(right_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_dual_g1[0]) - (wedge_g7[1] * right_dual_g1[1]) - (wedge_g7[2] * right_dual_g1[2])) + (float4(wedge_g0_y) * float4(other.group3.xyz, other.e1234)) - float4((right_dual_g0.yzx * wedge_g8.zxy), right_dual_g0[0] * wedge_g6[0]) - float4((wedge_g7.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (wedge_g8[0] * right_dual_g1[0]) - (wedge_g8[1] * right_dual_g1[1]) - (wedge_g8[2] * right_dual_g1[2]) - (wedge_g6[0] * other.e15) - (wedge_g6[1] * other.e25) - (wedge_g6[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[0] * wedge_g9[3]) + (wedge_g9[2] * other.e25), (right_dual_g1[1] * wedge_g9[3]) + (wedge_g9[0] * other.e35), (right_dual_g1[2] * wedge_g9[3]) + (wedge_g9[1] * other.e15), -(right_dual_g1[1] * wedge_g9[1]) - (right_dual_g1[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(other.group2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * right_dual_g1.xyz) + (right_dual_g0.zxy * wedge_g9.yzx) - (right_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(wedge_g9[3])) + (float3(wedge_g10) * other.group2.xyz) - (float3(right_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g0_y), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g0_y), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(DualNum other) {
        float wedge_g0_y = (other.e5 * this.e1234) + (other.e12345 * this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0_y * other.e12345, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * other.e5 * this.e4, 
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
            float3(other.e5 * other.e5) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * other.e5), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(FlatPoint other) {
        float wedge_g0_y = -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz);
        float4 wedge_g9 = float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group4.zxy, this.e23));
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e45 * other.e45 * this.scalar, 0.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_xyz * float3(other.e45)), 0.0), 
            /* e5 */
            -(wedge_g6_xyz[0] * other.e15) - (wedge_g6_xyz[1] * other.e25) - (wedge_g6_xyz[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * wedge_g9.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * other.e45), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0_y = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz);
        float4 wedge_g9 = float4((other.e45 * this.e23) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e4125 * this.scalar), -(other.e25 * this.e31) - (other.e35 * this.e12)) + float4((this.group4.yzx * other.group0.zxy), other.e3215 * this.scalar) - (other.group0.yzxx * float4(this.group4.zxy, this.e23));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e4235) + (wedge_g9[1] * other.e4315) + (wedge_g9[2] * other.e4125) - (other.e45 * other.e45 * this.scalar), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g6_xyz * float3(other.e45)) + (float3(wedge_g0_y) * other.group1.xyz)), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (wedge_g6_xyz[0] * other.e15) - (wedge_g6_xyz[1] * other.e25) - (wedge_g6_xyz[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * wedge_g9.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * other.e45), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Line other) {
        float wedge_g0_y = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float3 wedge_g6_xyz = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g6_xyz[0] * other.e415) - (wedge_g6_xyz[1] * other.e425) - (wedge_g6_xyz[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((other.group0.zxy * wedge_g9.yzx) - (other.group0.yzx * wedge_g9.zxy)), 0.0), 
            /* e5 */
            -(wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Motor other) {
        float wedge_g0_y = (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float3 wedge_g6_xyz = (float3(other.e5) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g9 = float4((other.e235 * this.e4) + (other.e5 * this.e423), (other.e315 * this.e4) + (other.e5 * this.e431), (other.e125 * this.e4) + (other.e5 * this.e412), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.e5 * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        float right_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g0_y * other.e12345) - (wedge_g6_xyz[0] * other.e415) - (wedge_g6_xyz[1] * other.e425) - (wedge_g6_xyz[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g9.yzx * other.group0.zxy) - (wedge_g9.zxy * other.group0.yzx)), 0.0), 
            /* e5 */
            (right_dual_g1_w * other.e5 * this.e4) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g6_xyz * float3(right_dual_g1_w)) + (float3(wedge_g0_y) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g1_w) * wedge_g9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g1_w * wedge_g0_y), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float wedge_g2 = (other.scalar * this.e5) + (other.e5 * this.scalar);
        float4 wedge_g3 = (float4(other.scalar) * this.group3) + (float4(other.e5) * this.group1) + (float4(this.scalar) * other.group3) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group5.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((other.e5 * this.e41) + (other.e15 * this.e4) + (other.e41 * this.e5) + (other.e415 * this.scalar), (other.e5 * this.e42) + (other.e25 * this.e4) + (other.e42 * this.e5) + (other.e425 * this.scalar), (other.e5 * this.e43) + (other.e35 * this.e4) + (other.e43 * this.e5) + (other.e435 * this.scalar), -(other.e3 * this.e12) - (other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6) + float4((this.group3.xyz * other.group1.www), other.e321 * this.scalar) - (other.group1.xyzx * float4(this.group3.www, this.e23)) - float4((this.group1.xyz * other.group3.www), other.e2 * this.e31);
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group1.zxy) + (this.group4.yzx * other.group1.zxy) - (other.group4.zxy * this.group1.yzx) - (this.group4.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group8.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group1.yzx * this.group3.zxy) + (other.group3.zxy * this.group1.yzx) - (other.group1.zxy * this.group3.yzx) - (other.group3.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e425 * this.e3) + (other.e235 * this.e4) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e435 * this.e1) + (other.e315 * this.e4) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e415 * this.e2) + (other.e125 * this.e4) + (other.e4125 * this.scalar), -(other.e321 * this.e5) - (other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3)) + (float4(other.scalar) * this.group9) + (float4(other.e5, other.e5, other.e5 * this.e412, other.e2 * this.e315) * float4(this.group7.xyz.xy, 1.0, 1.0)) + (other.group1.yzxx * float4(this.group6.zxy, this.e235)) + float4((this.group5.xyz * other.group3.www), other.e5 * this.e321) + float4((other.group4.yzx * this.group3.zxy), other.e3215 * this.scalar) + float4((this.group4.yzx * other.group3.zxy), other.e3 * this.e125) - (float4(this.e5, this.e5, this.e5, other.e12 * this.e35) * float4(other.group7.xyz, 1.0)) - (other.group3.yzxz * float4(this.group4.zxy, this.e12)) - (this.group3.yzxx * float4(other.group4.zxy, other.e23)) - float4((this.group8.xyz * other.group1.www), other.e25 * this.e31) - float4((other.group1.zxy * this.group6.yzx), other.e15 * this.e23) - float4((other.group6.zxy * this.group1.yzx), other.e31 * this.e25);
        float wedge_g10 = (other.scalar * this.e1234) + (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e1234 * this.scalar) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) + (right_dual_g0[1] * other.scalar * this.scalar) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Plane other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215);
        float4 wedge_g9 = float4(this.scalar) * other.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e4235) + (wedge_g9[1] * other.e4315) + (wedge_g9[2] * other.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0_y) * other.group0.xyz), 0.0), 
            /* e5 */
            wedge_g0_y * other.e3215, 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(RoundPoint other) {
        float wedge_g0_y = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5);
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * other.group0);
        float3 wedge_g4 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy);
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5, other.e5, other.e5, this.e235 * other.e1) * float4(this.group7.xyz, 1.0)) + (this.group6.zxyw * float4(other.group0.yzx, other.e5));
        float wedge_g10 = -(this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * wedge_g1[0]) + (right_dual_g0[1] * wedge_g1[1]) + (right_dual_g0[2] * wedge_g1[2]) + (right_dual_g0[3] * wedge_g1[3]) + (right_dual_g1 * this.scalar * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g0[3]) + (wedge_g5[1] * right_dual_g0[2]), (wedge_g4[1] * right_dual_g0[3]) + (wedge_g5[2] * right_dual_g0[0]), (wedge_g4[2] * right_dual_g0[3]) + (wedge_g5[0] * right_dual_g0[1]), -(wedge_g4[1] * right_dual_g0[1]) - (wedge_g4[2] * right_dual_g0[2])) - (wedge_g3 * float4(right_dual_g1)) - (right_dual_g0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            (right_dual_g0[0] * wedge_g3[0]) + (right_dual_g0[1] * wedge_g3[1]) + (right_dual_g0[2] * wedge_g3[2]) + (right_dual_g0[3] * wedge_g3[3]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_dual_g0[2]) + (right_dual_g0[3] * wedge_g6[0]), (wedge_g8[2] * right_dual_g0[0]) + (right_dual_g0[3] * wedge_g6[1]), (wedge_g8[0] * right_dual_g0[1]) + (right_dual_g0[3] * wedge_g6[2]), -(right_dual_g0[1] * wedge_g6[1]) - (right_dual_g0[2] * wedge_g6[2])) - (right_dual_g0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g1) * wedge_g6.xyz) + (wedge_g7.zxy * right_dual_g0.yzx) - (wedge_g7.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_dual_g0[3])) + (wedge_g8 * float3(right_dual_g1)) - (float3(wedge_g6[3]) * right_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g0.yzxw * float4(wedge_g9.zxy, wedge_g10)) - (wedge_g9.yzxw * float4(right_dual_g0.zxy, right_dual_g1)), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_dual_g0.xyz) - (float3(right_dual_g1) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0_y), 
            /* e1234 */
            right_dual_g1 * wedge_g0_y
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        float4 wedge_g9 = float4(this.scalar) * other.group0;
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * wedge_g9[0]) + (right_dual_g0[1] * wedge_g9[1]) + (right_dual_g0[2] * wedge_g9[2]) + (right_dual_g0[3] * wedge_g9[3]) + (this.scalar * other.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(wedge_g0_y), 
            /* e5 */
            wedge_g0_y * other.e3215, 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(VersorEven other) {
        float wedge_g0_y = (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 wedge_g1 = float4(this.scalar) * other.group3;
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * other.group3);
        float3 wedge_g4 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(this.scalar) * other.group0.xyz) + (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx);
        float3 wedge_g8 = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group3.yzx) - (this.group3.yzx * other.group3.zxy);
        float4 wedge_g9 = (other.group3.yzxy * float4(this.group6.zxy, this.e315)) + float4((this.group7.xyz * other.group2.www), this.e125 * other.e3) + float4((this.group1.zxy * other.group1.yzx), this.e321 * other.e5) + float4((other.group2.xyz * this.group1.www), this.e235 * other.e1) - (float4(this.e5, this.e5, this.e5 * other.e412, this.e2 * other.e315) * float4(other.group0.xy, 1.0, 1.0)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - float4((this.group8.xyz * other.group3.www), this.e5 * other.e321) - float4((this.group6.yzx * other.group3.zxy), this.e3 * other.e125);
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g3[0] * wedge_g1[0]) + (right_dual_g3[1] * wedge_g1[1]) + (right_dual_g3[2] * wedge_g1[2]) + (right_dual_g3[3] * wedge_g1[3]) + (right_dual_g2_w * this.scalar * other.e5) - (right_dual_g1_w * wedge_g6[3]) - (wedge_g0_y * other.e12345) - (wedge_g7[0] * other.e235) - (wedge_g7[1] * other.e315) - (wedge_g7[2] * other.e125) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g10) * float4(other.group2.xyz, right_dual_g1_w)) + (wedge_g9.yzxz * float4(other.group1.zxy, other.e412)) + float4((wedge_g4 * right_dual_g3.www), wedge_g9[0] * other.e423) + float4((wedge_g5.yzx * right_dual_g3.zxy), wedge_g9[1] * other.e431) - (wedge_g3 * float4(right_dual_g2_w)) - (right_dual_g3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) - float4((wedge_g9.zxy * other.group1.yzx), wedge_g4[1] * right_dual_g3[1]) - float4((other.group0.xyz * wedge_g9.www), wedge_g4[2] * right_dual_g3[2]), 
            /* e5 */
            (right_dual_g3[0] * wedge_g3[0]) + (right_dual_g3[1] * wedge_g3[1]) + (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) - (right_dual_g1_w * wedge_g9[3]) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_dual_g3[2]) + (right_dual_g3[3] * wedge_g6[0]), (wedge_g8[2] * right_dual_g3[0]) + (right_dual_g3[3] * wedge_g6[1]), (wedge_g8[0] * right_dual_g3[1]) + (right_dual_g3[3] * wedge_g6[2]), -(right_dual_g3[1] * wedge_g6[1]) - (right_dual_g3[2] * wedge_g6[2])) + (float4(wedge_g0_y) * float4(other.group2.xyz, right_dual_g1_w)) - (right_dual_g3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g2_w) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group0.xyz) + (wedge_g7.zxy * right_dual_g3.yzx) - (wedge_g7.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_dual_g3[3])) + (wedge_g8 * float3(right_dual_g2_w)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * right_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(wedge_g9.zxy, wedge_g10)) - (wedge_g9.yzxw * float4(right_dual_g3.zxy, right_dual_g2_w)), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_dual_g3.xyz) - (float3(right_dual_g2_w) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g3[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0_y), 
            /* e1234 */
            right_dual_g2_w * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(VersorOdd other) {
        float wedge_g0_y = (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g3 = (float4(this.scalar) * float4(other.group2.xyz, other.e45)) + (float4(other.scalar) * this.group3);
        float3 wedge_g4 = (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group4.xyz);
        float3 wedge_g5 = (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group5.xyz);
        float4 wedge_g6 = float4((this.e5 * other.e41) + (this.e415 * other.scalar), (this.e5 * other.e42) + (this.e425 * other.scalar), (this.e5 * other.e43) + (this.e435 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group2.xyz * this.group1.www), this.e321 * other.scalar) - (this.group1.xyzx * other.group1.wwwx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group7.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group8.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx);
        float4 wedge_g9 = float4((this.e45 * other.e23) + (this.e42 * other.e35) + (this.e23 * other.e45) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e4125 * other.scalar), -(this.e35 * other.e12) - (this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) + (float4(this.scalar) * other.group3) + (other.group0.yzxw * float4(this.group3.zxy, this.e3215)) - (this.group3.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group4.zxy * other.group2.yzx), this.e25 * other.e31);
        float wedge_g10 = (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e3215) + (right_dual_g3[0] * wedge_g9[0]) + (right_dual_g3[1] * wedge_g9[1]) + (right_dual_g3[2] * wedge_g9[2]) + (right_dual_g3[3] * wedge_g9[3]) + (this.scalar * other.scalar * other.scalar) - (right_dual_g2_xyz[0] * wedge_g4[0]) - (right_dual_g2_xyz[1] * wedge_g4[1]) - (right_dual_g2_xyz[2] * wedge_g4[2]) - (wedge_g5[0] * right_dual_g1[0]) - (wedge_g5[1] * right_dual_g1[1]) - (wedge_g5[2] * right_dual_g1[2]) - (right_dual_g1[3] * wedge_g3[3]) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((wedge_g8[1] * other.e43) + (right_dual_g1[0] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[0]) + (wedge_g1[0] * other.scalar), (wedge_g8[2] * other.e41) + (right_dual_g1[1] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[1]) + (wedge_g1[1] * other.scalar), (wedge_g8[0] * other.e42) + (right_dual_g1[2] * wedge_g6[3]) + (right_dual_g1[3] * wedge_g6[2]) + (wedge_g1[2] * other.scalar), -(wedge_g7[2] * right_dual_g1[2]) - (wedge_g6[0] * other.e41) - (wedge_g6[1] * other.e42) - (wedge_g6[2] * other.e43)) + (right_dual_g3 * float4(wedge_g0_y)) + float4((right_dual_g2_xyz.yzx * wedge_g7.zxy), wedge_g1[3] * other.scalar) - float4((right_dual_g2_xyz.zxy * wedge_g7.yzx), wedge_g7[0] * right_dual_g1[0]) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g7[1] * right_dual_g1[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (other.scalar * other.scalar * this.e5) - (right_dual_g2_xyz[0] * wedge_g6[0]) - (right_dual_g2_xyz[1] * wedge_g6[1]) - (right_dual_g2_xyz[2] * wedge_g6[2]) - (wedge_g8[0] * right_dual_g1[0]) - (wedge_g8[1] * right_dual_g1[1]) - (wedge_g8[2] * right_dual_g1[2]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[0] * wedge_g9[3]) + (wedge_g3[0] * other.scalar), (right_dual_g1[1] * wedge_g9[3]) + (wedge_g3[1] * other.scalar), (right_dual_g1[2] * wedge_g9[3]) + (wedge_g3[2] * other.scalar), -(right_dual_g1[1] * wedge_g9[1]) - (right_dual_g1[2] * wedge_g9[2])) + float4((right_dual_g2_xyz.yzx * wedge_g9.zxy), wedge_g3[3] * other.scalar) - (wedge_g9.yzxx * float4(right_dual_g2_xyz.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.scalar)) + (float3(wedge_g10) * right_dual_g1.xyz) + (wedge_g9.yzx * other.group0.zxy) - (wedge_g9.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (right_dual_g2_xyz * float3(wedge_g10)) + (wedge_g5 * float3(other.scalar)) + (float3(wedge_g9[3]) * other.group0.xyz) - (float3(right_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g6 * float4(other.scalar)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.scalar)) + (float3(wedge_g0_y) * other.group0.xyz), 
            /* e235, e315, e125 */
            (right_dual_g2_xyz * float3(wedge_g0_y)) + (wedge_g8 * float3(other.scalar)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(other.scalar), 
            /* e1234 */
            wedge_g10 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Plane> operator /(Plane lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public DipoleInversion anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public DipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float wedge_g0 = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Plane anti_reject_via_horizon_from(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Flector anti_reject_via_horizon_from(AntiFlector other) {
        float wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Flector anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[2] * other.e25) + (wedge_g0[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g0[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g0[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e23)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Plane anti_reject_via_horizon_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * other.e1 * this.e4235) + (other.e1 * other.e2 * this.e4315) + (other.e1 * other.e3 * this.e4125), (other.e2 * other.e2 * this.e4315) + (other.e1 * other.e2 * this.e4235) + (other.e2 * other.e3 * this.e4125), (other.e3 * other.e3 * this.e4125) + (other.e1 * other.e3 * this.e4235) + (other.e2 * other.e3 * this.e4315), -(other.e1 * other.e5 * this.e4235) - (other.e2 * other.e5 * this.e4315) - (other.e3 * other.e5 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215);
        float4 wedge_g9 = float4(other.scalar) * this.group0;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g9[2] * other.e425) - (wedge_g9[3] * other.e423), -(wedge_g9[0] * other.e435) - (wedge_g9[3] * other.e431), -(wedge_g9[1] * other.e415) - (wedge_g9[3] * other.e412), (wedge_g9[1] * other.e431) + (wedge_g9[2] * other.e412)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * wedge_g9[2]) + (right_dual_g6[0] * wedge_g9[3]), (right_dual_g8[2] * wedge_g9[0]) + (right_dual_g6[1] * wedge_g9[3]), (right_dual_g8[0] * wedge_g9[1]) + (right_dual_g6[2] * wedge_g9[3]), -(right_dual_g6[1] * wedge_g9[1]) - (right_dual_g6[2] * wedge_g9[2])) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_dual_g6[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g9[1] * wedge_g9[2], right_dual_g9[2] * wedge_g9[0], right_dual_g9[0] * wedge_g9[1], 0.0) + (right_dual_g6 * float4(wedge_g0_y)) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            right_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Sphere anti_reject_via_horizon_from(RoundPoint other) {
        float wedge_g0 = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            wedge_g0 * other.e4 * -1.0
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
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float wedge_g0 = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[2] * other.e42 * -1.0, wedge_g0[0] * other.e43 * -1.0, wedge_g0[1] * other.e41 * -1.0, (right_dual_g3_xyz[1] * wedge_g0[1]) + (right_dual_g3_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234)) + (wedge_g0.yzxx * float4(other.group0.zxy, right_dual_g3_xyz[0])), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0[3] * other.e41, wedge_g0[3] * other.e42, wedge_g0[3] * other.e43, -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((right_dual_g2.yzx * wedge_g0.zxy), 0.0) - float4((right_dual_g2.zxy * wedge_g0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float3 wedge_g2_xyz = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group0.xyz, this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.scalar), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.scalar), 
            /* e235, e315, e125, e4 */
            float4(other.scalar, other.scalar, other.scalar, (other.scalar * other.scalar * this.e4) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) * float4(wedge_g2_xyz, 1.0), 
            /* e1, e2, e3, e5 */
            float4((wedge_g2_xyz[1] * other.e43) + (wedge_g1[0] * other.e45) + (wedge_g1[3] * other.e23) + (wedge_g3[0] * other.scalar), (wedge_g2_xyz[2] * other.e41) + (wedge_g1[1] * other.e45) + (wedge_g1[3] * other.e31) + (wedge_g3[1] * other.scalar), (wedge_g2_xyz[0] * other.e42) + (wedge_g1[2] * other.e45) + (wedge_g1[3] * other.e12) + (wedge_g3[2] * other.scalar), -(wedge_g2_xyz[2] * other.e12) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (other.group2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2_xyz[0] * other.e23) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g2_xyz[1] * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = (other.group3.zxyw * this.group0.yzxw) - float4((other.group3.yzx * this.group0.zxy), other.e4 * this.e5);
        float4 wedge_g2 = float4(other.e1 * this.e5 * -1.0, other.e2 * this.e5 * -1.0, other.e3 * this.e5 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (this.group0.xyzx * float4(other.group3.www, other.e423));
        float4 wedge_g3 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_via_horizon_from(AntiDualNum other) {
        float wedge_g0_w = other.e3215 * this.e4;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), wedge_g0_w * other.scalar), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (wedge_g0_w * other.e3215) + (other.scalar * other.scalar * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        float4 wedge_g0 = float4(this.e4, this.e4, this.e4, -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) * float4(other.group0.xyz, 1.0);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (right_dual_g0[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz);
        float wedge_g2_w = other.e321 * this.e4;
        float4 wedge_g3 = float4(this.e4, this.e4, this.e4, -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) * float4(other.group0.xyz, 1.0);
        float right_dual_g0_w = other.e321 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2_w) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), right_dual_g1_w * wedge_g2_w), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g1_w) * wedge_g3.xy), right_dual_g1_w * wedge_g3[2], right_dual_g0_w * wedge_g2_w) + float4(float3(0.0), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group1.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyz * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[2])) + float4((float2(right_dual_g1_w) * wedge_g0.xy), right_dual_g1_w * wedge_g0[2], wedge_g2_xyz[0] * other.e1) + float4((float2(wedge_g2_w) * other.group0.xy), wedge_g2_w * other.e125, wedge_g2_xyz[1] * other.e2) + float4(float3(0.0), (right_dual_g1_w * other.e5 * this.e4) - (wedge_g3[0] * other.e235) - (wedge_g3[1] * other.e315) - (wedge_g3[2] * other.e125)) - float4((wedge_g1_xyz.zxy * other.group1.yzx), right_dual_g0_w * wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float4 wedge_g1 = float4(this.e4, this.e4, this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) * float4(other.group1.xyz, 1.0);
        float3 wedge_g2 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e25) + (wedge_g1[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g1[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g1[3] * other.e12), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e23), 
            /* e5 */
            -(wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.e4) * float4(other.group0.xyz, other.e3215);
        float4 wedge_g1 = float4(this.e4, this.e4, this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) * float4(other.group1.xyz, 1.0);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.scalar * this.e5);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g2[0]) - (right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[3] * wedge_g3[0]) + (right_dual_g1[1] * wedge_g0[2]), (right_dual_g0[3] * wedge_g3[1]) + (right_dual_g1[2] * wedge_g0[0]), (right_dual_g0[3] * wedge_g3[2]) + (right_dual_g1[0] * wedge_g0[1]), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (right_dual_g0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float right_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0_w * wedge_g0[0]) + (wedge_g1_xyz[1] * other.e3), (right_dual_g0_w * wedge_g0[1]) + (wedge_g1_xyz[2] * other.e1), (right_dual_g0_w * wedge_g0[2]) + (wedge_g1_xyz[0] * other.e2), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e5 */
            (wedge_g2[0] * other.e1) + (wedge_g2[1] * other.e2) + (wedge_g2[2] * other.e3) + (right_dual_g0_w * other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g0[2] * other.e412) * float4((other.group2.xyz * other.group0.yyy), 1.0)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group0.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + (this.group0.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group2.xyz * other.group0.xxx * this.group0.xxx), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g0[2] * other.e412) * float4((other.group0.yyy * other.group2.xyz), 1.0)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group0.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + (this.group0.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group0.xxx * this.group0.xxx * other.group2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float3 wedge_g2 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e25) + (wedge_g2[1] * other.e43) + (wedge_g1[0] * other.e45) + (wedge_g1[3] * other.e23), (wedge_g0[0] * other.e35) + (wedge_g2[2] * other.e41) + (wedge_g1[1] * other.e45) + (wedge_g1[3] * other.e31), (wedge_g0[1] * other.e15) + (wedge_g2[0] * other.e42) + (wedge_g1[2] * other.e45) + (wedge_g1[3] * other.e12), -(wedge_g0[2] * other.e12) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e23) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * other.e31), 
            /* e5 */
            -(wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4(other.e25 * this.e3 * -1.0, other.e35 * this.e1 * -1.0, other.e15 * this.e2 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235));
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g2.www), (wedge_g2[3] * other.e1234) - (right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[3] * other.e4235), (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[3] * other.e4315), (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[3] * other.e4125), -(right_dual_g1[2] * wedge_g2[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (wedge_g2.yzxw * float4(right_dual_g0.zxy, other.e3215)) - (wedge_g2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public DualNum anti_reject_via_horizon_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * other.e5 * this.e4, 0.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_via_horizon_from(FlatPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e45, other.e45, other.e45, -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) * float4(wedge_g0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(other.e45 * this.e1 * -1.0, other.e45 * this.e2 * -1.0, other.e45 * this.e3 * -1.0, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) + (this.group0.wwwx * float4(other.group0.xyz, other.e4235));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[3] * other.e4235, wedge_g0[3] * other.e4315, wedge_g0[3] * other.e4125, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_dual_g0.www, other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(right_dual_g1_w) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((right_dual_g0.zxy * wedge_g1.yzx), right_dual_g1_w * other.e5 * this.e4) - (wedge_g1.zxyx * float4(right_dual_g0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float wedge_g2 = other.scalar * this.e5;
        float4 wedge_g3 = (float4(other.e5) * this.group0) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g5 = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy);
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5, this.e5, this.e5, other.e235 * this.e1) * float4(other.group7.xyz, 1.0)) - (other.group6.zxyw * float4(this.group0.yzx, this.e5));
        float wedge_g10 = (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_via_horizon_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(other.e4235 * other.e4235 * this.e1) - (other.e4235 * other.e4315 * this.e2) - (other.e4235 * other.e4125 * this.e3) - (other.e4235 * other.e3215 * this.e4), -(other.e4315 * other.e4315 * this.e2) - (other.e4235 * other.e4315 * this.e1) - (other.e4315 * other.e4125 * this.e3) - (other.e4315 * other.e3215 * this.e4), -(other.e4125 * other.e4125 * this.e3) - (other.e4235 * other.e4125 * this.e1) - (other.e4315 * other.e4125 * this.e2) - (other.e4125 * other.e3215 * this.e4), (other.e3215 * other.e3215 * this.e4) + (other.e4235 * other.e3215 * this.e1) + (other.e4315 * other.e3215 * this.e2) + (other.e4125 * other.e3215 * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(other.group0.zxy, other.e5)) - (other.group0.yzxw * float4(this.group0.zxy, this.e5));
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float right_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e4) + (wedge_g1[1] * other.e3), (wedge_g2[1] * other.e4) + (wedge_g1[2] * other.e1), (wedge_g2[2] * other.e4) + (wedge_g1[0] * other.e2), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) + float4((float2(right_dual_g0_w) * wedge_g0.xy), right_dual_g0_w * wedge_g0[2], wedge_g1[3] * other.e4) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e5 */
            (right_dual_g0_w * wedge_g1[3]) + (wedge_g2[0] * other.e1) + (wedge_g2[1] * other.e2) + (wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(other.group3.zxy, other.e5)) - (other.group3.yzxw * float4(this.group0.zxy, this.e5));
        float4 wedge_g2 = float4(this.e5 * other.e1 * -1.0, this.e5 * other.e2 * -1.0, this.e5 * other.e3 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.xyzx * float4(other.group2.www, other.e423));
        float4 wedge_g3 = float4((this.e3 * other.e425) + (this.e4 * other.e235), (this.e1 * other.e435) + (this.e4 * other.e315), (this.e2 * other.e415) + (this.e4 * other.e125), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (float4(this.e5, this.e5, this.e5 * other.e412, this.e2 * other.e315) * float4(other.group0.xy, 1.0, 1.0)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.e2 * other.e43 * -1.0, this.e3 * other.e41 * -1.0, this.e1 * other.e42 * -1.0, (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315));
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2]) - (right_dual_g2[0] * wedge_g1[0]) - (right_dual_g2[1] * wedge_g1[1]) - (right_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[3] * wedge_g1[0]) + (right_dual_g2[1] * wedge_g0[2]) + (wedge_g2[1] * other.e43) + (wedge_g3[0] * other.scalar), (right_dual_g1[3] * wedge_g1[1]) + (right_dual_g2[2] * wedge_g0[0]) + (wedge_g2[2] * other.e41) + (wedge_g3[1] * other.scalar), (right_dual_g1[3] * wedge_g1[2]) + (right_dual_g2[0] * wedge_g0[1]) + (wedge_g2[0] * other.e42) + (wedge_g3[2] * other.scalar), -(right_dual_g1[2] * wedge_g0[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) + float4((right_dual_g1.xyz * wedge_g1.www), wedge_g3[3] * other.scalar) - (wedge_g0.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])) - float4((wedge_g2.zxy * other.group0.yzx), right_dual_g1[1] * wedge_g0[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Scalar> operator /(Scalar lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            wedge_g0 * float3(other.scalar), 
            /* e23, e31, e12, e45 */
            wedge_g1 * float4(other.scalar), 
            /* e15, e25, e35, scalar */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g2[3] * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) * float4(wedge_g2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g2_w) * wedge_g1.xyz) + (wedge_g0.zxy * other.group3.yzx) - (wedge_g0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2_w * wedge_g2[0]) + (right_dual_g3_w * wedge_g0[0]), (right_dual_g2_w * wedge_g2[1]) + (right_dual_g3_w * wedge_g0[1]), (right_dual_g2_w * wedge_g2[2]) + (right_dual_g3_w * wedge_g0[2]), -(wedge_g1[1] * other.e2) - (wedge_g1[2] * other.e3)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (wedge_g2.yzxw * float4(other.group3.zxy, right_dual_g3_w)) + float4((float2(right_dual_g3_w) * wedge_g1.xy), right_dual_g3_w * wedge_g1[2], right_dual_g2_w * wedge_g3[3]) + float4(float3(0.0), (wedge_g1[3] * other.e321) + (wedge_g3[0] * other.e1) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g2.zxy * other.group3.yzx), wedge_g0[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiDualNum anti_reject_via_horizon_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar) * float2(other.e3215 * other.scalar, other.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g0[3] * other.e321) + (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) * float4(wedge_g0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g1.xyz)), wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            (wedge_g1[3] * other.e321) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * wedge_g2.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2[3] * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2[3] * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g3.zxy * other.group2.yzx), wedge_g2[3] * other.e3215) - (other.group2.zxyx * float4(wedge_g3.yzx, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiDualNum anti_reject_via_horizon_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * -1.0) * float2(other.e5 * other.e12345, other.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.scalar)) * float4((wedge_g1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
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
        float wedge_g2 = other.e5 * this.scalar;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float3 wedge_g4 = float3(this.scalar) * other.group4.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group5.xyz;
        float4 wedge_g6 = float4(this.scalar) * other.group6;
        float3 wedge_g7 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group8.xyz;
        float4 wedge_g9 = float4(this.scalar) * other.group9;
        float wedge_g10 = other.e1234 * this.scalar;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g10 * other.e3215) + (right_dual_g0[0] * wedge_g0[1]) + (right_dual_g0[1] * wedge_g0[0]) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), right_dual_g0[1] * wedge_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0[1])) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g2 * right_dual_g0[1]) + (wedge_g0[1] * other.e3215) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0[1])) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0[1]) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0[1]) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + (float4(wedge_g0.yy, wedge_g0[1], right_dual_g0[1] * wedge_g6[3]) * float4(right_dual_g6_xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), wedge_g0[1] * other.e45) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0[1])) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0[1])) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0[1])) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0[1]) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) - (wedge_g0[3] * other.e5) - (other.e4 * other.e5 * this.scalar)
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
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0 * float4(wedge_g0[3])) + (float4(right_dual_g2_w) * float4(wedge_g1.xyz, wedge_g2[3])) + (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * wedge_g3[1]) + (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) - (right_dual_g0[0] * wedge_g2[0]) - (right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (right_dual_g3.zxy * wedge_g2.yzx) - (right_dual_g3.yzx * wedge_g2.zxy)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g0[3] * other.scalar) + (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) - (right_dual_g2_xyz[1] * wedge_g0[1]) - (right_dual_g2_xyz[2] * wedge_g0[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.xyz * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Sphere> operator /(Sphere lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public DipoleInversion anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = other.scalar * this.e1234;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e15) + (wedge_g0[3] * other.e41), (wedge_g1 * other.e25) + (wedge_g0[3] * other.e42), (wedge_g1 * other.e35) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (wedge_g0.zxy * other.group2.yzx) - (wedge_g0.yzx * other.group2.zxy)), wedge_g1 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public DipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float wedge_g0 = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_via_horizon_from(AntiDualNum other) {
        float wedge_g1 = other.scalar * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), wedge_g1 * other.e3215), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), wedge_g1 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public Flector anti_reject_via_horizon_from(AntiFlector other) {
        float wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = other.scalar * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g1) * float4(other.group0.xyz, other.e3215), 
            /* e23, e31, e12, e45 */
            float4((float2(wedge_g1) * other.group1.xy), wedge_g1 * other.e35, -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), wedge_g1 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public Plane anti_reject_via_horizon_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * other.e1 * this.e4235) + (other.e1 * other.e2 * this.e4315) + (other.e1 * other.e3 * this.e4125) + (other.e1 * other.e5 * this.e1234), (other.e2 * other.e2 * this.e4315) + (other.e1 * other.e2 * this.e4235) + (other.e2 * other.e3 * this.e4125) + (other.e2 * other.e5 * this.e1234), (other.e3 * other.e3 * this.e4125) + (other.e1 * other.e3 * this.e4235) + (other.e2 * other.e3 * this.e4315) + (other.e3 * other.e5 * this.e1234), -(other.e5 * other.e5 * this.e1234) - (other.e1 * other.e5 * this.e4235) - (other.e2 * other.e5 * this.e4315) - (other.e3 * other.e5 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public AntiDualNum anti_reject_via_horizon_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e1234 * -1.0) * float2(other.e5 * other.e5, other.e5 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float wedge_g0 = other.e5 * this.e1234;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        float4 wedge_g9 = float4(other.scalar) * this.group0;
        float wedge_g10 = other.scalar * this.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g9[2] * other.e425) - (wedge_g9[3] * other.e423), -(wedge_g9[0] * other.e435) - (wedge_g9[3] * other.e431), -(wedge_g9[1] * other.e415) - (wedge_g9[3] * other.e412), (wedge_g9[1] * other.e431) + (wedge_g9[2] * other.e412)) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]), -(right_dual_g6_xyz[1] * wedge_g9[1]) - (right_dual_g6_xyz[2] * wedge_g9[2])) + (right_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (right_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public Sphere anti_reject_via_horizon_from(RoundPoint other) {
        float wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public Sphere anti_reject_via_horizon_from(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float wedge_g0 = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = this.e1234 * other.scalar;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0[2] * other.e42 * -1.0, wedge_g0[0] * other.e43 * -1.0, wedge_g0[1] * other.e41 * -1.0, (right_dual_g3_xyz[1] * wedge_g0[1]) + (right_dual_g3_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234)) + (float4(wedge_g1) * float4(other.group1.xyz, other.e3215)) + (wedge_g0.yzxx * float4(other.group0.zxy, right_dual_g3_xyz[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_dual_g2_xyz[0]) + (wedge_g0[3] * other.e41), (wedge_g1 * right_dual_g2_xyz[1]) + (wedge_g0[3] * other.e42), (wedge_g1 * right_dual_g2_xyz[2]) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (right_dual_g2_xyz.yzx * wedge_g0.zxy) - (right_dual_g2_xyz.zxy * wedge_g0.yzx)), wedge_g1 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<VersorEven> operator /(VersorEven lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_via_horizon_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.yzx * this.group3.zxy), other.scalar * this.e12345) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group2.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(wedge_g0[3])) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0[3] * other.e15, wedge_g0[3] * other.e25, wedge_g0[3] * other.e35, -(wedge_g2_xyz[0] * right_dual_g1[0]) - (wedge_g2_xyz[1] * right_dual_g1[1]) - (wedge_g2_xyz[2] * right_dual_g1[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (float4(other.scalar, other.scalar, other.scalar, other.scalar * other.scalar * this.e5) * float4(wedge_g2_xyz, 1.0)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g0[2] * other.e25) + (wedge_g3[0] * other.scalar), (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g0[0] * other.e35) + (wedge_g3[1] * other.scalar), (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g0[1] * other.e15) + (wedge_g3[2] * other.scalar), -(right_dual_g0[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g0[0]) - (right_dual_g1[1] * wedge_g0[1]) - (right_dual_g1[2] * wedge_g0[2])) + float4((right_dual_g0.zxy * wedge_g2_xyz.yzx), wedge_g3[3] * other.scalar) - float4((right_dual_g0.yzx * wedge_g2_xyz.zxy), right_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (other.group3.zxyw * this.group3.yzxw) - float4((other.group3.yzx * this.group3.zxy), other.e4 * this.e5);
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((other.group3.xyz * this.group2.www), other.e4 * this.e321);
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e1 * this.e235) + float4((other.group2.xyz * this.group3.www), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.yy, other.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0);
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g2[3] * other.scalar)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_via_horizon_from(AntiFlatPoint other) {
        float4 wedge_g0 = float4(this.e4, this.e4, this.e4, -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) * float4(other.group0.xyz, 1.0);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (right_dual_g0[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy);
        float4 wedge_g2 = (this.group3 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group1.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + (other.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group0.xyz * this.group3.www), other.e1 * this.e235) - float4((other.group1.zxy * this.group1.yzx), other.e235 * this.e1);
        float right_dual_g0_w = other.e321 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), right_dual_g1_w * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g1_w) * wedge_g3.xy), right_dual_g1_w * wedge_g3[2], right_dual_g0_w * wedge_g2[3]) + float4(float3(0.0), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group1.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (wedge_g2.wwwz * float4(other.group0.xyz, other.e3)) + (other.group1.zxyy * float4(wedge_g1_xyz.yzx, wedge_g2[1])) + float4((float2(right_dual_g1_w) * wedge_g0.xy), right_dual_g1_w * wedge_g0[2], wedge_g2[0] * other.e1) + float4(float3(0.0), (right_dual_g1_w * other.e5 * this.e4) - (wedge_g3[0] * other.e235) - (wedge_g3[1] * other.e315) - (wedge_g3[2] * other.e125)) - float4((wedge_g1_xyz.zxy * other.group1.yzx), right_dual_g0_w * wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float4 wedge_g1 = float4(this.e4, this.e4, this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) * float4(other.group1.xyz, 1.0);
        float4 wedge_g2 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group1.yzx * this.group3.zxy), other.e23 * this.e415);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * float3(wedge_g2[3])), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g1 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * wedge_g1[3]) + (right_dual_g1[1] * wedge_g0[2]), (right_dual_g0[1] * wedge_g1[3]) + (right_dual_g1[2] * wedge_g0[0]), (right_dual_g0[2] * wedge_g1[3]) + (right_dual_g1[0] * wedge_g0[1]), -(right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])) - float4((right_dual_g1.zxy * wedge_g0.yzx), right_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = (other.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((this.group0.xyz * other.group0.www), other.e3215 * this.e4);
        float4 wedge_g1 = float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1);
        float4 wedge_g2 = float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.scalar * this.e5);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g2[0]) - (right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[3] * wedge_g3[0]) + (right_dual_g1[1] * wedge_g0[2]), (right_dual_g0[3] * wedge_g3[1]) + (right_dual_g1[2] * wedge_g0[0]), (right_dual_g0[3] * wedge_g3[2]) + (right_dual_g1[0] * wedge_g0[1]), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (right_dual_g0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 wedge_g2 = float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = float4(other.e3 * this.e425 * -1.0, other.e1 * this.e435 * -1.0, other.e2 * this.e415 * -1.0, (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315));
        float right_dual_g0_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.zxy * other.group0.yzx) - (wedge_g3.yzx * other.group0.zxy)), right_dual_g0_w * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0_w * wedge_g3[0], right_dual_g0_w * wedge_g3[1], right_dual_g0_w * wedge_g3[2], -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(wedge_g1_xyz[2] * other.e2 * -1.0, wedge_g1_xyz[0] * other.e3 * -1.0, wedge_g1_xyz[1] * other.e1 * -1.0, (wedge_g2[2] * other.e3) + (right_dual_g0_w * other.e5 * this.e4)) + (other.group0.zxyy * float4(wedge_g1_xyz.yzx, wedge_g2[1])) + float4((float2(right_dual_g0_w) * wedge_g0.xy), right_dual_g0_w * wedge_g0[2], wedge_g2[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_via_horizon_from(Circle other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g0[2] * other.e412) * float4((other.group2.xyz * other.group0.yyy), 1.0)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group3.zzzx * float4((other.group2.xyz * other.group0.zzz), right_dual_g1_w * other.e423)) + (this.group3.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group2.xyz * other.group0.xxx * this.group3.xxx), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_via_horizon_from(CircleRotor other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235));
        float right_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[2] * other.e425) - (wedge_g0[3] * other.e423), -(wedge_g0[0] * other.e435) - (wedge_g0[3] * other.e431), -(wedge_g0[1] * other.e415) - (wedge_g0[3] * other.e412), (right_dual_g1_w * other.e412 * this.e3) + (right_dual_g1_w * other.e321 * this.e4)) + (float4(this.e2, this.e2, this.e2, wedge_g0[2] * other.e412) * float4((other.group0.yyy * other.group2.xyz), 1.0)) + (wedge_g0.yzxx * float4(other.group1.zxy, other.e423)) + (this.group3.zzzx * float4((other.group0.zzz * other.group2.xyz), right_dual_g1_w * other.e423)) + (this.group3.wwwy * float4((other.group2.xyz * other.group1.www), right_dual_g1_w * other.e431)) + float4((other.group0.xxx * this.group3.xxx * other.group2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_dual_g1_w * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * wedge_g2[1]) + (right_dual_g2[1] * wedge_g0[2]) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]), (right_dual_g0[0] * wedge_g2[2]) + (right_dual_g2[2] * wedge_g0[0]) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]), (right_dual_g0[1] * wedge_g2[0]) + (right_dual_g2[0] * wedge_g0[1]) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]), -(right_dual_g2[2] * wedge_g1[2]) - (right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2])) - float4((right_dual_g0.yzx * wedge_g2.zxy), right_dual_g2[0] * wedge_g1[0]) - float4((right_dual_g2.zxy * wedge_g0.yzx), right_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * wedge_g2.www), (wedge_g2[3] * other.e1234) - (right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_dual_g1[0]) - (wedge_g0[1] * right_dual_g1[1]) - (wedge_g0[2] * right_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e25) + (right_dual_g1[0] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[0]) + (wedge_g2[3] * other.e4235), (wedge_g0[0] * other.e35) + (right_dual_g1[1] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[1]) + (wedge_g2[3] * other.e4315), (wedge_g0[1] * other.e15) + (right_dual_g1[2] * wedge_g1[3]) + (right_dual_g1[3] * wedge_g1[2]) + (wedge_g2[3] * other.e4125), -(right_dual_g1[2] * wedge_g2[2]) - (wedge_g1[0] * other.e15) - (wedge_g1[1] * other.e25) - (wedge_g1[2] * other.e35)) + (wedge_g2.yzxw * float4(right_dual_g0.zxy, other.e3215)) - (wedge_g2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), right_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_via_horizon_from(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e5 * other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g0 = float4(other.e15 * this.e4, other.e25 * this.e4, other.e35 * this.e4, -(other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) * float4(right_dual_g0.www, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = (this.group3.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(wedge_g0[3] * other.e4235, wedge_g0[3] * other.e4315, wedge_g0[3] * other.e4125, -(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_dual_g0.www, other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiPlane anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(wedge_g0[1] * other.e435, wedge_g0[2] * other.e415, wedge_g0[0] * other.e425, -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4((other.e235 * this.e4) + (other.e5 * this.e423), (other.e315 * this.e4) + (other.e5 * this.e431), (other.e125 * this.e4) + (other.e5 * this.e412), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group0.yzx * this.group3.zxy), other.e5 * this.e321) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(right_dual_g1_w) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((right_dual_g0.zxy * wedge_g1.yzx), right_dual_g1_w * other.e5 * this.e4) - (wedge_g1.zxyx * float4(right_dual_g0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4(other.scalar) * this.group3;
        float wedge_g2 = other.scalar * this.e5;
        float4 wedge_g3 = (float4(other.e5) * this.group3) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz);
        float3 wedge_g5 = (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (float3(other.scalar) * this.group0.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group3.yzx) - (other.group3.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e5, other.e5, other.e5 * this.e412, other.e2 * this.e315) * float4(this.group0.xy, 1.0, 1.0)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4((other.group8.xyz * this.group3.www), other.e5 * this.e321) + float4((other.group6.yzx * this.group3.zxy), other.e3 * this.e125) - (this.group3.yzxy * float4(other.group6.zxy, other.e315)) - float4((other.group7.xyz * this.group2.www), other.e125 * this.e3) - float4((other.group1.zxy * this.group1.yzx), other.e321 * this.e5) - float4((this.group2.xyz * other.group1.www), other.e235 * this.e1);
        float wedge_g10 = (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiPlane anti_reject_via_horizon_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(other.e4235 * other.e4235 * this.e1) - (other.e4235 * other.e4315 * this.e2) - (other.e4235 * other.e4125 * this.e3) - (other.e4235 * other.e3215 * this.e4), -(other.e4315 * other.e4315 * this.e2) - (other.e4235 * other.e4315 * this.e1) - (other.e4315 * other.e4125 * this.e3) - (other.e4315 * other.e3215 * this.e4), -(other.e4125 * other.e4125 * this.e3) - (other.e4235 * other.e4125 * this.e1) - (other.e4315 * other.e4125 * this.e2) - (other.e4125 * other.e3215 * this.e4), (other.e3215 * other.e3215 * this.e4) + (other.e4235 * other.e3215 * this.e1) + (other.e4315 * other.e3215 * this.e2) + (other.e4125 * other.e3215 * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (this.group3.yzxw * float4(other.group0.zxy, other.e5)) - (other.group0.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = float4(other.e5 * this.e1, other.e5 * this.e2, other.e5 * this.e3, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = float4(-(other.e3 * this.e425) - (other.e4 * this.e235), -(other.e1 * this.e435) - (other.e4 * this.e315), -(other.e2 * this.e415) - (other.e4 * this.e125), (other.e3 * this.e125) + (other.e5 * this.e321)) + (float4(other.e5, other.e5, other.e5 * this.e412, other.e2 * this.e315) * float4(this.group0.xy, 1.0, 1.0)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group0.xyz) - (float3(right_dual_g1) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group0.yzx), right_dual_g0_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group0.zxy, right_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0_w * wedge_g3[0], right_dual_g0_w * wedge_g3[1], right_dual_g0_w * wedge_g3[2], -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group0.xyz * wedge_g3.www), right_dual_g1 * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * wedge_g2[0]) - (wedge_g1[2] * other.e2), -(right_dual_g1 * wedge_g2[1]) - (wedge_g1[0] * other.e3), -(right_dual_g1 * wedge_g2[2]) - (wedge_g1[1] * other.e1), (wedge_g2[1] * other.e2) + (wedge_g2[2] * other.e3)) + (float4(right_dual_g0_w) * float4(wedge_g0, wedge_g1[3])) + (other.group0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_via_horizon_from(Scalar other) {
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
public RoundPoint operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_via_horizon_from(Sphere other) {
        float wedge_g0 = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_via_horizon_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (this.group3.yzxw * float4(other.group3.zxy, other.e5)) - (other.group3.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group3.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e5 * this.e321) + float4((other.group2.xyz * this.group3.www), other.e1 * this.e235) + float4((this.group0.xyz * other.group2.www), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e315 * this.e2) - float4((this.group2.xyz * other.group3.www), other.e125 * this.e3);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * other.group3.xyz) - (float3(right_dual_g2_w) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3.zxy * other.group3.yzx), right_dual_g3_w * wedge_g2[3]) - (wedge_g3.yzxw * float4(other.group3.zxy, right_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            float4((float2(right_dual_g3_w) * wedge_g3.xy), right_dual_g3_w * wedge_g3[2], right_dual_g1_w * wedge_g2[3]) + float4(float3(0.0), (wedge_g3[0] * other.e423) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((other.group3.xyz * wedge_g3.www), right_dual_g2_w * wedge_g1[3]), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3_w) * float4(wedge_g0, wedge_g1[3])) + (wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e3) - (wedge_g3.zxyy * float4(other.group1.yzx, other.e315)) - (wedge_g3.wwwz * float4(other.group0.xyz, other.e125)) - float4((float2(right_dual_g2_w) * wedge_g2.xy), right_dual_g2_w * wedge_g2[2], right_dual_g1_w * wedge_g3[3]) - float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectViaHorizonFromInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_via_horizon_from(VersorOdd other) {
        float4 wedge_g0 = (float4(other.scalar) * this.group0) + (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.zxy * other.group0.yzx), this.e5 * other.e1234) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15);
        float4 wedge_g1 = float4((this.e5 * other.e41) + (this.e4 * other.e15), (this.e5 * other.e42) + (this.e4 * other.e25), (this.e5 * other.e43) + (this.e4 * other.e35), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * wedge_g0.xyz)), wedge_g0[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(other.scalar)), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(right_dual_g1[0] * wedge_g2[0]) - (right_dual_g1[1] * wedge_g2[1]) - (right_dual_g1[2] * wedge_g2[2]) - (right_dual_g2[0] * wedge_g1[0]) - (right_dual_g2[1] * wedge_g1[1]) - (right_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[3] * wedge_g1[0]) + (right_dual_g2[1] * wedge_g0[2]) + (wedge_g2[1] * other.e43) + (wedge_g3[0] * other.scalar), (right_dual_g1[3] * wedge_g1[1]) + (right_dual_g2[2] * wedge_g0[0]) + (wedge_g2[2] * other.e41) + (wedge_g3[1] * other.scalar), (right_dual_g1[3] * wedge_g1[2]) + (right_dual_g2[0] * wedge_g0[1]) + (wedge_g2[0] * other.e42) + (wedge_g3[2] * other.scalar), -(right_dual_g1[2] * wedge_g0[2]) - (wedge_g1[0] * other.e41) - (wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + (float4(wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)) + float4((right_dual_g1.xyz * wedge_g1.www), wedge_g3[3] * other.scalar) - (wedge_g0.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])) - float4((wedge_g2.zxy * other.group0.yzx), right_dual_g1[1] * wedge_g0[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float4 wedge_g2 = float4(other.scalar * this.e15, other.scalar * this.e25, other.scalar * this.e35, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group2 * float4(this.group0.www, this.e1234));
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0_xyz[0] * other.scalar) + (wedge_g2[3] * other.e23), (wedge_g0_xyz[1] * other.scalar) + (wedge_g2[3] * other.e31), (wedge_g0_xyz[2] * other.scalar) + (wedge_g2[3] * other.e12), -(right_dual_g0[1] * wedge_g2[1]) - (right_dual_g0[2] * wedge_g2[2]) - (wedge_g0_xyz[0] * other.e15) - (wedge_g0_xyz[1] * other.e25) - (wedge_g0_xyz[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], other.scalar * other.scalar * this.scalar) * float4(right_dual_g0.zxy, 1.0)) - float4((right_dual_g0.yzx * wedge_g3.zxy), right_dual_g0[0] * wedge_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g2[3] * other.e15), (wedge_g1[1] * other.scalar) + (wedge_g2[3] * other.e25), (wedge_g1[2] * other.scalar) + (wedge_g2[3] * other.e35), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g0 * wedge_g3.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiDipoleInversion other) {
        float4 wedge_g0 = (other.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.xyz * this.group0.www), other.e4 * this.e3215) + float4((this.group1.xyz * other.group2.www), other.e1 * this.e4235) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], right_dual_g3[0] * wedge_g3[0]) + float4(float3(0.0), (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * other.e5 * this.scalar) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[2] * wedge_g3[2]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar, other.scalar, other.scalar, (wedge_g0[3] * other.scalar) + (wedge_g2[3] * other.e3215)) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.scalar * other.scalar) * this.group3.xyz), (other.scalar * other.scalar * this.e3215) + (other.e3215 * other.scalar * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiFlatPoint other) {
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) * float4(other.group0.xyz, 1.0);
        float right_dual_g0_w = other.e321 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g0_w * wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            other.group0 * float4(wedge_g2.www, right_dual_g0_w * this.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiFlector other) {
        float4 wedge_g0 = (other.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e235));
        float4 wedge_g1 = (this.group0 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx)), other.e5 * this.scalar);
        float3 wedge_g3_xyz = float3(this.scalar) * other.group1.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * right_dual_g1[1]) + (wedge_g3_xyz[2] * right_dual_g1[2]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - float4((right_dual_g1.zxy * wedge_g0.yzx), right_dual_g0_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(right_dual_g1.www, right_dual_g0_w)) + float4(float3(0.0), -(right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2])) - (right_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((right_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(AntiLine other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float4(other.group1.xyz, 1.0);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_dual_g0 * float3(wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * float3(wedge_g2[3])), -(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g0[0] * wedge_g3[3]) + (right_dual_g1[1] * wedge_g3[2]), (right_dual_g0[1] * wedge_g3[3]) + (right_dual_g1[2] * wedge_g3[0]), (right_dual_g0[2] * wedge_g3[3]) + (right_dual_g1[0] * wedge_g3[1]), -(right_dual_g0[1] * wedge_g1_xyz[1]) - (right_dual_g0[2] * wedge_g1_xyz[2])) - float4((right_dual_g1.zxy * wedge_g3.yzx), right_dual_g0[0] * wedge_g1_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.e45);
        float4 wedge_g2 = float4(other.e15 * this.scalar, other.e25 * this.scalar, other.e35 * this.scalar, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float4(other.scalar) * this.group2);
        float4 wedge_g3 = float4(other.e35 * this.e42, other.e15 * this.e43, other.e25 * this.e41, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group1.www, this.e3215)) + float4((this.group3.xyz * other.group0.www), other.e3215 * this.scalar) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0 * float4(wedge_g2.www, wedge_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35)) + float4((wedge_g0.xyz * right_dual_g0.www), wedge_g2[3] * other.e3215), 
            /* e23, e31, e12, e45 */
            float4(wedge_g2[3] * other.e15, wedge_g2[3] * other.e25, wedge_g2[3] * other.e35, -(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2])) + (wedge_g1 * float4(right_dual_g0[3])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_dual_g0.xyz) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), right_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(AntiPlane other) {
        float4 wedge_g0 = float4(other.e2 * this.e43 * -1.0, other.e3 * this.e41 * -1.0, other.e1 * this.e42 * -1.0, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235));
        float4 wedge_g1 = float4(other.e5 * this.e41, other.e5 * this.e42, other.e5 * this.e43, -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4(((float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx)), other.e5 * this.scalar);
        float3 wedge_g3_xyz = float3(this.scalar) * other.group0.xyz;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[2] * wedge_g0[1] * -1.0, right_dual_g0[0] * wedge_g0[2] * -1.0, right_dual_g0[1] * wedge_g0[0] * -1.0, (wedge_g3_xyz[1] * right_dual_g0[1]) + (wedge_g3_xyz[2] * right_dual_g0[2])) + (right_dual_g0.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3] * wedge_g0[0], right_dual_g0[3] * wedge_g0[1], right_dual_g0[3] * wedge_g0[2], -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (right_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * wedge_g1.xyz), 0.0) + float4((right_dual_g0.zxy * wedge_g2.yzx), 0.0) - float4((right_dual_g0.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_via_horizon_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) * float4(other.group2.xyz, 1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * wedge_g2.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) * float4(other.group2.xyz, 1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * wedge_g2.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (right_dual_g1[0] * wedge_g1[0]) - (right_dual_g1[1] * wedge_g1[1]) - (right_dual_g1[2] * wedge_g1[2]) - (right_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g2[3] * other.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float4(other.group2.xyz, 1.0);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2[3] * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2[3] * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e25) + (wedge_g3[3] * other.e23), (wedge_g3[0] * other.e35) + (wedge_g3[3] * other.e31), (wedge_g3[1] * other.e15) + (wedge_g3[3] * other.e12), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_via_horizon_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar, this.scalar, this.scalar, (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float4(other.group2.xyz, 1.0);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e15) + (wedge_g3[3] * other.e41), (wedge_g2[3] * other.e25) + (wedge_g3[3] * other.e42), (wedge_g2[3] * other.e35) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g3.zxy * other.group2.yzx), wedge_g2[3] * other.e3215) - (other.group2.zxyx * float4(wedge_g3.yzx, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_via_horizon_from(DualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e5, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.e12345), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_via_horizon_from(FlatPoint other) {
        float4 wedge_g1 = float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, other.e45 * other.e45 * this.scalar) * float4((wedge_g1.xyz * float3(-1.0)), 1.0) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4((other.e45 * this.e23) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e4125 * this.scalar), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group0.yzxw * float4(other.group0.zxy, other.e3215)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45, other.e45, other.e45, (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.scalar)) * float4((wedge_g1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4(this.scalar, this.scalar, this.scalar, -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) * float4(other.group0.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = (float4(other.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43));
        float right_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g0[3] * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1_w) * wedge_g0.xyz) + (float3(wedge_g0[3]) * other.group1.xyz)), right_dual_g1_w * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = other.e5 * this.scalar;
        float4 wedge_g3 = (float4(other.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * other.group3);
        float3 wedge_g4 = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group4.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group5.xyz);
        float4 wedge_g6 = float4((other.e5 * this.e41) + (other.e415 * this.scalar), (other.e5 * this.e42) + (other.e425 * this.scalar), (other.e5 * this.e43) + (other.e435 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group2.xyz * other.group1.www), other.e321 * this.scalar) - (other.group1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group7.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx);
        float4 wedge_g9 = float4((other.e45 * this.e23) + (other.e42 * this.e35) + (other.e23 * this.e45) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e4125 * this.scalar), -(other.e35 * this.e12) - (other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) + (float4(other.scalar) * this.group3) + (this.group0.yzxw * float4(other.group3.zxy, other.e3215)) - (other.group3.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group4.zxy * this.group2.yzx), other.e25 * this.e31);
        float wedge_g10 = (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * wedge_g2) + (wedge_g0_y * right_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_dual_g1[0] * wedge_g9[0]) + (right_dual_g1[1] * wedge_g9[1]) + (right_dual_g1[2] * wedge_g9[2]) + (right_dual_g1[3] * wedge_g9[3]) + (right_dual_g9[0] * wedge_g1[0]) + (right_dual_g9[1] * wedge_g1[1]) + (right_dual_g9[2] * wedge_g1[2]) + (right_dual_g9[3] * wedge_g1[3]) + (right_dual_g0[1] * other.scalar * this.scalar) - (right_dual_g6_xyz[0] * wedge_g5[0]) - (right_dual_g6_xyz[1] * wedge_g5[1]) - (right_dual_g6_xyz[2] * wedge_g5[2]) - (right_dual_g7[0] * wedge_g3[0]) - (right_dual_g7[1] * wedge_g3[1]) - (right_dual_g7[2] * wedge_g3[2]) - (right_dual_g8[0] * wedge_g4[0]) - (right_dual_g8[1] * wedge_g4[1]) - (right_dual_g8[2] * wedge_g4[2]) - (wedge_g7[0] * right_dual_g3[0]) - (wedge_g7[1] * right_dual_g3[1]) - (wedge_g7[2] * right_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_dual_g3[3] * wedge_g6[3]) - (wedge_g3[3] * other.e45) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_dual_g9[3]) + (wedge_g5[1] * right_dual_g9[2]) + (wedge_g6[0] * other.e45) + (wedge_g9[1] * other.e435), (wedge_g4[1] * right_dual_g9[3]) + (wedge_g5[2] * right_dual_g9[0]) + (wedge_g6[1] * other.e45) + (wedge_g9[2] * other.e415), (wedge_g4[2] * right_dual_g9[3]) + (wedge_g5[0] * right_dual_g9[1]) + (wedge_g6[2] * other.e45) + (wedge_g9[0] * other.e425), -(right_dual_g7[2] * wedge_g6[2]) - (wedge_g4[0] * right_dual_g9[0]) - (wedge_g4[1] * right_dual_g9[1]) - (wedge_g4[2] * right_dual_g9[2])) + (right_dual_g1 * float4(wedge_g0_y)) + (right_dual_g3 * float4(wedge_g10)) + (wedge_g1 * float4(right_dual_g0[1])) + float4((right_dual_g6_xyz * wedge_g6.www), wedge_g9[0] * other.e423) + float4((right_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (wedge_g3 * float4(right_dual_g10)) - float4((other.group7.xyz * wedge_g9.www), right_dual_g7[1] * wedge_g6[1]) - float4((right_dual_g7.yzx * wedge_g8.zxy), right_dual_g6_xyz[0] * wedge_g7[0]) - float4((right_dual_g8.zxy * wedge_g7.yzx), right_dual_g6_xyz[1] * wedge_g7[1]) - float4((wedge_g5.zxy * right_dual_g9.yzx), right_dual_g6_xyz[2] * wedge_g7[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_dual_g7[0] * wedge_g6[0]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_dual_g0[1]) + (right_dual_g9[0] * wedge_g3[0]) + (right_dual_g9[1] * wedge_g3[1]) + (right_dual_g9[2] * wedge_g3[2]) + (right_dual_g9[3] * wedge_g3[3]) - (right_dual_g6_xyz[0] * wedge_g8[0]) - (right_dual_g6_xyz[1] * wedge_g8[1]) - (right_dual_g6_xyz[2] * wedge_g8[2]) - (right_dual_g8[0] * wedge_g6[0]) - (right_dual_g8[1] * wedge_g6[1]) - (right_dual_g8[2] * wedge_g6[2]) - (right_dual_g3[0] * wedge_g9[0]) - (right_dual_g3[1] * wedge_g9[1]) - (right_dual_g3[2] * wedge_g9[2]) - (right_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g6_xyz[0] * wedge_g9[3]) + (right_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_dual_g9[2]) + (right_dual_g9[3] * wedge_g6[0]), (right_dual_g6_xyz[1] * wedge_g9[3]) + (right_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_dual_g9[0]) + (right_dual_g9[3] * wedge_g6[1]), (right_dual_g6_xyz[2] * wedge_g9[3]) + (right_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_dual_g9[1]) + (right_dual_g9[3] * wedge_g6[2]), -(right_dual_g6_xyz[2] * wedge_g9[2]) - (right_dual_g9[0] * wedge_g6[0]) - (right_dual_g9[1] * wedge_g6[1]) - (right_dual_g9[2] * wedge_g6[2])) + (right_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_dual_g0[1])) - (wedge_g9.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0])) - float4((wedge_g8.zxy * right_dual_g9.yzx), right_dual_g6_xyz[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (right_dual_g6_xyz * float3(wedge_g10)) + (wedge_g4 * float3(right_dual_g0[1])) + (float3(right_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(wedge_g9[3])) + (right_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_dual_g0[1])) + (wedge_g7 * float3(right_dual_g9[3])) + (wedge_g8 * float3(right_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(wedge_g6[3]) * right_dual_g9.xyz) - (float3(other.e45) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * float4(right_dual_g6_xyz, other.e45)) + (float4(right_dual_g0.yy, right_dual_g0[1], wedge_g10 * right_dual_g9[3]) * float4(wedge_g6.xyz, 1.0)) + float4((right_dual_g9.yzx * wedge_g9.zxy), right_dual_g0[1] * wedge_g6[3]) - (wedge_g9.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_dual_g0[1])) + (float3(wedge_g10) * right_dual_g9.xyz) - (float3(right_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_dual_g0[1])) + (float3(right_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_dual_g0[1])), 
            /* e1234 */
            (right_dual_g10 * wedge_g0_y) + (wedge_g10 * right_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(RoundPoint other) {
        float4 wedge_g0 = float4(other.e2 * this.e43 * -1.0, other.e3 * this.e41 * -1.0, other.e1 * this.e42 * -1.0, (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315));
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[2] * wedge_g0[1] * -1.0, right_dual_g0[0] * wedge_g0[2] * -1.0, right_dual_g0[1] * wedge_g0[0] * -1.0, (right_dual_g0[2] * wedge_g3[2]) + (right_dual_g0[3] * wedge_g3[3]) + (right_dual_g1 * other.e5 * this.scalar)) + (right_dual_g0.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4((float2(right_dual_g1) * wedge_g1.xy), right_dual_g1 * wedge_g1[2], right_dual_g0[0] * wedge_g3[0]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * wedge_g2_xyz[0]) + (right_dual_g0[3] * wedge_g0[0]), (right_dual_g1 * wedge_g2_xyz[1]) + (right_dual_g0[3] * wedge_g0[1]), (right_dual_g1 * wedge_g2_xyz[2]) + (right_dual_g0[3] * wedge_g0[2]), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (right_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * right_dual_g0.zxy) - (wedge_g2_xyz.zxy * right_dual_g0.yzx)), right_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(Scalar other) {
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
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_via_horizon_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2]) + (wedge_g0[3] * other.e1234) + (other.e3215 * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(VersorEven other) {
        float4 wedge_g0 = (float4(this.scalar) * other.group0) + (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.zxy * this.group0.yzx), other.e5 * this.e1234) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15);
        float4 wedge_g1 = float4((other.e5 * this.e41) + (other.e4 * this.e15), (other.e5 * this.e42) + (other.e4 * this.e25), (other.e5 * this.e43) + (other.e4 * this.e35), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((float2(right_dual_g2_w) * wedge_g1.xy), right_dual_g2_w * wedge_g1[2], wedge_g0[3] * other.e12345 * -1.0) + float4(float3(0.0), (right_dual_g3[2] * wedge_g3[2]) + (right_dual_g3[3] * wedge_g3[3]) + (right_dual_g2_w * other.e5 * this.scalar) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[1] * wedge_g3[1]) - float4((right_dual_g3.zxy * wedge_g0.yzx), right_dual_g1_w * wedge_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3[3] * wedge_g0[0]) + (wedge_g0[3] * other.e415), (right_dual_g3[3] * wedge_g0[1]) + (wedge_g0[3] * other.e425), (right_dual_g3[3] * wedge_g0[2]) + (wedge_g0[3] * other.e435), -(right_dual_g3[1] * wedge_g1[1]) - (right_dual_g3[2] * wedge_g1[2])) + float4((float2(right_dual_g2_w) * wedge_g2_xyz.xy), right_dual_g2_w * wedge_g2_xyz[2], right_dual_g1_w * wedge_g0[3]) - (right_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * other.group2.xyz) + (wedge_g2_xyz.yzx * right_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_dual_g3.yzx)), right_dual_g2_w * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectViaHorizonFromInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_via_horizon_from(VersorOdd other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float4 wedge_g2 = (float4(other.scalar) * this.group2) + (float4(this.scalar) * other.group2) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((this.group3.xyz * other.group0.www), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxz * float4(other.group0.zxy, right_dual_g3_xyz[2])) + float4(float3(0.0), (wedge_g2[3] * other.e3215) + (wedge_g3[3] * other.e1234) + (other.scalar * other.scalar * this.scalar) - (right_dual_g2_xyz[1] * wedge_g0_xyz[1]) - (right_dual_g2_xyz[2] * wedge_g0_xyz[2]) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0_xyz * other.group0.www), right_dual_g3_xyz[0] * wedge_g3[0]) + float4((other.group1.xyz * wedge_g2.www), right_dual_g3_xyz[1] * wedge_g3[1]) - float4((wedge_g3.zxy * other.group0.yzx), right_dual_g2_xyz[0] * wedge_g0_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.scalar) + (wedge_g3[3] * other.e41), (wedge_g1[1] * other.scalar) + (wedge_g3[3] * other.e42), (wedge_g1[2] * other.scalar) + (wedge_g3[3] * other.e43), -(wedge_g3[1] * other.e31) - (wedge_g3[2] * other.e12)) + float4((right_dual_g2_xyz * wedge_g2.www), wedge_g1[3] * other.scalar) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.scalar) * wedge_g2.xyz) + (right_dual_g2_xyz.yzx * wedge_g3.zxy) - (right_dual_g2_xyz.zxy * wedge_g3.yzx)), wedge_g2[3] * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.scalar)
        );
    }
}
