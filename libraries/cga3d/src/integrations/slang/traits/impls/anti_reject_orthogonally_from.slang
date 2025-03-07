implementing cga3d;
using traits;
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float wedge_g2_x = (other.e15 * this.scalar) + (other.scalar * this.e15);
        float wedge_g2_y = (other.e25 * this.scalar) + (other.scalar * this.e25);
        float wedge_g2_z = (other.e35 * this.scalar) + (other.scalar * this.e35);
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * right_anti_dual_g1[0]) + (wedge_g0_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[1]) + (wedge_g0_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[2]) + (wedge_g0_xyz[2] * right_anti_dual_g2[3]), -(wedge_g2_y * right_anti_dual_g0[1]) - (wedge_g2_z * right_anti_dual_g0[2]) - (wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((right_anti_dual_g0.zxy * wedge_g3.yzx), right_anti_dual_g2[3] * other.scalar * this.scalar) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), wedge_g2_x * right_anti_dual_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g2[3] * wedge_g1[0]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g2[3] * wedge_g1[1]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), right_anti_dual_g2[3] * wedge_g1[3]) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2[3]) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e31 * other.e4) + (this.scalar * other.e431), (this.e41 * other.e2) + (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group3.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((other.group0.xyz * wedge_g0.www), right_anti_dual_g2[3] * this.scalar * other.e5) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g3.zxy * wedge_g0.yzx), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[0]), (wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[1]), (wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (right_anti_dual_g1 * float4(wedge_g0[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.scalar) * float4(this.group0.xyz, this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.scalar, wedge_g0[3] * other.scalar) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz * float3(other.group0.yy, other.scalar) * float3(other.group0.yy, other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e3215 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiFlatPoint other) {
        float wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * right_anti_dual_g0.xyz), right_anti_dual_g0[3] * this.scalar * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.e42 * other.e3, this.e43 * other.e1, this.e41 * other.e2, -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group0.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float3 wedge_g3_xyz = other.group1.xyz * this.group2.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * right_anti_dual_g1[1]) + (wedge_g3_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g1[3])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) - (right_anti_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), 0.0) + float4((wedge_g2_xyz.yzx * right_anti_dual_g1.zxy), 0.0) - float4((wedge_g2_xyz.zxy * right_anti_dual_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g1_xyz = other.group0.xyz * this.group2.www;
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e45 * other.e23), (this.e43 * other.e15) + (this.e45 * other.e31), (this.e41 * other.e25) + (this.e45 * other.e12), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group1.yzx), this.e15 * other.e23);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(wedge_g2_w)), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g0[0] * wedge_g3[3]) + (right_anti_dual_g1[1] * wedge_g3[2]), (right_anti_dual_g0[1] * wedge_g3[3]) + (right_anti_dual_g1[2] * wedge_g3[0]), (right_anti_dual_g0[2] * wedge_g3[3]) + (right_anti_dual_g1[0] * wedge_g3[1]), -(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * float4(this.group0.xyz, this.scalar);
        float4 wedge_g1 = float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e45 * other.scalar);
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12);
        float4 wedge_g3 = (other.group1.zxyw * float4(this.group0.yzx, this.scalar)) + float4((float3(this.e45) * other.group0.xyz), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0 * float4(float3(wedge_g2_w), wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((wedge_g0.xyz * right_anti_dual_g0.www), wedge_g2_w * right_anti_dual_g1[3]), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(right_anti_dual_g0[3])) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * float3((this.e15 * other.scalar) + (this.scalar * other.e15), (this.e25 * other.scalar) + (this.scalar * other.e25), (this.e35 * other.scalar) + (this.scalar * other.e35))) + (float3(wedge_g3[3]) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * wedge_g3.zxy) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float4 wedge_g2 = float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0 * float3(right_anti_dual_g0[3])), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], 1.0) * float4(right_anti_dual_g0.yzx, (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(right_anti_dual_g0[3]) * float4(wedge_g1.xyz, wedge_g2[3])) + (right_anti_dual_g0.zxyx * float4(wedge_g2.yzx, wedge_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e12345, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = other.group2.xyz * this.group2.www;
        float wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * right_anti_dual_g2.xyz), (wedge_g2_w * right_anti_dual_g2[3]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group2.yzx), this.e23 * other.e15) - float4((other.group0.zxy * this.group2.yzx), this.e31 * other.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g2[1] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g3[3]), (right_anti_dual_g2[2] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g3[3]), (right_anti_dual_g2[0] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g3[3]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = other.group2.xyz * this.group2.www;
        float wedge_g2_w = (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group0.zxy * this.group2.yzx), this.e31 * other.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (wedge_g2_w * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) - (right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_orthogonally_from(DualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e5, other.e12345) * float4(this.group0.xyz, this.scalar);
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual_g0[1] * wedge_g0[3]), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g1 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0[3]) * float4(wedge_g1.xyz, this.scalar * other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g1 = float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((this.group0.yzx * other.group0.zxy), this.scalar * other.e3215) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * this.scalar * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float3 wedge_g0_xyz = (float3(this.scalar) * other.group0.xyz) + (this.group0.xyz * other.group1.www);
        float wedge_g0_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), (wedge_g0_w * right_anti_dual_g0[3]) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = this.scalar * other.e5;
        float4 wedge_g3 = (float4(this.scalar) * other.group3) + (float4(other.scalar) * float4(this.group2.xyz, this.e45));
        float3 wedge_g4 = (float3(this.scalar) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz);
        float3 wedge_g5 = (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group3.zxy), this.scalar * other.e3215) - (this.group2.yzxy * float4(other.group4.zxy, other.e31)) - float4((this.group0.zxy * other.group3.yzx), this.e15 * other.e23);
        float wedge_g10 = (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) + (right_anti_dual_g0[1] * this.scalar * other.scalar) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group0.xyz, other.e5);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2_xyz[0]) + (wedge_g0[0] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2_xyz[1]) + (wedge_g0[1] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2_xyz[2]) + (wedge_g0[2] * right_anti_dual_g0[3]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual_g0[3]) * float4(wedge_g1.xyz, this.scalar * other.e4)) + (right_anti_dual_g0.zxyx * float4(wedge_g2_xyz.yzx, wedge_g3[0])) + float4((wedge_g2_xyz.zxy * right_anti_dual_g0.yzx * float3(-1.0)), (right_anti_dual_g1 * wedge_g3[3]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_reject_orthogonally_from(Scalar other) {
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
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_reject_orthogonally_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.yzx * other.group3.zxy), this.scalar * other.e12345) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235);
        float4 wedge_g1 = float4((this.e15 * other.e4) + (this.scalar * other.e415), (this.e25 * other.e4) + (this.scalar * other.e425), (this.e35 * other.e4) + (this.scalar * other.e435), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.xyz * other.group2.www), this.scalar * other.e321) - (this.group1.wwwx * other.group3.xyzx);
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), right_anti_dual_g2[3] * this.scalar * other.e5) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (wedge_g0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g1[0] * wedge_g0[3]), (wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g1[1] * wedge_g0[3]), (wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g1[2] * wedge_g0[3]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0_xyz = (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group0.xyz);
        float4 wedge_g1 = (float4(this.scalar) * other.group1) + (float4(other.scalar) * this.group1);
        float4 wedge_g2 = (this.group2 * float4(other.group0.www, other.e1234)) + float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43));
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.scalar * other.e4125), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.scalar * other.e3215) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.zxyw * float4(wedge_g3.yzx, this.scalar * other.scalar)) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0_xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) + float4((right_anti_dual_g1.xyz * wedge_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (right_anti_dual_g0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[3] * wedge_g1[0]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[1]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[2]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (right_anti_dual_g0 * float4(wedge_g3.www, wedge_g1[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4((other.e42 * this.e3) + (other.e23 * this.e4) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e31 * this.e4) + (other.scalar * this.e431), (other.e41 * this.e2) + (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group3.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(wedge_g0[3])) + (float3(right_anti_dual_g2[3]) * wedge_g0.xyz)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0.www, other.scalar * this.e5)) + float4((wedge_g2_xyz * right_anti_dual_g2.www), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g2[3] * wedge_g3[0]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g2[3] * wedge_g3[1]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g2[3] * wedge_g3[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((right_anti_dual_g0.zxy * wedge_g2_xyz.yzx), right_anti_dual_g2[3] * wedge_g3[3]) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (other.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(other.group3.yzx, other.e4));
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((other.group3.xyz * this.group3.www), other.e1 * this.e423);
        float4 wedge_g3 = (other.group3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(other.group2.xyz, other.e3)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235) + float4((other.group1.yzx * this.group3.zxy), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * wedge_g2[3]) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) - float4((right_anti_dual_g3.xyz * wedge_g3.www), wedge_g0[0] * right_anti_dual_g3[0]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g3.zxyw * wedge_g1.yzxw) + (wedge_g2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g3[0] * wedge_g2[0]) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[1] * wedge_g2[1]) - (float4(wedge_g3[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[2] * wedge_g3[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.yy, other.scalar, other.e3215) * float4(this.group0.xyz, this.e4);
        float4 wedge_g2 = float4(other.scalar) * float4(this.group2.xyz, this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g2[3] * other.scalar)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group2.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e4 * other.e321), 
            /* e5 */
            (right_anti_dual_g0[3] * this.e1 * other.e235) + (right_anti_dual_g0[3] * this.e2 * other.e315) + (right_anti_dual_g0[3] * this.e3 * other.e125) + (right_anti_dual_g0[3] * this.e5 * other.e321) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx);
        float4 wedge_g2 = float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((this.group3.xyz * other.group1.www), this.e4 * other.e321) - (other.group1.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * wedge_g3.zxy) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), right_anti_dual_g1[3] * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) + float4((wedge_g3.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * wedge_g2[3]) - (right_anti_dual_g1.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1[3]) * float4(wedge_g0, this.e4 * other.e5)) + (right_anti_dual_g1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (right_anti_dual_g0[3] * wedge_g3[3])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * wedge_g2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g1 * float3(wedge_g2[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (wedge_g1_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (wedge_g1_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1_xyz[0]) - (right_anti_dual_g1[1] * wedge_g1_xyz[1]) - (right_anti_dual_g1[2] * wedge_g1_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g0.yzx), right_anti_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(((float3(this.e4) * other.group0.xyz) + (this.group0.xyz * other.group0.www)), (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12));
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + float4((float3(this.e4) * other.group1.xyz), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12));
        float4 wedge_g2 = float4(((float3(this.e5) * other.group0.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[3] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (right_anti_dual_g0[3] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (right_anti_dual_g0[3] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (right_anti_dual_g0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx);
        float4 wedge_g2 = float4((float3(other.e5) * this.group3.xyz), -(this.e431 * other.e2) - (this.e412 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(this.e425 * other.e3, this.e435 * other.e1, this.e415 * other.e2, (this.e315 * other.e2) + (this.e125 * other.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * right_anti_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g3.zxy) - (right_anti_dual_g0.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            (float4(wedge_g3[0], wedge_g3[1], wedge_g3[2], 1.0) * float4(right_anti_dual_g0.www, -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))) - (right_anti_dual_g0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0[3]) * float4(wedge_g0, this.e4 * other.e5)) + (right_anti_dual_g0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g1_xyz.zxy * right_anti_dual_g0.yzx * float3(-1.0)), (right_anti_dual_g0[1] * wedge_g2[1]) + (right_anti_dual_g0[2] * wedge_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group3.yzx * other.group1.zxy), this.e2 * other.e315);
        float wedge_g1 = (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group3.www), this.e1 * other.e235);
        float wedge_g1 = (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g1) * right_anti_dual_g2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((other.group2.yzx * this.group3.zxy), this.e423 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(wedge_g2[3])), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g2[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g2[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.yzx * other.group2.zxy), this.e4 * other.e3215) - (other.group2.yzxx * float4(this.group3.zxy, this.e423));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (wedge_g2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (wedge_g2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float4(other.e5) * float4(this.group0.xyz, this.e321)).xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0])), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual_g0[0] * this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g0 = float4((float3(this.e4) * other.group0.xyz), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.zxy * wedge_g0.yzx), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g1 = float4((this.e4 * other.e235) + (this.e3 * other.e425), (this.e4 * other.e315) + (this.e1 * other.e435), (this.e4 * other.e125) + (this.e2 * other.e415), -(this.e2 * other.e315) - (this.e3 * other.e125)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1.yzx), right_anti_dual_g1[3] * this.e4 * other.e5) - (wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g1 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = (float4(other.e5) * float4(this.group3.xyz, this.e4)) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz);
        float3 wedge_g5 = (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx);
        float4 wedge_g6 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - float4((this.group3.xyz * other.group3.www), this.e1 * other.e23);
        float3 wedge_g7 = (float3(this.e4) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group3.zxy) - (this.group3.zxy * other.group3.yzx);
        float4 wedge_g9 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group2.www), this.e235 * other.e1) + float4((this.group3.zxy * other.group6.yzx), this.e125 * other.e3) - (this.group3.yzxw * other.group6.zxyw) - (this.group3.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315) - float4((this.group2.xyz * other.group1.www), this.e3 * other.e125);
        float wedge_g10 = (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = float4((this.group3.yzx * other.group0.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group0.yzxw);
        float4 wedge_g2 = float4((float3(other.e5) * this.group3.xyz), -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g1)), 
            /* e235, e315, e125, e4 */
            (float4(wedge_g3[0], wedge_g3[1], wedge_g3[2], 1.0) * float4(right_anti_dual_g0.www, -(right_anti_dual_g1 * wedge_g1[3]) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))) - (right_anti_dual_g0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(-(right_anti_dual_g1 * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g1[2]), -(right_anti_dual_g1 * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g1[0]), -(right_anti_dual_g1 * wedge_g2[2]) - (right_anti_dual_g0[0] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[1]) + (right_anti_dual_g0[2] * wedge_g2[2])) + (float4(right_anti_dual_g0[3]) * float4(wedge_g0, wedge_g1[3])) + (right_anti_dual_g0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(Scalar other) {
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
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = float4((this.group3.yzx * other.group3.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group3.yzxw);
        float4 wedge_g2 = float4(float3(0.0), (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + float4((this.group3.xyz * other.group2.www), this.e4 * other.e321) - (other.group3.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group2.xyz, other.e2)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group1.yzx), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxz * float4(other.group1.zxy, other.e125)) - float4((this.group1.yzx * other.group3.zxy), this.e1 * other.e235) - float4((this.group2.xyz * other.group3.www), this.e2 * other.e315);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) + float4((wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g0, wedge_g1[3])) + (right_anti_dual_g3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[2] * wedge_g2[2]) - (float4(wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (this.group3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.xyz * other.group0.www), this.e4 * other.e3215) + float4((other.group1.xyz * this.group2.www), this.e1 * other.e4235) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15);
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group3.xyz, this.e4);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (right_anti_dual_g0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * right_anti_dual_g0.www), right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = other.group2.xyz * float3(this.group0.yy, this.scalar);
        float wedge_g3_w = other.scalar * this.e3215;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], 1.0) * float4(right_anti_dual_g2.www, -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]))) + float4(float3(0.0), right_anti_dual_g2[3] * wedge_g0[3]) - float4(float3(0.0), right_anti_dual_g0[0] * wedge_g2_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g3_w)), right_anti_dual_g2[3] * wedge_g1[3]) + float4((wedge_g1.xyz * right_anti_dual_g2.www), 0.0) - (right_anti_dual_g1.wwwx * float4(float3(0.0), wedge_g3_w)), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g3_w) * right_anti_dual_g1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g3_w * right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, this.e3215) * float4(other.group0.xyz, other.e4);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e5);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((other.group0.xyz * wedge_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g3.zxy * wedge_g0.yzx), wedge_g2[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g2[3] * wedge_g2[0]) + (right_anti_dual_g3[3] * wedge_g0[0]), (right_anti_dual_g2[3] * wedge_g2[1]) + (right_anti_dual_g3[3] * wedge_g0[1]), (right_anti_dual_g2[3] * wedge_g2[2]) + (right_anti_dual_g3[3] * wedge_g0[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (right_anti_dual_g1 * float4(wedge_g0[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * float2((other.e3215 * this.scalar) + (other.scalar * this.e3215), other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g1.xyz * wedge_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) + (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1.zxy * wedge_g0.yzx) - (right_anti_dual_g1.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.group0.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group1.xyz, 1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), (right_anti_dual_g0[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g1[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e12345, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[0] * wedge_g3[3]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[1] * wedge_g3[3]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[2] * wedge_g3[3]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g2[3]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_orthogonally_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * other.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e45, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * this.scalar * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) + (right_anti_dual_g0[1] * this.scalar * other.scalar) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_orthogonally_from(RoundPoint other) {
        float wedge_g0_w = this.e3215 * other.e4;
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) + (right_anti_dual_g0[3] * wedge_g3[3]) + (right_anti_dual_g1 * this.scalar * other.e5)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_anti_dual_g1 * wedge_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_reject_orthogonally_from(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * float2(this.e3215 * other.scalar, this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Scalar anti_reject_orthogonally_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (wedge_g0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.group0.yy, this.scalar, (this.e3215 * other.scalar) + (this.scalar * other.e3215)) * float4(other.group3.xyz, 1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g1.xyz * wedge_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) + float4((wedge_g0.xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[3] * wedge_g1[0]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[1]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[2]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (right_anti_dual_g0 * float4(wedge_g3.www, wedge_g1[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g1 = float4(float3(0.0), other.scalar * this.e321);
        float3 wedge_g2_xyz = this.group0.xyz * other.group2.www;
        float wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual_g0 * float3(wedge_g2_w)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (wedge_g2_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (wedge_g2_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * wedge_g2_xyz.yzx), 0.0) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0_xyz = this.group0.xyz * other.group2.www * float3(-1.0);
        float wedge_g0_w = (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321);
        float wedge_g1 = other.e4 * this.e321 * -1.0;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (wedge_g0_xyz * float3(right_anti_dual_g2[3])), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)) - float4((wedge_g0_xyz.yzx * right_anti_dual_g3.zxy), wedge_g0_w * right_anti_dual_g2[3]), 
            /* e235, e315, e125, e4 */
            float4(((wedge_g0_xyz * right_anti_dual_g3.www) - (float3(wedge_g0_w) * right_anti_dual_g3.xyz)), (wedge_g0_w * other.e423) + (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0_xyz[1] * other.e431) + (wedge_g0_xyz[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (wedge_g0_xyz[1] * right_anti_dual_g1[2]), (wedge_g1 * right_anti_dual_g2[1]) + (wedge_g0_xyz[2] * right_anti_dual_g1[0]), (wedge_g1 * right_anti_dual_g2[2]) + (wedge_g0_xyz[0] * right_anti_dual_g1[1]), -(wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2])) - (float4(wedge_g0_w) * float4(other.group0.xyz, right_anti_dual_g1[3])) - float4((wedge_g0_xyz.zxy * right_anti_dual_g1.yzx), wedge_g0_w * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * float4(other.scalar * this.e235, other.scalar * this.e315, other.scalar * this.e125, other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_orthogonally_from(AntiFlector other) {
        float wedge_g0_x = (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g0_x) * (other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), wedge_g0_x * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0.xyz * wedge_g0.www), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_orthogonally_from(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_reject_orthogonally_from(Dipole other) {
        float wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
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
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e321 * other.e45 * -1.0) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float4 wedge_g6 = float4(float3(0.0), this.e321 * other.scalar);
        float3 wedge_g8 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g9 = float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5));
        float wedge_g10 = this.e321 * other.e4 * -1.0;
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g10 * right_anti_dual_g3[3]) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((float3(wedge_g10) * right_anti_dual_g3.xyz), 0.0) + float4((float3(right_anti_dual_g6[3]) * wedge_g6.xyz), 0.0) + float4((float3(wedge_g6[3]) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0_xyz = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float wedge_g0_w = (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5);
        float wedge_g1 = this.e321 * other.e4 * -1.0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g0.xyz) - (wedge_g0_xyz * float3(right_anti_dual_g1)), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)) - float4((wedge_g0_xyz.yzx * right_anti_dual_g0.zxy), right_anti_dual_g1 * wedge_g0_w), 
            /* e235, e315, e125 */
            (wedge_g0_xyz * float3(right_anti_dual_g0[3])) - (float3(wedge_g0_w) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_reject_orthogonally_from(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * float4(this.e235 * other.scalar, this.e315 * other.scalar, this.e125 * other.scalar, this.e321 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0_xyz = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float wedge_g0_w = (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5);
        float wedge_g1 = this.e321 * other.e4 * -1.0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (wedge_g0_xyz * float3(right_anti_dual_g2[3])), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz.zxy * right_anti_dual_g3.yzx) - (wedge_g0_xyz.yzx * right_anti_dual_g3.zxy)), (wedge_g1 * right_anti_dual_g3[3]) - (wedge_g0_w * right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(((wedge_g0_xyz * right_anti_dual_g3.www) - (float3(wedge_g0_w) * right_anti_dual_g3.xyz)), (wedge_g0_w * right_anti_dual_g0[0]) + (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (wedge_g0_xyz[1] * right_anti_dual_g1[2]), (wedge_g1 * right_anti_dual_g2[1]) + (wedge_g0_xyz[2] * right_anti_dual_g1[0]), (wedge_g1 * right_anti_dual_g2[2]) + (wedge_g0_xyz[0] * right_anti_dual_g1[1]), -(wedge_g0_w * right_anti_dual_g1[3]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2])) - (float4(wedge_g0_xyz, wedge_g0_w).wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1])) - float4((wedge_g0_xyz.zxy * right_anti_dual_g1.yzx), wedge_g0_w * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g1 = float4(float3(0.0), this.e321 * other.scalar);
        float4 wedge_g2 = float4((float3(other.scalar) * this.group0.xyz), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g0.xyz), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + float4(float3(0.0), right_anti_dual_g3[3] * wedge_g2[3]) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2, -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group1.www), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float3 wedge_g3_xyz = this.group1.xyz * other.group2.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(wedge_g0[3])) + (float3(right_anti_dual_g2[3]) * wedge_g0.xyz)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0.www, other.scalar * this.e5)) + float4((wedge_g2_xyz * right_anti_dual_g2.www), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g3_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]), (wedge_g3_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]), (wedge_g3_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((right_anti_dual_g0.zxy * wedge_g2_xyz.yzx), 0.0) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy);
        float wedge_g1_w = other.e4 * this.e5 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((other.group3.xyz * this.group1.www), other.e4 * this.e321);
        float4 wedge_g3 = float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * wedge_g2[3]) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g1_w * right_anti_dual_g2[3]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2])) - float4((right_anti_dual_g3.xyz * wedge_g3.www), wedge_g0[0] * right_anti_dual_g3[0]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g3.zxyw * float4(wedge_g1_xyz.yzx, wedge_g1_w)) + (wedge_g2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g3[0] * wedge_g2[0]) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[1] * wedge_g2[1]) - (float4(wedge_g3[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[2] * wedge_g3[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public DualNum anti_reject_orthogonally_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e321 * other.e321 * this.e5) + (other.e235 * other.e321 * this.e1) + (other.e315 * other.e321 * this.e2) + (other.e125 * other.e321 * this.e3), 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0_xyz = (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g1 = (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group1.wwwx * float4(other.group1.xyz, other.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual_g1.xyz * wedge_g1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0])) + float4(float3(0.0), (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2])) - float4((wedge_g0_xyz.zxy * right_anti_dual_g1.yzx), wedge_g1[3] * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_orthogonally_from(AntiLine other) {
        float4 wedge_g0 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.zxyx * float4(other.group1.yzx, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23));
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g0 * float4(wedge_g0.www, wedge_g1[3])) + (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], 1.0) * float4(right_anti_dual_g0.www, -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0_xyz = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 wedge_g1 = (float4((float3(this.e5) * other.group0.xyz), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group1.xyz, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual_g0.xyz * wedge_g1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g0.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0])) + float4((wedge_g0_xyz.zxy * right_anti_dual_g0.yzx * float3(-1.0)), (right_anti_dual_g0[1] * wedge_g1[1]) + (right_anti_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group1.yzx * other.group1.zxy), this.e2 * other.e315);
        float wedge_g1 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group1.www), this.e1 * other.e235);
        float wedge_g1 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g1) * right_anti_dual_g2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4((this.e2 * other.e35) + (this.e5 * other.e23), (this.e3 * other.e15) + (this.e5 * other.e31), (this.e1 * other.e25) + (this.e5 * other.e12), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((other.group2.yzx * this.group1.zxy), this.e235 * other.e41);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(wedge_g2[3])), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g2[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g2[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = (this.group1.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group1.wwwy * float4(other.group1.xyz, other.e4315)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group1.zxy * other.group2.yzx), this.e235 * other.e41);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (wedge_g2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (wedge_g2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g0 = float4(other.e45) * float4(this.group1.xyz, this.e321) * float4(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4((float3(other.e45) * this.group1.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.zxy * wedge_g0.yzx), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g1 = float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group0.yzx), this.e321 * other.e5) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1.yzx), 0.0) - (wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float4 wedge_g1 = float4((this.group1.xyz * float3(other.group0.xx, other.scalar)), 0.0);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0);
        float3 wedge_g4 = float3(other.e4) * this.group1.xyz * float3(-1.0);
        float3 wedge_g5 = (this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx);
        float4 wedge_g6 = float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group4.xyz * this.group1.www), this.e321 * other.scalar) - float4((this.group1.xyz * other.group3.www), this.e1 * other.e23);
        float3 wedge_g7 = (other.group4.yzx * this.group1.zxy) - (other.group4.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float4 wedge_g9 = float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group6.yzx), this.e235 * other.e1) - (this.group1.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group1.yzx * other.group6.zxy), this.e3 * other.e125) - float4((this.group0.xyz * other.group1.www), this.e2 * other.e315);
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e321 * other.e4);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz);
        float wedge_g2_w = this.e321 * other.e4 * -1.0;
        float3 wedge_g3_xyz = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float wedge_g3_w = (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2_w) * right_anti_dual_g0.xyz) - (wedge_g3_xyz * float3(right_anti_dual_g1)), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g3_xyz.zxy, wedge_g2_w)) - float4((wedge_g3_xyz.yzx * right_anti_dual_g0.zxy), right_anti_dual_g1 * wedge_g3_w), 
            /* e235, e315, e125, e4 */
            float4((wedge_g3_xyz * right_anti_dual_g0.www), -(right_anti_dual_g1 * wedge_g1_w) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(-(right_anti_dual_g1 * wedge_g2_xyz[0]) - (wedge_g1_xyz[2] * right_anti_dual_g0[1]), -(right_anti_dual_g1 * wedge_g2_xyz[1]) - (wedge_g1_xyz[0] * right_anti_dual_g0[2]), -(right_anti_dual_g1 * wedge_g2_xyz[2]) - (wedge_g1_xyz[1] * right_anti_dual_g0[0]), (wedge_g2_xyz[1] * right_anti_dual_g0[1]) + (wedge_g2_xyz[2] * right_anti_dual_g0[2])) + (float4(right_anti_dual_g0[3]) * float4(wedge_g0, wedge_g1_w)) + (right_anti_dual_g0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_reject_orthogonally_from(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - (other.group3 * float4(this.group1.www, this.e321));
        float4 wedge_g3 = float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e5 * other.e321)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e1) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - (this.group1.wwwz * float4(other.group0.xyz, other.e125)) - float4((this.group0.xyz * other.group3.www), this.e1 * other.e235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g1_w * right_anti_dual_g2[3]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2])) + float4((wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g0, wedge_g1_w)) + (right_anti_dual_g3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[2] * wedge_g2[2]) - (float4(wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (this.group1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (other.group0.zxyx * float4(this.group1.yzx, this.e235));
        float4 wedge_g1 = (other.group0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (right_anti_dual_g0.zxyw * float4(wedge_g2.yzx, 0.0)) + float4((this.group1.xyz * right_anti_dual_g0.www * other.group0.www), right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiLine> operator /(AntiLine lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g1_xyz = this.group0.xyz * other.group2.www;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group1.xyz;
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e45 * this.e23), (other.e43 * this.e15) + (other.e45 * this.e31), (other.e41 * this.e25) + (other.e45 * this.e12), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group1.yzx), other.e15 * this.e23);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * right_anti_dual_g1[0]) + (right_anti_dual_g0[2] * wedge_g3[1]), (wedge_g2_w * right_anti_dual_g1[1]) + (right_anti_dual_g0[0] * wedge_g3[2]), (wedge_g2_w * right_anti_dual_g1[2]) + (right_anti_dual_g0[1] * wedge_g3[0]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), right_anti_dual_g0[0] * wedge_g2_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (wedge_g1_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (wedge_g1_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (wedge_g1_xyz[2] * right_anti_dual_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), 0.0) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e4) * this.group1.xyz;
        float wedge_g1_w = -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) - float4((this.group1.yzx * other.group3.zxy), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * right_anti_dual_g3[1]) + (wedge_g1_xyz[0] * right_anti_dual_g2[3]) + (wedge_g2[3] * other.e423), (wedge_g0[0] * right_anti_dual_g3[2]) + (wedge_g1_xyz[1] * right_anti_dual_g2[3]) + (wedge_g2[3] * other.e431), (wedge_g0[1] * right_anti_dual_g3[0]) + (wedge_g1_xyz[2] * right_anti_dual_g2[3]) + (wedge_g2[3] * other.e412), -(wedge_g1_w * right_anti_dual_g1[3]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(wedge_g1_xyz[1] * right_anti_dual_g3[1]) - (wedge_g1_xyz[2] * right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g1_xyz, wedge_g1_w).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(right_anti_dual_g3[3])) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, other.e321 * -1.0) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1.zxy * wedge_g0.yzx) - (right_anti_dual_g1.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public FlatPoint anti_reject_orthogonally_from(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(-(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0_xyz = this.group0.xyz * other.group0.www;
        float wedge_g1_w = -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(right_anti_dual_g0[3])), -(wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1_w) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * float3(other.scalar) * this.group1.xyz)), wedge_g1_w * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0[3]) * right_anti_dual_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_orthogonally_from(Circle other) {
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
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
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
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_orthogonally_from(Dipole other) {
        float4 wedge_g0 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float wedge_g1 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float wedge_g1 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g2.zxy, (wedge_g1 * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_orthogonally_from(FlatPoint other) {
        float3 wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            wedge_g0_xyz * float3(right_anti_dual_g0[3]) * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge_g0_xyz.zxy * right_anti_dual_g0.yzx) - (wedge_g0_xyz.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float3 wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * right_anti_dual_g0.www * float3(-1.0)), (wedge_g0_xyz[0] * right_anti_dual_g1[0]) + (wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (wedge_g0_xyz[2] * right_anti_dual_g1[2])), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz.zxy * right_anti_dual_g0.yzx) - (wedge_g0_xyz.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_orthogonally_from(Line other) {
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
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), wedge_g0_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * right_anti_dual_g1.xyz), wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4((this.group1.xyz * float3(other.group0.xx, other.scalar)), 0.0);
        float3 wedge_g5 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g6 = float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3));
        float3 wedge_g7 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g8 = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group4.zxy), this.e23 * other.e15);
        float wedge_g10 = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, 0.0)) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), 0.0) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public Dipole anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e4) * this.group1.xyz;
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g1_xyz * float3(right_anti_dual_g1)) + (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2[0]) + (wedge_g0[0] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[1]) + (wedge_g0[1] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[2]) + (wedge_g0[2] * right_anti_dual_g0[3]), -(wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (wedge_g1_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(wedge_g1_xyz, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)).wwwx), 
            /* e15, e25, e35 */
            (wedge_g1_xyz * float3(right_anti_dual_g0[3])) + (wedge_g2.yzx * right_anti_dual_g0.zxy) - (wedge_g2.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine anti_reject_orthogonally_from(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e4) * this.group1.xyz;
        float wedge_g1_w = -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3);
        float4 wedge_g2 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group1.yzx * other.group3.zxy), this.e23 * other.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g1_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g0[0] * wedge_g2[3]), (wedge_g1_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g0[1] * wedge_g2[3]), (wedge_g1_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g0[2] * wedge_g2[3]), -(wedge_g1_w * right_anti_dual_g1[3]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2])) + float4((wedge_g0.zxy * right_anti_dual_g3.yzx), right_anti_dual_g0[3] * wedge_g2[3]) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(wedge_g1_xyz[1] * right_anti_dual_g3[1]) - (wedge_g1_xyz[2] * right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g1_xyz, wedge_g1_w).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(right_anti_dual_g3[3])) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g1 = float4((this.group0.xyz * other.group0.www), 0.0);
        float3 wedge_g2_xyz = float3(other.scalar) * this.group1.xyz;
        float wedge_g2_w = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[1])) + float4(float3(0.0), wedge_g2_w * right_anti_dual_g2[3]) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (wedge_g1 * float4(right_anti_dual_g0[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.scalar);
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz);
        float wedge_g1_w = other.e45 * this.scalar;
        float wedge_g2_x = (other.e15 * this.scalar) + (other.scalar * this.e15);
        float wedge_g2_y = (other.e25 * this.scalar) + (other.scalar * this.e25);
        float wedge_g2_z = (other.e35 * this.scalar) + (other.scalar * this.e35);
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12);
        float4 wedge_g3 = (this.group1.zxyw * float4(other.group0.yzx, other.scalar)) + float4((float3(other.e45) * this.group0.xyz), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * right_anti_dual_g1[0]) + (right_anti_dual_g2[3] * wedge_g0[0]), (wedge_g2_w * right_anti_dual_g1[1]) + (right_anti_dual_g2[3] * wedge_g0[1]), (wedge_g2_w * right_anti_dual_g1[2]) + (right_anti_dual_g2[3] * wedge_g0[2]), -(wedge_g1_w * right_anti_dual_g1[3]) - (wedge_g2_y * right_anti_dual_g0[1]) - (wedge_g2_z * right_anti_dual_g0[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.zxy * wedge_g3.yzx), right_anti_dual_g2[3] * wedge_g0[3]) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), wedge_g2_x * right_anti_dual_g0[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (wedge_g1_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (wedge_g1_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (wedge_g1_xyz[2] * right_anti_dual_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), wedge_g1_w * right_anti_dual_g2[3]) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2[3]) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0_xyz = (float3(other.e4) * this.group0.xyz) + (other.group0.xyz * this.group0.www);
        float wedge_g0_w = (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        float4 wedge_g1 = (float4(this.scalar) * other.group1) + float4((float3(other.e4) * this.group1.xyz), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12));
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g3.yzxy * float4(wedge_g0_xyz.zxy, wedge_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((float3(wedge_g0_w) * other.group0.xyz), right_anti_dual_g2[3] * other.e5 * this.scalar) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - float4((wedge_g0_xyz.yzx * right_anti_dual_g3.zxy), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0_xyz[0] * right_anti_dual_g3[3]) + (wedge_g2_xyz[0] * right_anti_dual_g2[3]), (wedge_g0_xyz[1] * right_anti_dual_g3[3]) + (wedge_g2_xyz[1] * right_anti_dual_g2[3]), (wedge_g0_xyz[2] * right_anti_dual_g3[3]) + (wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (right_anti_dual_g1 * float4(wedge_g0_w)) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0_w) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), wedge_g0_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * float4(other.group0.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_orthogonally_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g1.xyz * wedge_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) + (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1.zxy * wedge_g0.yzx) - (right_anti_dual_g1.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group0.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g0_xyz[0]) - (right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])), 
            /* e15, e25, e35, e3215 */
            float4((right_anti_dual_g0 * float3(-(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(right_anti_dual_g0[3])), (right_anti_dual_g0[3] * other.scalar * this.scalar) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g1[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g0.xyz * wedge_g0.www * float3(-1.0)), (right_anti_dual_g0[0] * wedge_g1[0]) + (right_anti_dual_g0[1] * wedge_g1[1]) + (right_anti_dual_g0[2] * wedge_g1[2])), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e12345, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_orthogonally_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = other.group2.xyz * this.group0.www;
        float wedge_g2_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * right_anti_dual_g2.xyz), (wedge_g2_w * right_anti_dual_g2[3]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e35 * other.e42), (this.e31 * other.e45) + (this.e15 * other.e43), (this.e12 * other.e45) + (this.e25 * other.e41), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g2[1] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g3[3]), (right_anti_dual_g2[2] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g3[3]), (right_anti_dual_g2[0] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g3[3]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = other.group2.xyz * this.group0.www;
        float wedge_g2_w = (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.scalar * other.e4235), (this.e31 * other.e45) + (this.scalar * other.e4315), (this.e12 * other.e45) + (this.scalar * other.e4125), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((other.group0.yzx * this.group1.zxy), this.scalar * other.e3215) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (wedge_g2_w * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) - (right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiDualNum anti_reject_orthogonally_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * other.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(FlatPoint other) {
        float3 wedge_g1_xyz = float3(other.e45) * this.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0[3]) * float4(wedge_g1_xyz, this.scalar * other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1_xyz.zxy * right_anti_dual_g0.yzx) - (wedge_g1_xyz.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g1 = (this.group0 * float4(other.group0.www, other.e3215)) + float4((float3(this.scalar) * other.group1.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * this.scalar * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group0.www;
        float wedge_g0_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), (wedge_g0_w * right_anti_dual_g0[3]) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = this.scalar * other.e5;
        float4 wedge_g3 = float4(((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz)), this.scalar * other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group4.xyz;
        float3 wedge_g5 = (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz);
        float4 wedge_g6 = (float4(this.scalar) * other.group6) + float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3));
        float3 wedge_g7 = (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group0.xyz);
        float3 wedge_g8 = (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy);
        float4 wedge_g9 = (this.group0 * float4(other.group3.www, other.e3215)) + (this.group1.zxyw * float4(other.group4.yzx, other.scalar)) + float4((float3(this.scalar) * other.group9.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group1.yzxx * float4(other.group4.zxy, other.e23));
        float wedge_g10 = (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) + (right_anti_dual_g0[1] * this.scalar * other.scalar) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(other.e4) * float4(this.group0.xyz, this.e3215);
        float3 wedge_g1_xyz = float3(other.e4) * this.group1.xyz;
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g0.zxy, (right_anti_dual_g0[2] * wedge_g3[2]) + (right_anti_dual_g0[3] * wedge_g3[3]) + (right_anti_dual_g1 * this.scalar * other.e5)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((wedge_g1_xyz * float3(right_anti_dual_g1)), right_anti_dual_g0[1] * wedge_g3[1]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2_xyz[0]) + (right_anti_dual_g0[3] * wedge_g0[0]), (right_anti_dual_g1 * wedge_g2_xyz[1]) + (right_anti_dual_g0[3] * wedge_g0[1]), (right_anti_dual_g1 * wedge_g2_xyz[2]) + (right_anti_dual_g0[3] * wedge_g0[2]), -(wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (wedge_g1_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(wedge_g1_xyz, -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(right_anti_dual_g0[3])) + (wedge_g2_xyz.yzx * right_anti_dual_g0.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g0.yzx)), right_anti_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_reject_orthogonally_from(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e15, e25, e35, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Scalar anti_reject_orthogonally_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = (this.group0 * float4(other.group3.www, other.e12345)) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((other.group0.xyz * this.group0.www), this.e3215 * other.e4);
        float4 wedge_g1 = (float4(this.scalar) * other.group1) + float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3));
        float3 wedge_g2_xyz = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), right_anti_dual_g2[3] * this.scalar * other.e5) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (wedge_g0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g1[0] * wedge_g0[3]), (wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g1[1] * wedge_g0[3]), (wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g1[2] * wedge_g0[3]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 wedge_g1_xyz = (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz);
        float wedge_g1_w = this.scalar * other.e45;
        float4 wedge_g2 = (float4(this.scalar) * other.group2) + float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43));
        float4 wedge_g3 = float4(this.e35 * other.e42, this.e15 * other.e43, this.e25 * other.e41, -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + (this.group0 * float4(other.group1.www, other.e3215)) + float4((other.group3.xyz * this.group0.www), this.e3215 * other.scalar) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g1_w * right_anti_dual_g1[3]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g1.xyz * wedge_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) + float4((wedge_g0.xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz[0] * right_anti_dual_g0[3]) + (right_anti_dual_g2[0] * wedge_g2[3]), (wedge_g1_xyz[1] * right_anti_dual_g0[3]) + (right_anti_dual_g2[1] * wedge_g2[3]), (wedge_g1_xyz[2] * right_anti_dual_g0[3]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (right_anti_dual_g0 * float4(wedge_g3.www, wedge_g1_w)) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual_g2[3]) * wedge_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[3] * wedge_g3[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[3] * wedge_g3[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[3] * wedge_g3[2]), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (right_anti_dual_g2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - (right_anti_dual_g2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy);
        float wedge_g1_w = other.e4 * this.e5 * -1.0;
        float4 wedge_g2 = (float4((float3(this.e5) * other.group3.xyz), (other.e431 * this.e2) + (other.e412 * this.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e423));
        float4 wedge_g3 = float4(other.e425 * this.e3, other.e435 * this.e1, other.e415 * this.e2, -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * wedge_g2[3]) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g1_w * right_anti_dual_g2[3]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2])) - float4((right_anti_dual_g3.xyz * wedge_g3.www), wedge_g0[0] * right_anti_dual_g3[0]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g3.zxyw * float4(wedge_g1_xyz.yzx, wedge_g1_w)) + (wedge_g2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g3[0] * wedge_g2[0]) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[1] * wedge_g2[1]) - (float4(wedge_g3[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[2] * wedge_g3[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public DualNum anti_reject_orthogonally_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e321 * other.e321 * this.e5) + (other.e235 * other.e321 * this.e1) + (other.e315 * other.e321 * this.e2) + (other.e125 * other.e321 * this.e3), 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0_xyz = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) - (this.group0.wwwx * float4(other.group1.xyz, other.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual_g1.xyz * wedge_g1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0])) + float4(float3(0.0), (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2])) - float4((wedge_g0_xyz.zxy * right_anti_dual_g1.yzx), wedge_g1[3] * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.zxyx * float4(other.group1.yzx, other.e23));
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g0 * float4(wedge_g0.www, wedge_g1[3])) + (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], 1.0) * float4(right_anti_dual_g0.www, -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 wedge_g1 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(wedge_g0.zxy, (wedge_g1[1] * right_anti_dual_g0[1]) + (wedge_g1[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group0.xyz, other.e235)) - float4((this.group0.yzx * other.group1.zxy), this.e2 * other.e315);
        float wedge_g1 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group0.www), this.e1 * other.e235);
        float wedge_g1 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g1) * right_anti_dual_g2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float3 wedge_g2 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1]), 
            /* e5 */
            -(right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2]) - (wedge_g2[0] * right_anti_dual_g1[0]) - (wedge_g2[1] * right_anti_dual_g1[1]) - (wedge_g2[2] * right_anti_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = (float4(this.e3 * other.e25, this.e1 * other.e35, this.e2 * other.e15, (this.e3 * other.e4125) + (this.e5 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (wedge_g2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (wedge_g2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(FlatPoint other) {
        float3 wedge_g0 = float3(other.e45) * this.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(right_anti_dual_g0[3])), -(wedge_g0[0] * right_anti_dual_g0[0]) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4((float3(other.e45) * this.group0.xyz * float3(-1.0)), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.zxy * wedge_g0.yzx), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g1 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e2 * other.e315) - (this.e3 * other.e125)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1.yzx), 0.0) - (wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        float4 wedge_g1 = float4((this.group0.xyz * float3(other.group0.xx, other.scalar)), 0.0);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0);
        float3 wedge_g4 = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float3 wedge_g5 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g6 = float4(((float3(this.e5) * other.group4.xyz) - (this.group0.xyz * other.group3.www)), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12));
        float3 wedge_g7 = (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g9 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.wwwx * float4(other.group7.xyz, other.e235)) - float4((this.group0.yzx * other.group6.zxy), this.e2 * other.e315);
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * right_anti_dual_g0[3]) + (wedge_g1_xyz[1] * right_anti_dual_g0[2]), (wedge_g0[1] * right_anti_dual_g0[3]) + (wedge_g1_xyz[2] * right_anti_dual_g0[0]), (wedge_g0[2] * right_anti_dual_g0[3]) + (wedge_g1_xyz[0] * right_anti_dual_g0[1]), -(right_anti_dual_g1 * wedge_g1_w) - (wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxy * float4(wedge_g1_xyz.zxy, wedge_g0[1])) - float4((wedge_g2 * float3(right_anti_dual_g1)), wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e5 */
            (wedge_g1_w * right_anti_dual_g0[3]) + (wedge_g2[0] * right_anti_dual_g0[0]) + (wedge_g2[1] * right_anti_dual_g0[1]) + (wedge_g2[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_reject_orthogonally_from(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e5 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = float3(other.e4) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float wedge_g1_w = this.e5 * other.e4 * -1.0;
        float4 wedge_g2 = (float4((float3(this.e5) * other.group3.xyz), (this.e2 * other.e431) + (this.e3 * other.e412)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group2.www, other.e423));
        float4 wedge_g3 = float4(this.e3 * other.e425, this.e1 * other.e435, this.e2 * other.e415, -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - (this.group0.wwwy * float4(other.group0.xyz, other.e315));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g1_w * right_anti_dual_g2[3]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2])) + float4((wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g0, wedge_g1_w)) + (right_anti_dual_g3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[2] * wedge_g2[2]) - (float4(wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (float4(this.e2 * other.e43, this.e3 * other.e41, this.e1 * other.e42, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235));
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (right_anti_dual_g0.zxyw * float4(wedge_g2.yzx, 0.0)) + float4((this.group0.xyz * right_anti_dual_g0.www * other.group0.www), right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_reject_orthogonally_from(AntiCircleRotor other) {
        float wedge_g0 = other.scalar * this.e12345;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public DualNum anti_reject_orthogonally_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor anti_reject_orthogonally_from(AntiMotor other) {
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0 = this.e12345 * other.scalar;
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float wedge_g0 = this.e12345 * other.scalar;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(wedge_g0) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Circle> operator /(Circle lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(wedge_g2_w)) + (wedge_g0 * float3(right_anti_dual_g2[3]))), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (wedge_g2_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (wedge_g2_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2_xyz[1]) + (wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2_xyz[2]) + (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2_xyz[0]) + (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group3.yzx * this.group1.zxy), other.e2 * this.e315);
        float wedge_g1 = -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g0.zxy, wedge_g1)) - (wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g0[1]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g0[2]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g0[0]), -(right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) - (float4(wedge_g0[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle anti_reject_orthogonally_from(AntiDualNum other) {
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
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group1.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315);
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g1[3]), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g1) * right_anti_dual_g0.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g0[3]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Line anti_reject_orthogonally_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0[3]) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * right_anti_dual_g1[1]) + (right_anti_dual_g0[0] * wedge_g1[3]), (wedge_g0[0] * right_anti_dual_g1[2]) + (right_anti_dual_g0[1] * wedge_g1[3]), (wedge_g0[1] * right_anti_dual_g1[0]) + (right_anti_dual_g0[2] * wedge_g1[3]), -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])) - float4((wedge_g0.yzx * right_anti_dual_g1.zxy), wedge_g0[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group0.xyz, this.e235)) + float4((other.group0.yzx * this.group1.zxy), other.e2 * this.e315);
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * right_anti_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle anti_reject_orthogonally_from(Dipole other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
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
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5) * this.group0.xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
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
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e321);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g0.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g0.zxy, -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2]))) - (wedge_g0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4(other.scalar) * this.group1;
        float3 wedge_g7 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g8 = float3(other.scalar) * this.group2.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1);
        float wedge_g10 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), 0.0) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e5) * this.group0.xyz), this.e235 * other.e1);
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g0.zxy, wedge_g1)) - float4((right_anti_dual_g0.zxy * wedge_g0.yzx), right_anti_dual_g1 * wedge_g0[3]), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle anti_reject_orthogonally_from(Scalar other) {
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
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group2.www), this.e235 * other.e1);
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3.yzx * wedge_g0.zxy), wedge_g1 * right_anti_dual_g3[3]) - (wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (wedge_g1 * right_anti_dual_g1[3]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g0[1]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g0[2]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g0[0]), -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[2] * wedge_g0[2])) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - (wedge_g0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((float3(other.scalar) * this.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(right_anti_dual_g0[3])) + (float3(wedge_g2[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(wedge_g0[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + float4((wedge_g0.zxy * right_anti_dual_g2.yzx), right_anti_dual_g3[3] * wedge_g2[3]) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), wedge_g0[0] * right_anti_dual_g1[0]) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), wedge_g0[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = this.group2.xyz * other.group2.www;
        float wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(wedge_g2_w)) + (wedge_g0 * float3(right_anti_dual_g2[3]))), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2_w)) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (wedge_g2_xyz[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (wedge_g2_xyz[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2_xyz[1]) + (wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2_xyz[2]) + (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2_xyz[0]) + (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group3.www), other.e1 * this.e235);
        float wedge_g1 = -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g0.zxy, wedge_g1)) - (wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g0[1]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g0[2]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g0[0]), -(right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) - (float4(wedge_g0[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4((float3(other.scalar) * this.group0.xyz), wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            wedge_g2 * float4(other.group0.yy, other.scalar, other.e3215), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group1.www), other.e1 * this.e235);
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g1[3]), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g1) * right_anti_dual_g0.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g0[3]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Line anti_reject_orthogonally_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((this.group2.xyz * other.group0.www), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0[3]) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * right_anti_dual_g1[1]) + (right_anti_dual_g0[0] * wedge_g1[3]), (wedge_g0[0] * right_anti_dual_g1[2]) + (right_anti_dual_g0[1] * wedge_g1[3]), (wedge_g0[1] * right_anti_dual_g1[0]) + (right_anti_dual_g0[2] * wedge_g1[3]), -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])) - float4((wedge_g0.yzx * right_anti_dual_g1.zxy), wedge_g0[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * other.group0.www), other.e1 * this.e235);
        float wedge_g1 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * right_anti_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_orthogonally_from(Dipole other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
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
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint anti_reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5) * this.group0.xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint anti_reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
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
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e321);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g0.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g0.zxy, -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2]))) - (wedge_g0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4(other.scalar) * this.group1;
        float3 wedge_g7 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g8 = float3(other.scalar) * this.group2.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g10 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), 0.0) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Circle anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g0.zxy, wedge_g1)) - float4((right_anti_dual_g0.zxy * wedge_g0.yzx), right_anti_dual_g1 * wedge_g0[3]), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_reject_orthogonally_from(Scalar other) {
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
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g1) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3.yzx * wedge_g0.zxy), wedge_g1 * right_anti_dual_g3[3]) - (wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (wedge_g1 * right_anti_dual_g1[3]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g0[1]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g0[2]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g0[0]), -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[2] * wedge_g0[2])) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - (wedge_g0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g2 = float4((float3(other.scalar) * this.group2.xyz), (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(right_anti_dual_g0[3])) + (float3(wedge_g2[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g2[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(wedge_g0[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + float4((wedge_g0.zxy * right_anti_dual_g2.yzx), right_anti_dual_g3[3] * wedge_g2[3]) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), wedge_g0[0] * right_anti_dual_g1[0]) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), wedge_g0[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Dipole> operator /(Dipole lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e23 * this.e15) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * right_anti_dual_g1[0]) + (right_anti_dual_g0[2] * wedge_g3[1]) + (wedge_g0[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[1]) + (right_anti_dual_g0[0] * wedge_g3[2]) + (wedge_g0[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[2]) + (right_anti_dual_g0[1] * wedge_g3[0]) + (wedge_g0[2] * right_anti_dual_g2[3]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), right_anti_dual_g0[0] * wedge_g2_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g2[3] * wedge_g1[0]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g2[3] * wedge_g1[1]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), right_anti_dual_g2[3] * wedge_g1[3]) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((this.group2.yzx * other.group3.zxy), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * right_anti_dual_g3[1]) + (right_anti_dual_g2[3] * wedge_g1[0]) + (wedge_g2[3] * other.e423), (wedge_g0[0] * right_anti_dual_g3[2]) + (right_anti_dual_g2[3] * wedge_g1[1]) + (wedge_g2[3] * other.e431), (wedge_g0[1] * right_anti_dual_g3[0]) + (right_anti_dual_g2[3] * wedge_g1[2]) + (wedge_g2[3] * other.e412), -(wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(AntiDualNum other) {
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
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public FlatPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((other.e2 * this.e35) + (other.e5 * this.e23), (other.e3 * this.e15) + (other.e5 * this.e31), (other.e1 * this.e25) + (other.e5 * this.e12), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((this.group2.yzx * other.group1.zxy), other.e235 * this.e41);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(wedge_g0.zxy, -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * wedge_g1[3]))) - float4((wedge_g0.yzx * right_anti_dual_g1.zxy), wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g1.www), right_anti_dual_g0[3] * wedge_g2[3]) - (right_anti_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g2[3]) * right_anti_dual_g0.xyz), 0.0) + float4((right_anti_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float wedge_g1 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(wedge_g1), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(wedge_g1)), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35 */
            (right_anti_dual_g0 * float3(wedge_g0[3])) + (right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float wedge_g2_w = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((this.group0.zxy * other.group1.yzx), other.e23 * this.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0 * right_anti_dual_g0.www) + (float3(wedge_g2_w) * right_anti_dual_g0.xyz)), (wedge_g2_w * right_anti_dual_g1[3]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(right_anti_dual_g0[3])) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * wedge_g3.zxy) + (float3(right_anti_dual_g0[3]) * float3(other.scalar) * this.group2.xyz) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float3 wedge_g2 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0 * float3(right_anti_dual_g0[3])), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (wedge_g2.yzx * right_anti_dual_g0.zxy) - (wedge_g2.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(Circle other) {
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
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
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
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e25 * this.e31);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.yzx * other.group0.zxy), this.e25 * other.e31);
        float wedge_g1 = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g2.zxy, (wedge_g1 * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public FlatPoint anti_reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5) * this.group0.xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiLine anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[3]) * wedge_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiLine anti_reject_orthogonally_from(Line other) {
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
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float3 wedge_g0_xyz = float3(other.e5) * this.group0.xyz;
        float wedge_g0_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), (wedge_g0_w * right_anti_dual_g0[3]) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g5 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g6 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group3.yzx), this.e15 * other.e23) - float4((this.group2.yzx * other.group4.zxy), this.e25 * other.e31);
        float wedge_g10 = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float3 wedge_g2 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2[0]) + (wedge_g0[0] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[1]) + (wedge_g0[1] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[2]) + (wedge_g0[2] * right_anti_dual_g0[3]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (wedge_g2.yzx * right_anti_dual_g0.zxy) - (wedge_g2.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole anti_reject_orthogonally_from(Scalar other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx);
        float4 wedge_g2 = float4((this.e23 * other.e5) + (this.e35 * other.e2), (this.e31 * other.e5) + (this.e15 * other.e3), (this.e12 * other.e5) + (this.e25 * other.e1), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_anti_dual_g0[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[0]), (right_anti_dual_g0[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[1]), (right_anti_dual_g0[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0.zxy * right_anti_dual_g3.yzx), right_anti_dual_g0[3] * wedge_g2[3]) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) - float4((this.group0.zxy * other.group2.yzx), this.e15 * other.e23) - float4((this.group2.yzx * other.group0.zxy), this.e25 * other.e31);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0 * right_anti_dual_g0.www), wedge_g2_w * right_anti_dual_g2[3]) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (wedge_g1 * float4(right_anti_dual_g0[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = this.group2.xyz * other.group2.www;
        float wedge_g2_w = (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * right_anti_dual_g1[0]) + (right_anti_dual_g0[2] * wedge_g3[1]) + (wedge_g0[0] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[1]) + (right_anti_dual_g0[0] * wedge_g3[2]) + (wedge_g0[1] * right_anti_dual_g2[3]), (wedge_g2_w * right_anti_dual_g1[2]) + (right_anti_dual_g0[1] * wedge_g3[0]) + (wedge_g0[2] * right_anti_dual_g2[3]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), right_anti_dual_g0[0] * wedge_g2_xyz[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g2[3] * wedge_g1[0]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g2[3] * wedge_g1[1]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), right_anti_dual_g2[3] * wedge_g1[3]) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.yzx * this.group2.zxy), other.e4 * this.e3215) - (this.group2.yzxx * float4(other.group3.zxy, other.e423));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * right_anti_dual_g3[1]) + (right_anti_dual_g2[3] * wedge_g1[0]) + (wedge_g2[3] * other.e423), (wedge_g0[0] * right_anti_dual_g3[2]) + (right_anti_dual_g2[3] * wedge_g1[1]) + (wedge_g2[3] * other.e431), (wedge_g0[1] * right_anti_dual_g3[0]) + (right_anti_dual_g2[3] * wedge_g1[2]) + (wedge_g2[3] * other.e412), -(wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.scalar, other.e3215) * float4((float3(other.scalar) * this.group0.xyz), wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public FlatPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group1.zxy * this.group2.yzx), other.e235 * this.e41);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(wedge_g0.zxy, -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * wedge_g1[3]))) - float4((wedge_g0.yzx * right_anti_dual_g1.zxy), wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g1.www), right_anti_dual_g0[3] * wedge_g2[3]) - (right_anti_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g2[3]) * right_anti_dual_g0.xyz), 0.0) + float4((right_anti_dual_g1.zxy * wedge_g2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_orthogonally_from(AntiLine other) {
        float4 wedge_g0 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float wedge_g1 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(wedge_g1), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(wedge_g1)), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35 */
            (right_anti_dual_g0 * float3(wedge_g0[3])) + (right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float wedge_g2_w = (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((this.group0.yzx * other.group1.zxy), other.scalar * this.e3215) - float4((this.group0.zxy * other.group1.yzx), other.e23 * this.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0 * right_anti_dual_g0.www) + (float3(wedge_g2_w) * right_anti_dual_g0.xyz)), (wedge_g2_w * right_anti_dual_g1[3]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(right_anti_dual_g0[3])) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * wedge_g3.zxy) + (float3(right_anti_dual_g0[3]) * this.group2.xyz * other.group0.www) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (float4(other.e3 * this.e25, other.e1 * this.e35, other.e2 * this.e15, (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0 * float3(right_anti_dual_g0[3])), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * wedge_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * wedge_g2.yzx), 0.0) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_orthogonally_from(Circle other) {
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
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
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
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Dipole anti_reject_orthogonally_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e25 * this.e31);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((this.group0.zxy * other.group2.yzx), other.e31 * this.e25);
        float wedge_g1 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g2.zxy, (wedge_g1 * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_orthogonally_from(DualNum other) {
        float4 wedge_g0 = float4(other.e5) * float4(this.group0.xyz, this.e1234);
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual_g0[1] * wedge_g0[3]), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiLine anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[3]) * wedge_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4((this.e42 * other.e35) + (this.e23 * other.e45), (this.e43 * other.e15) + (this.e31 * other.e45), (this.e41 * other.e25) + (this.e12 * other.e45), -(this.e31 * other.e25) - (this.e12 * other.e35)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiLine anti_reject_orthogonally_from(Line other) {
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
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float3 wedge_g0_xyz = float3(other.e5) * this.group0.xyz;
        float wedge_g0_w = (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), (wedge_g0_w * right_anti_dual_g0[3]) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g5 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g6 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group1.xyzx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e42 * other.e35) + (this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42), (this.e43 * other.e15) + (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43), (this.e41 * other.e25) + (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group3) - (this.group2.yzxy * float4(other.group4.zxy, other.e31)) - float4((this.group0.zxy * other.group3.yzx), this.e15 * other.e23);
        float wedge_g10 = (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group0.xyzx);
        float4 wedge_g2 = (float4(this.e25 * other.e3, this.e35 * other.e1, this.e15 * other.e2, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * wedge_g1.xyz) + (wedge_g0.zxy * right_anti_dual_g0.yzx) - (wedge_g0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2[0]) + (wedge_g0[0] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[1]) + (wedge_g0[1] * right_anti_dual_g0[3]), (right_anti_dual_g1 * wedge_g2[2]) + (wedge_g0[2] * right_anti_dual_g0[3]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g2.yzx) - (right_anti_dual_g0.yzx * wedge_g2.zxy)), right_anti_dual_g1 * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_reject_orthogonally_from(Scalar other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 wedge_g1 = float4((this.e41 * other.e5) + (this.e15 * other.e4), (this.e42 * other.e5) + (this.e25 * other.e4), (this.e43 * other.e5) + (this.e35 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (this.group1.wwwx * other.group3.xyzx);
        float4 wedge_g2 = (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_anti_dual_g0[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[0]), (right_anti_dual_g0[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[1]), (right_anti_dual_g0[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0.zxy * right_anti_dual_g3.yzx), right_anti_dual_g0[3] * wedge_g2[3]) - float4((wedge_g0.yzx * right_anti_dual_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g2[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float wedge_g2_w = (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g3 = float4((this.e23 * other.e45) + (this.e45 * other.e23) + (this.e35 * other.e42) + (this.e4235 * other.scalar), (this.e31 * other.e45) + (this.e45 * other.e31) + (this.e15 * other.e43) + (this.e4315 * other.scalar), (this.e12 * other.e45) + (this.e45 * other.e12) + (this.e25 * other.e41) + (this.e4125 * other.scalar), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((this.group0.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0 * right_anti_dual_g0.www), wedge_g2_w * right_anti_dual_g2[3]) + float4((float3(wedge_g2_w) * right_anti_dual_g1.xyz), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (wedge_g1 * float4(right_anti_dual_g0[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<DualNum> operator /(DualNum lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.group0.xx, this.e5, this.e12345) * float4(other.group0.xyz, other.scalar);
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, other.scalar);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * float4(right_anti_dual_g0, right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g2[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0.www, wedge_g1[3])) + (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], 1.0) * float4(right_anti_dual_g2.www, -(right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]))), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1[1]) + (right_anti_dual_g1[3] * wedge_g0[0]), (right_anti_dual_g0[0] * wedge_g1[2]) + (right_anti_dual_g1[3] * wedge_g0[1]), (right_anti_dual_g0[1] * wedge_g1[0]) + (right_anti_dual_g1[3] * wedge_g0[2]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) - float4((right_anti_dual_g0.yzx * wedge_g1.zxy), right_anti_dual_g0[0] * wedge_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g1.zxy) - (right_anti_dual_g3.zxy * wedge_g1.yzx)), right_anti_dual_g2[3] * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((right_anti_dual_g3.xyz * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g2[2] * wedge_g1[2])) + float4((right_anti_dual_g1.zxy * wedge_g1.yzx), right_anti_dual_g3[0] * wedge_g0[0]) - (float4(wedge_g1[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g0.xyz, wedge_g1[1])) - (wedge_g1.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(AntiDualNum other) {
        float2 wedge_g0 = float2(other.scalar) * this.group0.xy;
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0[0] * other.scalar) + (wedge_g0[1] * other.e3215), wedge_g0[1] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * other.e321 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiFlector anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0_xyz = other.group1.xyz * float3(this.group0.xx, this.e5) * float3(-1.0);
        float4 wedge_g1 = float4(float3(0.0), other.e321 * this.e5) * float4(0.0, 0.0, 0.0, -1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g1.xyz) - (float3(wedge_g1[3]) * right_anti_dual_g1.xyz)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g0_xyz[0] * right_anti_dual_g1[0]) + (wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e5);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2(-(right_anti_dual_g0[0] * wedge_g0_xyz[0]) - (right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2]), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_orthogonally_from(AntiMotor other) {
        float wedge_g0_w = other.scalar * this.e12345;
        float4 wedge_g1 = float4(this.e5) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(wedge_g0_w) * right_anti_dual_g0.xyz), wedge_g0_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            (float4(wedge_g0_w) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e5) * float3(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0_xyz[0] * right_anti_dual_g0[0]) + (wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (wedge_g0_xyz[2] * right_anti_dual_g0[2]), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.e5) * other.group0.xyz;
        float3 wedge_g1 = float3(this.e5) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1[1]) + (wedge_g0[0] * right_anti_dual_g1[3]), (right_anti_dual_g0[0] * wedge_g1[2]) + (wedge_g0[1] * right_anti_dual_g1[3]), (right_anti_dual_g0[1] * wedge_g1[0]) + (wedge_g0[2] * right_anti_dual_g1[3]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) - float4((right_anti_dual_g0.yzx * wedge_g1.zxy), right_anti_dual_g0[0] * wedge_g0[0]), 
            /* e5 */
            -(right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (wedge_g1[0] * right_anti_dual_g1[0]) - (wedge_g1[1] * right_anti_dual_g1[1]) - (wedge_g1[2] * right_anti_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e1234);
        float3 wedge_g1_xyz = other.group1.xyz * float3(this.group0.xx, this.e5);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g0[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g1[3] * wedge_g0[0]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[3] * wedge_g0[1]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[3] * wedge_g0[2]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.zxy * wedge_g1_xyz.yzx), right_anti_dual_g3[3] * wedge_g0[3]) - float4((right_anti_dual_g0.yzx * wedge_g1_xyz.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e5 * other.e1234) + (this.e12345 * other.scalar);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(this.e5) * other.group1 * float4(-1.0);
        float4 wedge_g6 = float4((other.group4.xyz * float3(this.group0.xx, this.e5)), 0.0);
        float3 wedge_g8 = float3(this.e5) * other.group5.xyz;
        float4 wedge_g9 = float4(this.e5) * float4(other.group7.xyz, other.e321) * float4(-1.0);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (float4(((float3(right_anti_dual_g6[3]) * wedge_g6.xyz) + (float3(wedge_g6[3]) * right_anti_dual_g6.xyz)), right_anti_dual_g10 * wedge_g3[3]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.e5) * other.group0 * float4(-1.0);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            wedge_g0 * float4(other.e4 * -1.0) * float4(-1.0), 
            /* e5 */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * float2(this.e5 * other.scalar, this.e12345 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = this.e5 * other.e1234;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(this.e5) * other.group3 * float4(-1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g1.zxy) - (right_anti_dual_g3.zxy * wedge_g1.yzx)), right_anti_dual_g2[3] * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * wedge_g1[1]) + (right_anti_dual_g0[2] * wedge_g1[2])) - float4((right_anti_dual_g3.xyz * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((right_anti_dual_g1.zxy * wedge_g1.yzx), right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g2.wwwx * float4(wedge_g0.xyz, wedge_g1[0])) - (wedge_g1.zxyy * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[1])) - (wedge_g1.wwwz * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.group0.xx, this.e5, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, other.scalar);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1[1]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g0[0] * wedge_g1[2]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g0[1] * wedge_g1[0]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], 1.0) * float4(right_anti_dual_g0.zxy, -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g1[3] * wedge_g0[3]))) - float4((right_anti_dual_g0.yzx * wedge_g1.zxy), right_anti_dual_g0[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0 * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]) - (right_anti_dual_g1.wwwx * wedge_g1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4((float3(other.e4) * this.group0.xyz), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], 1.0) * float4(right_anti_dual_g2.www, -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2]))) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g3[1] * wedge_g0[1]) - (right_anti_dual_g3[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g1_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_orthogonally_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(other.scalar * this.e15, other.scalar * this.e25, other.scalar * this.e35, other.scalar * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e321 * this.e45 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Flector anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.e45) * float4(other.group1.xyz, other.e321) * float4(-1.0);
        float3 wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1_xyz[1] * right_anti_dual_g1[2]) + (right_anti_dual_g1[3] * wedge_g0[0]), (wedge_g1_xyz[2] * right_anti_dual_g1[0]) + (right_anti_dual_g1[3] * wedge_g0[1]), (wedge_g1_xyz[0] * right_anti_dual_g1[1]) + (right_anti_dual_g1[3] * wedge_g0[2]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (float4(wedge_g0[3]) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)) - (right_anti_dual_g1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        float wedge_g0_w = -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g0_xyz[2]), (wedge_g0_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g0_xyz[0]), (wedge_g0_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g0_xyz[1]), -(right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g0_xyz.yzx), wedge_g0_w * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Flector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g1 = float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g0[0] * wedge_g1[3]) + (right_anti_dual_g1[1] * wedge_g1[2]), (right_anti_dual_g0[1] * wedge_g1[3]) + (right_anti_dual_g1[2] * wedge_g1[0]), (right_anti_dual_g0[2] * wedge_g1[3]) + (right_anti_dual_g1[0] * wedge_g1[1]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + (float4(right_anti_dual_g0[3]) * float4(other.scalar) * this.group0) - (wedge_g1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e45) * other.group0.xyz * float3(-1.0);
        float3 wedge_g1 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[0] * right_anti_dual_g0[3]) + (wedge_g1[1] * right_anti_dual_g0[2]), (wedge_g0[1] * right_anti_dual_g0[3]) + (wedge_g1[2] * right_anti_dual_g0[0]), (wedge_g0[2] * right_anti_dual_g0[3]) + (wedge_g1[0] * right_anti_dual_g0[1]), -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_orthogonally_from(Circle other) {
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
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
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
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_orthogonally_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g2.zxy, (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0);
        float3 wedge_g8 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group4.zxy, other.e23));
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (float4(((float3(right_anti_dual_g6[3]) * wedge_g6.xyz) + (float3(wedge_g6[3]) * right_anti_dual_g6.xyz)), right_anti_dual_g10 * wedge_g3[3]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Dipole anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group0.xyz);
        float3 wedge_g1 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            wedge_g0 * float3(right_anti_dual_g1), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * float3(right_anti_dual_g1)), -(wedge_g0[0] * right_anti_dual_g0[0]) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e15, e25, e35 */
            (wedge_g0 * float3(right_anti_dual_g0[3])) + (wedge_g1.yzx * right_anti_dual_g0.zxy) - (wedge_g1.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_reject_orthogonally_from(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.e15 * other.scalar, this.e25 * other.scalar, this.e35 * other.scalar, this.e45 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4((float3(other.e4) * this.group0.xyz), -(this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(right_anti_dual_g2.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (wedge_g1_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g3[1] * wedge_g0[1]) - (right_anti_dual_g3[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g1_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g1[1]) + (right_anti_dual_g3[2] * wedge_g1[2]) + (right_anti_dual_g3[3] * wedge_g1[3]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g1[3] * wedge_g0[3])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) - (right_anti_dual_g1.wwwx * wedge_g1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Flector> operator /(Flector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((other.e23 * this.e45) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.scalar * this.e4125), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((other.group0.yzx * this.group0.zxy), other.scalar * this.e3215) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], 1.0) * float4(right_anti_dual_g0.zxy, -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g1[3] * wedge_g0[3]))) - float4((right_anti_dual_g0.yzx * wedge_g1.zxy), right_anti_dual_g0[0] * wedge_g0[0]), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0 * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]) - (right_anti_dual_g1.wwwx * wedge_g1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g2[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], 1.0) * float4(right_anti_dual_g2.www, -(wedge_g1_xyz[0] * other.e423) - (wedge_g1_xyz[1] * other.e431) - (wedge_g1_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2]))) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g3[1] * wedge_g0[1]) - (right_anti_dual_g3[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g1_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public FlatPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e321 * this.e45 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group1.xyz * float3(-1.0);
        float wedge_g0_w = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45);
        float3 wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_xyz[0] * right_anti_dual_g1[3]) + (wedge_g1_xyz[1] * right_anti_dual_g1[2]), (wedge_g0_xyz[1] * right_anti_dual_g1[3]) + (wedge_g1_xyz[2] * right_anti_dual_g1[0]), (wedge_g0_xyz[2] * right_anti_dual_g1[3]) + (wedge_g1_xyz[0] * right_anti_dual_g1[1]), -(wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (wedge_g0_xyz[2] * right_anti_dual_g1[2])) + (float4(wedge_g0_w) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)) - (right_anti_dual_g1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0_xyz[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public FlatPoint anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        float wedge_g0_w = -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g0_xyz[2]), (wedge_g0_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g0_xyz[0]), (wedge_g0_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g0_xyz[1]), -(right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g0_xyz.yzx), wedge_g0_w * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g1 = (other.group0 * float4(this.group0.www, this.e3215)) + float4((float3(other.scalar) * this.group1.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g0[0] * wedge_g1[3]) + (right_anti_dual_g1[1] * wedge_g1[2]), (right_anti_dual_g0[1] * wedge_g1[3]) + (right_anti_dual_g1[2] * wedge_g1[0]), (right_anti_dual_g0[2] * wedge_g1[3]) + (right_anti_dual_g1[0] * wedge_g1[1]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + (float4(right_anti_dual_g0[3]) * float4(other.scalar) * this.group0) - (wedge_g1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_xyz[0] * right_anti_dual_g0[3]) + (wedge_g1_xyz[1] * right_anti_dual_g0[2]), (wedge_g0_xyz[1] * right_anti_dual_g0[3]) + (wedge_g1_xyz[2] * right_anti_dual_g0[0]), (wedge_g0_xyz[2] * right_anti_dual_g0[3]) + (wedge_g1_xyz[0] * right_anti_dual_g0[1]), -(wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0_xyz[0])), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Dipole anti_reject_orthogonally_from(Circle other) {
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
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
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
public Dipole operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Dipole anti_reject_orthogonally_from(Dipole other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float4 wedge_g0 = float4((other.e42 * this.e35) + (other.e23 * this.e45), (other.e43 * this.e15) + (other.e31 * this.e45), (other.e41 * this.e25) + (other.e12 * this.e45), -(other.e31 * this.e25) - (other.e12 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g2.zxy, (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(wedge_g0[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0);
        float3 wedge_g8 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g9 = float4((this.e35 * other.e42) + (this.e45 * other.e23), (this.e15 * other.e43) + (this.e45 * other.e31), (this.e25 * other.e41) + (this.e45 * other.e12), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group4.zxy, other.e23));
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (float4(((float3(right_anti_dual_g6[3]) * wedge_g6.xyz) + (float3(wedge_g6[3]) * right_anti_dual_g6.xyz)), right_anti_dual_g10 * wedge_g3[3]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = (float4((float3(this.e45) * other.group0.xyz), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group0.xyz, this.e4235));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g1) * wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz * float3(right_anti_dual_g1)), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * right_anti_dual_g0.zxy) - (wedge_g1_xyz.zxy * right_anti_dual_g0.yzx)), right_anti_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = (other.group3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (this.group0.wwwx * float4(other.group3.xyz, other.e423));
        float3 wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(right_anti_dual_g2.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (wedge_g1_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g3[1] * wedge_g0[1]) - (right_anti_dual_g3[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g1_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g1_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((this.e45 * other.e23) + (this.e4235 * other.scalar), (this.e45 * other.e31) + (this.e4315 * other.scalar), (this.e45 * other.e12) + (this.e4125 * other.scalar), -(this.e25 * other.e31) - (this.e35 * other.e12)) + (other.group0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g1[1]) + (right_anti_dual_g3[2] * wedge_g1[2]) + (right_anti_dual_g3[3] * wedge_g1[3]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g1[3] * wedge_g0[3])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) - (right_anti_dual_g1.wwwx * wedge_g1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), 0.0) + float4((float3(wedge_g1[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Line> operator /(Line lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g0_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = this.group1.xyz * other.group2.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(right_anti_dual_g0, right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(float3(wedge_g0_w), 0.0)) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1_xyz[1]) + (wedge_g0_xyz[0] * right_anti_dual_g1[3]), (right_anti_dual_g0[0] * wedge_g1_xyz[2]) + (wedge_g0_xyz[1] * right_anti_dual_g1[3]), (right_anti_dual_g0[1] * wedge_g1_xyz[0]) + (wedge_g0_xyz[2] * right_anti_dual_g1[3]), -(right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])) - float4((right_anti_dual_g0.yzx * wedge_g1_xyz.zxy), right_anti_dual_g0[0] * wedge_g0_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g0.zxy) - (right_anti_dual_g3.zxy * wedge_g0.yzx)), right_anti_dual_g2[3] * wedge_g0[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g1.zxy, -(right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]))) - (float4(wedge_g0[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e2 * this.e315) + (other.e3 * this.e125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], 1.0) * float4(right_anti_dual_g1.www, -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g0[3]))) - (wedge_g0.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(AntiLine other) {
        float wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4((this.group1.xyz * other.group0.www), 0.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_w) * right_anti_dual_g0.xyz)), wedge_g0_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(wedge_g0_w)) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(wedge_g0_xyz[0] * right_anti_dual_g1[0]) - (wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e2 * this.e315) + (other.e3 * this.e125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Line(
            /* e415, e425, e435 */
            (right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Circle anti_reject_orthogonally_from(Dipole other) {
        float wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz * float3(-1.0), 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g6 = float4((this.group0.xyz * float3(other.group0.xx, other.scalar)), 0.0);
        float3 wedge_g8 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235));
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4(((float3(right_anti_dual_g6[3]) * wedge_g6.xyz) + (float3(wedge_g6[3]) * right_anti_dual_g6.xyz)), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Circle anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g1) * wedge_g0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g0.zxy) - (right_anti_dual_g0.zxy * wedge_g0.yzx)), right_anti_dual_g1 * wedge_g0[3] * -1.0), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g0.xyz) - (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g0.zxy) - (right_anti_dual_g3.zxy * wedge_g0.yzx)), right_anti_dual_g2[3] * wedge_g0[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float3(wedge_g0[3]) * right_anti_dual_g3.xyz * float3(-1.0)), (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g1.zxy, -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[2] * wedge_g0[2]))) - (wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - (wedge_g0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4((float3(other.scalar) * this.group0.xyz), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 wedge_g1 = float4((this.group1.xyz * other.group0.www), 0.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1[1]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g0[0] * wedge_g1[2]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g0[1] * wedge_g1[0]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Motor> operator /(Motor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (other.group0.xyz * this.group1.www);
        float wedge_g0_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(right_anti_dual_g0, right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g2[3])) + (float3(wedge_g0_w) * right_anti_dual_g1.xyz)), wedge_g0_w * right_anti_dual_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(float3(wedge_g0_w), other.scalar * this.e5)) + float4((wedge_g1_xyz * right_anti_dual_g2.www), -(wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1_xyz[1]) + (wedge_g0_xyz[0] * right_anti_dual_g1[3]), (right_anti_dual_g0[0] * wedge_g1_xyz[2]) + (wedge_g0_xyz[1] * right_anti_dual_g1[3]), (right_anti_dual_g0[1] * wedge_g1_xyz[0]) + (wedge_g0_xyz[2] * right_anti_dual_g1[3]), -(right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])) - float4((right_anti_dual_g0.yzx * wedge_g1_xyz.zxy), right_anti_dual_g0[0] * wedge_g0_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e4) * float4(-1.0);
        float4 wedge_g1 = float4(-(other.e4 * this.e235) - (other.e3 * this.e425), -(other.e4 * this.e315) - (other.e1 * this.e435), -(other.e4 * this.e125) - (other.e2 * this.e415), (other.e2 * this.e315) + (other.e3 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4(other.group0.xyz, other.e321));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g1.zxy) - (right_anti_dual_g3.zxy * wedge_g1.yzx)), right_anti_dual_g2[3] * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (wedge_g1[1] * other.e431) + (wedge_g1[2] * other.e412)) - float4((right_anti_dual_g3.xyz * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g2[2] * wedge_g1[2])) + float4((right_anti_dual_g1.zxy * wedge_g1.yzx), right_anti_dual_g3[0] * wedge_g0[0]) - (float4(wedge_g1[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g0.xyz, wedge_g1[1])) - (wedge_g1.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g1[3] * other.scalar)) * float4(wedge_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public DualNum anti_reject_orthogonally_from(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * other.e321 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0_xyz = other.group1.xyz * this.group1.www * float3(-1.0);
        float4 wedge_g1 = (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group0.yzx), other.e321 * this.e5);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g1.yzx * wedge_g1.zxy) - (right_anti_dual_g1.zxy * wedge_g1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1.wwwx * float4(wedge_g1.xyz, wedge_g0_xyz[0])) + float4(float3(0.0), (wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * wedge_g1[3])) - (wedge_g1.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(AntiLine other) {
        float wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = other.group0.xyz * this.group1.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float3(wedge_g0_w)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(wedge_g0_w) * other.group1.xyz * float3(-1.0)), -(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(AntiMotor other) {
        float3 wedge_g0_xyz = this.group0.xyz * other.group0.www;
        float wedge_g0_w = (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), other.scalar * this.e5);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_w) * right_anti_dual_g0.xyz)), wedge_g0_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(wedge_g0_w)) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(wedge_g0_xyz[0] * right_anti_dual_g1[0]) - (wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group1.www * float3(-1.0);
        float4 wedge_g1 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e2 * this.e315) + (other.e3 * this.e125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g0.wwwx * float4(wedge_g1.xyz, wedge_g0_xyz[0])) + float4((float3(wedge_g1[3]) * right_anti_dual_g0.xyz * float3(-1.0)), (wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (wedge_g0_xyz[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e321) * float4(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g0[1] * other.e431) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = other.group1.xyz * this.group1.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g0_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g0_w), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(wedge_g0_w)), -(right_anti_dual_g0[0] * wedge_g0_xyz[0]) - (right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g1_xyz[1]) + (wedge_g0_xyz[0] * right_anti_dual_g1[3]), (right_anti_dual_g0[0] * wedge_g1_xyz[2]) + (wedge_g0_xyz[1] * right_anti_dual_g1[3]), (right_anti_dual_g0[1] * wedge_g1_xyz[0]) + (wedge_g0_xyz[2] * right_anti_dual_g1[3]), -(right_anti_dual_g2[1] * wedge_g0_xyz[1]) - (right_anti_dual_g2[2] * wedge_g0_xyz[2]) - (wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g1_xyz.zxy), right_anti_dual_g2[0] * wedge_g0_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 wedge_g1_xyz = other.group1.xyz * this.group1.www;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g0_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g0_w), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0_w) * right_anti_dual_g2.xyz), (wedge_g0_w * right_anti_dual_g2[3]) - (right_anti_dual_g0[0] * wedge_g0_xyz[0]) - (right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0_w * right_anti_dual_g3[0]) + (wedge_g0_xyz[0] * right_anti_dual_g1[3]), (wedge_g0_w * right_anti_dual_g3[1]) + (wedge_g0_xyz[1] * right_anti_dual_g1[3]), (wedge_g0_w * right_anti_dual_g3[2]) + (wedge_g0_xyz[2] * right_anti_dual_g1[3]), -(wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1_xyz.yzx), wedge_g0_w * right_anti_dual_g3[3]) - float4((right_anti_dual_g0.yzx * wedge_g1_xyz.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float wedge_g2 = this.e5 * other.scalar;
        float4 wedge_g3 = float4(this.e5) * other.group1 * float4(-1.0);
        float4 wedge_g6 = float4(((float3(this.e5) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz)), 0.0);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235)) - float4((other.group7.xyz * this.group1.www), this.e5 * other.e321);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (float4(((float3(right_anti_dual_g6[3]) * wedge_g6.xyz) + (float3(wedge_g6[3]) * right_anti_dual_g6.xyz)), right_anti_dual_g10 * wedge_g3[3]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), 0.0) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.e5) * other.group0 * float4(-1.0);
        float4 wedge_g1 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g1) * wedge_g1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), right_anti_dual_g1 * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) - (float3(wedge_g1[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g1 * wedge_g0[3] * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual_g1) * wedge_g0.xyz * float3(-1.0)), (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e235, e315, e125, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = this.e5 * other.e1234;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(this.e5) * other.group3 * float4(-1.0);
        float4 wedge_g1 = float4(-(this.e235 * other.e4) - (this.e5 * other.e423), -(this.e315 * other.e4) - (this.e5 * other.e431), -(this.e125 * other.e4) - (this.e5 * other.e412), (this.e315 * other.e2) + (this.e125 * other.e3)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235)) - float4((this.group0.yzx * other.group3.zxy), this.e5 * other.e321);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2[3]) * wedge_g1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3.yzx * wedge_g1.zxy) - (right_anti_dual_g3.zxy * wedge_g1.yzx)), right_anti_dual_g2[3] * wedge_g1[3] * -1.0), 
            /* e235, e315, e125, e4 */
            (wedge_g1.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * wedge_g1[1]) + (right_anti_dual_g0[2] * wedge_g1[2])) - float4((right_anti_dual_g3.xyz * wedge_g1.www), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((right_anti_dual_g1.zxy * wedge_g1.yzx), right_anti_dual_g3[0] * wedge_g0[0]) - (right_anti_dual_g2.wwwx * float4(wedge_g0.xyz, wedge_g1[0])) - (wedge_g1.zxyy * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[1])) - (wedge_g1.wwwz * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (float4(this.e5) * float4(other.group0.xyz, other.e1234)) + (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 wedge_g1 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e5 * other.scalar);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g1[1]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g0[0] * wedge_g1[2]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g0[1] * wedge_g1[0]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) - (right_anti_dual_g0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<MultiVector> operator /(MultiVector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiCircleRotor other) {
        float wedge_g0_y = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g3 = (float4(other.scalar) * this.group3) + (float4(this.scalar) * float4(other.group2.xyz, other.e45));
        float3 wedge_g4 = (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group3.zxy), other.scalar * this.e3215) - (other.group2.yzxy * float4(this.group4.zxy, this.e31)) - float4((other.group0.zxy * this.group3.yzx), other.e15 * this.e23);
        float wedge_g10 = (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g2[3] * other.scalar * this.scalar) - (right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g4[0] * right_anti_dual_g2[0]) - (wedge_g4[1] * right_anti_dual_g2[1]) - (wedge_g4[2] * right_anti_dual_g2[2]) - (wedge_g5[0] * right_anti_dual_g1[0]) - (wedge_g5[1] * right_anti_dual_g1[1]) - (wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * wedge_g3[3]), wedge_g0_y * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[0]) + (right_anti_dual_g2[3] * wedge_g1[0]), (wedge_g7[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[1]) + (right_anti_dual_g2[3] * wedge_g1[1]), (wedge_g7[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[2]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(right_anti_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_anti_dual_g1[0]) - (wedge_g7[1] * right_anti_dual_g1[1]) - (wedge_g7[2] * right_anti_dual_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g8.yzx), right_anti_dual_g2[3] * wedge_g1[3]) - float4((right_anti_dual_g0.yzx * wedge_g8.zxy), right_anti_dual_g0[0] * wedge_g6[0]) - float4((wedge_g7.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            (right_anti_dual_g2[3] * other.scalar * this.e5) - (wedge_g8[0] * right_anti_dual_g1[0]) - (wedge_g8[1] * right_anti_dual_g1[1]) - (wedge_g8[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g6[0]) - (right_anti_dual_g2[1] * wedge_g6[1]) - (right_anti_dual_g2[2] * wedge_g6[2]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g2[1] * wedge_g9[2]) + (right_anti_dual_g2[3] * wedge_g3[0]), (right_anti_dual_g2[2] * wedge_g9[0]) + (right_anti_dual_g2[3] * wedge_g3[1]), (right_anti_dual_g2[0] * wedge_g9[1]) + (right_anti_dual_g2[3] * wedge_g3[2]), -(right_anti_dual_g1[1] * wedge_g9[1]) - (right_anti_dual_g1[2] * wedge_g9[2])) + float4((right_anti_dual_g1.xyz * wedge_g9.www), right_anti_dual_g2[3] * wedge_g3[3]) - (wedge_g9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g2[3])) + (float3(wedge_g10) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g9.yzx) - (right_anti_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(wedge_g9[3])) + (wedge_g5 * float3(right_anti_dual_g2[3])) + (float3(wedge_g10) * right_anti_dual_g2.xyz) - (float3(right_anti_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g2[3])), 
            /* e423, e431, e412 */
            (right_anti_dual_g0 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g2[3])), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(right_anti_dual_g2[3])) + (float3(wedge_g0_y) * right_anti_dual_g2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(right_anti_dual_g2[3]), 
            /* e1234 */
            wedge_g10 * right_anti_dual_g2[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float wedge_g0_y = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * float4(other.group3.xyz, other.e4));
        float3 wedge_g4 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - float4((other.group3.xyz * this.group3.www), other.e1 * this.e23);
        float3 wedge_g7 = (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group3.zxy) - (other.group3.zxy * this.group3.yzx);
        float4 wedge_g9 = (other.group3.yzxw * this.group6.zxyw) + (other.group3.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group1.yzx * this.group1.zxy), other.e2 * this.e315) + float4((other.group2.xyz * this.group1.www), other.e3 * this.e125) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group2.www), other.e235 * this.e1) - float4((other.group3.zxy * this.group6.yzx), other.e125 * this.e3);
        float wedge_g10 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g3[0] * wedge_g1[0]) + (right_anti_dual_g3[1] * wedge_g1[1]) + (right_anti_dual_g3[2] * wedge_g1[2]) + (right_anti_dual_g3[3] * wedge_g1[3]) + (right_anti_dual_g2[3] * other.e5 * this.scalar) - (wedge_g7[0] * right_anti_dual_g2[0]) - (wedge_g7[1] * right_anti_dual_g2[1]) - (wedge_g7[2] * right_anti_dual_g2[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g6[0]) - (right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2]) - (right_anti_dual_g1[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g10) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) + (wedge_g9.yzxz * float4(right_anti_dual_g1.zxy, other.e412)) + float4((wedge_g4 * right_anti_dual_g3.www), wedge_g9[0] * other.e423) + float4((wedge_g5.yzx * right_anti_dual_g3.zxy), wedge_g9[1] * other.e431) - (wedge_g3 * float4(right_anti_dual_g2[3])) - (right_anti_dual_g3.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((other.group0.xyz * wedge_g9.www), wedge_g4[0] * right_anti_dual_g3[0]) - float4((right_anti_dual_g1.yzx * wedge_g9.zxy), wedge_g4[2] * right_anti_dual_g3[2]), 
            /* e5 */
            (right_anti_dual_g3[0] * wedge_g3[0]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g2[0] * wedge_g9[0]) - (right_anti_dual_g2[1] * wedge_g9[1]) - (right_anti_dual_g2[2] * wedge_g9[2]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_anti_dual_g3[2]) + (right_anti_dual_g3[3] * wedge_g6[0]), (wedge_g8[2] * right_anti_dual_g3[0]) + (right_anti_dual_g3[3] * wedge_g6[1]), (wedge_g8[0] * right_anti_dual_g3[1]) + (right_anti_dual_g3[3] * wedge_g6[2]), -(right_anti_dual_g3[1] * wedge_g6[1]) - (right_anti_dual_g3[2] * wedge_g6[2])) + (float4(wedge_g0_y) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) - float4((wedge_g8.zxy * right_anti_dual_g3.yzx), right_anti_dual_g3[0] * wedge_g6[0]), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group0.xyz) + (float3(right_anti_dual_g2[3]) * wedge_g6.xyz) + (wedge_g7.zxy * right_anti_dual_g3.yzx) - (wedge_g7.yzx * right_anti_dual_g3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_anti_dual_g3[3])) + (wedge_g8 * float3(right_anti_dual_g2[3])) + (float3(wedge_g0_y) * right_anti_dual_g1.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g9.zxy, wedge_g10)) - (wedge_g9.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g3[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0_y), 
            /* e1234 */
            wedge_g0_y * right_anti_dual_g2[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiDualNum other) {
        float wedge_g0_y = (other.e3215 * this.e4) + (other.scalar * this.e12345);
        float wedge_g10 = other.scalar * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e3215) + (other.scalar * other.scalar * this.scalar), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e5 */
            (wedge_g0_y * other.e3215) + (other.scalar * other.scalar * this.e5), 
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
            float4(other.scalar) * float4(other.group0.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            wedge_g10 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiFlatPoint other) {
        float3 wedge_g9_xyz = float3(this.e4) * other.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[3] * other.e321 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            (right_anti_dual_g0[3] * other.e235 * this.e1) + (right_anti_dual_g0[3] * other.e315 * this.e2) + (right_anti_dual_g0[3] * other.e125 * this.e3) + (right_anti_dual_g0[3] * other.e321 * this.e5) - (wedge_g9_xyz[0] * right_anti_dual_g0[0]) - (wedge_g9_xyz[1] * right_anti_dual_g0[1]) - (wedge_g9_xyz[2] * right_anti_dual_g0[2]), 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiFlector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45);
        float3 wedge_g1_xyz = other.group1.xyz * float3(this.group0.xx, this.scalar);
        float3 wedge_g3_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz);
        float3 wedge_g4 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g5 = (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group4.xyz * other.group1.www), other.e321 * this.scalar) - float4((other.group1.xyz * this.group3.www), other.e1 * this.e23);
        float3 wedge_g7 = (this.group4.yzx * other.group1.zxy) - (this.group4.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx);
        float4 wedge_g9 = (other.group1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group1.yzx * this.group6.zxy), other.e3 * this.e125) + float4((other.group0.xyz * this.group1.www), other.e2 * this.e315) - float4((other.group1.zxy * this.group6.yzx), other.e235 * this.e1);
        float wedge_g10 = (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * right_anti_dual_g1[0]) + (wedge_g1_xyz[1] * right_anti_dual_g1[1]) + (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (wedge_g7[0] * right_anti_dual_g0[0]) - (wedge_g7[1] * right_anti_dual_g0[1]) - (wedge_g7[2] * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g0[0]) + (wedge_g5[1] * right_anti_dual_g1[2]), (wedge_g10 * right_anti_dual_g0[1]) + (wedge_g5[2] * right_anti_dual_g1[0]), (wedge_g10 * right_anti_dual_g0[2]) + (wedge_g5[0] * right_anti_dual_g1[1]), -(wedge_g4[1] * right_anti_dual_g1[1]) - (wedge_g4[2] * right_anti_dual_g1[2])) + float4((wedge_g4 * right_anti_dual_g1.www), wedge_g10 * right_anti_dual_g0[3]) - (right_anti_dual_g1.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            (wedge_g3_xyz[0] * right_anti_dual_g1[0]) + (wedge_g3_xyz[1] * right_anti_dual_g1[1]) + (wedge_g3_xyz[2] * right_anti_dual_g1[2]) + (right_anti_dual_g1[3] * other.e5 * this.e4) - (right_anti_dual_g0[0] * wedge_g9[0]) - (right_anti_dual_g0[1] * wedge_g9[1]) - (right_anti_dual_g0[2] * wedge_g9[2]) - (right_anti_dual_g0[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_anti_dual_g1[2]) + (right_anti_dual_g1[3] * wedge_g6[0]), (wedge_g8[2] * right_anti_dual_g1[0]) + (right_anti_dual_g1[3] * wedge_g6[1]), (wedge_g8[0] * right_anti_dual_g1[1]) + (right_anti_dual_g1[3] * wedge_g6[2]), -(right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2])) + (right_anti_dual_g0 * float4(wedge_g0_y)) - float4((wedge_g8.zxy * right_anti_dual_g1.yzx), right_anti_dual_g1[0] * wedge_g6[0]), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * right_anti_dual_g1.yzx) - (wedge_g7.yzx * right_anti_dual_g1.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_anti_dual_g1[3])) - (float3(wedge_g6[3]) * right_anti_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * wedge_g9.zxy) - (right_anti_dual_g1.zxy * wedge_g9.yzx)), wedge_g10 * right_anti_dual_g1[3]), 
            /* e423, e431, e412 */
            float3(wedge_g10) * right_anti_dual_g1.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g1[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g0_y), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiLine other) {
        float wedge_g0_y = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 wedge_g5 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g6_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g6_w = -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3);
        float3 wedge_g7 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g8 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group4.zxy), other.e23 * this.e15);
        float wedge_g10 = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g0[0] * wedge_g5[0]) - (right_anti_dual_g0[1] * wedge_g5[1]) - (right_anti_dual_g0[2] * wedge_g5[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g7[2]), (wedge_g6_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g7[0]), (wedge_g6_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g7[1]), -(right_anti_dual_g0[1] * wedge_g7[1]) - (right_anti_dual_g0[2] * wedge_g7[2])) - float4((right_anti_dual_g1.zxy * wedge_g7.yzx), right_anti_dual_g0[0] * wedge_g7[0]), 
            /* e5 */
            -(right_anti_dual_g0[0] * wedge_g8[0]) - (right_anti_dual_g0[1] * wedge_g8[1]) - (right_anti_dual_g0[2] * wedge_g8[2]) - (right_anti_dual_g1[0] * wedge_g6_xyz[0]) - (right_anti_dual_g1[1] * wedge_g6_xyz[1]) - (right_anti_dual_g1[2] * wedge_g6_xyz[2]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g0[0] * wedge_g9[3]) + (right_anti_dual_g1[1] * wedge_g9[2]), (right_anti_dual_g0[1] * wedge_g9[3]) + (right_anti_dual_g1[2] * wedge_g9[0]), (right_anti_dual_g0[2] * wedge_g9[3]) + (right_anti_dual_g1[0] * wedge_g9[1]), -(right_anti_dual_g0[1] * wedge_g9[1]) - (right_anti_dual_g0[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(wedge_g10), 
            /* e23, e31, e12 */
            right_anti_dual_g1 * float3(wedge_g10), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * float3(wedge_g0_y)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_anti_dual_g1 * float3(wedge_g0_y), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiMotor other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g3_xyz = (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz);
        float3 wedge_g4 = float3(other.scalar) * this.group4.xyz;
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g6 = (float4(other.scalar) * this.group6) + float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3));
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = (other.group0 * float4(this.group3.www, this.e3215)) + (other.group1.zxyw * float4(this.group4.yzx, this.scalar)) + float4((float3(other.scalar) * this.group9.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group1.yzxx * float4(this.group4.zxy, this.e23));
        float wedge_g10 = (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * right_anti_dual_g1[3]) + (right_anti_dual_g0[3] * other.scalar * this.scalar) - (wedge_g4[0] * right_anti_dual_g1[0]) - (wedge_g4[1] * right_anti_dual_g1[1]) - (wedge_g4[2] * right_anti_dual_g1[2]) - (wedge_g5[0] * right_anti_dual_g0[0]) - (wedge_g5[1] * right_anti_dual_g0[1]) - (wedge_g5[2] * right_anti_dual_g0[2]), wedge_g0_y * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[0] * wedge_g6[3]) + (right_anti_dual_g0[3] * wedge_g1[0]), (right_anti_dual_g0[1] * wedge_g6[3]) + (right_anti_dual_g0[3] * wedge_g1[1]), (right_anti_dual_g0[2] * wedge_g6[3]) + (right_anti_dual_g0[3] * wedge_g1[2]), -(wedge_g7[1] * right_anti_dual_g0[1]) - (wedge_g7[2] * right_anti_dual_g0[2])) + float4((wedge_g7.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[3] * wedge_g1[3]) - float4((wedge_g7.yzx * right_anti_dual_g1.zxy), wedge_g7[0] * right_anti_dual_g0[0]), 
            /* e5 */
            (wedge_g0_y * right_anti_dual_g1[3]) + (right_anti_dual_g0[3] * other.scalar * this.e5) - (wedge_g8[0] * right_anti_dual_g0[0]) - (wedge_g8[1] * right_anti_dual_g0[1]) - (wedge_g8[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g6[0]) - (right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g3_xyz[0] * right_anti_dual_g0[3]) + (right_anti_dual_g1[1] * wedge_g9[2]), (wedge_g3_xyz[1] * right_anti_dual_g0[3]) + (right_anti_dual_g1[2] * wedge_g9[0]), (wedge_g3_xyz[2] * right_anti_dual_g0[3]) + (right_anti_dual_g1[0] * wedge_g9[1]), -(right_anti_dual_g0[1] * wedge_g9[1]) - (right_anti_dual_g0[2] * wedge_g9[2])) + (right_anti_dual_g0 * float4(wedge_g9.www, other.scalar * this.e45)) - (wedge_g9.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[3])) + (float3(wedge_g10) * right_anti_dual_g0.xyz), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(right_anti_dual_g0[3])) + (float3(wedge_g10) * right_anti_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0_y) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * wedge_g6.xyz)), right_anti_dual_g0[3] * wedge_g6[3]), 
            /* e423, e431, e412 */
            wedge_g7 * float3(right_anti_dual_g0[3]), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_y) * right_anti_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(right_anti_dual_g0[3]), 
            /* e1234 */
            wedge_g10 * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g1_xyz = other.group0.xyz * float3(this.group0.xx, this.scalar);
        float3 wedge_g3_xyz = (float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz);
        float3 wedge_g4 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g5 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float3 wedge_g6_xyz = (float3(other.e5) * this.group4.xyz) - (other.group0.xyz * this.group3.www);
        float3 wedge_g7 = (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g9 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group7.xyz, this.e235)) + float4((other.group0.yzx * this.group6.zxy), other.e2 * this.e315);
        float wedge_g10 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * right_anti_dual_g0[0]) + (wedge_g1_xyz[1] * right_anti_dual_g0[1]) + (wedge_g1_xyz[2] * right_anti_dual_g0[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_anti_dual_g0[3]) + (wedge_g5[1] * right_anti_dual_g0[2]), (wedge_g4[1] * right_anti_dual_g0[3]) + (wedge_g5[2] * right_anti_dual_g0[0]), (wedge_g4[2] * right_anti_dual_g0[3]) + (wedge_g5[0] * right_anti_dual_g0[1]), -(wedge_g4[1] * right_anti_dual_g0[1]) - (wedge_g4[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            (wedge_g3_xyz[0] * right_anti_dual_g0[0]) + (wedge_g3_xyz[1] * right_anti_dual_g0[1]) + (wedge_g3_xyz[2] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * other.e5 * this.e4), 
            /* e15, e25, e35, e45 */
            float4((wedge_g6_xyz[0] * right_anti_dual_g0[3]) + (wedge_g8[1] * right_anti_dual_g0[2]), (wedge_g6_xyz[1] * right_anti_dual_g0[3]) + (wedge_g8[2] * right_anti_dual_g0[0]), (wedge_g6_xyz[2] * right_anti_dual_g0[3]) + (wedge_g8[0] * right_anti_dual_g0[1]), -(wedge_g6_xyz[1] * right_anti_dual_g0[1]) - (wedge_g6_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g8.zxy, wedge_g6_xyz[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * right_anti_dual_g0.yzx) - (wedge_g7.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_anti_dual_g0[3])) - (float3(-(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)) * right_anti_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g9.zxy) - (right_anti_dual_g0.zxy * wedge_g9.yzx)), wedge_g10 * right_anti_dual_g0[3]), 
            /* e423, e431, e412 */
            float3(wedge_g10) * right_anti_dual_g0.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Circle other) {
        float wedge_g0_y = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g6 = float4(this.scalar) * other.group1;
        float3 wedge_g7 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group2.xyz;
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1);
        float wedge_g10 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g7[0] * other.e235) - (wedge_g7[1] * other.e315) - (wedge_g7[2] * other.e125) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g6[0]) - (right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2]) - (right_anti_dual_g1[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g9[2]) - (wedge_g9[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g9[0]) - (wedge_g9[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g9[1]) - (wedge_g9[3] * other.e412), (wedge_g10 * right_anti_dual_g1[3]) + (wedge_g9[2] * other.e412)) + (wedge_g9.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g10) * other.group2.xyz), wedge_g9[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g9[3]) - (wedge_g9[0] * other.e235) - (wedge_g9[1] * other.e315) - (wedge_g9[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(other.group2.xyz, right_anti_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * right_anti_dual_g1.xyz, 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(CircleRotor other) {
        float wedge_g0_y = (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g6 = float4(this.scalar) * other.group1;
        float3 wedge_g7 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g8 = float3(this.scalar) * other.group2.xyz;
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235));
        float wedge_g10 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_anti_dual_g2[3]) - (wedge_g7[0] * right_anti_dual_g2[0]) - (wedge_g7[1] * right_anti_dual_g2[1]) - (wedge_g7[2] * right_anti_dual_g2[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g6[0]) - (right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2]) - (right_anti_dual_g1[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g9[2]) - (wedge_g9[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g9[0]) - (wedge_g9[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g9[1]) - (wedge_g9[3] * other.e412), (wedge_g10 * right_anti_dual_g1[3]) + (wedge_g9[2] * other.e412)) + (wedge_g9.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g10) * right_anti_dual_g2.xyz), wedge_g9[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g2[0] * wedge_g9[0]) - (right_anti_dual_g2[1] * wedge_g9[1]) - (right_anti_dual_g2[2] * wedge_g9[2]), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * right_anti_dual_g1.xyz, 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Dipole other) {
        float wedge_g0_y = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g6 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group3.yzx), other.e15 * this.e23) - float4((other.group2.yzx * this.group4.zxy), other.e25 * this.e31);
        float wedge_g10 = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g1[0]) - (wedge_g5[1] * right_anti_dual_g1[1]) - (wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * wedge_g3[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g8[1]) + (right_anti_dual_g2[1] * wedge_g7[2]) + (right_anti_dual_g1[0] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[0]), (right_anti_dual_g0[0] * wedge_g8[2]) + (right_anti_dual_g2[2] * wedge_g7[0]) + (right_anti_dual_g1[1] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[1]), (right_anti_dual_g0[1] * wedge_g8[0]) + (right_anti_dual_g2[0] * wedge_g7[1]) + (right_anti_dual_g1[2] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[2]), -(right_anti_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_anti_dual_g1[0]) - (wedge_g7[1] * right_anti_dual_g1[1]) - (wedge_g7[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g8.zxy), right_anti_dual_g0[0] * wedge_g6[0]) - float4((right_anti_dual_g2.zxy * wedge_g7.yzx), right_anti_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            -(right_anti_dual_g2[0] * wedge_g6[0]) - (right_anti_dual_g2[1] * wedge_g6[1]) - (right_anti_dual_g2[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g1[0]) - (wedge_g8[1] * right_anti_dual_g1[1]) - (wedge_g8[2] * right_anti_dual_g1[2]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g2[1] * wedge_g9[2]) + (right_anti_dual_g1[0] * wedge_g9[3]), (right_anti_dual_g2[2] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[3]), (right_anti_dual_g2[0] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[3]), -(right_anti_dual_g1[1] * wedge_g9[1]) - (right_anti_dual_g1[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g9.yzx) - (right_anti_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(wedge_g9[3])) + (right_anti_dual_g2 * float3(wedge_g10)) - (float3(right_anti_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g0_y), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g0_y), 
            /* e235, e315, e125 */
            right_anti_dual_g2 * float3(wedge_g0_y), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(DipoleInversion other) {
        float wedge_g0_y = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, other.e45);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g6 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group3) - (other.group2.yzxy * float4(this.group4.zxy, this.e31)) - float4((other.group0.zxy * this.group3.yzx), other.e15 * this.e23);
        float wedge_g10 = (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * right_anti_dual_g3[3]) + (right_anti_dual_g2[3] * wedge_g9[3]) + (right_anti_dual_g3[0] * wedge_g9[0]) + (right_anti_dual_g3[1] * wedge_g9[1]) + (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g4[0] * right_anti_dual_g2[0]) - (wedge_g4[1] * right_anti_dual_g2[1]) - (wedge_g4[2] * right_anti_dual_g2[2]) - (wedge_g5[0] * right_anti_dual_g1[0]) - (wedge_g5[1] * right_anti_dual_g1[1]) - (wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * wedge_g3[3]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g8[1]) + (wedge_g7[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[0]), (right_anti_dual_g0[0] * wedge_g8[2]) + (wedge_g7[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[1]), (right_anti_dual_g0[1] * wedge_g8[0]) + (wedge_g7[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[2]), -(right_anti_dual_g0[2] * wedge_g6[2]) - (wedge_g7[0] * right_anti_dual_g1[0]) - (wedge_g7[1] * right_anti_dual_g1[1]) - (wedge_g7[2] * right_anti_dual_g1[2])) + (float4(wedge_g0_y) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])) - float4((right_anti_dual_g0.yzx * wedge_g8.zxy), right_anti_dual_g0[0] * wedge_g6[0]) - float4((wedge_g7.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * right_anti_dual_g3[3]) - (wedge_g8[0] * right_anti_dual_g1[0]) - (wedge_g8[1] * right_anti_dual_g1[1]) - (wedge_g8[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g6[0]) - (right_anti_dual_g2[1] * wedge_g6[1]) - (right_anti_dual_g2[2] * wedge_g6[2]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g1[0] * wedge_g9[3]) + (right_anti_dual_g2[1] * wedge_g9[2]), (right_anti_dual_g1[1] * wedge_g9[3]) + (right_anti_dual_g2[2] * wedge_g9[0]), (right_anti_dual_g1[2] * wedge_g9[3]) + (right_anti_dual_g2[0] * wedge_g9[1]), -(right_anti_dual_g1[1] * wedge_g9[1]) - (right_anti_dual_g1[2] * wedge_g9[2])) - (wedge_g9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g9.yzx) - (right_anti_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(wedge_g9[3])) + (float3(wedge_g10) * right_anti_dual_g2.xyz) - (float3(right_anti_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g0_y), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g0_y), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * right_anti_dual_g2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(DualNum other) {
        float wedge_g0_y = (other.e5 * this.e1234) + (other.e12345 * this.scalar);
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0_y * right_anti_dual_g0[1], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_anti_dual_g0[0] * other.e5 * this.e4, 
            /* e15, e25, e35, e45 */
            float4((this.group4.xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0]) * float3(other.group0.xx, other.e5)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * (float4(other.e5) * float4(this.group7.xyz, this.e321)).xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * right_anti_dual_g0[0]), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(FlatPoint other) {
        float wedge_g0_y = -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz);
        float4 wedge_g9 = float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group4.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[3] * other.e45 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_xyz * float3(right_anti_dual_g0[3])), 0.0), 
            /* e5 */
            -(wedge_g6_xyz[0] * right_anti_dual_g0[0]) - (wedge_g6_xyz[1] * right_anti_dual_g0[1]) - (wedge_g6_xyz[2] * right_anti_dual_g0[2]), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g0.yzx * wedge_g9.zxy) - (right_anti_dual_g0.zxy * wedge_g9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[3]) * wedge_g9.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * right_anti_dual_g0[3]), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * right_anti_dual_g0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Flector other) {
        float wedge_g0_y = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz);
        float4 wedge_g9 = float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group4.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) - (right_anti_dual_g0[3] * other.e45 * this.scalar), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g6_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_y) * right_anti_dual_g1.xyz)), 0.0), 
            /* e5 */
            (wedge_g0_y * right_anti_dual_g1[3]) - (wedge_g6_xyz[0] * right_anti_dual_g0[0]) - (wedge_g6_xyz[1] * right_anti_dual_g0[1]) - (wedge_g6_xyz[2] * right_anti_dual_g0[2]), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g0.yzx * wedge_g9.zxy) - (right_anti_dual_g0.zxy * wedge_g9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[3]) * wedge_g9.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * right_anti_dual_g0[3]), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * right_anti_dual_g0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Line other) {
        float wedge_g0_y = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float3 wedge_g6_xyz = other.group0.xyz * float3(this.group0.xx, this.scalar);
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Motor other) {
        float wedge_g0_y = (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float3 wedge_g6_xyz = (float3(other.e5) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((this.group7.xyz * other.group1.www), other.e5 * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_anti_dual_g0[3]) - (wedge_g6_xyz[0] * right_anti_dual_g0[0]) - (wedge_g6_xyz[1] * right_anti_dual_g0[1]) - (wedge_g6_xyz[2] * right_anti_dual_g0[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((right_anti_dual_g0.zxy * wedge_g9.yzx) - (right_anti_dual_g0.yzx * wedge_g9.zxy)), 0.0), 
            /* e5 */
            (right_anti_dual_g1[3] * other.e5 * this.e4) - (right_anti_dual_g1[0] * wedge_g9[0]) - (right_anti_dual_g1[1] * wedge_g9[1]) - (right_anti_dual_g1[2] * wedge_g9[2]), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g6_xyz * float3(right_anti_dual_g1[3])) + (float3(wedge_g0_y) * right_anti_dual_g1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * right_anti_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g1[3]) * wedge_g9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * right_anti_dual_g1[3]), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float wedge_g2 = (other.scalar * this.e5) + (other.e5 * this.scalar);
        float4 wedge_g3 = (float4(other.scalar) * this.group3) + (float4(other.e5) * this.group1) + (float4(this.scalar) * other.group3) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group5.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((other.e4 * this.e15) + (other.e5 * this.e41) + (other.e15 * this.e4) + (other.e41 * this.e5), (other.e4 * this.e25) + (other.e5 * this.e42) + (other.e25 * this.e4) + (other.e42 * this.e5), (other.e4 * this.e35) + (other.e5 * this.e43) + (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group6) + (float4(this.scalar) * other.group6) - (this.group1.xyzy * float4(other.group3.www, other.e31)) - float4((other.group1.xyz * this.group3.www), other.e23 * this.e1);
        float3 wedge_g7 = (float3(other.scalar) * this.group7.xyz) + (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group1.zxy) + (this.group4.yzx * other.group1.zxy) - (other.group4.zxy * this.group1.yzx) - (this.group4.zxy * other.group1.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group8.xyz) + (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group8.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group1.yzx * this.group3.zxy) + (other.group3.zxy * this.group1.yzx) - (other.group1.zxy * this.group3.yzx) - (other.group3.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e2 * this.e435) + (other.e5 * this.e423) + (other.e45 * this.e23) + (other.e425 * this.e3), (other.e3 * this.e415) + (other.e5 * this.e431) + (other.e45 * this.e31) + (other.e435 * this.e1), (other.e1 * this.e425) + (other.e5 * this.e412) + (other.e45 * this.e12) + (other.e415 * this.e2), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e321 * this.e5)) + (float4(other.scalar) * this.group9) + (float4(this.scalar) * other.group9) + float4((other.group5.xyz * this.group3.www), other.e2 * this.e315) + float4((other.group8.xyz * this.group1.www), other.e3 * this.e125) + float4((other.group4.yzx * this.group3.zxy), other.e1 * this.e235) + float4((this.group4.yzx * other.group3.zxy), other.e5 * this.e321) - (this.group1.yzxz * float4(other.group6.zxy, other.e125)) - (this.group3.yzxx * float4(other.group4.zxy, other.e23)) - float4((float3(this.e5) * other.group7.xyz), other.e31 * this.e25) - float4((this.group8.xyz * other.group1.www), other.e235 * this.e1) - float4((this.group4.zxy * other.group3.yzx), other.e12 * this.e35) - float4((other.group1.zxy * this.group6.yzx), other.e315 * this.e2);
        float wedge_g10 = (other.scalar * this.e1234) + (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e1234 * this.scalar) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) + (right_anti_dual_g0[1] * other.scalar * this.scalar) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Plane other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215);
        float4 wedge_g9 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * wedge_g9[0]) + (right_anti_dual_g0[1] * wedge_g9[1]) + (right_anti_dual_g0[2] * wedge_g9[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0_y) * right_anti_dual_g0.xyz), 0.0), 
            /* e5 */
            wedge_g0_y * right_anti_dual_g0[3], 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(RoundPoint other) {
        float wedge_g0_y = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5);
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * other.group0);
        float3 wedge_g4 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx);
        float3 wedge_g8 = (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy);
        float4 wedge_g9 = float4(-(this.e425 * other.e3) - (this.e235 * other.e4), -(this.e435 * other.e1) - (this.e315 * other.e4), -(this.e415 * other.e2) - (this.e125 * other.e4), (this.e321 * other.e5) + (this.e125 * other.e3)) + (other.group0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(other.e5) * this.group7.xyz), this.e235 * other.e1);
        float wedge_g10 = -(this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * wedge_g1[0]) + (right_anti_dual_g0[1] * wedge_g1[1]) + (right_anti_dual_g0[2] * wedge_g1[2]) + (right_anti_dual_g0[3] * wedge_g1[3]) + (right_anti_dual_g1 * this.scalar * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g4[0] * right_anti_dual_g0[3]) + (wedge_g5[1] * right_anti_dual_g0[2]), (wedge_g4[1] * right_anti_dual_g0[3]) + (wedge_g5[2] * right_anti_dual_g0[0]), (wedge_g4[2] * right_anti_dual_g0[3]) + (wedge_g5[0] * right_anti_dual_g0[1]), -(right_anti_dual_g1 * wedge_g3[3]) - (wedge_g4[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) - float4((float3(right_anti_dual_g1) * wedge_g3.xyz), wedge_g4[1] * right_anti_dual_g0[1]), 
            /* e5 */
            (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) + (right_anti_dual_g0[3] * wedge_g3[3]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * wedge_g6[0]), (wedge_g8[2] * right_anti_dual_g0[0]) + (right_anti_dual_g0[3] * wedge_g6[1]), (wedge_g8[0] * right_anti_dual_g0[1]) + (right_anti_dual_g0[3] * wedge_g6[2]), -(right_anti_dual_g0[1] * wedge_g6[1]) - (right_anti_dual_g0[2] * wedge_g6[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * wedge_g6.xyz) + (wedge_g7.zxy * right_anti_dual_g0.yzx) - (wedge_g7.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_anti_dual_g0[3])) + (wedge_g8 * float3(right_anti_dual_g1)) - (float3(wedge_g6[3]) * right_anti_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0.yzx * wedge_g9.zxy), wedge_g10 * right_anti_dual_g0[3]) - (wedge_g9.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g1)), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0_y), 
            /* e1234 */
            right_anti_dual_g1 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Scalar other) {
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0_y = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        float4 wedge_g9 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * wedge_g9[0]) + (right_anti_dual_g0[1] * wedge_g9[1]) + (right_anti_dual_g0[2] * wedge_g9[2]) + (right_anti_dual_g0[3] * wedge_g9[3]) + (this.scalar * other.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(wedge_g0_y), 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(VersorEven other) {
        float wedge_g0_y = (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 wedge_g1 = float4(this.scalar) * other.group3;
        float4 wedge_g3 = (float4(other.e5) * this.group1) - (float4(this.e5) * other.group3);
        float3 wedge_g4 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g5 = (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float4 wedge_g6 = float4((this.e15 * other.e4) + (this.e41 * other.e5), (this.e25 * other.e4) + (this.e42 * other.e5), (this.e35 * other.e4) + (this.e43 * other.e5), -(this.e31 * other.e2) - (this.e12 * other.e3)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(this.scalar) * other.group0.xyz) + (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx);
        float3 wedge_g8 = (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group3.yzx) - (this.group3.yzx * other.group3.zxy);
        float4 wedge_g9 = (this.group6.zxyw * float4(other.group3.yzx, other.e5)) + float4((this.group7.xyz * other.group2.www), this.e235 * other.e1) + float4((this.group1.zxy * other.group1.yzx), this.e315 * other.e2) + float4((other.group2.xyz * this.group1.www), this.e125 * other.e3) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxy * float4(other.group1.zxy, other.e315)) - float4((this.group8.xyz * other.group3.www), this.e1 * other.e235) - float4((this.group6.yzx * other.group3.zxy), this.e3 * other.e125);
        float wedge_g10 = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * right_anti_dual_g0[3]) + (right_anti_dual_g3[0] * wedge_g1[0]) + (right_anti_dual_g3[1] * wedge_g1[1]) + (right_anti_dual_g3[2] * wedge_g1[2]) + (right_anti_dual_g3[3] * wedge_g1[3]) + (right_anti_dual_g2[3] * this.scalar * other.e5) - (wedge_g7[0] * right_anti_dual_g2[0]) - (wedge_g7[1] * right_anti_dual_g2[1]) - (wedge_g7[2] * right_anti_dual_g2[2]) - (wedge_g8[0] * right_anti_dual_g0[0]) - (wedge_g8[1] * right_anti_dual_g0[1]) - (wedge_g8[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g6[0]) - (right_anti_dual_g1[1] * wedge_g6[1]) - (right_anti_dual_g1[2] * wedge_g6[2]) - (right_anti_dual_g1[3] * wedge_g6[3]), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g10) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) + (wedge_g9.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[2])) + float4((wedge_g4 * right_anti_dual_g3.www), right_anti_dual_g0[0] * wedge_g9[0]) + float4((wedge_g5.yzx * right_anti_dual_g3.zxy), right_anti_dual_g0[1] * wedge_g9[1]) - (right_anti_dual_g3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) - float4((right_anti_dual_g1.yzx * wedge_g9.zxy), wedge_g4[2] * right_anti_dual_g3[2]) - float4((right_anti_dual_g0.xyz * wedge_g9.www), right_anti_dual_g2[3] * wedge_g3[3]) - float4((wedge_g3.xyz * right_anti_dual_g2.www), wedge_g4[1] * right_anti_dual_g3[1]), 
            /* e5 */
            (right_anti_dual_g3[0] * wedge_g3[0]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g1[3] * wedge_g9[3]) - (right_anti_dual_g2[0] * wedge_g9[0]) - (right_anti_dual_g2[1] * wedge_g9[1]) - (right_anti_dual_g2[2] * wedge_g9[2]), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * right_anti_dual_g3[2]) + (right_anti_dual_g3[3] * wedge_g6[0]), (wedge_g8[2] * right_anti_dual_g3[0]) + (right_anti_dual_g3[3] * wedge_g6[1]), (wedge_g8[0] * right_anti_dual_g3[1]) + (right_anti_dual_g3[3] * wedge_g6[2]), -(right_anti_dual_g3[1] * wedge_g6[1]) - (right_anti_dual_g3[2] * wedge_g6[2])) + (float4(wedge_g0_y) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g2[3]) * wedge_g6.xyz) + (wedge_g7.zxy * right_anti_dual_g3.yzx) - (wedge_g7.yzx * right_anti_dual_g3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(right_anti_dual_g3[3])) + (wedge_g8 * float3(right_anti_dual_g2[3])) + (float3(wedge_g0_y) * right_anti_dual_g1.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3.yzx * wedge_g9.zxy), wedge_g10 * right_anti_dual_g3[3]) - (wedge_g9.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e423, e431, e412 */
            (float3(wedge_g10) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g3[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0_y), 
            /* e1234 */
            wedge_g0_y * right_anti_dual_g2[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(VersorOdd other) {
        float wedge_g0_y = (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float4 wedge_g3 = (float4(this.scalar) * float4(other.group2.xyz, other.e45)) + (float4(other.scalar) * this.group3);
        float3 wedge_g4 = (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group4.xyz);
        float3 wedge_g5 = (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group5.xyz);
        float4 wedge_g6 = float4((this.e5 * other.e41) + (this.e415 * other.scalar), (this.e5 * other.e42) + (this.e425 * other.scalar), (this.e5 * other.e43) + (this.e435 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((other.group2.xyz * this.group1.www), this.e321 * other.scalar) - (this.group1.xyzx * other.group1.wwwx);
        float3 wedge_g7 = (float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group7.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float3 wedge_g8 = (float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group8.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx);
        float4 wedge_g9 = float4((this.e35 * other.e42) + (this.e45 * other.e23) + (this.e23 * other.e45) + (this.e4235 * other.scalar), (this.e15 * other.e43) + (this.e45 * other.e31) + (this.e31 * other.e45) + (this.e4315 * other.scalar), (this.e25 * other.e41) + (this.e45 * other.e12) + (this.e12 * other.e45) + (this.e4125 * other.scalar), -(this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12) - (this.e12 * other.e35)) + (float4(this.scalar) * other.group3) + float4((this.group4.yzx * other.group2.zxy), this.e3215 * other.scalar) - (other.group2.yzxx * float4(this.group4.zxy, this.e23)) - float4((this.group3.yzx * other.group0.zxy), this.e31 * other.e25);
        float wedge_g10 = (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * right_anti_dual_g2[3]) + (right_anti_dual_g3[0] * wedge_g9[0]) + (right_anti_dual_g3[1] * wedge_g9[1]) + (right_anti_dual_g3[2] * wedge_g9[2]) + (right_anti_dual_g3[3] * wedge_g9[3]) + (right_anti_dual_g0[3] * this.scalar * other.scalar) - (wedge_g4[0] * right_anti_dual_g2[0]) - (wedge_g4[1] * right_anti_dual_g2[1]) - (wedge_g4[2] * right_anti_dual_g2[2]) - (wedge_g5[0] * right_anti_dual_g1[0]) - (wedge_g5[1] * right_anti_dual_g1[1]) - (wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (right_anti_dual_g1[3] * wedge_g3[3]), wedge_g0_y * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g8[1] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * wedge_g1[0]) + (right_anti_dual_g1[0] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[0]), (wedge_g8[2] * right_anti_dual_g0[0]) + (right_anti_dual_g0[3] * wedge_g1[1]) + (right_anti_dual_g1[1] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[1]), (wedge_g8[0] * right_anti_dual_g0[1]) + (right_anti_dual_g0[3] * wedge_g1[2]) + (right_anti_dual_g1[2] * wedge_g6[3]) + (right_anti_dual_g1[3] * wedge_g6[2]), -(wedge_g7[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g6[0]) - (right_anti_dual_g0[1] * wedge_g6[1]) - (right_anti_dual_g0[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + float4((wedge_g7.zxy * right_anti_dual_g2.yzx), right_anti_dual_g0[3] * wedge_g1[3]) - float4((wedge_g7.yzx * right_anti_dual_g2.zxy), wedge_g7[0] * right_anti_dual_g1[0]) - float4((wedge_g8.zxy * right_anti_dual_g0.yzx), wedge_g7[1] * right_anti_dual_g1[1]), 
            /* e5 */
            (wedge_g0_y * right_anti_dual_g2[3]) + (right_anti_dual_g0[3] * this.e5 * other.scalar) - (wedge_g8[0] * right_anti_dual_g1[0]) - (wedge_g8[1] * right_anti_dual_g1[1]) - (wedge_g8[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g6[0]) - (right_anti_dual_g2[1] * wedge_g6[1]) - (right_anti_dual_g2[2] * wedge_g6[2]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g1[0] * wedge_g9[3]) + (right_anti_dual_g2[1] * wedge_g9[2]), (right_anti_dual_g1[1] * wedge_g9[3]) + (right_anti_dual_g2[2] * wedge_g9[0]), (right_anti_dual_g1[2] * wedge_g9[3]) + (right_anti_dual_g2[0] * wedge_g9[1]), -(right_anti_dual_g1[1] * wedge_g9[1]) - (right_anti_dual_g1[2] * wedge_g9[2])) + (wedge_g3 * float4(right_anti_dual_g0[3])) - (wedge_g9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[3])) + (float3(wedge_g10) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g9.yzx) - (right_anti_dual_g0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(right_anti_dual_g0[3])) + (float3(wedge_g10) * right_anti_dual_g2.xyz) + (float3(wedge_g9[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[3])), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_y) * right_anti_dual_g0.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(right_anti_dual_g0[3])) + (float3(wedge_g0_y) * right_anti_dual_g2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(right_anti_dual_g0[3]), 
            /* e1234 */
            wedge_g10 * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Plane> operator /(Plane lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g0.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
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
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * float4(other.scalar * this.e4235, other.scalar * this.e4315, other.scalar * this.e4125, other.scalar * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(AntiFlector other) {
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
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g0[0] * wedge_g0[3]) + (right_anti_dual_g1[1] * wedge_g0[2]), (right_anti_dual_g0[1] * wedge_g0[3]) + (right_anti_dual_g1[2] * wedge_g0[0]), (right_anti_dual_g0[2] * wedge_g0[3]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215);
        float4 wedge_g9 = float4(other.scalar) * this.group0;
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g9[1] * other.e431) + float4(float3(0.0), wedge_g9[2] * other.e412) - float4((other.group7.xyz * wedge_g9.www), 0.0) - float4((wedge_g9.zxy * other.group6.yzx), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            (float4(((float3(wedge_g9[3]) * right_anti_dual_g6.xyz) + (right_anti_dual_g8.yzx * wedge_g9.zxy)), right_anti_dual_g6[2] * wedge_g9[2]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4(float3(0.0), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, 0.0)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            right_anti_dual_g10 * wedge_g0_y
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere anti_reject_orthogonally_from(RoundPoint other) {
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
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * float4(this.e4235 * other.scalar, this.e4315 * other.scalar, this.e4125 * other.scalar, this.e3215 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, (right_anti_dual_g3[1] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])), 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * wedge_g0.zxy), 0.0) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float3 wedge_g2_xyz = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        float4 wedge_g3 = float4(other.scalar) * float4(this.group0.xyz, this.e5);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((wedge_g2_xyz * float3(right_anti_dual_g2[3])), (right_anti_dual_g2[3] * other.scalar * this.e4) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g2_xyz[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[3] * wedge_g3[0]), (right_anti_dual_g0[0] * wedge_g2_xyz[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[3] * wedge_g3[1]), (right_anti_dual_g0[1] * wedge_g2_xyz[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[3] * wedge_g3[2]), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (right_anti_dual_g2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - (right_anti_dual_g2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = (other.group3.zxyw * this.group0.yzxw) - float4((other.group3.yzx * this.group0.zxy), other.e4 * this.e5);
        float4 wedge_g2 = (float4(other.e1 * this.e5, other.e2 * this.e5, other.e3 * this.e5, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e423));
        float4 wedge_g3 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * wedge_g2[3]) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) - float4((right_anti_dual_g3.xyz * wedge_g3.www), wedge_g0[0] * right_anti_dual_g3[0]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g3.zxyw * wedge_g1.yzxw) + (wedge_g2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g3[0] * wedge_g2[0]) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[1] * wedge_g2[1]) - (float4(wedge_g3[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[2] * wedge_g3[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_orthogonally_from(AntiDualNum other) {
        float wedge_g0_w = other.e3215 * this.e4;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), wedge_g0_w * other.scalar), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (wedge_g0_w * other.e3215) + (other.scalar * other.scalar * this.e5)), 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4) * other.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            (right_anti_dual_g0[3] * other.e235 * this.e1) + (right_anti_dual_g0[3] * other.e315 * this.e2) + (right_anti_dual_g0[3] * other.e125 * this.e3) + (right_anti_dual_g0[3] * other.e321 * this.e5) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz);
        float wedge_g2_w = other.e321 * this.e4;
        float3 wedge_g3_xyz = float3(this.e4) * other.group0.xyz;
        float wedge_g3_w = -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2_w) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3_xyz.zxy * right_anti_dual_g1.yzx) - (wedge_g3_xyz.yzx * right_anti_dual_g1.zxy)), wedge_g2_w * right_anti_dual_g1[3]), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) + float4((wedge_g3_xyz * right_anti_dual_g1.www), wedge_g2_w * right_anti_dual_g0[3]) - (right_anti_dual_g1.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1[3]) * float4(wedge_g0, other.e5 * this.e4)) + (right_anti_dual_g1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0])) + (float4(wedge_g2_xyz, wedge_g2_w).wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (wedge_g3_w * right_anti_dual_g0[3]) - (wedge_g3_xyz[1] * right_anti_dual_g0[1]) - (wedge_g3_xyz[2] * right_anti_dual_g0[2])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g1.yzx), wedge_g3_xyz[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3);
        float3 wedge_g2 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g1_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (wedge_g1_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (wedge_g1_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) - float4((right_anti_dual_g1.zxy * wedge_g0.yzx), right_anti_dual_g0[0] * wedge_g0[0]), 
            /* e5 */
            -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1_xyz[0]) - (right_anti_dual_g1[1] * wedge_g1_xyz[1]) - (right_anti_dual_g1[2] * wedge_g1_xyz[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.e4) * float4(other.group0.xyz, other.e3215);
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.scalar * this.e5);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1_xyz * float3(right_anti_dual_g0[3])) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), wedge_g1_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[3] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (right_anti_dual_g0[3] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (right_anti_dual_g0[3] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (right_anti_dual_g0 * float4(float3(wedge_g1_w), wedge_g3[3])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * right_anti_dual_g0[3]) + (wedge_g1_xyz[1] * right_anti_dual_g0[2]), (wedge_g0[1] * right_anti_dual_g0[3]) + (wedge_g1_xyz[2] * right_anti_dual_g0[0]), (wedge_g0[2] * right_anti_dual_g0[3]) + (wedge_g1_xyz[0] * right_anti_dual_g0[1]), -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e5 */
            (wedge_g2[0] * right_anti_dual_g0[0]) + (wedge_g2[1] * right_anti_dual_g0[1]) + (wedge_g2[2] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group1.zxy, other.e315)) - float4((float3(this.e5) * other.group0.xyz), other.e235 * this.e1);
        float wedge_g1 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235));
        float wedge_g1 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g1) * right_anti_dual_g2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float3 wedge_g2 = (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1]), 
            /* e5 */
            -(right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2]) - (wedge_g2[0] * right_anti_dual_g1[0]) - (wedge_g2[1] * right_anti_dual_g1[1]) - (wedge_g2[2] * right_anti_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = (float4(other.e25 * this.e3, other.e35 * this.e1, other.e15 * this.e2, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (wedge_g2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (wedge_g2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public DualNum anti_reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * other.e5 * this.e4 * -1.0, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_orthogonally_from(FlatPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(right_anti_dual_g0[3])), -(wedge_g0[0] * right_anti_dual_g0[0]) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = (float4(other.e45 * this.e1, other.e45 * this.e2, other.e45 * this.e3, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.wwwx * float4(other.group0.xyz, other.e4235));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.zxy * wedge_g0.yzx), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g1 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1.yzx), right_anti_dual_g1[3] * other.e5 * this.e4) - (wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float wedge_g2 = other.scalar * this.e5;
        float4 wedge_g3 = (float4(other.e5) * this.group0) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g5 = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group0.yzx) - (other.group3.yzx * this.group0.zxy);
        float4 wedge_g9 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group0.yzxy * float4(other.group6.zxy, other.e315)) - float4((float3(this.e5) * other.group7.xyz), other.e235 * this.e1);
        float wedge_g10 = (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = float4((other.group0.zxy * this.group0.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group0.zxy, this.e5));
        float3 wedge_g2 = (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * right_anti_dual_g0[3]) + (right_anti_dual_g0[2] * wedge_g1[1]), (wedge_g0[1] * right_anti_dual_g0[3]) + (right_anti_dual_g0[0] * wedge_g1[2]), (wedge_g0[2] * right_anti_dual_g0[3]) + (right_anti_dual_g0[1] * wedge_g1[0]), -(right_anti_dual_g1 * wedge_g1[3]) - (wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxy * float4(wedge_g1.zxy, wedge_g0[1])) - float4((wedge_g2 * float3(right_anti_dual_g1)), wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e5 */
            (wedge_g2[0] * right_anti_dual_g0[0]) + (wedge_g2[1] * right_anti_dual_g0[1]) + (wedge_g2[2] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e5 */
            this.e5 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(other.group3.zxy, other.e5)) - float4((this.group0.zxy * other.group3.yzx), this.e5 * other.e4);
        float4 wedge_g2 = (float4(this.e5 * other.e1, this.e5 * other.e2, this.e5 * other.e3, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group2.www, other.e423));
        float4 wedge_g3 = float4((this.e3 * other.e425) + (this.e4 * other.e235), (this.e1 * other.e435) + (this.e4 * other.e315), (this.e2 * other.e415) + (this.e4 * other.e125), -(this.e3 * other.e125) - (this.e5 * other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(this.e5) * other.group0.xyz), this.e2 * other.e315);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) + float4((wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g0, wedge_g1[3])) + (right_anti_dual_g3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[2] * wedge_g2[2]) - (float4(wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (float4(this.e2 * other.e43, this.e3 * other.e41, this.e1 * other.e42, (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235)) + (this.group0.wwwy * float4(other.group1.xyz, other.e4315));
        float4 wedge_g1 = float4((this.e4 * other.e15) + (this.e5 * other.e41), (this.e4 * other.e25) + (this.e5 * other.e42), (this.e4 * other.e35) + (this.e5 * other.e43), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (right_anti_dual_g0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * right_anti_dual_g0.www), right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Scalar> operator /(Scalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((float3(right_anti_dual_g2[3]) * wedge_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2[3]) * wedge_g1.xyz) + (wedge_g0.zxy * right_anti_dual_g3.yzx) - (wedge_g0.yzx * right_anti_dual_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * right_anti_dual_g3[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (wedge_g0[1] * right_anti_dual_g3[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (wedge_g0[2] * right_anti_dual_g3[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g1.xyz, wedge_g2[3])) + (right_anti_dual_g3.zxyx * float4(wedge_g2.yzx, wedge_g3[0])) + float4(float3(0.0), (right_anti_dual_g2[3] * wedge_g3[3]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((right_anti_dual_g3.yzx * wedge_g2.zxy), wedge_g0[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiDualNum anti_reject_orthogonally_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * float2(other.e3215 * this.scalar, other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g1.xyz * wedge_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) + (wedge_g0[3] * other.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1.zxy * wedge_g0.yzx) - (right_anti_dual_g1.yzx * wedge_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), (right_anti_dual_g0[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g1[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[0] * wedge_g3[3]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[1] * wedge_g3[3]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[2] * wedge_g3[3]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g2[3]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiDualNum anti_reject_orthogonally_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * other.e45 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g10 * other.e3215) + (right_anti_dual_g0[0] * wedge_g0[1]) + (right_anti_dual_g0[1] * wedge_g0[0]) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), right_anti_dual_g0[1] * wedge_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0[1])) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g2 * right_anti_dual_g0[1]) + (wedge_g0[1] * other.e3215) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0[1])) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (float3(wedge_g0[1]) * other.group7.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0[1]) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0[1])) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0[1])) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0[1])) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0[1])) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0[1]) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) - (other.e4 * other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (wedge_g0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1[0] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[0]), (right_anti_dual_g1[1] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[1]), (right_anti_dual_g1[2] * wedge_g0[3]) + (right_anti_dual_g2[3] * wedge_g2[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * wedge_g2.yzx) - (right_anti_dual_g3.yzx * wedge_g2.zxy)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 wedge_g2 = float4(this.scalar) * other.group2;
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g1.xyz * wedge_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) + float4((wedge_g0.xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[3] * wedge_g1[0]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[1]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[2]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (right_anti_dual_g0 * float4(wedge_g3.www, wedge_g1[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Sphere> operator /(Sphere lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = other.scalar * this.e1234;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(wedge_g1) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g0.yzx) - (right_anti_dual_g0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_orthogonally_from(AntiDualNum other) {
        float wedge_g1 = other.scalar * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), wedge_g1 * other.e3215), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), wedge_g1 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Flector anti_reject_orthogonally_from(AntiFlector other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = other.scalar * this.e1234;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g1) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g1) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * wedge_g0.zxy) - (right_anti_dual_g1.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Plane anti_reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public AntiDualNum anti_reject_orthogonally_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e5 * this.e1234) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        float4 wedge_g9 = float4(other.scalar) * this.group0;
        float wedge_g10 = other.scalar * this.e1234;
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
            float2((wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4(float3(0.0), wedge_g10 * right_anti_dual_g3[3]) + float4(float3(0.0), wedge_g9[1] * other.e431) + float4(float3(0.0), wedge_g9[2] * other.e412) + float4(float3((float2(wedge_g10) * right_anti_dual_g3.xy), wedge_g10 * right_anti_dual_g3[2]), 0.0) - float4((other.group7.xyz * wedge_g9.www), 0.0) - float4((wedge_g9.zxy * other.group6.yzx), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e3215) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            (float4(((float3(wedge_g9[3]) * right_anti_dual_g6.xyz) + (right_anti_dual_g8.yzx * wedge_g9.zxy)), right_anti_dual_g6[2] * wedge_g9[2]) * float4(1.0, 1.0, 1.0, -1.0)) + (right_anti_dual_g3 * float4(wedge_g0_y)) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4(float3(0.0), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere anti_reject_orthogonally_from(RoundPoint other) {
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
public Sphere operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere anti_reject_orthogonally_from(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
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
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float wedge_g1 = this.e1234 * other.scalar;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, (wedge_g1 * right_anti_dual_g2[3]) + (right_anti_dual_g3[2] * wedge_g0[2]) + (right_anti_dual_g3[3] * wedge_g0[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4((float3(wedge_g1) * right_anti_dual_g1.xyz), right_anti_dual_g3[1] * wedge_g0[1]), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g0[3]), (wedge_g1 * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g0[3]), -(right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) - (right_anti_dual_g1.wwwx * wedge_g0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g0.zxy) - (right_anti_dual_g2.zxy * wedge_g0.yzx)), wedge_g1 * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<VersorEven> operator /(VersorEven lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_orthogonally_from(AntiCircleRotor other) {
        float4 wedge_g0 = float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.yzx * this.group3.zxy), other.scalar * this.e12345) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235);
        float4 wedge_g1 = float4((other.e15 * this.e4) + (other.scalar * this.e415), (other.e25 * this.e4) + (other.scalar * this.e425), (other.e35 * this.e4) + (other.scalar * this.e435), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.xyz * this.group2.www), other.scalar * this.e321) - (other.group1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(wedge_g0[3])) + (float3(right_anti_dual_g2[3]) * wedge_g0.xyz)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g2[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0.www, other.scalar * this.e5)) + float4((wedge_g2_xyz * right_anti_dual_g2.www), -(wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g2[3] * wedge_g3[0]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g2[3] * wedge_g3[1]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g2[3] * wedge_g3[2]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((right_anti_dual_g0.zxy * wedge_g2_xyz.yzx), right_anti_dual_g2[3] * wedge_g3[3]) - float4((right_anti_dual_g0.yzx * wedge_g2_xyz.zxy), right_anti_dual_g0[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (other.group3.zxyw * this.group3.yzxw) - float4((other.group3.yzx * this.group3.zxy), other.e4 * this.e5);
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - float4((other.group3.xyz * this.group2.www), other.e4 * this.e321);
        float4 wedge_g3 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e1 * this.e235) + float4((other.group2.xyz * this.group3.www), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e125 * this.e3);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (wedge_g3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * wedge_g2[3]) + (wedge_g3[1] * other.e431) + (wedge_g3[2] * other.e412) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) - float4((right_anti_dual_g3.xyz * wedge_g3.www), wedge_g0[0] * right_anti_dual_g3[0]), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g3.zxyw * wedge_g1.yzxw) + (wedge_g2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((wedge_g0 * right_anti_dual_g3.www), right_anti_dual_g3[0] * wedge_g2[0]) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[1] * wedge_g2[1]) - (float4(wedge_g3[3]) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[2] * wedge_g3[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.yy, other.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0);
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.scalar), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.scalar, (wedge_g0[3] * other.e3215) + (wedge_g2[3] * other.scalar)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_orthogonally_from(AntiFlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4) * other.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(other.e321 * this.e4), 
            /* e5 */
            (right_anti_dual_g0[3] * other.e235 * this.e1) + (right_anti_dual_g0[3] * other.e315 * this.e2) + (right_anti_dual_g0[3] * other.e125 * this.e3) + (right_anti_dual_g0[3] * other.e321 * this.e5) - (wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (wedge_g0_xyz[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiFlector other) {
        float3 wedge_g0 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy);
        float4 wedge_g2 = (this.group3 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group1.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + (other.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (other.e5 * this.e321) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((other.group0.xyz * this.group3.www), other.e1 * this.e235) - float4((other.group1.zxy * this.group1.yzx), other.e235 * this.e1);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * wedge_g3.zxy) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), right_anti_dual_g1[3] * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])) + float4((wedge_g3.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * wedge_g2[3]) - (right_anti_dual_g1.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1[3]) * float4(wedge_g0, other.e5 * this.e4)) + (right_anti_dual_g1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]) - (right_anti_dual_g0[3] * wedge_g3[3])) - float4((wedge_g1_xyz.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3);
        float4 wedge_g2 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group1.yzx * this.group3.zxy), other.e23 * this.e415);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * wedge_g2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g1 * float3(wedge_g2[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_w * right_anti_dual_g0[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (wedge_g1_w * right_anti_dual_g0[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (wedge_g1_w * right_anti_dual_g0[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1_xyz[0]) - (right_anti_dual_g1[1] * wedge_g1_xyz[1]) - (right_anti_dual_g1[2] * wedge_g1_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g0.yzx), right_anti_dual_g0[0] * wedge_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = (other.group0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((this.group0.xyz * other.group0.www), other.e3215 * this.e4);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3));
        float4 wedge_g2 = float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.scalar * this.e5);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0[3] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g0[2]), (right_anti_dual_g0[3] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g0[0]), (right_anti_dual_g0[3] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g0[1]), -(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])) + (right_anti_dual_g0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(AntiPlane other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 wedge_g2 = float4((float3(other.e5) * this.group3.xyz), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (float4(other.e3 * this.e425, other.e1 * this.e435, other.e2 * this.e415, (other.e3 * this.e125) + (other.e5 * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * right_anti_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * wedge_g3.zxy) - (right_anti_dual_g0.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            (float4(wedge_g3[0], wedge_g3[1], wedge_g3[2], 1.0) * float4(right_anti_dual_g0.www, -(wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))) - (right_anti_dual_g0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0[3]) * float4(wedge_g0, other.e5 * this.e4)) + (right_anti_dual_g0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) + float4((wedge_g1_xyz.zxy * right_anti_dual_g0.yzx * float3(-1.0)), (right_anti_dual_g0[1] * wedge_g2[1]) + (right_anti_dual_g0[2] * wedge_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_orthogonally_from(Circle other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e321 * this.e5) - (other.e125 * this.e3)) - (this.group3.yzxy * float4(other.group1.zxy, other.e315)) - float4((other.group0.xyz * this.group2.www), other.e235 * this.e1);
        float wedge_g1 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e423), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_orthogonally_from(CircleRotor other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235));
        float wedge_g1 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_anti_dual_g1[1] * wedge_g0[2]) - (wedge_g0[3] * other.e423), -(right_anti_dual_g1[2] * wedge_g0[0]) - (wedge_g0[3] * other.e431), -(right_anti_dual_g1[0] * wedge_g0[1]) - (wedge_g0[3] * other.e412), (wedge_g1 * right_anti_dual_g1[3]) + (wedge_g0[2] * other.e412)) + (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(wedge_g1) * right_anti_dual_g2.xyz), wedge_g0[1] * other.e431), 
            /* e5 */
            -(right_anti_dual_g1[3] * wedge_g0[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4((other.e23 * this.e5) + (other.e35 * this.e2), (other.e31 * this.e5) + (other.e15 * this.e3), (other.e12 * this.e5) + (other.e25 * this.e1), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(wedge_g2[3])), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[2] * wedge_g2[1]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]), (right_anti_dual_g0[0] * wedge_g2[2]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[0]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]), -(right_anti_dual_g2[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2])) - float4((right_anti_dual_g0.yzx * wedge_g2.zxy), right_anti_dual_g2[0] * wedge_g1[0]) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g2[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((other.e41 * this.e5) + (other.e15 * this.e4), (other.e42 * this.e5) + (other.e25 * this.e4), (other.e43 * this.e5) + (other.e35 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (other.group1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(wedge_g2[3]), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * right_anti_dual_g2.xyz), (right_anti_dual_g2[3] * wedge_g2[3]) - (right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (wedge_g0[0] * right_anti_dual_g1[0]) - (wedge_g0[1] * right_anti_dual_g1[1]) - (wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * right_anti_dual_g2[1]) + (right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g3[0] * wedge_g2[3]), (wedge_g0[0] * right_anti_dual_g2[2]) + (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g3[1] * wedge_g2[3]), (wedge_g0[1] * right_anti_dual_g2[0]) + (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g3[2] * wedge_g2[3]), -(right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])) + (wedge_g2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (wedge_g2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((wedge_g0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * wedge_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float4(other.e5) * float4(this.group0.xyz, this.e321)).xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0])), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual_g0[0] * other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g0 = float4((float3(this.e4) * other.group0.xyz), -(other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = (this.group3.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (other.group0.wwwx * float4(this.group3.xyz, this.e423));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + float4((float3(wedge_g0[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiPlane anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4((other.e425 * this.e3) + (other.e235 * this.e4), (other.e435 * this.e1) + (other.e315 * this.e4), (other.e415 * this.e2) + (other.e125 * this.e4), -(other.e315 * this.e2) - (other.e125 * this.e3)) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.zxy * wedge_g0.yzx), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) - (wedge_g0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiFlector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g1 = float4((other.e235 * this.e4) + (other.e5 * this.e423), (other.e315 * this.e4) + (other.e5 * this.e431), (other.e125 * this.e4) + (other.e5 * this.e412), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group0.yzx * this.group3.zxy), other.e5 * this.e321) - (this.group3.yzxx * float4(other.group0.zxy, other.e235));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) + float4((right_anti_dual_g0.zxy * wedge_g1.yzx), right_anti_dual_g1[3] * other.e5 * this.e4) - (wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 wedge_g1 = float4(other.scalar) * this.group3;
        float wedge_g2 = other.scalar * this.e5;
        float4 wedge_g3 = (float4(other.e5) * this.group3) - (float4(this.e5) * other.group1);
        float3 wedge_g4 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz);
        float3 wedge_g5 = (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy);
        float4 wedge_g6 = float4((other.e15 * this.e4) + (other.e41 * this.e5), (other.e25 * this.e4) + (other.e42 * this.e5), (other.e35 * this.e4) + (other.e43 * this.e5), -(other.e31 * this.e2) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * float4(other.group3.www, other.e23));
        float3 wedge_g7 = (float3(other.scalar) * this.group0.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group3.yzx) - (other.group3.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + float4((other.group8.xyz * this.group3.www), other.e1 * this.e235) + float4((other.group6.yzx * this.group3.zxy), other.e3 * this.e125) - (other.group6.zxyw * float4(this.group3.yzx, this.e5)) - float4((other.group7.xyz * this.group2.www), other.e235 * this.e1) - float4((other.group1.zxy * this.group1.yzx), other.e315 * this.e2) - float4((this.group2.xyz * other.group1.www), other.e125 * this.e3);
        float wedge_g10 = (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiPlane anti_reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(RoundPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = float4((other.group0.zxy * this.group3.yzx), other.e5 * this.e4) - (other.group0.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = float4((float3(other.e5) * this.group3.xyz), -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = float4(-(other.e3 * this.e425) - (other.e4 * this.e235), -(other.e1 * this.e435) - (other.e4 * this.e315), -(other.e2 * this.e415) - (other.e4 * this.e125), (other.e3 * this.e125) + (other.e5 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(other.e5) * this.group0.xyz), other.e2 * this.e315);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g1)), 
            /* e235, e315, e125, e4 */
            (float4(wedge_g3[0], wedge_g3[1], wedge_g3[2], 1.0) * float4(right_anti_dual_g0.www, -(right_anti_dual_g1 * wedge_g1[3]) - (wedge_g0[1] * right_anti_dual_g0[1]) - (wedge_g0[2] * right_anti_dual_g0[2]))) - (right_anti_dual_g0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(-(right_anti_dual_g1 * wedge_g2[0]) - (right_anti_dual_g0[1] * wedge_g1[2]), -(right_anti_dual_g1 * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g1[0]), -(right_anti_dual_g1 * wedge_g2[2]) - (right_anti_dual_g0[0] * wedge_g1[1]), (right_anti_dual_g0[1] * wedge_g2[1]) + (right_anti_dual_g0[2] * wedge_g2[2])) + (float4(right_anti_dual_g0[3]) * float4(wedge_g0, wedge_g1[3])) + (right_anti_dual_g0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_orthogonally_from(Scalar other) {
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
public RoundPoint operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public RoundPoint anti_reject_orthogonally_from(Sphere other) {
        float wedge_g0 = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_reject_orthogonally_from(VersorEven other) {
        float3 wedge_g0 = (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz);
        float4 wedge_g1 = (this.group3.yzxw * float4(other.group3.zxy, other.e5)) - (other.group3.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = (this.group3.xyzx * float4(other.group2.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group3.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e5 * this.e321) + float4((other.group2.xyz * this.group3.www), other.e1 * this.e235) + float4((this.group0.xyz * other.group2.www), other.e2 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), other.e315 * this.e2) - float4((this.group2.xyz * other.group3.www), other.e125 * this.e3);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * wedge_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])) - (wedge_g3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_anti_dual_g0[0] * wedge_g3[0]) + (right_anti_dual_g0[1] * wedge_g3[1]) + (right_anti_dual_g0[2] * wedge_g3[2]) - (wedge_g0[1] * right_anti_dual_g3[1]) - (wedge_g0[2] * right_anti_dual_g3[2]) - (right_anti_dual_g2[3] * wedge_g1[3])) + float4((wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * wedge_g2[3]) - (right_anti_dual_g3.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3[3]) * float4(wedge_g0, wedge_g1[3])) + (right_anti_dual_g3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) + (wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * wedge_g3.yzx), right_anti_dual_g3[2] * wedge_g2[2]) - (float4(wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) - (wedge_g3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((right_anti_dual_g3.yzx * wedge_g1.zxy), right_anti_dual_g2[0] * wedge_g3[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiRejectOrthogonallyFromInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven anti_reject_orthogonally_from(VersorOdd other) {
        float4 wedge_g0 = (float4(other.scalar) * this.group0) + (this.group3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group3.zxy * other.group0.yzx), this.e5 * other.e1234) - float4((this.group3.yzx * other.group0.zxy), this.e423 * other.e15);
        float4 wedge_g1 = float4((this.e5 * other.e41) + (this.e4 * other.e15), (this.e5 * other.e42) + (this.e4 * other.e25), (this.e5 * other.e43) + (this.e4 * other.e35), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group3.xyzx * other.group1.wwwx);
        float4 wedge_g2 = float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), this.e5 * other.scalar);
        float4 wedge_g3 = float4(other.scalar) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(wedge_g0[3])) + (wedge_g1 * float4(right_anti_dual_g0[3])), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(wedge_g0[3])) + (wedge_g2 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[0] * wedge_g2[0]) - (right_anti_dual_g1[1] * wedge_g2[1]) - (right_anti_dual_g1[2] * wedge_g2[2]) - (right_anti_dual_g2[0] * wedge_g1[0]) - (right_anti_dual_g2[1] * wedge_g1[1]) - (right_anti_dual_g2[2] * wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1[0] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[0]) + (right_anti_dual_g2[1] * wedge_g0[2]) + (right_anti_dual_g3[0] * wedge_g0[3]), (right_anti_dual_g1[1] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[1]) + (right_anti_dual_g2[2] * wedge_g0[0]) + (right_anti_dual_g3[1] * wedge_g0[3]), (right_anti_dual_g1[2] * wedge_g1[3]) + (right_anti_dual_g1[3] * wedge_g1[2]) + (right_anti_dual_g2[0] * wedge_g0[1]) + (right_anti_dual_g3[2] * wedge_g0[3]), -(right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + (right_anti_dual_g0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * right_anti_dual_g0.www), right_anti_dual_g3[3] * wedge_g0[3]) - (right_anti_dual_g0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((right_anti_dual_g2.zxy * wedge_g0.yzx), right_anti_dual_g0[1] * wedge_g1[1])
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiCircleRotor other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float4 wedge_g2 = (other.group2 * float4(this.group0.www, this.e1234)) + float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.scalar * this.e4235), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.scalar * this.e4315), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.scalar * this.e4125), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.scalar * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g1[0] * wedge_g2[3]), (wedge_g0_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g1[1] * wedge_g2[3]), (wedge_g0_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g1[2] * wedge_g2[3]), -(right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((right_anti_dual_g0.zxy * wedge_g3.yzx), right_anti_dual_g2[3] * other.scalar * this.scalar) - float4((right_anti_dual_g0.yzx * wedge_g3.zxy), right_anti_dual_g0[0] * wedge_g2[0]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g2[0] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[0]), (right_anti_dual_g2[1] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[1]), (right_anti_dual_g2[2] * wedge_g2[3]) + (right_anti_dual_g2[3] * wedge_g1[2]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + float4((right_anti_dual_g0 * wedge_g3.www), right_anti_dual_g2[3] * wedge_g1[3]) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g2[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiDipoleInversion other) {
        float4 wedge_g0 = (other.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.xyz * this.group0.www), other.e4 * this.e3215) + float4((this.group1.xyz * other.group2.www), other.e1 * this.e4235) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15);
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e4);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((other.group0.xyz * wedge_g0.www), right_anti_dual_g2[3] * other.e5 * this.scalar) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - float4((right_anti_dual_g3.zxy * wedge_g0.yzx), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[0]), (wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[1]), (wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * wedge_g0[2]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (right_anti_dual_g1 * float4(wedge_g0[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiDualNum other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.scalar, (wedge_g0[3] * other.scalar) + (wedge_g2[3] * other.e3215)) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * float4(other.group0.yy, other.scalar, (other.e3215 * this.scalar) + (other.scalar * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiFlatPoint other) {
        float wedge_g2_w = -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * right_anti_dual_g0.xyz), right_anti_dual_g0[3] * other.e321 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiFlector other) {
        float4 wedge_g0 = (other.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e235));
        float4 wedge_g1 = (this.group0 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx);
        float3 wedge_g3_xyz = other.group1.xyz * this.group0.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * right_anti_dual_g1[1]) + (wedge_g3_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g1[3])) - (wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2])) - (right_anti_dual_g1.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g1[3]) * wedge_g1.xyz), 0.0) + float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), 0.0) + float4((wedge_g2_xyz.yzx * right_anti_dual_g1.zxy), 0.0) - float4((wedge_g2_xyz.zxy * right_anti_dual_g1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(AntiLine other) {
        float3 wedge_g1_xyz = other.group0.xyz * this.group0.www;
        float wedge_g2_w = -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e35 * this.e42), (other.e31 * this.e45) + (other.e15 * this.e43), (other.e12 * this.e45) + (other.e25 * this.e41), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(wedge_g2_w)), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g0[0] * wedge_g3[3]) + (right_anti_dual_g1[1] * wedge_g3[2]), (right_anti_dual_g0[1] * wedge_g3[3]) + (right_anti_dual_g1[2] * wedge_g3[0]), (right_anti_dual_g0[2] * wedge_g3[3]) + (right_anti_dual_g1[0] * wedge_g3[1]), -(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2])) - float4((right_anti_dual_g1.zxy * wedge_g3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiMotor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.e45);
        float4 wedge_g2 = (float4(other.scalar) * this.group2) + float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 wedge_g3 = float4(other.e35 * this.e42, other.e15 * this.e43, other.e25 * this.e41, -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0 * float4(this.group1.www, this.e3215)) + float4((this.group3.xyz * other.group0.www), other.e3215 * this.scalar) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0 * float4(wedge_g2.www, wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]) - (right_anti_dual_g1[0] * wedge_g0[0]) - (right_anti_dual_g1[1] * wedge_g0[1]) - (right_anti_dual_g1[2] * wedge_g0[2])) + float4((wedge_g0.xyz * right_anti_dual_g0.www), right_anti_dual_g1[3] * wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(right_anti_dual_g0[3])) + float4((float3(wedge_g2[3]) * right_anti_dual_g1.xyz), -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2])), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * wedge_g3.zxy) - (right_anti_dual_g1.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(AntiPlane other) {
        float4 wedge_g0 = (float4(other.e2 * this.e43, other.e3 * this.e41, other.e1 * this.e42, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235));
        float4 wedge_g1 = float4((float3(other.e5) * this.group0.xyz), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx);
        float3 wedge_g3_xyz = other.group0.xyz * this.group0.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g0.zxy, (wedge_g3_xyz[1] * right_anti_dual_g0[1]) + (wedge_g3_xyz[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], 1.0) * float4(right_anti_dual_g0.www, -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * right_anti_dual_g0.zxy), 0.0) - float4((wedge_g2_xyz.zxy * right_anti_dual_g0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(wedge_g2_w), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * right_anti_dual_g2.xyz), (wedge_g2_w * right_anti_dual_g2[3]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e42 * this.e35) + (other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42), (other.e43 * this.e15) + (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43), (other.e41 * this.e25) + (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e35 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e15 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e25 * this.e31);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g2[1] * wedge_g3[2]) + (right_anti_dual_g1[0] * wedge_g3[3]), (right_anti_dual_g2[2] * wedge_g3[0]) + (right_anti_dual_g1[1] * wedge_g3[3]), (right_anti_dual_g2[0] * wedge_g3[1]) + (right_anti_dual_g1[2] * wedge_g3[3]), -(right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (right_anti_dual_g2[0] * wedge_g0[0]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_reject_orthogonally_from(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((other.group0.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * wedge_g3.yzx) - (right_anti_dual_g0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * right_anti_dual_g2[0]) + (right_anti_dual_g0[0] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[1]) + (right_anti_dual_g0[1] * wedge_g3[3]), (wedge_g2_w * right_anti_dual_g2[2]) + (right_anti_dual_g0[2] * wedge_g3[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (wedge_g3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (wedge_g2_w * right_anti_dual_g3[3]) + (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) - (right_anti_dual_g0[1] * wedge_g2_xyz[1]) - (right_anti_dual_g0[2] * wedge_g2_xyz[2]) - (wedge_g0[0] * right_anti_dual_g2[0]) - (wedge_g0[1] * right_anti_dual_g2[1]) - (wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) - float4((right_anti_dual_g2.zxy * wedge_g3.yzx), right_anti_dual_g0[0] * wedge_g2_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_orthogonally_from(DualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e5, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0);
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual_g0[1] * wedge_g0[3]), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_orthogonally_from(FlatPoint other) {
        float4 wedge_g1 = float4((other.e35 * this.e42) + (other.e45 * this.e23), (other.e15 * this.e43) + (other.e45 * this.e31), (other.e25 * this.e41) + (other.e45 * this.e12), -(other.e25 * this.e31) - (other.e35 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0[3]) * float4(wedge_g1.xyz, other.e45 * this.scalar) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g1 = float4((other.e45 * this.e23) + (other.e4235 * this.scalar), (other.e45 * this.e31) + (other.e4315 * this.scalar), (other.e45 * this.e12) + (other.e4125 * this.scalar), -(other.e25 * this.e31) - (other.e35 * this.e12)) + (this.group0.yzxw * float4(other.group0.zxy, other.e3215)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * wedge_g1[0]) + (right_anti_dual_g1[1] * wedge_g1[1]) + (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g0[3] * other.e45 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0.yzx * wedge_g1.zxy) - (right_anti_dual_g0.zxy * wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g0_w = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = (float4(other.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * wedge_g0[3]) - (right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2])), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float wedge_g2 = other.e5 * this.scalar;
        float4 wedge_g3 = (float4(other.scalar) * float4(this.group2.xyz, this.e45)) + (float4(this.scalar) * other.group3);
        float3 wedge_g4 = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group4.xyz);
        float3 wedge_g5 = (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group5.xyz);
        float4 wedge_g6 = float4((other.e5 * this.e41) + (other.e415 * this.scalar), (other.e5 * this.e42) + (other.e425 * this.scalar), (other.e5 * this.e43) + (other.e435 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((this.group2.xyz * other.group1.www), other.e321 * this.scalar) - (other.group1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group7.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float3 wedge_g8 = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx);
        float4 wedge_g9 = float4((other.e35 * this.e42) + (other.e45 * this.e23) + (other.e23 * this.e45) + (other.e4235 * this.scalar), (other.e15 * this.e43) + (other.e45 * this.e31) + (other.e31 * this.e45) + (other.e4315 * this.scalar), (other.e25 * this.e41) + (other.e45 * this.e12) + (other.e12 * this.e45) + (other.e4125 * this.scalar), -(other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12) - (other.e12 * this.e35)) + (float4(other.scalar) * this.group3) + float4((other.group4.yzx * this.group2.zxy), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group4.zxy, other.e23)) - float4((other.group3.yzx * this.group0.zxy), other.e31 * this.e25);
        float wedge_g10 = (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
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
            float2((right_anti_dual_g10 * wedge_g2) + (wedge_g0_y * right_anti_dual_g0[0]) + (wedge_g10 * other.e3215) + (right_anti_dual_g1[0] * wedge_g9[0]) + (right_anti_dual_g1[1] * wedge_g9[1]) + (right_anti_dual_g1[2] * wedge_g9[2]) + (right_anti_dual_g1[3] * wedge_g9[3]) + (right_anti_dual_g9[0] * wedge_g1[0]) + (right_anti_dual_g9[1] * wedge_g1[1]) + (right_anti_dual_g9[2] * wedge_g1[2]) + (right_anti_dual_g9[3] * wedge_g1[3]) + (right_anti_dual_g0[1] * other.scalar * this.scalar) - (right_anti_dual_g7[0] * wedge_g3[0]) - (right_anti_dual_g7[1] * wedge_g3[1]) - (right_anti_dual_g7[2] * wedge_g3[2]) - (right_anti_dual_g8[0] * wedge_g4[0]) - (right_anti_dual_g8[1] * wedge_g4[1]) - (right_anti_dual_g8[2] * wedge_g4[2]) - (wedge_g5[0] * right_anti_dual_g6[0]) - (wedge_g5[1] * right_anti_dual_g6[1]) - (wedge_g5[2] * right_anti_dual_g6[2]) - (wedge_g7[0] * right_anti_dual_g3[0]) - (wedge_g7[1] * right_anti_dual_g3[1]) - (wedge_g7[2] * right_anti_dual_g3[2]) - (wedge_g8[0] * other.e423) - (wedge_g8[1] * other.e431) - (wedge_g8[2] * other.e412) - (right_anti_dual_g3[3] * wedge_g6[3]) - (right_anti_dual_g6[3] * wedge_g3[3]) - (wedge_g6[0] * other.e415) - (wedge_g6[1] * other.e425) - (wedge_g6[2] * other.e435), wedge_g0_y * right_anti_dual_g0[1]), 
            /* e1, e2, e3, e4 */
            float4((wedge_g10 * right_anti_dual_g3[0]) + (wedge_g5[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[0]), (wedge_g10 * right_anti_dual_g3[1]) + (wedge_g5[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[1]), (wedge_g10 * right_anti_dual_g3[2]) + (wedge_g5[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g6[3]) + (right_anti_dual_g6[3] * wedge_g6[2]), -(right_anti_dual_g10 * wedge_g3[3]) - (wedge_g7[0] * right_anti_dual_g6[0]) - (wedge_g7[1] * right_anti_dual_g6[1]) - (wedge_g7[2] * right_anti_dual_g6[2])) + (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(right_anti_dual_g0[1])) + (wedge_g9.yzxx * float4(other.group6.zxy, other.e423)) + float4((wedge_g4 * right_anti_dual_g9.www), wedge_g10 * right_anti_dual_g3[3]) + float4((right_anti_dual_g7.zxy * wedge_g8.yzx), wedge_g9[1] * other.e431) + float4((right_anti_dual_g8.yzx * wedge_g7.zxy), wedge_g9[2] * other.e412) - (right_anti_dual_g9.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) - float4((float3(right_anti_dual_g10) * wedge_g3.xyz), wedge_g4[2] * right_anti_dual_g9[2]) - float4((other.group7.xyz * wedge_g9.www), right_anti_dual_g7[0] * wedge_g6[0]) - float4((right_anti_dual_g7.yzx * wedge_g8.zxy), right_anti_dual_g7[2] * wedge_g6[2]) - float4((right_anti_dual_g8.zxy * wedge_g7.yzx), wedge_g4[0] * right_anti_dual_g9[0]) - float4((wedge_g9.zxy * other.group6.yzx), right_anti_dual_g7[1] * wedge_g6[1]), 
            /* e5 */
            (wedge_g0_y * other.e3215) + (wedge_g2 * right_anti_dual_g0[1]) + (right_anti_dual_g9[0] * wedge_g3[0]) + (right_anti_dual_g9[1] * wedge_g3[1]) + (right_anti_dual_g9[2] * wedge_g3[2]) + (right_anti_dual_g9[3] * wedge_g3[3]) - (right_anti_dual_g8[0] * wedge_g6[0]) - (right_anti_dual_g8[1] * wedge_g6[1]) - (right_anti_dual_g8[2] * wedge_g6[2]) - (wedge_g8[0] * right_anti_dual_g6[0]) - (wedge_g8[1] * right_anti_dual_g6[1]) - (wedge_g8[2] * right_anti_dual_g6[2]) - (right_anti_dual_g3[0] * wedge_g9[0]) - (right_anti_dual_g3[1] * wedge_g9[1]) - (right_anti_dual_g3[2] * wedge_g9[2]) - (right_anti_dual_g3[3] * wedge_g9[3]), 
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g8[1] * wedge_g9[2]) + (wedge_g8[1] * right_anti_dual_g9[2]) + (right_anti_dual_g6[0] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[0]), (right_anti_dual_g8[2] * wedge_g9[0]) + (wedge_g8[2] * right_anti_dual_g9[0]) + (right_anti_dual_g6[1] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[1]), (right_anti_dual_g8[0] * wedge_g9[1]) + (wedge_g8[0] * right_anti_dual_g9[1]) + (right_anti_dual_g6[2] * wedge_g9[3]) + (right_anti_dual_g9[3] * wedge_g6[2]), -(right_anti_dual_g6[2] * wedge_g9[2]) - (right_anti_dual_g9[0] * wedge_g6[0]) - (right_anti_dual_g9[1] * wedge_g6[1]) - (right_anti_dual_g9[2] * wedge_g6[2])) + (right_anti_dual_g3 * float4(wedge_g0_y)) + (wedge_g3 * float4(right_anti_dual_g0[1])) - (wedge_g9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((wedge_g8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * wedge_g9[1]), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g10) * wedge_g6.xyz) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(wedge_g10) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * wedge_g9.yzx) + (wedge_g7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * wedge_g9.zxy) - (wedge_g7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(wedge_g9[3])) + (right_anti_dual_g8 * float3(wedge_g10)) + (wedge_g5 * float3(right_anti_dual_g0[1])) + (wedge_g7 * float3(right_anti_dual_g9[3])) + (wedge_g8 * float3(right_anti_dual_g10)) + (float3(wedge_g0_y) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * wedge_g9.xyz) - (float3(wedge_g6[3]) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(wedge_g0_y)) + (wedge_g6 * float4(right_anti_dual_g0[1])) + (right_anti_dual_g9.yzxw * float4(wedge_g9.zxy, wedge_g10)) - float4((right_anti_dual_g9.zxy * wedge_g9.yzx), right_anti_dual_g10 * wedge_g9[3]), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(wedge_g0_y)) + (wedge_g7 * float3(right_anti_dual_g0[1])) + (float3(wedge_g10) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(wedge_g0_y)) + (wedge_g8 * float3(right_anti_dual_g0[1])) + (float3(right_anti_dual_g9[3]) * wedge_g9.xyz) - (float3(wedge_g9[3]) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(wedge_g0_y)) + (wedge_g9 * float4(right_anti_dual_g0[1])), 
            /* e1234 */
            (right_anti_dual_g10 * wedge_g0_y) + (wedge_g10 * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(RoundPoint other) {
        float4 wedge_g0 = (float4(other.e2 * this.e43, other.e3 * this.e41, other.e1 * this.e42, (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)) + (other.group0.wwwy * float4(this.group1.xyz, this.e4315));
        float4 wedge_g1 = float4((other.e4 * this.e15) + (other.e5 * this.e41), (other.e4 * this.e25) + (other.e5 * this.e42), (other.e4 * this.e35) + (other.e5 * this.e43), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[1], wedge_g0[2], wedge_g0[0], 1.0) * float4(right_anti_dual_g0.zxy, (right_anti_dual_g0[2] * wedge_g3[2]) + (right_anti_dual_g0[3] * wedge_g3[3]) + (right_anti_dual_g1 * other.e5 * this.scalar)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((float3(right_anti_dual_g1) * wedge_g1.xyz), right_anti_dual_g0[1] * wedge_g3[1]), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * wedge_g2_xyz[0]) + (right_anti_dual_g0[3] * wedge_g0[0]), (right_anti_dual_g1 * wedge_g2_xyz[1]) + (right_anti_dual_g0[3] * wedge_g0[1]), (right_anti_dual_g1 * wedge_g2_xyz[2]) + (right_anti_dual_g0[3] * wedge_g0[2]), -(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) - (right_anti_dual_g0.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g0.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g0.yzx)), right_anti_dual_g1 * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(Scalar other) {
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
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Scalar anti_reject_orthogonally_from(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0[0] * wedge_g0[0]) + (right_anti_dual_g0[1] * wedge_g0[1]) + (right_anti_dual_g0[2] * wedge_g0[2]) + (right_anti_dual_g0[3] * wedge_g0[3]) + (other.e3215 * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(VersorEven other) {
        float4 wedge_g0 = (float4(this.scalar) * other.group0) + (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group3.zxy * this.group0.yzx), other.e5 * this.e1234) - float4((other.group3.yzx * this.group0.zxy), other.e423 * this.e15);
        float4 wedge_g1 = float4((other.e5 * this.e41) + (other.e4 * this.e15), (other.e5 * this.e42) + (other.e4 * this.e25), (other.e5 * this.e43) + (other.e4 * this.e35), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3]) - (right_anti_dual_g2[1] * wedge_g0[1]) - (right_anti_dual_g2[2] * wedge_g0[2])) + float4((right_anti_dual_g0.xyz * wedge_g0.www), right_anti_dual_g2[3] * other.e5 * this.scalar) + float4((wedge_g1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (wedge_g0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g1[0] * wedge_g0[3]), (wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g1[1] * wedge_g0[3]), (wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g1[2] * wedge_g0[3]), -(right_anti_dual_g3[1] * wedge_g1[1]) - (right_anti_dual_g3[2] * wedge_g1[2])) + (wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * wedge_g1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * wedge_g1.xyz) + (float3(wedge_g0[3]) * right_anti_dual_g2.xyz) + (wedge_g2_xyz.yzx * right_anti_dual_g3.zxy) - (wedge_g2_xyz.zxy * right_anti_dual_g3.yzx)), right_anti_dual_g2[3] * wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiRejectOrthogonallyFromInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_reject_orthogonally_from(VersorOdd other) {
        float3 wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float4 wedge_g2 = (float4(other.scalar) * this.group2) + (float4(this.scalar) * other.group2) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 wedge_g3 = float4((other.e23 * this.e45) + (other.e45 * this.e23) + (other.e35 * this.e42) + (other.e4235 * this.scalar), (other.e31 * this.e45) + (other.e45 * this.e31) + (other.e15 * this.e43) + (other.e4315 * this.scalar), (other.e12 * this.e45) + (other.e45 * this.e12) + (other.e25 * this.e41) + (other.e4125 * this.scalar), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + (other.group0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((this.group3.xyz * other.group0.www), other.e3215 * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.zxyw * float4(wedge_g3.yzx, other.scalar * this.scalar)) + float4(float3(0.0), (right_anti_dual_g3[1] * wedge_g3[1]) + (right_anti_dual_g3[2] * wedge_g3[2]) + (right_anti_dual_g3[3] * wedge_g3[3]) - (wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[1] * wedge_g2[1]) - (right_anti_dual_g0[2] * wedge_g2[2]) - (right_anti_dual_g1[0] * wedge_g1[0]) - (right_anti_dual_g1[1] * wedge_g1[1]) - (right_anti_dual_g1[2] * wedge_g1[2]) - (right_anti_dual_g1[3] * wedge_g1[3])) + float4((wedge_g0_xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * wedge_g2[3]) + float4((right_anti_dual_g1.xyz * wedge_g2.www), right_anti_dual_g3[0] * wedge_g3[0]) - (right_anti_dual_g0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0[3] * wedge_g1[0]) + (right_anti_dual_g2[0] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[1]) + (right_anti_dual_g2[1] * wedge_g2[3]), (right_anti_dual_g0[3] * wedge_g1[2]) + (right_anti_dual_g2[2] * wedge_g2[3]), -(right_anti_dual_g1[1] * wedge_g3[1]) - (right_anti_dual_g1[2] * wedge_g3[2])) + (right_anti_dual_g0 * float4(wedge_g3.www, wedge_g1[3])) - (right_anti_dual_g1.wwwx * wedge_g3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * wedge_g2.xyz) + (float3(wedge_g3[3]) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * wedge_g3.zxy) - (right_anti_dual_g2.zxy * wedge_g3.yzx)), right_anti_dual_g0[3] * wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
