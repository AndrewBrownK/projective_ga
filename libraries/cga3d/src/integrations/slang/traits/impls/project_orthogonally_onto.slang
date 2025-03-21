implementing cga3d;
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar * -1.0) * other.group2.xyz;
        float wedge_g2_w = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xy), wedge_g2_w * other.e35, (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_anti_dual_g2_xyz * float3(this.scalar);
        float wedge_g2_w = -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12) - (right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (this.scalar * other.e5)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g2_xyz[0] * this.e23);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0]) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        float4 wedge_g0 = float4((float3(other.e3215) * this.group0.xyz), this.scalar * other.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.scalar), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(wedge_g1.xyz, right_anti_dual_g0_w * this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (this.scalar * other.e5)) + float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g1.xyz), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_anti_dual_g0_w * this.scalar * other.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_anti_dual_g0 * float3(this.scalar);
        float wedge_g0_w = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_anti_dual_g0 * float4(this.scalar)) + (this.group0.xyz.xyzx * other.group1.wwwx) + float4(float3(0.0), (this.e42 * other.e25) + (this.e43 * other.e35) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual_g0 * other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((this.group0.zxy * other.group2.yzx), right_anti_dual_g1[1] * this.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((float3(wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge_g3.zxy), 0.0) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0] * other.e235) * float4(other.group2.zxy, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(((float3(right_anti_dual_g2_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_anti_dual_g2_w * this.scalar);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g2_w) * this.group1);
        float wedge_g2_x = (right_anti_dual_g2_w * this.e15) + (right_anti_dual_g2_xyz[0] * this.scalar);
        float wedge_g2_y = (right_anti_dual_g2_w * this.e25) + (right_anti_dual_g2_xyz[1] * this.scalar);
        float wedge_g2_z = (right_anti_dual_g2_w * this.e35) + (right_anti_dual_g2_xyz[2] * this.scalar);
        float wedge_g2_w = -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12) - (right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g2_xyz[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g2_y * other.e431) - (wedge_g2_z * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_x * other.e423) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_anti_dual_g2 * float3(this.scalar);
        float wedge_g2_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (other.group2.wwwx * float4(this.group1.xyz, this.e41)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45)) + float4((right_anti_dual_g0 * float3(this.scalar)), this.e42 * other.e25) + float4((this.group0.zxy * other.group3.yzx), this.e43 * other.e35) - float4((this.group0.yzx * other.group3.zxy), right_anti_dual_g0[0] * this.e15);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(this.e41, this.e42, this.e43, this.e23) * other.group3.wwwx) + (this.group1.wwwy * other.group3.xyzy) + float4(float3(0.0), this.e12 * other.e4125) + float4((float3(other.e1234) * this.group2.xyz), 0.0);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + float3((this.group2.yz * other.group3.zx), 0.0) - (float3(this.scalar) * other.group2.xyz) - float3((this.group2.zx * other.group3.yz), 0.0);
        float4 wedge_g3 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], this.scalar * other.e3215)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((float3(wedge_g0[3]) * other.group0.xyz), wedge_g3[1] * other.e4315) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345 * other.e12345 * -1.0) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345 * other.e12345 * -1.0) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e5 * other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(FlatPoint other) {
        float wedge_g2_w = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * other.e45), 
            /* e15, e25, e35, scalar */
            other.group0 * float4(float3(wedge_g2_w), this.scalar * other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4(float3(0.0), (this.e42 * other.e25) + (this.e43 * other.e35)) + float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e15) - float4((this.group0.yzx * other.group1.zxy), this.e45 * other.e45);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group1.wwwx) + (this.group1.wwwy * other.group1.xyzy) + float4(float3(0.0), (this.e12 * other.e4125) + (this.scalar * other.e45));
        float4 wedge_g2 = float4(((float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + float3((this.group2.yz * other.group1.zx), 0.0) - (float3(this.scalar) * other.group0.xyz) - float3((this.group2.zx * other.group1.yz), 0.0)), this.scalar * other.e3215);
        float3 wedge_g3_xyz = float3(this.scalar * -1.0) * other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3_xyz[0]) * other.group1.yzxx) + float4(float3(0.0), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Line other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g2_w = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435);
        float4 wedge_g3 = float4(float3(0.0), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) + float4((other.group1.yzx * wedge_g3.zxy), 0.0) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g1_xyz[0] * other.e415) * float4(other.group1.zxy, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.scalar);
        float4 wedge_g1 = float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e45);
        float wedge_g2_w = -(right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12) - (this.scalar * other.e5)) + float4((right_anti_dual_g0_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(float3(wedge_g2_w), wedge_g0[3])) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(other.e12345) * wedge_g0.xyz), wedge_g2_w * other.e5), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g1_xyz * float3(this.scalar * other.e12345)) + (float3(wedge_g3[3]) * other.group0.xyz) + (float3(right_anti_dual_g0_w * other.e12345) * this.group2.xyz) + (wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), wedge_g3[0] * other.e315 * -1.0) - float3((wedge_g3.yz * other.group1.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g0[1] * this.scalar) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = (this.group2 * float4(float2(right_anti_dual_g0[0]), right_anti_dual_g0[0], right_anti_dual_g3_w)) + float4((float3(this.scalar) * other.group8.xyz), right_anti_dual_g0[0] * this.e45);
        float3 wedge_g4 = (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz);
        float3 wedge_g5 = (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz);
        float4 wedge_g6 = (this.group2 * float4(float3(right_anti_dual_g1[3]), other.e45)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((right_anti_dual_g6_xyz * float3(this.scalar)), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_anti_dual_g1.zxy) - (this.group0.zxy * right_anti_dual_g1.yzx);
        float3 wedge_g8 = (right_anti_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + float3((right_anti_dual_g1.yz * this.group2.zx), 0.0) - float3((right_anti_dual_g1.zx * this.group2.yz), 0.0);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g5[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g3_w) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((this.group0.yzx * other.group8.zxy), 0.0) + float4((other.group7.yzx * this.group2.zxy), 0.0) - (this.group2.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0])) - float4((this.group0.zxy * other.group8.yzx), right_anti_dual_g5[1] * this.e25);
        float wedge_g10 = -(right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_anti_dual_g0.zxy) - (this.group0.zxy * right_anti_dual_g0.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.yzx * this.group2.zxy), 0.0) - float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0);
        float4 wedge_g3 = right_anti_dual_g0 * float4(this.scalar);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g2[3])) + (float4(wedge_g2[1], wedge_g2[2], wedge_g2[0], wedge_g3[0]) * other.group0.zxyx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125)) - float4((wedge_g2.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0]));
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(right_anti_dual_g0_xyz, other.e3215);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], this.scalar * other.e1234)) + (other.group0.zxyx * float4(wedge_g2_xyz.yzx, wedge_g3[0])) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234)) - float4((wedge_g2_xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float4 wedge_g0 = float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), right_anti_dual_g0_w * this.scalar);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1);
        float wedge_g2_x = (right_anti_dual_g0_w * this.e15) + (right_anti_dual_g2_xyz[0] * this.scalar);
        float wedge_g2_y = (right_anti_dual_g0_w * this.e25) + (right_anti_dual_g2_xyz[1] * this.scalar);
        float wedge_g2_z = (right_anti_dual_g0_w * this.e35) + (right_anti_dual_g2_xyz[2] * this.scalar);
        float wedge_g2_w = -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (this.scalar * other.e4);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12) - (right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (this.scalar * other.e5)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) - (this.group2.yzxx * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g2_xyz[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2_w) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g2_y * other.e431) - (wedge_g2_z * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_x) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = (right_anti_dual_g0 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45)) + float4((float3(right_anti_dual_g3[3]) * this.group1.xyz), 0.0) + float4((this.group0.yzx * right_anti_dual_g3.zxy), 0.0) - (float4(right_anti_dual_g3[1], right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g2_xyz[0] * this.e41) * float4(this.group0.zxy, 1.0));
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12)) + float4((float3(right_anti_dual_g3[3]) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + float3((right_anti_dual_g3.yz * this.group2.zx), 0.0) - float3((right_anti_dual_g3.zx * this.group2.yz), 0.0);
        float4 wedge_g3 = right_anti_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], this.scalar * other.e3215)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = (this.group3.yzxy * float4(other.group1.zxy, other.e25)) + (this.group3.wwwx * float4(other.group0.xyz, other.e15)) + float4(float3(0.0), other.e35 * this.e3) - (other.group1.yzxw * this.group3.zxyw) - float4((float3(this.e4) * other.group2.xyz), 0.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group3.xyz), 0.0) - (float4(other.e45 * this.e4) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = (this.group3.yzxy * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[1])) + (this.group3.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e5 * this.e4)) - (this.group3.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e4));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * other.e3215 * -1.0) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e4 * other.e3215 * other.e3215 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) + float4((right_anti_dual_g0_xyz * float3(this.e4)), 0.0) - (float4(other.e321 * -1.0) * float4(this.group3.xyz, this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * wedge_g0.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (float4(this.e4) * float4(right_anti_dual_g0_xyz, other.e5 * -1.0)) + float4(float3(0.0), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) - (float4(other.e321 * -1.0) * float4(this.group3.xyz, this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((right_anti_dual_g1 * float3(this.e4)), 0.0) + float4((right_anti_dual_g0.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e31, other.e12, other.e23, other.e15) * wedge_g0.zxyx) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((other.group0.zxy * wedge_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((this.group3.yzx * other.group0.zxy), 0.0) - (this.group3.zxyx * float4(other.group0.yzx, right_anti_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) - (right_anti_dual_g1[3] * this.e4 * other.e3215)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e4 * other.e5 * -1.0) * other.group0) + (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual_g0 * other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) + float4((right_anti_dual_g1_xyz * float3(this.e5)), 0.0) + float4((other.group2.zxy * this.group3.yzx), 0.0) - float4((other.group2.yzx * this.group3.zxy), right_anti_dual_g1_w * this.e321);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) + float4((right_anti_dual_g1_xyz * float3(this.e4)), 0.0) + float4((float3(right_anti_dual_g2[3]) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - float4((other.group0.zxy * this.group3.yzx), right_anti_dual_g1_w * this.e321);
        float4 wedge_g1 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(right_anti_dual_g2[3]) * this.group1.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g2_xyz = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2[3]) * this.group2.xyz) + float3(float2(0.0), (right_anti_dual_g2[1] * this.e1) - (right_anti_dual_g2[0] * this.e2)) + float3((right_anti_dual_g2.zx * this.group3.yz), 0.0) - float3((right_anti_dual_g2.yz * this.group3.zx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(wedge_g0[3]), right_anti_dual_g2[3] * this.e5)) + float4(float3(0.0), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((wedge_g2_xyz * float3(other.e12345)), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g2[3] * other.e12345) * float4(this.e1, this.e2, this.e3, this.e4)) + float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415)) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3) - (this.e5 * other.e45)) + float4((right_anti_dual_g2 * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group3.yzx * other.group1.zxy), 0.0) - (this.group3.zxyx * float4(other.group1.yzx, right_anti_dual_g2[0]));
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], this.e1 * other.e25, this.e1 * other.e35, this.e1 * other.e45)) + (float4(other.e42) * float4(this.e2 * other.e15, wedge_g0[3], this.e2 * other.e35, this.e2 * other.e45)) + (float4(other.e43) * float4(this.e3 * other.e15, this.e3 * other.e25, wedge_g0[3], this.e3 * other.e45)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group3.xyz), 0.0) - (float4(this.e4 * other.e45) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = -(float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 wedge_g1 = (float4(this.e3, this.e1, this.e2, this.e4) * other.group3.yzxw) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group3.yzxw);
        float4 wedge_g2 = (other.group3.xyzy * float4(float3(this.e5), this.e431)) + (other.group3.wwwx * float4(this.group3.xyz, this.e423)) + float4(float3(0.0), (this.e412 * other.e4125) + (this.e4 * other.e45) - (this.e321 * other.e1234) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (this.group3.yzxx * float4(other.group1.zxy, other.e15)) + (this.group3.wwwy * float4(other.group0.xyz, other.e25)) + float4(float3(0.0), this.e3 * other.e35) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234, other.e1234, other.e1234, other.e4315) * this.group2.xyzy) - (this.group2.wwwz * float4(other.group2.xyz, other.e4125)) - (this.group3.zxyw * other.group1.yzxw) - (other.group3.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(DualNum other) {
        float4 wedge_g0 = float4((float3(other.e12345 * -1.0) * this.group0.xyz), this.e4 * other.e5 * -1.0);
        float4 wedge_g2 = float4(other.e12345 * -1.0) * float4(this.group2.xyz, this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((float2(other.e12345) * wedge_g2.xy), wedge_g2[2] * other.e12345, (wedge_g0[3] * other.e5) + (wedge_g2[3] * other.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * other.e12345 * -1.0) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e4 * other.e45 * -1.0) * other.group0, 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) + (this.e1 * other.e15 * other.e45) + (this.e2 * other.e25 * other.e45) + (this.e3 * other.e35 * other.e45) - (other.e45 * other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group1.xyz;
        float3 wedge_g1_xyz = (this.group3.zxy * other.group1.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group3.yz * other.group1.zx), 0.0);
        float4 wedge_g2 = (other.group1.xyzy * float4(float3(this.e5), this.e431)) + (other.group1.wwwx * float4(this.group3.xyz, this.e423)) + float4(float3(0.0), (this.e412 * other.e4125) + (this.e4 * other.e45));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (this.e1 * other.e15) + (this.e2 * other.e25) + (this.e3 * other.e35) - (this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group0 * float4(float3(this.e4), this.e5)) - (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3] * -1.0) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group1.zxy) + float3(float2(0.0), wedge_g3[1] * other.e4235 * -1.0) - float3((wedge_g3.zx * other.group1.yz), 0.0)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3[1] * other.e25) + (wedge_g3[2] * other.e35) + (wedge_g3[3] * other.e45) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, this.e4 * other.e3215)) - (wedge_g2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float4 wedge_g2 = float4(float3(0.0), -(this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group3.yzx), 0.0) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(-(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g1_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (float4(this.e4) * float4(right_anti_dual_g0_xyz, other.e5 * -1.0)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0);
        float4 wedge_g1 = (float4(right_anti_dual_g0_w) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e1) - (right_anti_dual_g0_xyz[1] * this.e2) - (right_anti_dual_g0_xyz[2] * this.e3)) + float4((right_anti_dual_g1_xyz * float3(this.e4)), 0.0);
        float4 wedge_g2 = float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group2.xyz) + (right_anti_dual_g1_xyz.zxy * this.group3.yzx) - (right_anti_dual_g1_xyz.yzx * this.group3.zxy)), right_anti_dual_g0_w * this.e5);
        float4 wedge_g3 = float4(right_anti_dual_g0_w) * float4(this.group3.xyz, this.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(wedge_g1[3]), wedge_g3[3])) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(other.e12345) * wedge_g3.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4);
        float4 wedge_g1 = float4(right_anti_dual_g0[0]) * float4(this.group3.xyz, this.e4);
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * float4(this.e1, this.e2, this.e3, this.e4)) - (right_anti_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group3.xyz);
        float3 wedge_g5 = (right_anti_dual_g1.zxy * this.group3.yzx) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e2) * -1.0) - float3((right_anti_dual_g1.yz * this.group3.zx), 0.0);
        float4 wedge_g6 = (float4(right_anti_dual_g0[0]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) + float4((float3(this.e4) * other.group8.xyz), 0.0) + float4((float3(this.e5) * other.group7.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g5[0]));
        float3 wedge_g7 = (right_anti_dual_g5 * float3(this.e4)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group2.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_anti_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + (this.group2.wwwx * float4(right_anti_dual_g8, right_anti_dual_g1[0])) + float4(float3(0.0), -(right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(this.e5) * other.group4.xyz), 0.0) + float4((right_anti_dual_g6_xyz.yzx * this.group3.zxy), right_anti_dual_g1[2] * this.e125) - (this.group3.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0])) - float4((float3(right_anti_dual_g1[3]) * this.group2.xyz), 0.0) - float4((right_anti_dual_g1.zxy * this.group1.yzx), 0.0);
        float wedge_g10 = (this.e4 * other.e45) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412) - (right_anti_dual_g1[3] * this.e321) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group0.xyz;
        float3 wedge_g1_xyz = (this.group3.zxy * other.group0.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group3.yz * other.group0.zx), 0.0);
        float4 wedge_g2 = (other.group0.xyzy * float4(float3(this.e5), this.e431)) + (other.group0.wwwx * float4(this.group3.xyz, this.e423)) + float4(float3(0.0), this.e412 * other.e4125);
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3] * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g3[1] * other.e4235 * -1.0) - float3((wedge_g3.zx * other.group0.yz), 0.0)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group0.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, this.e4 * other.e3215)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e4 * other.e5) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float3 wedge_g0 = (float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group3.xyz);
        float4 wedge_g1 = float4((right_anti_dual_g0.zxy * this.group3.yzx), this.e4 * other.e3215) - (right_anti_dual_g0.yzxw * this.group3.zxyw);
        float4 wedge_g2 = float4(float3(0.0), -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (right_anti_dual_g0 * float4(float3(this.e5), this.e321));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e315) + (right_anti_dual_g0[2] * this.e125)) - float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group0.zxy, other.e1234)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group0.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group0.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) + float4((wedge_g1.zxy * other.group0.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float4 wedge_g0 = (float4(this.e4) * float4(right_anti_dual_g1_xyz, other.e5 * -1.0)) + (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], other.e4 * -1.0) * this.group3.zxyw) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), right_anti_dual_g3_xyz[0] * this.e1) - (right_anti_dual_g0.zxyx * float4(this.group3.yzx, this.e235));
        float4 wedge_g1 = (right_anti_dual_g0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group3) + (float4(right_anti_dual_g0[3] * other.e12345) * float4(this.e1, this.e2, this.e3, this.e4)) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float3 wedge_g0 = (float3(this.e4) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * this.group3.xyz);
        float4 wedge_g1 = float4((right_anti_dual_g3.zxy * this.group3.yzx), right_anti_dual_g2[3] * this.e4) - (right_anti_dual_g3.yzxw * this.group3.zxyw);
        float4 wedge_g2 = float4(float3(0.0), -(right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) + float4((float3(right_anti_dual_g2[3]) * this.group3.xyz), this.e4 * other.e45) - (right_anti_dual_g3 * float4(float3(this.e5), this.e321));
        float4 wedge_g3 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + (right_anti_dual_g3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e125) - (right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(right_anti_dual_g2[3]) * this.group0.xyz), right_anti_dual_g3[1] * this.e315) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group3.yzx * other.group1.zxy), 0.0) - (this.group3.zxyx * float4(other.group1.yzx, right_anti_dual_g2[0])) - float4((float3(right_anti_dual_g3[3]) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3.zxy * this.group1.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2[3] * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0]) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * (other.e321 * -1.0) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g1.xyz), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (other.e321 * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Circle other) {
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, right_anti_dual_g2_w);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g3_w = right_anti_dual_g2_w * this.e3215;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(other.e12345) * wedge_g0.xyz), (wedge_g0[3] * other.e12345) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)), 
            /* e23, e31, e12, e45 */
            float4(((float3(wedge_g3_w) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3_w) * other.group1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g3_w * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Dipole other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g2 = float3(this.scalar * -1.0) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = this.group0.xy.yyyx * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 wedge_g3 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g2[3])) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group0.xyz), wedge_g3[1] * other.e4315) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_orthogonally_onto(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e12345 * other.e12345 * this.e3215) - (this.scalar * other.e5 * other.e12345), other.e12345 * other.e12345 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3] * -1.0) * other.group1.xyz), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 wedge_g0 = right_anti_dual_g0 * float4(this.scalar);
        float wedge_g1_w = (right_anti_dual_g0[3] * this.e3215) - (this.scalar * other.e5);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge_g0.xyz), (wedge_g0[3] * other.e12345) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1_w) * other.group0.xyz) + (float3(this.scalar * other.e12345) * other.group1.xyz)), wedge_g1_w * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float wedge_g0_y = (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[3] * this.e3215);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = float4(this.scalar) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group6.xyz;
        float4 wedge_g6 = float4(this.scalar) * float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 wedge_g7 = float3(this.scalar * -1.0) * other.group4.xyz;
        float3 wedge_g8 = float3(this.scalar * -1.0) * other.group3.xyz;
        float4 wedge_g9 = float4((float3(this.scalar) * other.group1.xyz), (right_anti_dual_g0[0] * this.e3215) - (this.scalar * other.e5));
        float wedge_g10 = this.scalar * other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float wedge_g0_w = right_anti_dual_g0[3] * this.e3215;
        float4 wedge_g3 = right_anti_dual_g0 * float4(this.scalar);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 wedge_g0 = right_anti_dual_g0 * float4(this.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4((float3(this.scalar) * other.group3.xyz), (right_anti_dual_g0[3] * this.e3215) - (this.scalar * other.e5));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2[3]) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = float4((float3(this.scalar * -1.0) * other.group0.xyz), (right_anti_dual_g3[3] * this.e3215) + (this.scalar * other.scalar));
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 wedge_g3 = right_anti_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g2[3])) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
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
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
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
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle project_orthogonally_onto(Circle other) {
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), right_anti_dual_g2_w * this.e321);
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float wedge_g2_w = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g2_w) * float4(other.group0.xyz, other.e12345), 
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
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0_xyz = float3(other.e1234 * -1.0) * this.group0.xyz;
        float wedge_g0_w = (this.e321 * other.e3215) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125);
        float wedge_g1 = this.e321 * other.e1234 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g0_xyz * float3(other.e1234)) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0_xyz.yzx * other.group3.zxy), wedge_g0_w * other.e1234) - (other.group3.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0_xyz[0] * other.e41) - (wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)) + float4((float3(wedge_g0_w) * other.group3.xyz), 0.0) - float4((wedge_g0_xyz * float3(other.e3215)), wedge_g1 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0_w) * float4(other.group0.xyz, other.e45)) + float4(float3(0.0), (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g0_xyz[0] * other.e15) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0_xyz.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(Flector other) {
        float wedge_g0_x = (this.e321 * other.e3215) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), wedge_g0_x * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float wedge_g0_y = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(float3(0.0), right_anti_dual_g0[0] * this.e321);
        float3 wedge_g8 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float4 wedge_g9 = float4(other.e1234, other.e1234, other.e1234, (right_anti_dual_g1_xyz[0] * this.e235) + (right_anti_dual_g1_xyz[1] * this.e315) + (right_anti_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0);
        float wedge_g10 = this.e321 * other.e1234 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_orthogonally_onto(Plane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(this.e321 * other.e3215) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group0.xyz), 0.0) - float4((float3(other.e4235) * this.group0.yxx * other.group0.yyz), 0.0) - float4((float3(other.e4125) * this.group0.zzy * other.group0.xyy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0_xyz = float3(other.e1234 * -1.0) * this.group0.xyz;
        float wedge_g0_w = (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215);
        float wedge_g1 = this.e321 * other.e1234 * -1.0;
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), right_anti_dual_g0_w * this.e321);
        float4 wedge_g2 = float4((float2(right_anti_dual_g0_w) * this.group0.xy), right_anti_dual_g0_w * this.e125, (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g2[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0_xyz = float3(other.e1234 * -1.0) * this.group0.xyz;
        float wedge_g0_w = (right_anti_dual_g3_xyz[0] * this.e235) + (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215);
        float wedge_g1 = this.e321 * other.e1234 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g0_xyz * float3(other.e1234)) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0_xyz.yzx * other.group3.zxy), wedge_g0_w * other.e1234) - (other.group3.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0_xyz[0] * other.e41) - (wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)) + float4((float3(wedge_g0_w) * other.group3.xyz), 0.0) - float4((wedge_g0_xyz * float3(other.e3215)), wedge_g1 * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0_w) * float4(other.group0.xyz, other.e45)) + float4(float3(0.0), (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g0_xyz[0] * other.e15) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0_xyz.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiFlector> operator /(AntiFlector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = (this.group1.yzxy * float4(other.group1.zxy, other.e25)) + (this.group1.wwwx * float4(other.group0.xyz, other.e15)) + float4(float3(0.0), other.e35 * this.e3) - (other.group1.yzxw * this.group1.zxyw);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group1.xyz), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = (this.group1.yzxy * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[1])) + (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - (this.group1.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e4));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        float4 wedge_g0 = float4(other.e321) * float4(this.group1.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * wedge_g0.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4((float2(right_anti_dual_g0_w * -1.0) * this.group1.xy), right_anti_dual_g0_w * this.e3 * -1.0, (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_w * this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = (this.group1.zxyx * float4(right_anti_dual_g0.yzx, other.e15)) + float4(float3(0.0), (this.e2 * other.e25) + (this.e3 * other.e35)) - float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e31, other.e12, other.e23, other.e15) * wedge_g0.zxyx) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((other.group0.zxy * wedge_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 wedge_g1 = (this.group1.yzxx * float4(other.group0.zxy, other.e15)) + float4(float3(0.0), (this.e321 * other.e3215) + (this.e2 * other.e25) + (this.e3 * other.e35)) - float4((this.group1.zxy * other.group0.yzx), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(float3(0.0), -(this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) + float4((right_anti_dual_g1_xyz * float3(this.e5)), 0.0) + float4((other.group2.zxy * this.group1.yzx), 0.0) - float4((other.group2.yzx * this.group1.zxy), right_anti_dual_g1_w * this.e321);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(float3(0.0), -(this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) + float4((other.group0.yzx * this.group1.zxy), 0.0) - float4((other.group0.zxy * this.group1.yzx), right_anti_dual_g1_w * this.e321);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), right_anti_dual_g2_w * this.e321) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g2_xyz = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group0.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(wedge_g0[3]), right_anti_dual_g2_w * this.e5)) + float4(float3(0.0), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((wedge_g2_xyz * float3(other.e12345)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((float3(right_anti_dual_g2_w * other.e12345) * this.group1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415)) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float4 wedge_g0 = (this.group1.yzxx * float4(other.group1.zxy, other.e15)) + (this.group1.wwwy * float4(other.group0.xyz, other.e25)) + float4(float3(0.0), this.e3 * other.e35) - (this.group1.zxyw * other.group1.yzxw);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], this.e1 * other.e25, this.e1 * other.e35, this.e1 * other.e45)) + (float4(other.e42) * float4(this.e2 * other.e15, wedge_g0[3], this.e2 * other.e35, this.e2 * other.e45)) + (float4(other.e43) * float4(this.e3 * other.e15, this.e3 * other.e25, wedge_g0[3], this.e3 * other.e45)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group1.xyz), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (this.group1.zxy * other.group3.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group1.yz * other.group3.zx), 0.0);
        float wedge_g1_w = this.e5 * other.e1234 * -1.0;
        float wedge_g2_x = (this.e1 * other.e3215) + (this.e5 * other.e4235);
        float wedge_g2_y = (this.e2 * other.e3215) + (this.e5 * other.e4315);
        float wedge_g2_z = (this.e3 * other.e3215) + (this.e5 * other.e4125);
        float wedge_g2_w = -(this.e321 * other.e1234) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        float4 wedge_g3 = (this.group1.yzxx * float4(other.group1.zxy, other.e15)) + (this.group1.wwwy * float4(other.group0.xyz, other.e25)) + float4(float3(0.0), (this.e321 * other.e3215) + (this.e3 * other.e35) - (this.e315 * other.e4315) - (this.e125 * other.e4125)) - (float4(other.e1234, other.e1234, other.e1234, other.e4235) * this.group0.xyzx) - (this.group1.zxyw * other.group1.yzxw);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2_w) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_w) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2_w * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2_x, wedge_g2_y, wedge_g2_z, wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), -(wedge_g2_y * other.e4315) - (wedge_g2_z * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_x)) - float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e1 * other.e15 * other.e45) + (this.e2 * other.e25 * other.e45) + (this.e3 * other.e35 * other.e45) - (this.e5 * other.e45 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Flector other) {
        float3 wedge_g0_xyz = (this.group1.zxy * other.group1.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group1.yz * other.group1.zx), 0.0);
        float4 wedge_g1 = (other.group1 * float4(float3(this.e5), this.e321)) + (this.group1.xyzx * float4(float3(other.e3215), other.e15)) + float4(float3(0.0), (this.e2 * other.e25) + (this.e3 * other.e35) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125) - (this.e5 * other.e45));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g1[3] * other.e45) - (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_orthogonally_onto(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(other.group1.yzx, other.e415));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (right_anti_dual_g0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e2) - (right_anti_dual_g0[2] * this.e3)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g0[0]));
        float4 wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            (other.group0 * float4(float3(wedge_g0[3]), wedge_g1[3])) + float4(float3(0.0), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4);
        float4 wedge_g1 = float4((float3(right_anti_dual_g0[0]) * this.group1.xyz), 0.0);
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(((float3(other.e3215) * this.group1.xyz) - (right_anti_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0);
        float3 wedge_g4 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float3 wedge_g5 = (right_anti_dual_g1_xyz.zxy * this.group1.yzx) - (right_anti_dual_g1_xyz.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) + float4((float3(this.e5) * other.group7.xyz), right_anti_dual_g0[0] * this.e321) - (this.group1.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g5[0]));
        float3 wedge_g7 = (other.group7.yzx * this.group1.zxy) - (other.group7.zxy * this.group1.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group8.zxy * this.group1.yzx) - (other.group8.yzx * this.group1.zxy);
        float4 wedge_g9 = float4(float3(0.0), (right_anti_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215) - (right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(this.e5) * other.group4.xyz), right_anti_dual_g1_xyz[1] * this.e315) + float4((right_anti_dual_g6_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e235) - (this.group1.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0])) - float4((float3(other.e1234) * this.group0.xyz), 0.0);
        float wedge_g10 = -(this.e321 * other.e1234) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_orthogonally_onto(Plane other) {
        float3 wedge_g0_xyz = (this.group1.zxy * other.group0.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group1.yz * other.group0.zx), 0.0);
        float4 wedge_g1 = (other.group0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((float3(other.e3215) * this.group1.xyz), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3]) * other.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0_xyz.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (right_anti_dual_g0_xyz.zxy * this.group1.yzx) - (right_anti_dual_g0_xyz.yzx * this.group1.zxy);
        float wedge_g1_w = this.e5 * other.e1234 * -1.0;
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) - (right_anti_dual_g0_xyz * float3(this.e5));
        float wedge_g2_w = this.e321 * other.e1234 * -1.0;
        float3 wedge_g3_xyz = float3(other.e1234 * -1.0) * this.group0.xyz;
        float wedge_g3_w = (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g3_xyz * float3(other.e1234)) - (float3(wedge_g2_w) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3_xyz.yzx * other.group0.zxy), wedge_g3_w * other.e1234) - (other.group0.yzxw * float4(wedge_g3_xyz.zxy, wedge_g2_w)), 
            /* e235, e315, e125, e4 */
            (other.group0.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g3_xyz * float3(other.e3215)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g2_xyz[1] * other.e4315) - (wedge_g2_xyz[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float4 wedge_g0 = (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g3_xyz[0]) * this.group1.zxyx) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (this.e5 * other.e4)) - (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e235));
        float4 wedge_g1 = (right_anti_dual_g0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group0.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy)), right_anti_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group3) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((float3(right_anti_dual_g0[3] * other.e12345) * this.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float3 wedge_g0 = float3(right_anti_dual_g3[3] * -1.0) * this.group1.xyz;
        float3 wedge_g1_xyz = (right_anti_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), (right_anti_dual_g3[0] * this.e2) * -1.0) - float3((right_anti_dual_g3.yz * this.group1.zx), 0.0);
        float wedge_g1_w = right_anti_dual_g3[3] * this.e5 * -1.0;
        float4 wedge_g2 = float4(float3(0.0), -(this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) - (right_anti_dual_g3 * float4(float3(this.e5), this.e321));
        float4 wedge_g3 = float4(float3(0.0), (right_anti_dual_g3[1] * this.e315) + (right_anti_dual_g3[2] * this.e125) + (this.e321 * other.e3215) - (right_anti_dual_g2_xyz[1] * this.e2) - (right_anti_dual_g2_xyz[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group1.yzx * other.group1.zxy), right_anti_dual_g3[0] * this.e235) - (this.group1.zxyx * float4(other.group1.yzx, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g3[3]) * this.group0.xyz), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g2[2] * other.e4125) * -1.0) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiLine> operator /(AntiLine lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12);
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - float4((other.group0.zxy * this.group1.yzx), right_anti_dual_g1_xyz[0] * this.e15);
        float wedge_g1 = -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)) - float4((wedge_g0.yzx * other.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group0.xyz;
        return AntiLine(
            /* e23, e31, e12 */
            wedge_g0_xyz * float3(other.e321 * -1.0), 
            /* e15, e25, e35 */
            (wedge_g0_xyz.zxy * other.group0.yzx) - (wedge_g0_xyz.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(other.e321 * -1.0)), (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz.zxy * other.group0.yzx) - (wedge_g0_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        float wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float wedge_g0_w = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0_w) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0 * other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((this.group1.yzx * other.group0.zxy), right_anti_dual_g1_xyz[0] * this.e15);
        float wedge_g1 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g1_xyz = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group1.xyz;
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), right_anti_dual_g1_xyz[0] * this.e15) - float4((this.group1.yzx * other.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g1_xyz[0] * other.e415) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((wedge_g1_xyz * float3(other.e12345)), 0.0) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Dipole other) {
        float wedge_g0 = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float wedge_g1_w = (this.e23 * other.e4235) + (this.e31 * other.e4315) + (this.e12 * other.e4125);
        float4 wedge_g2 = float4(float3(0.0), (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)) + float4((float3(other.e3215) * this.group0.xyz), this.e23 * other.e23) + float4((this.group1.yzx * other.group3.zxy), this.e31 * other.e31) - float4((this.group1.zxy * other.group3.yzx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g1_w * other.e45), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (other.group3.xyzx * float4(float3(wedge_g1_w), wedge_g1_xyz[0])), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (float3(wedge_g2[3]) * other.group2.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = (this.group0.xyz.xyzx * other.group1.wwwx) + (other.group1.zxyy * float4(this.group1.yzx, this.e31)) + float4(float3(0.0), this.e12 * other.e4125) - float4((this.group1.zxy * other.group1.yzx), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[3] * -1.0) * float4(other.group1.xyz, other.e45), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public FlatPoint project_orthogonally_onto(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            -float4((float3(this.e23 * other.e235) * other.group0.xyz), 0.0) - float4((float3(this.e31 * other.e315) * other.group0.xyz), 0.0) - float4((float3(this.e12 * other.e125) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group1.xyz), 0.0) - float4((other.group0.xyz * this.group1.yxx * other.group0.yxx), 0.0) - float4((other.group0.xyz * this.group1.zzy * other.group0.zzy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0_xyz = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float wedge_g1_w = -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(other.e12345)), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1_w) * other.group0.xyz) + (float3(right_anti_dual_g0_w * other.e12345) * this.group1.xyz)), wedge_g1_w * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43);
        float4 wedge_g3 = float4((float3(right_anti_dual_g0[0]) * this.group1.xyz), 0.0);
        float3 wedge_g5 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float4 wedge_g6 = float4(other.e1234, other.e1234, other.e1234, -(right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12)) * float4(this.group1.xyz, 1.0);
        float3 wedge_g7 = float3(other.e1234) * this.group0.xyz;
        float3 wedge_g8 = (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g1_xyz.yzx * this.group1.zxy) - (right_anti_dual_g1_xyz.zxy * this.group1.yzx);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g5[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((this.group1.zxy * other.group7.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), right_anti_dual_g5[0] * this.e15) - float4((this.group1.yzx * other.group7.zxy), right_anti_dual_g5[1] * this.e25);
        float wedge_g10 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g5.zxy * other.group9.yzx), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[0] * other.e415) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[1] * other.e425) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g7[2] * other.e435), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = (this.group0.xyz.xyzx * other.group0.wwwx) + (other.group0.zxyy * float4(this.group1.yzx, this.e31)) + float4(float3(0.0), this.e12 * other.e4125) - float4((this.group1.zxy * other.group0.yzx), 0.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), (wedge_g0[1] * other.e4235) * -1.0) - float3((wedge_g0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public Dipole project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx);
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g1_xyz * float3(other.e1234)) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234) + (right_anti_dual_g0_xyz[0] * this.e23 * other.e4235) + (right_anti_dual_g0_xyz[1] * this.e31 * other.e4235) + (right_anti_dual_g0_xyz[2] * this.e12 * other.e4235), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234) + (right_anti_dual_g0_xyz[0] * this.e23 * other.e4315) + (right_anti_dual_g0_xyz[1] * this.e31 * other.e4315) + (right_anti_dual_g0_xyz[2] * this.e12 * other.e4315), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234) + (right_anti_dual_g0_xyz[0] * this.e23 * other.e4125) + (right_anti_dual_g0_xyz[1] * this.e31 * other.e4125) + (right_anti_dual_g0_xyz[2] * this.e12 * other.e4125), -(wedge_g1_xyz[0] * other.e4235) - (wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)), 
            /* e15, e25, e35 */
            (wedge_g1_xyz * float3(other.e3215)) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float4 wedge_g1 = float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0);
        float3 wedge_g2_xyz = float3(right_anti_dual_g0_w) * this.group1.xyz;
        float wedge_g2_w = -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g1_xyz[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2_w) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_xyz[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float wedge_g1_w = -(right_anti_dual_g3_xyz[0] * this.e23) - (right_anti_dual_g3_xyz[1] * this.e31) - (right_anti_dual_g3_xyz[2] * this.e12);
        float4 wedge_g2 = float4(float3(0.0), (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)) + float4((float3(other.e3215) * this.group0.xyz), this.e31 * other.e31) + float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), this.e23 * other.e23) - float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2[3]) * other.group0) + float4(float3(0.0), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g1_w * other.e45), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (other.group3.xyzx * float4(float3(wedge_g1_w), wedge_g1_xyz[0])), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (float3(wedge_g2[3]) * other.group2.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiMotor> operator /(AntiMotor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar * -1.0) * other.group2.xyz;
        float wedge_g2_w = (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xy), wedge_g2_w * other.e35, (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_anti_dual_g2_xyz * float3(this.scalar);
        float wedge_g2_w = -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12) - (right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (other.e5 * this.scalar)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0]) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 wedge_g1 = float4((float2(right_anti_dual_g0_w) * this.group0.xy), right_anti_dual_g0_w * this.e12, -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(wedge_g1.xyz, right_anti_dual_g0_w * this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float wedge_g1_x = (right_anti_dual_g0_w * this.e23) + (right_anti_dual_g1_xyz[0] * this.scalar);
        float wedge_g1_y = (right_anti_dual_g0_w * this.e31) + (right_anti_dual_g1_xyz[1] * this.scalar);
        float wedge_g1_z = (right_anti_dual_g0_w * this.e12) + (right_anti_dual_g1_xyz[2] * this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g1_x, wedge_g1_y, wedge_g1_z * other.e321, (wedge_g1_x * other.e1) + (wedge_g1_y * other.e2) + (wedge_g1_z * other.e3) - (right_anti_dual_g0_w * other.e321 * this.scalar)) * float4((other.group0.ww * float2(-1.0)), 1.0, 1.0) * float4(1.0, 1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1_z, wedge_g1_x, wedge_g1_y) * other.group0.yzx) - (float3(wedge_g1_y, wedge_g1_z, wedge_g1_x) * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_anti_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float3 wedge_g0_xyz = float3(this.scalar * -1.0) * other.group0.xyz;
        float wedge_g0_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xy), wedge_g0_w * other.e12, (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((float3(wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge_g3.zxy), 0.0) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0] * other.e235) * float4(other.group2.zxy, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, right_anti_dual_g2_w);
        float3 wedge_g1_xyz = (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g2_w) * this.group0.xyz);
        float wedge_g1_w = right_anti_dual_g1_w * this.scalar;
        float wedge_g2_x = (right_anti_dual_g2_w * this.e15) + (right_anti_dual_g2_xyz[0] * this.scalar);
        float wedge_g2_y = (right_anti_dual_g2_w * this.e25) + (right_anti_dual_g2_xyz[1] * this.scalar);
        float wedge_g2_z = (right_anti_dual_g2_w * this.e35) + (right_anti_dual_g2_xyz[2] * this.scalar);
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = (this.group1.zxyw * float4(other.group0.yzx, right_anti_dual_g2_w)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g2_x * other.e423) - (wedge_g2_y * other.e431) - (wedge_g2_z * other.e412) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g1_w * other.e321) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(wedge_g2_w), wedge_g1_w)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((wedge_g1_xyz * float3(other.e12345)), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar * -1.0) * other.group2.xyz;
        float wedge_g2_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((right_anti_dual_g0 * float3(this.scalar)), 0.0);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), (this.e23 * other.e4235) + (this.e31 * other.e4315) + (this.e12 * other.e4125)) + float4((float3(other.e1234) * this.group1.xyz), 0.0);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + float3(float2(0.0), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + float3((this.group1.yz * other.group3.zx), 0.0) - (float3(this.scalar) * other.group2.xyz) - float3((this.group1.zx * other.group3.yz), 0.0);
        float4 wedge_g3 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], this.scalar * other.e3215)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((float3(wedge_g0[3]) * other.group0.xyz), wedge_g3[1] * other.e4315) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e12345 * -1.0) * float3((float2(other.e12345) * this.group1.xy), this.e35 * other.e12345)), -(other.e12345 * other.e12345 * this.e3215) - (this.scalar * other.e5 * other.e12345))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g1.yzx * this.group1.zxy), 0.0) - (right_anti_dual_g1.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3] * -1.0) * other.group1.xyz), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            -float4((float3(this.e23 * other.e235) * other.group0.xyz), 0.0) - float4((float3(this.e31 * other.e315) * other.group0.xyz), 0.0) - float4((float3(this.e12 * other.e125) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group1.xyz), 0.0) - float4((other.group0.xyz * other.group0.yxx * this.group1.yxx), 0.0) - float4((other.group0.xyz * other.group0.zzy * this.group1.zzy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float3 wedge_g0_xyz = (right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(other.e12345)), (right_anti_dual_g0_w * this.scalar * other.e12345) - (wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = float4(((float3(right_anti_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz)), right_anti_dual_g3_w * this.scalar);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz);
        float4 wedge_g6 = (right_anti_dual_g6 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0);
        float3 wedge_g7 = (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group0.xyz);
        float3 wedge_g8 = (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g1.yzx * this.group1.zxy) + float3(float2(0.0), (right_anti_dual_g1[1] * this.e15) * -1.0) - (float3(this.scalar) * other.group3.xyz) - float3((right_anti_dual_g1.zx * this.group1.yz), 0.0);
        float4 wedge_g9 = (this.group1.zxyw * float4(other.group7.yzx, right_anti_dual_g0[0])) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e25) - (right_anti_dual_g5[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5)) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) - (this.group1.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0]));
        float wedge_g10 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0.yzx * this.group1.zxy), 0.0) - (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = right_anti_dual_g0 * float4(this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3] * -1.0) * other.group0.xyz), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = float4(right_anti_dual_g0[3]) * float4(this.group0.xyz, this.e3215);
        float3 wedge_g1_xyz = float3(right_anti_dual_g0[3]) * this.group1.xyz;
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0.yzx * this.group1.zxy) + float3(float2(0.0), (right_anti_dual_g0[1] * this.e15) * -1.0) - float3((right_anti_dual_g0.zx * this.group1.yz), 0.0);
        float4 wedge_g3 = right_anti_dual_g0 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1_xyz, this.scalar * other.e3215)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group0.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215)) - float4((wedge_g0.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[0] * other.e3215) + (right_anti_dual_g0[0] * this.e23 * other.e4235) + (right_anti_dual_g0[1] * this.e31 * other.e4235) + (right_anti_dual_g0[2] * this.e12 * other.e4235), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[1] * other.e3215) + (right_anti_dual_g0[0] * this.e23 * other.e4315) + (right_anti_dual_g0[1] * this.e31 * other.e4315) + (right_anti_dual_g0[2] * this.e12 * other.e4315), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[2] * other.e3215) + (right_anti_dual_g0[0] * this.e23 * other.e4125) + (right_anti_dual_g0[1] * this.e31 * other.e4125) + (right_anti_dual_g0[2] * this.e12 * other.e4125), -(wedge_g1_xyz[0] * other.e4235) - (wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (wedge_g2_xyz.yzx * other.group0.zxy) - (wedge_g2_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        float4 wedge_g0 = right_anti_dual_g0 * float4(this.scalar);
        float4 wedge_g1 = float4(((right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0[3]) * this.group0.xyz)), right_anti_dual_g1_w * this.scalar);
        float4 wedge_g2 = (right_anti_dual_g2 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0);
        float4 wedge_g3 = (right_anti_dual_g0.yzxw * this.group1.zxyw) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2[0] * this.e23) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12) - (this.scalar * other.e5)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) - (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g1_xyz[0]) * this.group1.yzxx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2[3]) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = (right_anti_dual_g0 * float4(this.scalar)) + (float4(right_anti_dual_g3[3]) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12));
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g3[0] * this.e23) - (right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12)) + float4((float3(right_anti_dual_g3[3]) * this.group1.xyz), 0.0);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3.yzx * this.group1.zxy) + float3(float2(0.0), (right_anti_dual_g3[1] * this.e15) * -1.0) - (float3(this.scalar) * other.group2.xyz) - float3((right_anti_dual_g3.zx * this.group1.yz), 0.0);
        float4 wedge_g3 = right_anti_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], this.scalar * other.e3215)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiPlane> operator /(AntiPlane lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = (this.group0.yzxy * float4(other.group1.zxy, other.e25)) + (this.group0.wwwx * float4(other.group0.xyz, other.e15)) + float4(float3(0.0), other.e35 * this.e3) - (other.group1.yzxw * this.group0.zxyw);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group0.xyz), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = (this.group0.yzxy * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[1])) + (this.group0.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), right_anti_dual_g3_xyz[2] * this.e3) - (this.group0.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e4));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiFlatPoint other) {
        float3 wedge_g0 = float3(other.e321) * this.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(other.e321)), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4((float3(other.e321) * this.group0.xyz), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = (this.group0.zxyx * float4(right_anti_dual_g0.yzx, other.e15)) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3)) - float4((right_anti_dual_g0.zxy * this.group0.yzx), 0.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e31, other.e12, other.e23, other.e15) * wedge_g0.zxyx) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((other.group0.zxy * wedge_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 wedge_g1 = (this.group0.yzxx * float4(other.group0.zxy, other.e15)) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3)) - float4((other.group0.yzx * this.group0.zxy), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), right_anti_dual_g1_xyz[0]));
        float3 wedge_g2 = (right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * other.e425), 
            /* e5 */
            -(wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((this.e1 * other.e321) + (this.e5 * other.e423), (this.e2 * other.e321) + (this.e5 * other.e431), (this.e3 * other.e321) + (this.e5 * other.e412), -(right_anti_dual_g1_xyz[0] * this.e1) - (right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3));
        float4 wedge_g2 = float4((right_anti_dual_g1_xyz * float3(this.e5)), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.e12345), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * wedge_g2.xyz), (wedge_g2[3] * other.e12345) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e12345 * other.e12345) * this.group0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float4 wedge_g0 = (this.group0.yzxx * float4(other.group1.zxy, other.e15)) + (this.group0.wwwy * float4(other.group0.xyz, other.e25)) + float4(float3(0.0), this.e3 * other.e35) - (this.group0.zxyw * other.group1.yzxw);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], this.e1 * other.e25, this.e1 * other.e35, this.e1 * other.e45)) + (float4(other.e42) * float4(this.e2 * other.e15, wedge_g0[3], this.e2 * other.e35, this.e2 * other.e45)) + (float4(other.e43) * float4(this.e3 * other.e15, this.e3 * other.e25, wedge_g0[3], this.e3 * other.e45)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group0.xyz), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group0.yz * other.group3.zx), 0.0);
        float wedge_g1_w = this.e5 * other.e1234 * -1.0;
        float wedge_g2_x = (this.e1 * other.e3215) + (this.e5 * other.e4235);
        float wedge_g2_y = (this.e2 * other.e3215) + (this.e5 * other.e4315);
        float wedge_g2_z = (this.e3 * other.e3215) + (this.e5 * other.e4125);
        float wedge_g2_w = -(this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        float4 wedge_g3 = (this.group0.yzxx * float4(other.group1.zxy, other.e15)) + (this.group0.wwwy * float4(other.group0.xyz, other.e25)) + float4(float3(0.0), this.e3 * other.e35) - (this.group0.zxyw * other.group1.yzxw);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2_w) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_w) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2_w * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2_x, wedge_g2_y, wedge_g2_z, wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), -(wedge_g2_y * other.e4315) - (wedge_g2_z * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_x)) - float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e1 * other.e15 * other.e45) + (this.e2 * other.e25 * other.e45) + (this.e3 * other.e35 * other.e45) - (this.e5 * other.e45 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(Flector other) {
        float3 wedge_g0_xyz = (this.group0.zxy * other.group1.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group0.yz * other.group1.zx), 0.0);
        float4 wedge_g1 = (this.group0.xyzx * float4(float3(other.e3215), other.e15)) + (this.group0.wwwy * float4(other.group1.xyz, other.e25)) + float4(float3(0.0), (this.e3 * other.e35) - (this.e5 * other.e45));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g1[3] * other.e45) - (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, other.e415));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e2) - (right_anti_dual_g0_xyz[2] * this.e3)) + float4((right_anti_dual_g0_xyz * float3(this.e5)), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g0_xyz[0]));
        float4 wedge_g1 = float4(other.e12345 * -1.0) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            (other.group0 * float4(float3(wedge_g0[3]), wedge_g1[3])) + float4(float3(0.0), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (this.e5 * other.e4);
        float4 wedge_g1 = float4((float3(right_anti_dual_g0[0]) * this.group0.xyz), 0.0);
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(((float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0);
        float3 wedge_g4 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float3 wedge_g5 = (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) + float4((float3(this.e5) * other.group7.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), right_anti_dual_g5[0]));
        float3 wedge_g7 = (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(this.e5) * other.group4.xyz), 0.0) + float4((right_anti_dual_g6_xyz.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0]));
        float wedge_g10 = -(this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = (this.group0.zxy * other.group0.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group0.yz * other.group0.zx), 0.0);
        float3 wedge_g1 = (float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy);
        float wedge_g1_w = this.e5 * other.e1234 * -1.0;
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e5));
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e1234) * float4(wedge_g2, wedge_g1_w)) + (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g0 * float3(other.e3215)), 0.0) - float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(wedge_g1_w * other.e3215) - (wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float4 wedge_g0 = (this.group0.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3)) - (this.group0.yzxw * float4(right_anti_dual_g0_xyz.zxy, other.e4));
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g0_xyz * float3(this.e5)), 0.0) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group3) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((float3(right_anti_dual_g0_w * other.e12345) * this.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float3 wedge_g1_xyz = (right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy);
        float wedge_g1_w = this.e5 * other.e1234 * -1.0;
        float4 wedge_g2 = float4(float3(0.0), -(this.e2 * other.e42) - (this.e3 * other.e43)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group0.wwwx * float4(right_anti_dual_g3_xyz, other.e41));
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e2) - (right_anti_dual_g2_xyz[2] * this.e3) - (this.e5 * other.e45)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, right_anti_dual_g2_xyz[0]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g2[2] * other.e4125) * -1.0) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor project_orthogonally_onto(CircleRotor other) {
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
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * other.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public Motor project_orthogonally_onto(Motor other) {
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0 = this.e12345 * other.e12345 * -1.0;
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public VersorEven project_orthogonally_onto(VersorEven other) {
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
public ProjectOrthogonallyOntoInfixPartial<Circle> operator /(Circle lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint project_orthogonally_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * other.e3215 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            -(float4(other.e321 * this.e321 * -1.0) * other.group0) - (float4(right_anti_dual_g0_xyz[0] * this.e423) * other.group0) - (float4(right_anti_dual_g0_xyz[1] * this.e431) * other.group0) - (float4(right_anti_dual_g0_xyz[2] * this.e412) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((wedge_g0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0 * other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0 = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g2_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group2.xyz;
        float wedge_g2_w = (this.e321 * other.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2_w) * other.group0.xyz)), wedge_g2_w * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2_w) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e235) + (wedge_g2_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g2_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g2_xyz[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (this.e125 * other.e4125) * -1.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (other.group3.yzxx * float4(this.group1.zxy, this.e235)) - float4((float3(other.e1234) * this.group2.xyz), this.e315 * other.e4315);
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) - (this.e321 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g0.yzxw) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle project_orthogonally_onto(DualNum other) {
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
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1 * -1.0) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group1.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group1.yz), 0.0)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g0.xyz)), wedge_g1 * other.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float2(wedge_g1 * -1.0) * other.group0.xy), wedge_g1 * other.e35 * -1.0, (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Line project_orthogonally_onto(Line other) {
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float4 wedge_g2 = float4((float3(right_anti_dual_g0_w) * this.group2.xyz), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(right_anti_dual_g0[0]) * this.group1;
        float3 wedge_g7 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g8 = float3(right_anti_dual_g0[0]) * this.group2.xyz;
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g1_xyz[1] * this.e315) + (right_anti_dual_g1_xyz[2] * this.e125)) + float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g10 = -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group0.yz), 0.0)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125)) + float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g1 = -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group0.zxy, other.e1234)) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group0.yzxw), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float4 wedge_g2 = float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125)) + float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g1 = -(right_anti_dual_g3_xyz[0] * this.e423) - (right_anti_dual_g3_xyz[1] * this.e431) - (right_anti_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g0.yzxw) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<CircleRotor> operator /(CircleRotor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint project_orthogonally_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * other.e3215 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            -(float4(other.e321 * this.e321 * -1.0) * other.group0) - (float4(right_anti_dual_g0_xyz[0] * this.e423) * other.group0) - (float4(right_anti_dual_g0_xyz[1] * this.e431) * other.group0) - (float4(right_anti_dual_g0_xyz[2] * this.e412) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((wedge_g0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual_g0 * other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float wedge_g0 = (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0 = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g2_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group2.xyz;
        float wedge_g2_w = (right_anti_dual_g2_w * this.e12345) + (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2_w) * other.group0.xyz)), wedge_g2_w * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2_w) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e235) + (wedge_g2_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g2_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g2_xyz[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (this.e125 * other.e4125) * -1.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234, other.e1234, other.e1234, other.e4315) * this.group2.xyzy) - (other.group3.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) - (this.e321 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g0.yzxw) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(DualNum other) {
        float4 wedge_g2 = float4(other.e12345 * -1.0) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4((float3(other.e12345 * -1.0) * this.group0.xyz), wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            wedge_g2 * float4(float3(other.e12345), other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1 * -1.0) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group1.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group1.yz), 0.0)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g0.xyz)), wedge_g1 * other.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float2(wedge_g1 * -1.0) * other.group0.xy), wedge_g1 * other.e35 * -1.0, (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Line project_orthogonally_onto(Line other) {
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float4 wedge_g2 = float4((float2(right_anti_dual_g0_w) * this.group2.xy), right_anti_dual_g0_w * this.e125, (right_anti_dual_g0_w * this.e12345) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e12345) + (this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(right_anti_dual_g0[0]) * this.group1;
        float3 wedge_g7 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g8 = float3(right_anti_dual_g0[0]) * this.group2.xyz;
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g1_xyz[1] * this.e315) + (right_anti_dual_g1_xyz[2] * this.e125)) + float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g10 = -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group0.yz), 0.0)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Circle project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125)) + float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g1 = -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group0.zxy, other.e1234)) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group0.yzxw), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float4 wedge_g2 = float4((float2(right_anti_dual_g0_w) * this.group2.xy), right_anti_dual_g0_w * this.e125, (right_anti_dual_g0_w * this.e12345) + (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125)) + float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0);
        float wedge_g1 = -(right_anti_dual_g3_xyz[0] * this.e423) - (right_anti_dual_g3_xyz[1] * this.e431) - (right_anti_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g0.yzxw) - (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g1) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((float3(wedge_g1) * other.group2.xyz), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Dipole> operator /(Dipole lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[0] * this.e15) - float4((other.group0.zxy * this.group2.yzx), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g1 = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)) - float4((wedge_g0.yzx * other.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public FlatPoint project_orthogonally_onto(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215 * other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * wedge_g0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), (wedge_g0[0] * other.e315) * -1.0) - float3((wedge_g0.yz * other.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g0.xyz), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        float wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float wedge_g0_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xy), wedge_g0_w * other.e12, (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0 * other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), right_anti_dual_g1_xyz[0] * this.e15) - float4((other.group2.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g1 = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0 = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g2_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group2.xyz;
        float wedge_g2_w = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group1.xyzx * float4(float3(other.e321), right_anti_dual_g2_xyz[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[0] * this.e15) - float4((other.group0.zxy * this.group2.yzx), right_anti_dual_g1_xyz[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * float3(other.e12345)), 0.0) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g0[0] * other.e235) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Dipole other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group3.wwwx) + (this.group1.wwwy * other.group3.xyzy) + float4((float3(other.e1234) * this.group2.xyz), this.e12 * other.e4125);
        float4 wedge_g2 = (this.group1.xyzx * float4(float3(other.e3215), other.e23)) + float4(float3(0.0), (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)) + float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e15) - float4((this.group2.zxy * other.group3.yzx), this.e45 * other.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e1234) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(DualNum other) {
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
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            (float4(this.e41 * other.e15, this.e42 * other.e25, this.e43 * other.e35, this.e41 * other.e15) * other.group0) + (float4(this.e42 * other.e25, this.e41 * other.e15, this.e41 * other.e15, this.e42 * other.e25) * other.group0) + (other.group0 * float4(float2(this.e43 * other.e35), this.e42 * other.e25, this.e43 * other.e35)) - (float4(this.e45 * other.e45) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(Flector other) {
        float3 wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group1.wwwx) + (this.group1.wwwy * other.group1.xyzy) + float4(float3(0.0), this.e12 * other.e4125);
        float4 wedge_g2 = float4(float3(0.0), (this.e42 * other.e25) + (this.e43 * other.e35)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group1.zxy), this.e41 * other.e15) - float4((this.group2.zxy * other.group1.yzx), this.e45 * other.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float wedge_g2_w = -(right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12)) + float4((right_anti_dual_g0_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2_w) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((wedge_g0 * float3(other.e12345)), 0.0), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(right_anti_dual_g0_w * other.e12345) * this.group2.xyz) + (wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), wedge_g3[0] * other.e315 * -1.0) - float3((wedge_g3.yz * other.group1.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g5 = float3(right_anti_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy);
        float3 wedge_g8 = (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g1_xyz.yzx * this.group2.zxy) - (right_anti_dual_g1_xyz.zxy * this.group2.yzx);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g5[2] * this.e35) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((this.group0.yzx * other.group8.zxy), 0.0) + float4((this.group2.zxy * other.group7.yzx), 0.0) - (this.group1.xyzx * float4(float3(other.e321), other.e235)) - float4((this.group0.zxy * other.group8.yzx), right_anti_dual_g5[0] * this.e15) - float4((this.group2.yzx * other.group7.zxy), right_anti_dual_g5[1] * this.e25);
        float wedge_g10 = -(right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group0.wwwx) + (this.group1.wwwy * other.group0.xyzy) + float4(float3(0.0), this.e12 * other.e4125);
        float3 wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0]));
        float3 wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((wedge_g2 * float3(other.e1234)), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g0_w) * this.group2.xyz;
        float wedge_g2_w = -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) - (this.group1.xyzx * float4(float3(other.e321), right_anti_dual_g2_xyz[0])) - float4((right_anti_dual_g0_xyz.zxy * this.group2.yzx), right_anti_dual_g1_xyz[0] * this.e15) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2_w) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * float3(other.e12345)), wedge_g3[1] * other.e2) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_xyz[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e31) - (right_anti_dual_g3_xyz[2] * this.e12)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0]));
        float4 wedge_g2 = (this.group1.xyzx * float4(float3(other.e3215), other.e23)) + float4(float3(0.0), (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45)) + float4((right_anti_dual_g3_xyz.yzx * this.group2.zxy), this.e31 * other.e31) - float4((right_anti_dual_g3_xyz.zxy * this.group2.yzx), right_anti_dual_g2_xyz[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2[3]) * other.group0) + float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((float3(other.e1234) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g1 = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)) - float4((wedge_g0.yzx * other.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
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
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiLine project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * wedge_g0.xyz, 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), (wedge_g0[0] * other.e315) * -1.0) - float3((wedge_g0.yz * other.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g0.xyz), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiLine project_orthogonally_onto(AntiLine other) {
        float wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float3 wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float wedge_g0_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float2(wedge_g0_w) * other.group0.xy), wedge_g0_w * other.e12, (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0 * other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(other.e321 * -1.0) * this.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0])) - float4((other.group2.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g1 = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0 = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g2_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g2_w) * this.group2.xyz;
        float wedge_g2_w = (right_anti_dual_g2_w * this.e1234) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = (float4(right_anti_dual_g2_w) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group1.xyzx * float4(float3(other.e321), right_anti_dual_g2_xyz[0])) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * float3(other.e12345)), 0.0) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g0[0] * other.e235) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(wedge_g2_w), wedge_g1[3])) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Dipole other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 wedge_g1 = (other.group3.xyzx * this.group1.wwwx) + (other.group3.wwwy * float4(this.group0.xyz, this.e31)) + float4(float3(0.0), other.e4125 * this.e12) + float4((float3(other.e1234) * this.group2.xyz), 0.0);
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + (this.group2.yzxx * float4(other.group3.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group3.yzxx * float4(this.group2.zxy, this.e4235));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e1234) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        float4 wedge_g2 = float4(other.e12345 * -1.0) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0.xy.yyyx * float4((float3(other.e12345 * -1.0) * this.group0.xyz), wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            (float4(this.e41 * other.e15, this.e42 * other.e25, this.e43 * other.e35, this.e41 * other.e15) * other.group0) + (float4(this.e42 * other.e25, this.e41 * other.e15, this.e41 * other.e15, this.e42 * other.e25) * other.group0) + (other.group0 * float4(float2(this.e43 * other.e35), this.e42 * other.e25, this.e43 * other.e35)) - (float4(this.e45 * other.e45) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(Flector other) {
        float3 wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group1.wwwx) + (this.group1.wwwy * other.group1.xyzy) + float4(float3(0.0), this.e12 * other.e4125);
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + float4(float3(0.0), (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)) + float4((this.group2.yzx * other.group1.zxy), this.e41 * other.e15) - (other.group1.yzxx * float4(this.group2.zxy, this.e4235));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_orthogonally_onto(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float wedge_g2_w = (right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43);
        float4 wedge_g3 = (right_anti_dual_g0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2_w) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((wedge_g0 * float3(other.e12345)), 0.0), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(right_anti_dual_g0[3] * other.e12345) * this.group2.xyz) + (wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), wedge_g3[0] * other.e315 * -1.0) - float3((wedge_g3.yz * other.group1.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(right_anti_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g5 = float3(right_anti_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy);
        float3 wedge_g8 = (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g1_xyz.yzx * this.group2.zxy) - (right_anti_dual_g1_xyz.zxy * this.group2.yzx);
        float4 wedge_g9 = (float4(right_anti_dual_g0[0]) * this.group3) + float4(float3(0.0), -(right_anti_dual_g5[2] * this.e35) - (this.e31 * other.e315) - (this.e12 * other.e125)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((this.group0.yzx * other.group8.zxy), 0.0) + float4((other.group7.yzx * this.group2.zxy), 0.0) - (this.group1.xyzx * float4(float3(other.e321), other.e235)) - (this.group2.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0])) - float4((this.group0.zxy * other.group8.yzx), right_anti_dual_g5[1] * this.e25);
        float wedge_g10 = (right_anti_dual_g0[0] * this.e1234) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 wedge_g1 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group0.wwwx) + (this.group1.wwwy * other.group0.xyzy) + float4(float3(0.0), this.e12 * other.e4125);
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + float4(float3(0.0), -(this.e4315 * other.e4315) - (this.e4125 * other.e4125)) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group2.zxy, this.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - float4((wedge_g2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0]));
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)) + float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), right_anti_dual_g0_xyz[0] * this.e4235) - float4((right_anti_dual_g0_xyz.zxy * this.group2.yzx), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group0.zx), 0.0) - float3((wedge_g2.zx * other.group0.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float4 wedge_g1 = float4(right_anti_dual_g0_w) * this.group1;
        float3 wedge_g2_xyz = float3(right_anti_dual_g0_w) * this.group2.xyz;
        float wedge_g2_w = (right_anti_dual_g0_w * this.e1234) - (right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43);
        float4 wedge_g3 = (float4(right_anti_dual_g0_w) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) - (this.group1.xyzx * float4(float3(other.e321), right_anti_dual_g2_xyz[0])) - (this.group2.yzxx * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1_xyz[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2_w) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * float3(other.e12345)), wedge_g3[1] * other.e2) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2_xyz[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e31) - (right_anti_dual_g3_xyz[2] * this.e12)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0]));
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + (this.group2.zxyx * float4(right_anti_dual_g3_xyz.yzx, other.e41)) + float4(float3(0.0), (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45)) - float4((right_anti_dual_g3_xyz.zxy * this.group2.yzx), right_anti_dual_g2_xyz[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g2[3]) * other.group0) + float4(float3(0.0), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((float3(other.e1234) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group3.yzx), 0.0) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            (wedge_g2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<DualNum> operator /(DualNum lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g1_xyz = float3(this.e5) * other.group1.xyz;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0[3]) * other.group2.xyz), (wedge_g0[3] * other.e4) - (wedge_g0[0] * other.e423) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g1_xyz[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Circle other) {
        float3 wedge_g0 = float3(this.e5) * other.group0.xyz;
        float3 wedge_g1 = float3(this.e5) * other.group1.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + float4((wedge_g0 * float3(other.e321)), 0.0) + float4((wedge_g1.yzx * other.group0.zxy), 0.0) - float4((wedge_g1.zxy * other.group0.yzx), wedge_g0[0] * other.e423), 
            /* e5 */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = right_anti_dual_g2_w * this.e12345;
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, right_anti_dual_g2_w);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e321), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(wedge_g0_w), wedge_g1[3])) + float4(float3(0.0), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)) + float4((wedge_g0_xyz * float3(other.e321)), 0.0) + float4((other.group0.zxy * wedge_g1.yzx), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0_xyz[0] * other.e423) * float4(other.group0.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g1[1] * other.e4235 * -1.0) - float3((wedge_g1.zx * other.group3.yz), 0.0)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((float3(wedge_g1[3]) * other.group3.xyz), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], wedge_g1[0]) * other.group2.wwwx) + (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(DualNum other) {
        float2 wedge_g0 = float2(other.e12345 * -1.0) * this.group0.xy;
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0[0] * other.e12345) + (wedge_g0[1] * other.e5), wedge_g0[1] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e45 * other.e45 * this.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiFlector project_orthogonally_onto(Flector other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group1.xyz;
        float wedge_g1_w = this.e5 * other.e45 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g1_w) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g1_w * other.e45) - (wedge_g0_xyz[0] * other.e4235) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float wedge_g0_w = right_anti_dual_g0[3] * this.e12345;
        float4 wedge_g1 = right_anti_dual_g0 * float4(this.e5);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(wedge_g0_w) * other.group0, 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e12345) - (this.e5 * other.e4);
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(this.e5) * float4(other.group9.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g6 = float4((float3(this.e5) * other.group7.xyz), 0.0);
        float3 wedge_g8 = float3(this.e5) * other.group6.xyz;
        float4 wedge_g9 = float4(this.e5) * float4(other.group4.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g2 * other.e1234) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(Plane other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge_g0_xyz[0] * other.e4235) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
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
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            wedge_g0 * float4(other.e1234), 
            /* e5 */
            -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4((float3(this.e5) * other.group0.xyz), (right_anti_dual_g0_w * this.e12345) - (this.e5 * other.e4));
        float4 wedge_g1 = float4(this.e5) * float4(other.group1.xyz, right_anti_dual_g0_w);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (wedge_g0 * float4(float3(other.e321), other.e4)) + float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) + float4((wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g1[1] * other.e4235 * -1.0) - float3((wedge_g1.zx * other.group3.yz), 0.0)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((float3(wedge_g1[3]) * other.group3.xyz), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], wedge_g1[0]) * other.group2.wwwx) + (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<FlatPoint> operator /(FlatPoint lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)) - float4((wedge_g0.yzx * other.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) + float4((other.group0.zxy * wedge_g1.yzx), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0] * other.e423) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group0.xyz), wedge_g0[3] * other.e12345) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Dipole other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = (this.group0.xyzx * float4(float3(other.e1234), other.e41)) + (this.group0.wwwy * float4(other.group3.xyz, other.e42)) + float4(float3(0.0), (other.e43 * this.e35) - (other.e45 * this.e45));
        float3 wedge_g1_xyz = (other.group3.zxy * this.group0.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - float3((other.group3.yz * this.group0.zx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[0] * other.e1234) + (wedge_g0[3] * other.e41), (wedge_g0[1] * other.e1234) + (wedge_g0[3] * other.e42), (wedge_g0[2] * other.e1234) + (wedge_g0[3] * other.e43), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Flector project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4(this.e45) * float4(other.group1.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g1_xyz = (this.group0.yzx * other.group1.zxy) + float3(float2(0.0), (this.e25 * other.e4235) * -1.0) - float3((this.group0.zx * other.group1.yz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g1_xyz.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)) + float4((float3(-(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) * other.group0.xyz), 0.0) + float4((wedge_g0_xyz.zxy * other.group1.yzx), 0.0) - float4((wedge_g0_xyz.yzx * other.group1.zxy), wedge_g0_xyz[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Flector project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 wedge_g1 = float4((right_anti_dual_g0_xyz * float3(this.e45)), -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35));
        return Flector(
            /* e15, e25, e35, e45 */
            (other.group0 * float4(float3(wedge_g1[3]), right_anti_dual_g0_w * this.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(right_anti_dual_g0_w * other.e12345) * this.group0.xyz), 0.0) + float4((wedge_g1.zxy * other.group1.yzx), 0.0) - (wedge_g1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0);
        float3 wedge_g8 = (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g5[1] * this.e25) - (right_anti_dual_g5[2] * this.e35)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((other.group7.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0]));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = float3(this.e45) * other.group0.xyz;
        float3 wedge_g1 = (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e25 * other.e4235) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((wedge_g1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = (float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e45));
        float3 wedge_g1 = (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx);
        return Dipole(
            /* e41, e42, e43 */
            wedge_g0 * float3(other.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.e1234, other.e1234, other.e1234, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) * float4(wedge_g1, 1.0), 
            /* e15, e25, e35 */
            (wedge_g0 * float3(other.e3215)) + (wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1_xyz[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e4) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(float3(wedge_g1[3]), wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 wedge_g0 = (this.group0.xyzx * float4(float3(other.e1234), other.e41)) + float4(float3(0.0), (this.e25 * other.e42) + (this.e35 * other.e43)) - (float4(this.e45) * float4(right_anti_dual_g3_xyz, other.e45));
        float3 wedge_g1_xyz = (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(float3(other.e1234), other.scalar)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Flector> operator /(Flector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)) - float4((wedge_g0.yzx * other.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Dipole project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(right_anti_dual_g2_w) * this.group0;
        float4 wedge_g1 = (float4(right_anti_dual_g2_w) * this.group1) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) + float4((other.group0.zxy * wedge_g1.yzx), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0] * other.e423) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group0.xyz), wedge_g0[3] * other.e12345) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Dipole project_orthogonally_onto(Dipole other) {
        float wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + (this.group0.wwwx * float4(other.group3.xyz, other.e41)) + float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125));
        float3 wedge_g1_xyz = (other.group3.zxy * this.group0.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - float3((other.group3.yz * this.group0.zx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[0] * other.e1234) + (wedge_g0[3] * other.e41), (wedge_g0[1] * other.e1234) + (wedge_g0[3] * other.e42), (wedge_g0[2] * other.e1234) + (wedge_g0[3] * other.e43), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public FlatPoint project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4((float3(this.e45) * other.group1.xyz), -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125));
        float3 wedge_g1_xyz = (other.group1.zxy * this.group0.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g1_xyz.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public FlatPoint project_orthogonally_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)) + float4((float3(-(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) * other.group0.xyz), 0.0) + float4((wedge_g0_xyz.zxy * other.group1.yzx), 0.0) - float4((wedge_g0_xyz.yzx * other.group1.zxy), wedge_g0_xyz[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 wedge_g1 = (right_anti_dual_g0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            (other.group0 * float4(float3(wedge_g1[3]), right_anti_dual_g0[3] * this.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((float3(right_anti_dual_g0[3] * other.e12345) * this.group0.xyz), 0.0) + float4((wedge_g1.zxy * other.group1.yzx), 0.0) - (wedge_g1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(right_anti_dual_g0[0]) * this.group0;
        float4 wedge_g6 = float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0);
        float3 wedge_g8 = (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx);
        float4 wedge_g9 = (float4(right_anti_dual_g0[0]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e25) - (right_anti_dual_g5[2] * this.e35)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((other.group7.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0]));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Plane other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        float3 wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e25 * other.e4235) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) + float4((wedge_g0_xyz * float3(other.e3215)), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0_xyz[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125)) - float4((right_anti_dual_g0_xyz * float3(this.e45)), 0.0);
        float3 wedge_g1_xyz = (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e1234, other.e1234, other.e1234, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) * float4(wedge_g1_xyz, 1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group0.zxy) - (wedge_g1_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(right_anti_dual_g0_w) * this.group0;
        float4 wedge_g1 = (float4(right_anti_dual_g0_w) * this.group1) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1_xyz[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e4) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(float3(wedge_g1[3]), wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)) - (float4(this.e45) * float4(right_anti_dual_g3_xyz, other.e45));
        float3 wedge_g1_xyz = (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(float3(other.e1234), other.scalar)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + float4((wedge_g1_xyz * float3(other.e1234)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Line> operator /(Line lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float wedge_g0 = -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0 * other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Circle project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float wedge_g0 = -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0_xyz = float3(right_anti_dual_g2_w) * this.group0.xyz;
        float wedge_g0_w = -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 wedge_g1_xyz = float3(right_anti_dual_g2_w) * this.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e321), 
            /* e235, e315, e125, e5 */
            float4((wedge_g0_w * other.e235) + (wedge_g1_xyz[0] * other.e12345), (wedge_g0_w * other.e315) + (wedge_g1_xyz[1] * other.e12345), (wedge_g0_w * other.e125) + (wedge_g1_xyz[2] * other.e12345), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)) + float4((wedge_g0_xyz * float3(other.e321)), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = float4(float3(0.0), (other.e4125 * this.e125) * -1.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) - (float4(this.e435, this.e415, this.e425, this.e235) * other.group3.yzxx) - float4((float3(other.e1234) * this.group1.xyz), other.e4315 * this.e315);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group3.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4(float3(0.0), -(other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e435, this.e415, this.e425, this.e235) * other.group1.yzxx);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(wedge_g0[3]) * float4(other.e4235, other.e4315, other.e4125, other.e45)) + float4(float3(0.0), (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((float3(other.e3215) * wedge_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Line other) {
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
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 wedge_g0_xyz = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float wedge_g0_w = -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435);
        float4 wedge_g1 = float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = -(right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4((float3(right_anti_dual_g0[0]) * this.group0.xyz), 0.0);
        float3 wedge_g8 = float3(right_anti_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g9 = float4(float3(0.0), (right_anti_dual_g1_xyz[1] * this.e315) + (right_anti_dual_g1_xyz[2] * this.e125)) + float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), right_anti_dual_g1_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e435, this.e415, this.e425, this.e235) * other.group0.yzxx);
        return Line(
            /* e415, e425, e435 */
            (wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), (wedge_g0[1] * other.e4235) * -1.0) - float3((wedge_g0.zx * other.group0.yz), 0.0), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Circle project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125)) + float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0);
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group0.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4((float3(right_anti_dual_g0_w) * this.group0.xyz), -(right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435));
        float4 wedge_g1 = float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (wedge_g0 * float4(float3(other.e321), other.e4)) + float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) + float4((wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125)) + float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), right_anti_dual_g3_xyz[0] * this.e235) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((right_anti_dual_g3_xyz.zxy * this.group0.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g0[1] * other.e4235 * -1.0) - float3((wedge_g0.zx * other.group3.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) - (wedge_g0.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((wedge_g0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Motor> operator /(Motor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4((float3(this.e5) * other.group0.xyz), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5));
        float3 wedge_g1_xyz = right_anti_dual_g1_xyz * float3(this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0[3]) * other.group2.xyz), (wedge_g0[3] * other.e4) - (wedge_g0[0] * other.e423) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g1_xyz[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 wedge_g1_xyz = right_anti_dual_g1_xyz * float3(this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0_w) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0_w) * other.group2.xyz), -(wedge_g0_xyz[0] * other.e423) - (wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) + float4((wedge_g0_xyz * float3(other.e321)), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = (float4(right_anti_dual_g2_w) * this.group0) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(this.e5) * other.group0.xyz), 0.0);
        float3 wedge_g1_xyz = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group1.xyz);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(wedge_g0[3]), right_anti_dual_g2_w * this.e5)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((wedge_g1_xyz * float3(other.e12345)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + float4((float3(other.e321) * wedge_g0.xyz), 0.0) + float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float4 wedge_g0 = float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(float3(0.0), -(other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group3.zxy * this.group0.yzx), 0.0) - (this.group1 * float4(float3(other.e1234), other.e45)) - (other.group3.yzxx * float4(this.group0.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g1[1] * other.e4235 * -1.0) - float3((wedge_g1.zx * other.group3.yz), 0.0)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((float3(wedge_g1[3]) * other.group3.xyz), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], wedge_g1[0]) * other.group2.wwwx) + (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(DualNum other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g1 = float4(other.e12345 * -1.0) * this.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e235, e315, e125, e5 */
            float4((float2(other.e12345) * wedge_g1.xy), wedge_g1[2] * other.e12345, (wedge_g0[3] * other.e5) + (wedge_g1[3] * other.e12345))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public DualNum project_orthogonally_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e45 * other.e45 * this.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Flector other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group1.xyz;
        float4 wedge_g1 = float4(float3(0.0), -(other.e45 * this.e5) - (other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (other.group1.yzxx * float4(this.group0.zxy, this.e235));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.yzx * other.group1.zxy) - (wedge_g1.zxy * other.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(wedge_g1[3]) * float4(other.e4235, other.e4315, other.e4125, other.e45)) + float4(float3(0.0), (wedge_g1[0] * other.e15) + (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) - (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g0_xyz[0]) * other.group1.wwwx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Line other) {
        float wedge_g0_w = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435);
        float3 wedge_g1_xyz = float3(this.e5) * other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(wedge_g0_w) * other.group1.xyz), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 wedge_g0_xyz = float3(right_anti_dual_g0_w) * this.group0.xyz;
        float wedge_g0_w = (right_anti_dual_g0_w * this.e12345) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435);
        float4 wedge_g1 = float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e5);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e12345) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4);
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = right_anti_dual_g1 * float4(this.e5 * -1.0);
        float4 wedge_g6 = float4(((float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group7.xyz)), 0.0);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz);
        float4 wedge_g9 = (right_anti_dual_g1.yzxx * float4(this.group0.zxy, this.e235)) + (this.group1.wwwy * float4(other.group4.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), right_anti_dual_g1[2] * this.e125) - (this.group1 * float4(float3(right_anti_dual_g1[3]), other.e45)) - float4((right_anti_dual_g1.zxy * this.group0.yzx), 0.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g2 * other.e1234) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Plane other) {
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float4 wedge_g1 = float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e235));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) + float4((float3(wedge_g1[3]) * other.group0.xyz), 0.0) - (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g0_xyz[0]) * other.group0.wwwx)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
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
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = right_anti_dual_g0 * float4(this.e5 * -1.0);
        float4 wedge_g1 = (right_anti_dual_g0.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e315) + (right_anti_dual_g0[2] * this.e125)) - float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) - float4((right_anti_dual_g0.zxy * this.group0.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g1[1] * other.e4235 * -1.0) - float3((wedge_g1.zx * other.group0.yz), 0.0)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g1.xyz)), wedge_g0[3] * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e1234, other.e1234, other.e1234, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) * float4(wedge_g0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (right_anti_dual_g0 * float4(float3(this.e5), this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (this.e5 * other.e4)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0);
        float4 wedge_g1 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group1.xyz)), right_anti_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (wedge_g0 * float4(float3(other.e321), other.e4)) + float4(float3(0.0), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + float4((float3(wedge_g0[3]) * other.group3.xyz), 0.0) + float4((wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = right_anti_dual_g3 * float4(this.e5 * -1.0);
        float4 wedge_g1 = (right_anti_dual_g3.yzxx * float4(this.group0.zxy, this.e235)) + (this.group1.wwwy * float4(other.group0.xyz, right_anti_dual_g3[1])) + float4(float3(0.0), right_anti_dual_g3[2] * this.e125) - (this.group1 * float4(float3(right_anti_dual_g3[3]), other.e45)) - float4((right_anti_dual_g3.zxy * this.group0.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) + float3(float2(0.0), wedge_g1[1] * other.e4235 * -1.0) - float3((wedge_g1.zx * other.group3.yz), 0.0)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((float3(wedge_g1[3]) * other.group3.xyz), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g0[0], wedge_g0[1], wedge_g0[2], wedge_g1[0]) * other.group2.wwwx) + (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(MultiVector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
        float4 wedge_g6 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g7 = right_anti_dual_g0 * float3(this.scalar);
        float3 wedge_g8 = float3(this.scalar * -1.0) * other.group2.xyz;
        float4 wedge_g9 = (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], other.e15) * this.group1.zxyx) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3)) - (float4(this.e5) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - float4((float3(this.e4) * other.group2.xyz), 0.0) - float4((right_anti_dual_g1.zxy * this.group1.yzx), 0.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g9[3]) * other.group0.xyz), 0.0) + float4((wedge_g9.zxy * other.group1.yzx), 0.0) - (float4(right_anti_dual_g0[0] * other.e15, right_anti_dual_g0[1] * other.e25, right_anti_dual_g0[2] * other.e35, right_anti_dual_g1[3] * other.e45) * this.group1) - (float4(right_anti_dual_g0[1] * other.e15, right_anti_dual_g0[0] * other.e25, right_anti_dual_g0[0] * other.e35, right_anti_dual_g0[0] * other.e45) * this.group1.yxxx) - (float4(right_anti_dual_g0[2] * other.e15, right_anti_dual_g0[2] * other.e25, right_anti_dual_g0[1] * other.e35, right_anti_dual_g0[1] * other.e45) * this.group1.zzyy) - (float4(right_anti_dual_g1[3] * other.e15, right_anti_dual_g1[3] * other.e25, right_anti_dual_g1[3] * other.e35, right_anti_dual_g0[2] * other.e45) * this.group1.wwwz) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e41)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float wedge_g0_y = (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g3 = float4(this.scalar) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = right_anti_dual_g1_xyz * float3(this.scalar);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy);
        float4 wedge_g9 = (right_anti_dual_g3 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g1_w) * this.group5.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group4.yzx), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0])) - float4((right_anti_dual_g2_xyz.yzx * this.group4.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g10 = (right_anti_dual_g2_w * this.scalar) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * float4(other.e1, other.e2, other.e3, other.e4)) + float4(float3(0.0), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412)) + float4((float3(wedge_g6[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group2.yzx), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            (wedge_g0_y * other.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group1.xyz), 0.0) + float4((wedge_g9.zxy * other.group2.yzx), 0.0) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiDualNum other) {
        float wedge_g0_y = (other.e3215 * this.e1234) + (other.scalar * this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0_y * other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e3215 * other.e3215 * this.e4 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215 * other.e3215) * this.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215 * other.e3215 * -1.0) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float wedge_g0_y = -(right_anti_dual_g0_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412);
        float3 wedge_g6_xyz = (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(right_anti_dual_g0_w) * this.group1.xyz);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g0_w) * this.group5.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group4.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group4.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0_w * other.e321 * this.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_xyz * float3(other.e321)), 0.0), 
            /* e5 */
            -(wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * wedge_g9.xyz, 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float wedge_g0_y = (right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412);
        float3 wedge_g6_xyz = (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(right_anti_dual_g0_w) * this.group1.xyz);
        float4 wedge_g9 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g0_w) * this.group5.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group4.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group4.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) - (right_anti_dual_g0_w * other.e321 * this.scalar), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g6_xyz * float3(other.e321)) + (float3(wedge_g0_y) * other.group1.xyz)), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e5) - (wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * wedge_g9.xyz, 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float wedge_g0_y = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        float3 wedge_g6_xyz = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((right_anti_dual_g1 * float3(this.e4)), 0.0) + float4((right_anti_dual_g0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]));
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_anti_dual_g1[3] * this.e1234) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        float3 wedge_g6_xyz = (float3(right_anti_dual_g1[3]) * this.group4.xyz) - (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g9 = (right_anti_dual_g1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((float3(right_anti_dual_g1[3]) * this.group7.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(other.group0.yzx, right_anti_dual_g1[0]));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) - (wedge_g6_xyz[0] * other.e23) - (wedge_g6_xyz[1] * other.e31) - (wedge_g6_xyz[2] * other.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) - (right_anti_dual_g1[3] * other.e3215 * this.e4), 
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
            float3(other.e3215 * -1.0) * wedge_g9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4);
        float4 wedge_g9 = right_anti_dual_g0 * float4(this.scalar);
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0 * other.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e5 */
            right_anti_dual_g0 * other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0 * other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0 * other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0 * other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0 * other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float wedge_g0_y = -(right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, right_anti_dual_g1_w);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = right_anti_dual_g1_xyz * float3(this.scalar);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g1_w) * this.group5.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) + float4((other.group2.zxy * this.group4.yzx), 0.0) - (this.group3.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0])) - float4((other.group2.yzx * this.group4.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g10 = -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412)) + float4((float3(wedge_g6[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group2.yzx), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            -(wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        float wedge_g0_y = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g3 = (right_anti_dual_g2 * float4(float3(this.scalar), this.e45)) + float4((float3(right_anti_dual_g2[3]) * this.group3.xyz), right_anti_dual_g1_w * this.scalar);
        float3 wedge_g4 = (float3(right_anti_dual_g2[3]) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float3 wedge_g5 = (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g2[3]) * this.group5.xyz);
        float4 wedge_g6 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(right_anti_dual_g2[3]) * this.group6.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g2[3]) * this.group7.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2[3]) * this.group8.xyz) + float3(float2(0.0), (right_anti_dual_g2[1] * this.e1) - (right_anti_dual_g2[0] * this.e2)) + float3((right_anti_dual_g2.zx * this.group1.yz), 0.0) - float3((right_anti_dual_g2.yz * this.group1.zx), 0.0);
        float4 wedge_g9 = (float4(right_anti_dual_g2[3]) * this.group9) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e25) - (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g1_w) * this.group5.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) + float4((this.group4.yzx * right_anti_dual_g2.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * right_anti_dual_g2.yzxx) - (this.group3.yzxx * float4(other.group0.zxy, right_anti_dual_g1_xyz[0]));
        float wedge_g10 = (right_anti_dual_g2[3] * this.e1234) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g2[3] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g2[3] * other.e12345) * this.group1) + float4(float3(0.0), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412)) + float4((float3(wedge_g6[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group2.yzx), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            (right_anti_dual_g2[3] * other.e12345 * this.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group1.xyz), 0.0) + float4((wedge_g9.zxy * other.group2.yzx), 0.0) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float wedge_g0_y = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
        float4 wedge_g6 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g7 = right_anti_dual_g0 * float3(this.scalar);
        float3 wedge_g8 = right_anti_dual_g2 * float3(this.scalar);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3)) + float4((right_anti_dual_g2 * float3(this.e4)), 0.0) + float4((right_anti_dual_g1.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g2[0]) * this.group1.yzxx);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g9[3]) * other.group0.xyz), 0.0) + float4((wedge_g9.zxy * other.group1.yzx), 0.0) - (float4(right_anti_dual_g0[0] * other.e15, right_anti_dual_g0[1] * other.e25, right_anti_dual_g0[2] * other.e35, right_anti_dual_g1[3] * other.e45) * this.group1) - (float4(right_anti_dual_g0[1] * other.e15, right_anti_dual_g0[0] * other.e25, right_anti_dual_g0[0] * other.e35, right_anti_dual_g0[0] * other.e45) * this.group1.yxxx) - (float4(right_anti_dual_g0[2] * other.e15, right_anti_dual_g0[2] * other.e25, right_anti_dual_g0[1] * other.e35, right_anti_dual_g0[1] * other.e45) * this.group1.zzyy) - (float4(right_anti_dual_g1[3] * other.e15, right_anti_dual_g1[3] * other.e25, right_anti_dual_g1[3] * other.e35, right_anti_dual_g0[2] * other.e45) * this.group1.wwwz) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e41)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        float4 wedge_g1 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g3 = (float4(this.e5, this.e5, this.e5, this.e4) * other.group3) + float4(float3(0.0), (other.e1234 * this.e5) * -1.0) + float4((float3(other.e3215) * this.group1.xyz), 0.0);
        float3 wedge_g4 = -(float3(other.e1234) * this.group1.xyz) - (float3(this.e4) * other.group3.xyz);
        float3 wedge_g5 = (other.group3.yzx * this.group1.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group3.zx * this.group1.yz), 0.0);
        float4 wedge_g6 = (right_anti_dual_g1 * float4(this.scalar)) + (other.group3.xyzx * float4(float3(this.e45), this.e23)) + (other.group3.wwwy * float4(this.group4.xyz, this.e31)) + float4(float3(0.0), other.e4125 * this.e12) + float4((float3(other.e1234) * this.group3.xyz), 0.0);
        float3 wedge_g7 = (right_anti_dual_g0 * float3(this.scalar)) + (float3(other.e1234) * this.group5.xyz) + (this.group4.zxy * other.group3.yzx) - (this.group4.yzx * other.group3.zxy);
        float3 wedge_g8 = (float3(other.e3215) * this.group5.xyz) + (other.group3.zxy * this.group3.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - (float3(this.scalar) * other.group2.xyz) - float3((other.group3.yz * this.group3.zx), 0.0);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.e423, this.e431, this.e412, this.e321)) + (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], other.e15) * this.group1.zxyx) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e4125 * this.e125)) + float4((other.group3.zxy * this.group6.yzx), 0.0) - (float4(this.e5) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (other.group3.yzxx * float4(this.group6.zxy, this.e235)) - float4((float3(other.e1234) * this.group8.xyz), other.e4315 * this.e315) - float4((float3(this.e4) * other.group2.xyz), 0.0) - float4((right_anti_dual_g1.zxy * this.group1.yzx), 0.0);
        float wedge_g10 = (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) + (other.e4235 * this.e423) + (other.e4315 * this.e431) + (other.e4125 * this.e412) - (other.e1234 * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (other.e1234 * other.e3215 * this.scalar) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (other.group3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), wedge_g4[2] * other.e4125) + float4((float3(wedge_g9[3]) * other.group0.xyz), wedge_g4[1] * other.e4315) + float4((wedge_g9.zxy * other.group1.yzx), 0.0) - (float4(wedge_g10) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g9[1] * other.e42) - float4((wedge_g5.yzx * other.group3.zxy), wedge_g9[2] * other.e43), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * float4(other.e15, other.e25, other.e35, other.e45)) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group3.zxy), 0.0) - (other.group3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group0.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group3.yzx) - (wedge_g7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g9.yzxw) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group3.yzxw), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(DualNum other) {
        float wedge_g0_y = -(other.e5 * this.e4) - (other.e12345 * this.e12345);
        float wedge_g10 = other.e12345 * this.e1234 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) - (other.e12345 * other.e12345 * this.scalar), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e5 */
            (wedge_g0_y * other.e5) - (other.e12345 * other.e12345 * this.e5), 
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
            float4((float3(other.e12345 * -1.0) * float3((float2(other.e12345) * this.group9.xy), other.e12345 * this.e4125)), -(other.e12345 * other.e12345 * this.e3215) - (other.e5 * other.e12345 * this.scalar)), 
            /* e1234 */
            wedge_g10 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(FlatPoint other) {
        float3 wedge_g9_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e45 * other.e45 * this.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e45 * this.e4 * -1.0) * other.group0, 
            /* e5 */
            (wedge_g9_xyz[0] * other.e15) + (wedge_g9_xyz[1] * other.e25) + (wedge_g9_xyz[2] * other.e35) + (other.e15 * other.e45 * this.e1) + (other.e25 * other.e45 * this.e2) + (other.e35 * other.e45 * this.e3) - (other.e45 * other.e45 * this.e5), 
            /* e15, e25, e35, e45 */
            (float4(other.e15 * this.e41, other.e25 * this.e42, other.e35 * this.e43, other.e15 * this.e41) * other.group0) + (float4(other.e25 * this.e42, other.e15 * this.e41, other.e15 * this.e41, other.e25 * this.e42) * other.group0) + (other.group0 * float4(float2(other.e35 * this.e43), other.e25 * this.e42, other.e35 * this.e43)) - (float4(other.e45 * this.e45) * other.group0), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Flector other) {
        float wedge_g0_y = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        float3 wedge_g1_xyz = float3(this.scalar * -1.0) * other.group1.xyz;
        float3 wedge_g3_xyz = (float3(other.e3215) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz);
        float3 wedge_g4 = float3(this.e4 * -1.0) * other.group1.xyz;
        float3 wedge_g5 = (other.group1.yzx * this.group1.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group1.zx * this.group1.yz), 0.0);
        float4 wedge_g6 = (other.group1.xyzx * float4(float3(this.e45), this.e23)) + (other.group1.wwwy * float4(this.group4.xyz, this.e31)) + float4(float3(0.0), (other.e45 * this.scalar) + (other.e4125 * this.e12));
        float3 wedge_g7 = (this.group4.zxy * other.group1.yzx) - (this.group4.yzx * other.group1.zxy);
        float3 wedge_g8 = (float3(other.e3215) * this.group5.xyz) + (other.group1.zxy * this.group3.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - (float3(this.scalar) * other.group0.xyz) - float3((other.group1.yz * this.group3.zx), 0.0);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.e423, this.e431, this.e412, this.e321)) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((other.group1.zxy * this.group6.yzx), other.e15 * this.e1) - (float4(this.e4, this.e4, this.e4, this.e5) * other.group0) - (other.group1.yzxx * float4(this.group6.zxy, this.e235));
        float wedge_g10 = (other.e45 * this.e4) + (other.e4235 * this.e423) + (other.e4315 * this.e431) + (other.e4125 * this.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * other.e4235) + (wedge_g1_xyz[1] * other.e4315) + (wedge_g1_xyz[2] * other.e4125) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (other.group1.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) - (float4(wedge_g10) * other.group0) - float4((wedge_g4 * float3(other.e3215)), 0.0) - float4((wedge_g5.yzx * other.group1.zxy), 0.0), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3_xyz[0] * other.e4235) - (wedge_g3_xyz[1] * other.e4315) - (wedge_g3_xyz[2] * other.e4125) - (other.e3215 * other.e3215 * this.e4), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group0) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * other.group1.yzx) - (wedge_g7.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) - (float3(wedge_g6[3]) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g9.yzx * other.group1.zxy) + float3(float2(0.0), wedge_g9[1] * other.e4235 * -1.0) - float3((wedge_g9.zx * other.group1.yz), 0.0)), wedge_g10 * other.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge_g10 * -1.0) * other.group1.xyz, 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Line other) {
        float wedge_g0_y = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float3 wedge_g5 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g6_xyz = float3(this.e4) * other.group1.xyz;
        float3 wedge_g7 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g8 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = float4(float3(0.0), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group4.yzx), 0.0) - float4((other.group1.yzx * this.group4.zxy), other.e415 * this.e15);
        float wedge_g10 = -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7.zxy * other.group1.yzx), 0.0) - (float4(other.e415) * float4(other.e415 * this.e1, other.e425 * this.e1, other.e435 * this.e1, wedge_g7[0])) - (float4(other.e425) * float4(other.e415 * this.e2, other.e425 * this.e2, other.e435 * this.e2, wedge_g7[1])) - (float4(other.e435) * float4(other.e415 * this.e3, other.e425 * this.e3, other.e435 * this.e3, wedge_g7[2])) - float4((wedge_g7.yzx * other.group1.zxy), 0.0), 
            /* e5 */
            -(wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group0.xyz), 0.0) + float4((other.group1.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e415)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        float wedge_g0_y = (right_anti_dual_g1_w * this.e4) + (right_anti_dual_g0[3] * this.e12345) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        float3 wedge_g4 = float3(right_anti_dual_g0[3]) * this.group4.xyz;
        float3 wedge_g5 = (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual_g0.xyz);
        float4 wedge_g6 = (float4(right_anti_dual_g0[3]) * this.group6) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e1) - (right_anti_dual_g0[1] * this.e2) - (right_anti_dual_g0[2] * this.e3)) + float4((right_anti_dual_g1_xyz * float3(this.e4)), 0.0);
        float3 wedge_g7 = (float3(right_anti_dual_g0[3]) * this.group7.xyz) + (float3(this.e4) * right_anti_dual_g0.xyz);
        float3 wedge_g8 = (float3(right_anti_dual_g0[3]) * this.group8.xyz) + (float3(this.e5) * right_anti_dual_g0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group1.yzx) - (right_anti_dual_g1_xyz.yzx * this.group1.zxy);
        float4 wedge_g9 = (right_anti_dual_g0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((float3(right_anti_dual_g0[3]) * this.group9.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group4.yzx), right_anti_dual_g1_w * this.scalar) - float4((right_anti_dual_g1_xyz.yzx * this.group4.zxy), right_anti_dual_g1_xyz[0] * this.e23);
        float wedge_g10 = (right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (right_anti_dual_g0[3] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(wedge_g6[3]), right_anti_dual_g0[3] * this.e4)) + float4(float3(0.0), -(wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + float4((float3(right_anti_dual_g0[3] * other.e12345) * this.group1.xyz), 0.0) + float4((wedge_g7.zxy * other.group1.yzx), 0.0) - float4((wedge_g7.yzx * other.group1.zxy), wedge_g7[0] * other.e415), 
            /* e5 */
            (wedge_g0_y * other.e5) + (right_anti_dual_g0[3] * other.e12345 * this.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (other.group0 * float4(float3(wedge_g9[3]), right_anti_dual_g0[3] * this.e45)) + float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((right_anti_dual_g1_xyz * float3(other.e12345 * this.scalar)), 0.0) + float4((float3(right_anti_dual_g0[3] * other.e12345) * this.group3.xyz), 0.0) + float4((wedge_g9.zxy * other.group1.yzx), 0.0) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e415)), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float wedge_g0_y = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group1);
        float wedge_g2 = (right_anti_dual_g0[0] * this.e5) + (other.e3215 * this.scalar);
        float4 wedge_g3 = (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group3) + (float4(other.e3215) * this.group1) - (right_anti_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(right_anti_dual_g0[0]) * this.group4.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group5.xyz) + float3(float2(0.0), (right_anti_dual_g1[1] * this.e1) - (right_anti_dual_g1[0] * this.e2)) + float3((right_anti_dual_g1.zx * this.group1.yz), 0.0) - float3((right_anti_dual_g1.yz * this.group1.zx), 0.0);
        float4 wedge_g6 = (right_anti_dual_g6 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group6) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group3.xyz), 0.0) + float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e4) * right_anti_dual_g3.xyz), 0.0) + float4((float3(this.e5) * other.group7.xyz), 0.0) - (right_anti_dual_g1.xyzx * float4(float3(this.e45), this.e23)) - (this.group1.xyzx * float4(float3(right_anti_dual_g3[3]), right_anti_dual_g5[0]));
        float3 wedge_g7 = (right_anti_dual_g5 * float3(this.e4)) + (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group7.xyz) + (float3(right_anti_dual_g1[3]) * this.group5.xyz) + (other.group7.yzx * this.group1.zxy) + (this.group4.yzx * right_anti_dual_g1.zxy) - (other.group7.zxy * this.group1.yzx) - (this.group4.zxy * right_anti_dual_g1.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (right_anti_dual_g8 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group8.xyz) + (float3(other.e3215) * this.group5.xyz) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e25) + (right_anti_dual_g3[1] * this.e1) - (right_anti_dual_g1[1] * this.e15) - (right_anti_dual_g3[0] * this.e2)) + float3((right_anti_dual_g1.yz * this.group3.zx), 0.0) + float3((right_anti_dual_g3.zx * this.group1.yz), 0.0) - float3((right_anti_dual_g1.zx * this.group3.yz), 0.0) - float3((right_anti_dual_g3.yz * this.group1.zx), 0.0);
        float4 wedge_g9 = (right_anti_dual_g9 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group9) + (float4(other.e3215) * float4(this.e423, this.e431, this.e412, this.e321)) + (right_anti_dual_g1.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), -(right_anti_dual_g5[2] * this.e35) - (right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3) - (right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12)) + float4((right_anti_dual_g5 * float3(this.e45)), right_anti_dual_g1[1] * this.e315) + float4((right_anti_dual_g8 * float3(this.e4)), right_anti_dual_g1[2] * this.e125) + float4((float3(right_anti_dual_g3[3]) * this.group5.xyz), 0.0) + float4((other.group7.yzx * this.group3.zxy), 0.0) + float4((this.group4.yzx * right_anti_dual_g3.zxy), 0.0) + float4((right_anti_dual_g6.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(right_anti_dual_g7, right_anti_dual_g6[3])) - (float4(right_anti_dual_g6[2], right_anti_dual_g6[0], right_anti_dual_g6[1], right_anti_dual_g8[0]) * this.group1.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * right_anti_dual_g3.yzxx) - (this.group3.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0])) - float4((float3(right_anti_dual_g1[3]) * this.group8.xyz), right_anti_dual_g5[1] * this.e25) - float4((right_anti_dual_g1.zxy * this.group6.yzx), 0.0);
        float wedge_g10 = (right_anti_dual_g10 * this.scalar) + (right_anti_dual_g0[0] * this.e1234) + (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3) + (right_anti_dual_g6[3] * this.e4) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[0] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Plane other) {
        float3 wedge_g1_xyz = float3(this.scalar * -1.0) * other.group0.xyz;
        float3 wedge_g3_xyz = (float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group1.xyz);
        float3 wedge_g4 = float3(this.e4 * -1.0) * other.group0.xyz;
        float3 wedge_g5 = (this.group1.zxy * other.group0.yzx) + float3(float2(0.0), (this.e1 * other.e4315) * -1.0) - float3((this.group1.yz * other.group0.zx), 0.0);
        float4 wedge_g6 = (other.group0.xyzx * float4(float3(this.e45), this.e23)) + (other.group0.wwwy * float4(this.group4.xyz, this.e31)) + float4(float3(0.0), this.e12 * other.e4125);
        float3 wedge_g7 = (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy);
        float3 wedge_g8 = (float3(other.e3215) * this.group5.xyz) + (this.group3.yzx * other.group0.zxy) + float3(float2(0.0), (this.e25 * other.e4235) * -1.0) - float3((this.group3.zx * other.group0.yz), 0.0);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.e423, this.e431, this.e412, this.e321)) + float4(float3(0.0), -(this.e315 * other.e4315) - (this.e125 * other.e4125)) + float4((this.group6.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group6.zxy, this.e235));
        float wedge_g10 = (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * other.e4235) + (wedge_g1_xyz[1] * other.e4315) + (wedge_g1_xyz[2] * other.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            (other.group0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) - float4((wedge_g4 * float3(other.e3215)), 0.0) - float4((wedge_g5.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(wedge_g3_xyz[0] * other.e4235) - (wedge_g3_xyz[1] * other.e4315) - (wedge_g3_xyz[2] * other.e4125) - (this.e4 * other.e3215 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * other.group0.yzx) - (wedge_g7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) - (float3(wedge_g6[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g9.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g9[1] * other.e4235 * -1.0) - float3((wedge_g9.zx * other.group0.yz), 0.0)), wedge_g10 * other.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge_g10 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e1234 * other.e3215) * other.group0) - (float4(this.e4235 * other.e4235, this.e4315 * other.e4315, this.e4125 * other.e4125, this.e4235 * other.e4235) * other.group0) - (float4(this.e4315 * other.e4315, this.e4235 * other.e4235, this.e4235 * other.e4235, this.e4315 * other.e4315) * other.group0) - (other.group0 * float4(float2(this.e4125 * other.e4125), this.e4315 * other.e4315, this.e4125 * other.e4125)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float wedge_g0_y = (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4);
        float4 wedge_g9 = right_anti_dual_g0 * float4(this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g1 * this.scalar * other.e5), 0.0), 
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215);
        float4 wedge_g1 = right_anti_dual_g0 * float4(this.scalar);
        float4 wedge_g3 = (float4(other.e3215) * this.group1) - (right_anti_dual_g0 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_anti_dual_g0.zxy * this.group1.yzx) + float3(float2(0.0), (right_anti_dual_g0[0] * this.e2) * -1.0) - float3((right_anti_dual_g0.yz * this.group1.zx), 0.0);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) + float4((float3(other.e3215) * this.group4.xyz), 0.0) - (right_anti_dual_g0.xyzx * float4(float3(this.e45), this.e23));
        float3 wedge_g7 = (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g0.zxy) - (this.group4.zxy * right_anti_dual_g0.yzx);
        float3 wedge_g8 = (float3(other.e3215) * this.group5.xyz) + (right_anti_dual_g0.yzx * this.group3.zxy) + float3(float2(0.0), (right_anti_dual_g0[1] * this.e15) * -1.0) - float3((right_anti_dual_g0.zx * this.group3.yz), 0.0);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.e423, this.e431, this.e412, this.e321)) + (right_anti_dual_g0.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e315) + (right_anti_dual_g0[2] * this.e125)) - float4((float3(right_anti_dual_g0[3]) * this.group8.xyz), 0.0) - float4((right_anti_dual_g0.zxy * this.group6.yzx), 0.0);
        float wedge_g10 = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (other.group0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) - float4((wedge_g4 * float3(other.e3215)), 0.0) - float4((wedge_g5.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group0.yzx) - (wedge_g7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) - (float3(wedge_g6[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g9.yzxw * float4(other.group0.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group0.yzxw), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float wedge_g0_y = (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g0[3] * this.e12345) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125);
        float4 wedge_g3 = (float4(right_anti_dual_g0[3]) * this.group3) + (float4(this.scalar) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w));
        float3 wedge_g4 = (float3(right_anti_dual_g0[3]) * this.group4.xyz) + (float3(this.scalar) * right_anti_dual_g0.xyz);
        float3 wedge_g5 = (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0[3]) * this.group5.xyz);
        float4 wedge_g6 = (right_anti_dual_g0 * float4(this.e5, this.e5, this.e5, this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(right_anti_dual_g0[3]) * this.group6.xyz), 0.0) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g7 = (right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0[3]) * this.group7.xyz) + float3(float2(0.0), (right_anti_dual_g0[0] * this.e2) - (right_anti_dual_g0[1] * this.e1)) + float3((right_anti_dual_g0.yz * this.group1.zx), 0.0) - float3((right_anti_dual_g0.zx * this.group1.yz), 0.0);
        float3 wedge_g8 = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group8.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy);
        float4 wedge_g9 = (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[3]) * this.group9) + float4(float3(0.0), -(right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12)) + float4((right_anti_dual_g1_xyz * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g1_w) * this.group5.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group4.yzx), 0.0) + float4((right_anti_dual_g0.yzx * this.group3.zxy), 0.0) - (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g1_xyz[0]) * this.group3.yzxx) - float4((right_anti_dual_g2_xyz.yzx * this.group4.zxy), right_anti_dual_g1_xyz[1] * this.e25);
        float wedge_g10 = (right_anti_dual_g2_w * this.scalar) + (right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[3] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group3) + (float4(right_anti_dual_g0[3] * other.e12345) * this.group1) + float4(float3(0.0), -(wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412)) + float4((float3(wedge_g6[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group2.yzx), 0.0) + float4((wedge_g8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g7[0] * other.e415), 
            /* e5 */
            (wedge_g0_y * other.e5) + (right_anti_dual_g0[3] * this.e5 * other.e12345) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group1.xyz), 0.0) + float4((wedge_g9.zxy * other.group2.yzx), 0.0) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group1.xyz) + float3(float2(0.0), (wedge_g9[0] * other.e431) - (wedge_g9[1] * other.e423)) + float3((wedge_g9.yz * other.group0.zx), 0.0) - float3((wedge_g9.zx * other.group0.yz), 0.0), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float wedge_g0_y = (right_anti_dual_g2[3] * this.e1234) + (right_anti_dual_g3[0] * this.e4235) + (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        float4 wedge_g1 = right_anti_dual_g3 * float4(this.scalar);
        float4 wedge_g3 = (float4(right_anti_dual_g2[3]) * this.group1) - (right_anti_dual_g3 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_anti_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), (right_anti_dual_g3[0] * this.e2) * -1.0) - float3((right_anti_dual_g3.yz * this.group1.zx), 0.0);
        float4 wedge_g6 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12)) + float4((float3(right_anti_dual_g2[3]) * this.group4.xyz), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group3.xyz), 0.0) - (right_anti_dual_g3.xyzx * float4(float3(this.e45), this.e23));
        float3 wedge_g7 = (float3(right_anti_dual_g3[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g3.zxy) - (float3(this.scalar) * other.group0.xyz) - (this.group4.zxy * right_anti_dual_g3.yzx);
        float3 wedge_g8 = (float3(right_anti_dual_g2[3]) * this.group5.xyz) + (float3(this.scalar) * right_anti_dual_g2.xyz) + float3(float2(0.0), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + float3((right_anti_dual_g3.yz * this.group3.zx), 0.0) - float3((right_anti_dual_g3.zx * this.group3.yz), 0.0);
        float4 wedge_g9 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + (right_anti_dual_g3.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e125) - (right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3)) + float4((float3(right_anti_dual_g2[3]) * this.group7.xyz), right_anti_dual_g3[1] * this.e315) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((right_anti_dual_g1.yzx * this.group1.zxy), 0.0) - (right_anti_dual_g1.zxyw * float4(this.group1.yzx, this.e5)) - float4((float3(right_anti_dual_g3[3]) * this.group8.xyz), right_anti_dual_g2[0] * this.e1) - float4((right_anti_dual_g3.zxy * this.group6.yzx), 0.0);
        float wedge_g10 = (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (right_anti_dual_g2[3] * this.scalar * other.e1234) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (other.group3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) + float4((float3(wedge_g9[3]) * other.group0.xyz), 0.0) + float4((wedge_g9.zxy * other.group1.yzx), 0.0) - (float4(wedge_g10) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g9[1] * other.e42) - float4((wedge_g5.yzx * other.group3.zxy), wedge_g9[2] * other.e43), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * float4(other.e15, other.e25, other.e35, other.e45)) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group3.zxy), 0.0) - (other.group3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group0.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group3.yzx) - (wedge_g7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g9.yzxw) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group3.yzxw), 
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
public ProjectOrthogonallyOntoInfixPartial<Plane> operator /(Plane lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane project_orthogonally_onto(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion project_orthogonally_onto(CircleRotor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float wedge_g0 = (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
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
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane project_orthogonally_onto(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector project_orthogonally_onto(Flector other) {
        float wedge_g0 = -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e1234 * this.e3215);
        float4 wedge_g9 = float4(other.e12345 * -1.0) * this.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group9.yzx), 0.0), 
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
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            -(float4(other.e4235 * this.e4235, other.e4315 * this.e4315, other.e4125 * this.e4125, other.e4235 * this.e4235) * other.group0) - (float4(other.e4315 * this.e4315, other.e4235 * this.e4235, other.e4235 * this.e4235, other.e4315 * this.e4315) * other.group0) - (other.group0 * float4(float2(other.e4125 * this.e4125), other.e4315 * this.e4315, other.e4125 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group0, 
            /* e1234 */
            wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)) - float4((wedge_g0.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float wedge_g0 = (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234);
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
public ProjectOrthogonallyOntoInfixPartial<RoundPoint> operator /(RoundPoint lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = (float4(this.e5, this.e5, this.e5, other.e25 * this.e2) * float4(other.group0.xyz, 1.0)) + (this.group0.yzxx * float4(other.group1.zxy, other.e15)) + float4(float3(0.0), other.e35 * this.e3) - (other.group1.yzxw * float4(this.group0.zxy, this.e5)) - float4((float3(this.e4) * other.group2.xyz), 0.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group0.xyz), 0.0) - (float4(other.e45 * this.e4) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float wedge_g1_x = (right_anti_dual_g2_xyz[0] * this.e4) + (other.e423 * this.e5) + (other.e321 * this.e1);
        float wedge_g1_y = (right_anti_dual_g2_xyz[1] * this.e4) + (other.e431 * this.e5) + (other.e321 * this.e2);
        float wedge_g1_z = (right_anti_dual_g2_xyz[2] * this.e4) + (other.e412 * this.e5) + (other.e321 * this.e3);
        float4 wedge_g2 = (float4(this.e5, this.e5, this.e5, right_anti_dual_g3_xyz[1] * this.e2) * float4(right_anti_dual_g1_xyz, 1.0)) + (this.group0.yzxx * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e3) - (other.e4 * this.e5)) - (this.group0.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e5));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g1_x * other.e423) - (wedge_g1_y * other.e431) - (wedge_g1_z * other.e412) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1_y * other.e315) - (wedge_g1_z * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(other.e321) * float3(wedge_g1_x, wedge_g1_y, wedge_g1_z)), 0.0) + float4((float3(-(right_anti_dual_g1_xyz[0] * this.e1) - (right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1_x))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public DualNum project_orthogonally_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * other.e3215 * this.e4 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiFlatPoint other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e321 * -1.0) * this.group0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(other.e321)), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (float4(this.e4) * float4(other.group0.xyz, other.e5 * -1.0)) + float4(float3(0.0), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3)) - float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((right_anti_dual_g1 * float3(this.e4)), 0.0) + float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e31, other.e12, other.e23, other.e15) * wedge_g0.zxyx) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((other.group0.zxy * wedge_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 wedge_g1 = (this.group0.yzxx * float4(other.group0.zxy, other.e15)) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3)) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.yzx * this.group0.zxy), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) - (other.e3215 * other.e3215 * this.e4)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e5 * this.e4 * -1.0) * other.group0) + (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e5 */
            right_anti_dual_g0 * other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float wedge_g1_x = (other.e423 * this.e5) + (other.e321 * this.e1) + (other.e235 * this.e4);
        float wedge_g1_y = (other.e431 * this.e5) + (other.e321 * this.e2) + (other.e315 * this.e4);
        float wedge_g1_z = (other.e412 * this.e5) + (other.e321 * this.e3) + (other.e125 * this.e4);
        float3 wedge_g2 = (right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (wedge_g1_z * other.e412) * -1.0) + float4((float3(other.e321) * float3(wedge_g1_x, wedge_g1_y, wedge_g1_z)), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (other.group1.xyzx * float4(float3(right_anti_dual_g1_xyz[0] * this.e1), wedge_g0[0])) - (other.group1.xyzy * float4(float3(right_anti_dual_g1_xyz[1] * this.e2), wedge_g0[1])) - (other.group1.xyzz * float4(float3(right_anti_dual_g1_xyz[2] * this.e3), wedge_g0[2])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1_x * other.e423) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g1_y * other.e431), 
            /* e5 */
            -(wedge_g1_x * other.e235) - (wedge_g1_y * other.e315) - (wedge_g1_z * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((right_anti_dual_g2_xyz[0] * this.e4) + (other.e423 * this.e5) + (other.e321 * this.e1), (right_anti_dual_g2_xyz[1] * this.e4) + (other.e431 * this.e5) + (other.e321 * this.e2), (right_anti_dual_g2_xyz[2] * this.e4) + (other.e412 * this.e5) + (other.e321 * this.e3), -(right_anti_dual_g1_xyz[0] * this.e1) - (right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3));
        float3 wedge_g2_xyz = (right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.e12345), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.e12345), 
            /* e235, e315, e125, e4 */
            float4((wedge_g2_xyz * float3(other.e12345)), (right_anti_dual_g2_w * other.e12345 * this.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g2_w * other.e12345) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), -(wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g2_xyz[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3) - (other.e45 * this.e5)) + float4((right_anti_dual_g2 * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, right_anti_dual_g2[0]));
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group0.xyz), 0.0) - (float4(other.e45 * this.e4) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = -(float3(other.e1234) * this.group0.xyz) - (float3(this.e4) * other.group3.xyz);
        float4 wedge_g1 = (other.group3.yzxw * this.group0.zxyw) + float4(float3(0.0), (other.e1234 * this.e5) * -1.0) - float4((other.group3.zxy * this.group0.yzx), 0.0);
        float4 wedge_g2 = (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3)) + float4((float3(this.e5) * other.group3.xyz), 0.0);
        float4 wedge_g3 = (float4(this.e5, this.e5, this.e5, other.e25 * this.e2) * float4(other.group0.xyz, 1.0)) + (this.group0.yzxx * float4(other.group1.zxy, other.e15)) + float4(float3(0.0), other.e35 * this.e3) - (other.group1.yzxw * float4(this.group0.zxy, this.e5)) - float4((float3(this.e4) * other.group2.xyz), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(DualNum other) {
        float wedge_g0_w = other.e5 * this.e4 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), wedge_g0_w * other.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (wedge_g0_w * other.e5) - (other.e12345 * other.e12345 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e45 * this.e4 * -1.0) * other.group0, 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) + (other.e15 * other.e45 * this.e1) + (other.e25 * other.e45 * this.e2) + (other.e35 * other.e45 * this.e3) - (other.e45 * other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group1.zx * this.group0.yz), 0.0);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz);
        float wedge_g2_w = other.e45 * this.e4;
        float3 wedge_g3_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        float wedge_g3_w = (other.e15 * this.e1) + (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2_w * -1.0) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3_xyz.yzx * other.group1.zxy) - (wedge_g3_xyz.zxy * other.group1.yzx)), wedge_g2_w * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g3_xyz * float3(other.e3215)), wedge_g2_w * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3_xyz[0] * other.e15) + (wedge_g3_xyz[1] * other.e25) + (wedge_g3_xyz[2] * other.e35) - (wedge_g2_xyz[1] * other.e4315) - (wedge_g2_xyz[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3_w * other.e45) - (float4(other.e3215) * float4(wedge_g0, other.e3215 * this.e4)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0])) - float4((float3(wedge_g2_w) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float3 wedge_g2 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (float4(other.e415) * float4(other.e415 * this.e1, other.e425 * this.e1, other.e435 * this.e1, wedge_g0[0])) - (float4(other.e425) * float4(other.e415 * this.e2, other.e425 * this.e2, other.e435 * this.e2, wedge_g0[1])) - (float4(other.e435) * float4(other.e415 * this.e3, other.e425 * this.e3, other.e435 * this.e3, wedge_g0[2])) - float4((wedge_g0.yzx * other.group1.zxy), 0.0), 
            /* e5 */
            -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(this.e4) * float4(right_anti_dual_g0_xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g1_xyz = right_anti_dual_g1_xyz * float3(this.e4);
        float wedge_g1_w = -(right_anti_dual_g0_xyz[0] * this.e1) - (right_anti_dual_g0_xyz[1] * this.e2) - (right_anti_dual_g0_xyz[2] * this.e3);
        float4 wedge_g2 = float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5);
        float4 wedge_g3 = float4(right_anti_dual_g0_w) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1_xyz * float3(other.e12345)) + (float3(wedge_g0[3]) * other.group0.xyz)), wedge_g1_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(wedge_g1_w), wedge_g3[3])) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(other.e12345) * wedge_g3.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        float4 wedge_g1 = float4(right_anti_dual_g0[0]) * this.group0;
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * this.group0) - (right_anti_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group0.xyz);
        float3 wedge_g5 = (right_anti_dual_g1.zxy * this.group0.yzx) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e2) * -1.0) - float3((right_anti_dual_g1.yz * this.group0.zx), 0.0);
        float4 wedge_g6 = float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) + float4((float3(this.e4) * other.group8.xyz), 0.0) + float4((float3(this.e5) * other.group7.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), right_anti_dual_g5[0]));
        float3 wedge_g7 = (right_anti_dual_g5 * float3(this.e4)) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy);
        float4 wedge_g9 = float4(float3(0.0), -(right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3) - (other.e45 * this.e5)) + float4((right_anti_dual_g8 * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group4.xyz), 0.0) + float4((right_anti_dual_g6_xyz.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0]));
        float wedge_g10 = (other.e45 * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.yzx * this.group0.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0);
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g0 * float3(other.e3215)), 0.0) - float4((wedge_g1_xyz.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125) - (other.e3215 * other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float3 wedge_g0 = (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(right_anti_dual_g0_xyz.zxy, other.e3215)) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), this.e5 * other.e1234);
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e5));
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e1234) * float4(wedge_g2, wedge_g1[3])) + (float4(wedge_g1[2], wedge_g1[0], wedge_g1[1], wedge_g0[0]) * other.group0.yzxx) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g0 * float3(other.e3215)), 0.0) - float4((wedge_g1.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125) - (wedge_g1[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float4 wedge_g0 = (this.group0.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g3_xyz[0])) + (this.group0.wwwy * float4(right_anti_dual_g1_xyz, right_anti_dual_g3_xyz[1])) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e3) - (this.e5 * other.e4)) - (this.group0.yzxw * float4(right_anti_dual_g0_xyz.zxy, other.e5));
        float4 wedge_g1 = float4((right_anti_dual_g0_xyz[0] * this.e5) + (right_anti_dual_g2_xyz[0] * this.e4) + (this.e1 * other.e321), (right_anti_dual_g0_xyz[1] * this.e5) + (right_anti_dual_g2_xyz[1] * this.e4) + (this.e2 * other.e321), (right_anti_dual_g0_xyz[2] * this.e5) + (right_anti_dual_g2_xyz[2] * this.e4) + (this.e3 * other.e321), -(right_anti_dual_g1_xyz[0] * this.e1) - (right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3));
        float4 wedge_g2 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group3) + (float4(right_anti_dual_g0_w * other.e12345) * this.group0) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float3 wedge_g0 = (right_anti_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(right_anti_dual_g3_xyz.zxy, other.e3215)) - float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), this.e5 * other.e1234);
        float4 wedge_g2 = (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(this.e2 * other.e42) - (this.e3 * other.e43)) - (float4(this.e5, this.e5, this.e5, this.e1 * other.e41) * float4(right_anti_dual_g3_xyz, 1.0));
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e2) - (right_anti_dual_g2_xyz[2] * this.e3) - (this.e5 * other.e45)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, right_anti_dual_g2_xyz[0]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Scalar> operator /(Scalar lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2[3] * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0]) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.scalar * (other.e321 * -1.0) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g1.xyz), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (other.e321 * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Circle other) {
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
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(CircleRotor other) {
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Dipole other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g2 = float3(this.scalar * -1.0) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group3.yzx) - (wedge_g0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * float3(other.e3215)), 0.0) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g2[3])) + (float4(wedge_g2[1], wedge_g2[2], wedge_g2[0], wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - (wedge_g2.zxyx * float4(other.group3.yzx, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiDualNum project_orthogonally_onto(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * this.scalar * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3] * -1.0) * other.group1.xyz), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_orthogonally_onto(Motor other) {
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 wedge_g0 = float2(this.scalar) * other.group0.yx * float2(-1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float wedge_g2 = other.e3215 * this.scalar;
        float4 wedge_g3 = float4(this.scalar) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group6.xyz;
        float4 wedge_g6 = float4(this.scalar) * float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 wedge_g7 = float3(this.scalar * -1.0) * other.group4.xyz;
        float3 wedge_g8 = float3(this.scalar * -1.0) * other.group3.xyz;
        float4 wedge_g9 = float4(this.scalar) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g10 = other.e4 * this.scalar * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g0[0] * other.e12345) + (wedge_g0[1] * other.scalar) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0[1] * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0[1]) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g2 * other.e12345) + (wedge_g0[1] * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group6.xyz) + (float3(wedge_g0[1]) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g0[1]) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (other.e4 * other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2[3]) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 wedge_g3 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], wedge_g2[3])) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + float3(float2(0.0), (wedge_g2[0] * other.e4315) - (wedge_g2[1] * other.e4235)) + float3((wedge_g2.yz * other.group3.zx), 0.0) - float3((wedge_g2.zx * other.group3.yz), 0.0)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Sphere> operator /(Sphere lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public AntiDualNum project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e1234) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
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
public Sphere operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e1234 */
            right_anti_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion project_orthogonally_onto(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(right_anti_dual_g2_w) * this.group0;
        float wedge_g1 = right_anti_dual_g2_w * this.e1234;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (wedge_g0.zxy * other.group2.yzx) + float3(float2(0.0), wedge_g0[0] * other.e315 * -1.0) - float3((wedge_g0.yz * other.group2.zx), 0.0)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float wedge_g0 = (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        float wedge_g1 = other.e12345 * this.e1234 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), wedge_g1 * other.e5), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Flector project_orthogonally_onto(Flector other) {
        float wedge_g0 = (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(Motor other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(right_anti_dual_g0_w) * this.group0;
        float wedge_g1 = right_anti_dual_g0_w * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g1) * float4(other.group0.xyz, other.e5), 
            /* e23, e31, e12, e45 */
            float4((float2(wedge_g1) * other.group1.xy), wedge_g1 * other.e125, -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (wedge_g0.zxy * other.group1.yzx) + float3(float2(0.0), wedge_g0[0] * other.e315 * -1.0) - float3((wedge_g0.yz * other.group1.zx), 0.0)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215);
        float4 wedge_g9 = float4(right_anti_dual_g0[0]) * this.group0;
        float wedge_g10 = right_anti_dual_g0[0] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), -(wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215 * this.e1234) * other.group0) - (float4(other.e4235 * this.e4235, other.e4315 * this.e4315, other.e4125 * this.e4125, other.e4235 * this.e4235) * other.group0) - (float4(other.e4315 * this.e4315, other.e4235 * this.e4235, other.e4235 * this.e4235, other.e4315 * this.e4315) * other.group0) - (other.group0 * float4(float2(other.e4125 * this.e4125), other.e4315 * this.e4315, other.e4125 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere project_orthogonally_onto(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group0, 
            /* e1234 */
            wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 wedge_g0 = float4(right_anti_dual_g0_w) * this.group0;
        float wedge_g1 = right_anti_dual_g0_w * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g1) * float4(other.group1.xyz, other.e5)) + (wedge_g0.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)) - float4((wedge_g0.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(wedge_g1) * other.group2.xyz), 0.0) + float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (wedge_g0.zxy * other.group2.yzx) + float3(float2(0.0), wedge_g0[0] * other.e315 * -1.0) - float3((wedge_g0.yz * other.group2.zx), 0.0)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float wedge_g0 = (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215);
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
public ProjectOrthogonallyOntoInfixPartial<VersorEven> operator /(VersorEven lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(AntiCircleRotor other) {
        float4 wedge_g0 = (this.group3.yzxx * float4(other.group1.zxy, other.e15)) + float4(float3(0.0), other.e35 * this.e3) + float4((float3(this.e5) * other.group0.xyz), other.e25 * this.e2) - (float4(this.e3, this.e1, this.e2, this.e5) * other.group1.yzxw) - float4((float3(this.e4) * other.group2.xyz), 0.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group3.xyz), 0.0) - (float4(other.e45 * this.e4) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = (this.group3.yzxx * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5)) + float4((right_anti_dual_g1_xyz * float3(this.e5)), right_anti_dual_g3_xyz[1] * this.e2) - (this.group3.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e5));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g2[3]) * other.group3) + float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * other.e3215 * -1.0) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) + float4((right_anti_dual_g0_xyz * float3(this.e4)), 0.0) - (float4(other.e321 * -1.0) * float4(this.group3.xyz, this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * wedge_g0.xyz), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (float4(this.e4) * float4(right_anti_dual_g0_xyz, other.e5 * -1.0)) + float4(float3(0.0), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) - (float4(other.e321 * -1.0) * float4(this.group3.xyz, this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiPlane project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((right_anti_dual_g1 * float3(this.e4)), 0.0) + float4((right_anti_dual_g0.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e31, other.e12, other.e23, other.e15) * wedge_g0.zxyx) + float4(float3(0.0), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) - float4((other.group0.zxy * wedge_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e2) - (right_anti_dual_g1[2] * this.e3)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(other.group0.yzx, right_anti_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215 * -1.0) * wedge_g1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) - (right_anti_dual_g1[3] * other.e3215 * this.e4)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiPlane project_orthogonally_onto(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e5 * this.e4 * -1.0) * other.group0) + (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0 * other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 wedge_g0 = (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((float3(this.e4) * other.group2.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) + float4((right_anti_dual_g1_xyz * float3(this.e5)), 0.0) + float4((other.group2.zxy * this.group3.yzx), 0.0) - float4((other.group2.yzx * this.group3.zxy), right_anti_dual_g1_w * this.e321);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * wedge_g2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * wedge_g2.zxyx) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        float4 wedge_g0 = (float4(right_anti_dual_g2[3]) * this.group0) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((right_anti_dual_g1_xyz * float3(this.e4)), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - float4((other.group0.zxy * this.group3.yzx), right_anti_dual_g1_w * this.e321);
        float4 wedge_g1 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((float3(right_anti_dual_g2[3]) * this.group1.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float3 wedge_g2_xyz = (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2[3]) * this.group2.xyz) + float3(float2(0.0), (right_anti_dual_g2[1] * this.e1) - (right_anti_dual_g2[0] * this.e2)) + float3((right_anti_dual_g2.zx * this.group3.yz), 0.0) - float3((right_anti_dual_g2.yz * this.group3.zx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(wedge_g0[3]), right_anti_dual_g2[3] * this.e5)) + float4(float3(0.0), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)) + float4((wedge_g2_xyz * float3(other.e12345)), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g2[3] * other.e12345) * this.group3) + float4(float3(0.0), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415)) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4(float3(0.0), -(right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3) - (other.e45 * this.e5)) + float4((right_anti_dual_g2 * float3(this.e4)), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(other.group1.yzx, right_anti_dual_g2[0]));
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e41) * float4(wedge_g0[3], other.e25 * this.e1, other.e35 * this.e1, other.e45 * this.e1)) + (float4(other.e42) * float4(other.e15 * this.e2, wedge_g0[3], other.e35 * this.e2, other.e45 * this.e2)) + (float4(other.e43) * float4(other.e15 * this.e3, other.e25 * this.e3, wedge_g0[3], other.e45 * this.e3)) + float4(float3(0.0), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) + float4((other.group0.xyz * other.group2.xyz * this.group3.xyz), 0.0) - (float4(other.e45 * this.e4) * float4(other.e15, other.e25, other.e35, other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(DipoleInversion other) {
        float3 wedge_g0 = -(float3(other.e1234) * this.group3.xyz) - (float3(this.e4) * other.group3.xyz);
        float4 wedge_g1 = (other.group3.yzxw * this.group3.zxyw) + float4(float3(0.0), (other.e1234 * this.e5) * -1.0) - float4((other.group3.zxy * this.group3.yzx), 0.0);
        float4 wedge_g2 = (this.group3 * float4(float3(other.e3215), other.e45)) + (other.group3.xyzx * float4(float3(this.e5), this.e423)) + float4(float3(0.0), (other.e4315 * this.e431) + (other.e4125 * this.e412) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e1234 * this.e321));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (this.group3.yzxx * float4(other.group1.zxy, other.e15)) + float4(float3(0.0), (other.e35 * this.e3) - (other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((float3(this.e5) * other.group0.xyz), other.e25 * this.e2) + float4((other.group3.zxy * this.group1.yzx), 0.0) - (this.group2 * float4(float3(other.e1234), other.e45)) - (other.group3.yzxx * float4(this.group1.zxy, this.e235)) - float4((float3(this.e4) * other.group2.xyz), 0.0) - float4((other.group1.yzx * this.group3.zxy), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(DualNum other) {
        float4 wedge_g0 = float4((float3(other.e12345 * -1.0) * this.group0.xyz), -(other.e5 * this.e4) - (other.e12345 * this.e12345));
        float4 wedge_g2 = float4(other.e12345 * -1.0) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((float2(other.e12345) * wedge_g2.xy), wedge_g2[2] * other.e12345, (wedge_g0[3] * other.e5) + (wedge_g2[3] * other.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(FlatPoint other) {
        float3 wedge_g0_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e45 * this.e4 * -1.0) * other.group0, 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) + (other.e15 * other.e45 * this.e1) + (other.e25 * other.e45 * this.e2) + (other.e35 * other.e45 * this.e3) - (other.e45 * other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Flector other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group1.xyz;
        float3 wedge_g1_xyz = (other.group1.yzx * this.group3.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group1.zx * this.group3.yz), 0.0);
        float4 wedge_g2 = (this.group3 * float4(float3(other.e3215), other.e45)) + (other.group1.xyzx * float4(float3(this.e5), this.e423)) + float4(float3(0.0), (other.e4315 * this.e431) + (other.e4125 * this.e412));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((other.group1.zxy * this.group1.yzx), other.e15 * this.e1) - (other.group0 * float4(float3(this.e4), this.e5)) - (other.group1.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3] * -1.0) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group1.zxy) + float3(float2(0.0), wedge_g3[1] * other.e4235 * -1.0) - float3((wedge_g3.zx * other.group1.yz), 0.0)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3[1] * other.e25) + (wedge_g3[2] * other.e35) + (wedge_g3[3] * other.e45) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, other.e3215 * this.e4)) - (wedge_g2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float4 wedge_g2 = float4(float3(0.0), -(other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group3.yzx), 0.0) - float4((other.group1.yzx * this.group3.zxy), other.e415 * this.e415);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + float4((float3(-(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) * other.group0.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g1_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = (right_anti_dual_g0 * float4(float3(this.e4), this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), other.e5 * this.e4 * -1.0);
        float4 wedge_g1 = (float4(right_anti_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e1) - (right_anti_dual_g0[1] * this.e2) - (right_anti_dual_g0[2] * this.e3)) + float4((right_anti_dual_g1_xyz * float3(this.e4)), 0.0);
        float4 wedge_g2 = float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_anti_dual_g0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group3.yzx) - (right_anti_dual_g1_xyz.yzx * this.group3.zxy)), right_anti_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_anti_dual_g0[3]) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(wedge_g1[3]), wedge_g3[3])) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + float4((float3(other.e12345) * wedge_g3.xyz), 0.0) + float4((wedge_g0.zxy * other.group1.yzx), 0.0) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 wedge_g1 = float4(right_anti_dual_g0[0]) * this.group3;
        float wedge_g2 = right_anti_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * this.group3) - (right_anti_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group3.xyz);
        float3 wedge_g5 = (right_anti_dual_g1.zxy * this.group3.yzx) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e2) * -1.0) - float3((right_anti_dual_g1.yz * this.group3.zx), 0.0);
        float4 wedge_g6 = (float4(right_anti_dual_g0[0]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) + float4((float3(this.e5) * other.group7.xyz), 0.0) + float4((float3(this.e4) * other.group8.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g5[0]));
        float3 wedge_g7 = (right_anti_dual_g5 * float3(this.e4)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx);
        float3 wedge_g8 = (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group2.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g8[2] * this.e3) * -1.0) + float4((right_anti_dual_g8 * float3(this.e4)), right_anti_dual_g1[2] * this.e125) + float4((right_anti_dual_g6_xyz.yzx * this.group3.zxy), right_anti_dual_g1[1] * this.e315) - (this.group2 * float4(float3(right_anti_dual_g1[3]), other.e45)) - (this.group3.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0])) - float4((right_anti_dual_g7 * float3(this.e5)), right_anti_dual_g8[1] * this.e2) - float4((right_anti_dual_g1.zxy * this.group1.yzx), 0.0);
        float wedge_g10 = (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3) + (other.e45 * this.e4) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412) - (right_anti_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Plane other) {
        float3 wedge_g0 = float3(this.e4 * -1.0) * other.group0.xyz;
        float3 wedge_g1_xyz = (other.group0.yzx * this.group3.zxy) + float3(float2(0.0), (other.e4315 * this.e1) * -1.0) - float3((other.group0.zx * this.group3.yz), 0.0);
        float4 wedge_g2 = (other.group0.xyzx * float4(float3(this.e5), this.e423)) + (other.group0.wwwy * float4(this.group3.xyz, this.e431)) + float4(float3(0.0), other.e4125 * this.e412);
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + float4(float3(0.0), -(other.e4315 * this.e315) - (other.e4125 * this.e125)) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3] * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group0.zxy) + float3(float2(0.0), wedge_g3[1] * other.e4235 * -1.0) - float3((wedge_g3.zx * other.group0.yz), 0.0)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group0.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, other.e3215 * this.e4)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_orthogonally_onto(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float wedge_g0 = (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float3 wedge_g0 = (float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group3.xyz);
        float4 wedge_g1 = (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], other.e3215) * this.group3.yzxw) - (float4(this.e3, this.e1, this.e2, this.e5) * right_anti_dual_g0.yzxw);
        float4 wedge_g2 = float4(float3(0.0), -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (right_anti_dual_g0 * float4(float3(this.e5), this.e321));
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e315) + (right_anti_dual_g0[2] * this.e125)) - float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group0.zxy, other.e1234)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group0.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group0.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234)) - float4((float3(other.e3215) * wedge_g3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) + float4((float3(other.e1234) * wedge_g2.xyz), 0.0) + float4((wedge_g1.zxy * other.group0.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float4 wedge_g0 = (float4(right_anti_dual_g0[3]) * this.group0) + (float4(this.e4) * float4(right_anti_dual_g1_xyz, other.e5 * -1.0)) + (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g3_xyz[0]) * this.group3.zxyx) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (other.e4 * this.e5)) - (right_anti_dual_g0.zxyx * float4(this.group3.yzx, this.e235));
        float4 wedge_g1 = (right_anti_dual_g0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) + float4((right_anti_dual_g2_xyz * float3(this.e4)), 0.0) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) - (this.group3.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0]));
        float4 wedge_g2 = float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group3) + (float4(right_anti_dual_g0[3] * other.e12345) * this.group3) + float4(float3(0.0), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - (float4(wedge_g2[2], wedge_g2[0], wedge_g2[1], wedge_g1[0]) * other.group0.yzxx) - (wedge_g0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float3 wedge_g0 = (float3(this.e4) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * this.group3.xyz);
        float4 wedge_g1 = (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2[3]) * this.group3.yzxw) - (float4(this.e3, this.e1, this.e2, this.e5) * right_anti_dual_g3.yzxw);
        float4 wedge_g2 = (this.group3 * float4(float3(right_anti_dual_g2[3]), other.e45)) + float4(float3(0.0), -(right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) - (right_anti_dual_g3 * float4(float3(this.e5), this.e321));
        float4 wedge_g3 = (right_anti_dual_g2 * float4(float3(this.e4), this.e321)) + (right_anti_dual_g3.yzxx * float4(this.group1.zxy, this.e235)) + (this.group2.wwwy * float4(other.group0.xyz, right_anti_dual_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e125) - (right_anti_dual_g2[1] * this.e2) - (right_anti_dual_g2[2] * this.e3)) + float4((float3(right_anti_dual_g2[3]) * this.group0.xyz), 0.0) + float4((this.group3.yzx * other.group1.zxy), 0.0) - (this.group2 * float4(float3(right_anti_dual_g3[3]), other.e45)) - (this.group3.zxyx * float4(other.group1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.zxy * this.group1.yzx), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e4125, other.e4235, other.e4315, other.e1234) * wedge_g3.yzxw) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[3]) * other.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(float3(wedge_g3[3]), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - (wedge_g3.xyzx * float4(float3(other.e3215), other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (float4(wedge_g2[0], wedge_g2[1], wedge_g2[2], wedge_g3[0]) * other.group2.wwwx) + (wedge_g3.zxyy * float4(other.group1.yzx, other.e25)) + float4(float3(0.0), (wedge_g3[2] * other.e35) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1.zxy * other.group3.yzx), 0.0) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (float4(wedge_g1[1], wedge_g1[2], wedge_g1[0], wedge_g2[0]) * other.group3.zxyx) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - float4((wedge_g3.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<VersorOdd> operator /(VersorOdd lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar * -1.0) * other.group2.xyz;
        float wedge_g2_w = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float2(wedge_g2_w) * other.group2.xy), wedge_g2_w * other.e35, (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_anti_dual_g2_xyz * float3(this.scalar);
        float wedge_g2_w = -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g2_xyz[0] * this.e23) - (right_anti_dual_g2_xyz[1] * this.e31) - (right_anti_dual_g2_xyz[2] * this.e12) - (right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (this.group0.zxyw * float4(right_anti_dual_g2_xyz.yzx, other.e5)) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0]) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiDualNum other) {
        float4 wedge_g0 = float4((float3(other.e3215) * this.group0.xyz), (other.e3215 * this.e1234) + (other.scalar * this.scalar));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.scalar), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_xyz[0] * this.e23);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(wedge_g1.xyz, right_anti_dual_g0_w * this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)) + float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyw * float4(right_anti_dual_g0_xyz.yzx, other.e5));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * wedge_g1.xyz), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_anti_dual_g0_w * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_anti_dual_g0 * float3(this.scalar);
        float wedge_g0_w = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_anti_dual_g0 * float4(this.scalar)) + (float4(other.e3215) * float4(this.group0.xyz, this.e1234)) + float4(float3(0.0), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0 * other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0 * other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4(float3(0.0), -(right_anti_dual_g1[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((other.group2.yzx * this.group0.zxy), right_anti_dual_g1[1] * this.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((float3(wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.yzx * wedge_g3.zxy), 0.0) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g0[0] * other.e235) * float4(other.group2.zxy, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        float4 wedge_g0 = float4(((float3(right_anti_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_anti_dual_g2[3] * this.scalar);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g2[3]) * this.group1);
        float4 wedge_g2 = (right_anti_dual_g2 * float4(float3(this.scalar), this.e1234)) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + float4((float3(right_anti_dual_g2[3]) * this.group2.xyz), 0.0);
        float4 wedge_g3 = (float4(right_anti_dual_g2[3]) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g2.zxy * this.group0.yzx), 0.0) - (right_anti_dual_g2.yzxx * float4(this.group0.zxy, this.e23)) - (this.group2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0) + float4((other.group0.zxy * wedge_g3.yzx), 0.0) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g0[0] * other.e235) * float4(other.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 wedge_g0 = right_anti_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_anti_dual_g2 * float3(this.scalar);
        float wedge_g2_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(DipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group1.xyz, this.e3215)) + (this.group0.zxyx * float4(other.group3.yzx, other.e15)) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group3.zxyx * float4(this.group0.yzx, this.e4235)) - float4((float3(this.scalar) * other.group0.xyz), right_anti_dual_g1[0] * this.e23);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (other.group3.xyzx * this.group1.wwwx) + (other.group3.wwwy * float4(this.group0.xyz, this.e31)) + float4(float3(0.0), other.e4125 * this.e12) + float4((float3(other.e1234) * this.group2.xyz), 0.0);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (other.group3.zxy * this.group2.yzx) + float3(float2(0.0), (other.e4235 * this.e25) * -1.0) - (float3(this.scalar) * other.group2.xyz) - float3((other.group3.yz * this.group2.zx), 0.0);
        float4 wedge_g3 = float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], other.e3215 * this.scalar)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((float3(wedge_g0[3]) * other.group0.xyz), wedge_g3[1] * other.e4315) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(DualNum other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g2 = float4(other.e12345 * -1.0) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float2(other.e12345) * wedge_g0.xy), wedge_g0[2] * other.e12345, (wedge_g0[3] * other.e12345) + (wedge_g2[3] * other.e5)), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e12345 * -1.0) * float3((float2(other.e12345) * this.group3.xy), other.e12345 * this.e4125)), -(other.e12345 * other.e12345 * this.e3215) - (other.e5 * other.e12345 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(FlatPoint other) {
        float wedge_g2_w = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * other.e45), 
            /* e15, e25, e35, scalar */
            other.group0 * float4(float3(wedge_g2_w), other.e45 * this.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Flector other) {
        float4 wedge_g0 = (this.group0.zxyx * float4(other.group1.yzx, other.e15)) + float4(float3(0.0), (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1.zxyx * float4(this.group0.yzx, this.e4235));
        float4 wedge_g1 = (this.group0 * float4(float3(other.e3215), other.e45)) + (other.group1.xyzx * this.group1.wwwx) + float4(float3(0.0), (other.e4315 * this.e31) + (other.e4125 * this.e12));
        float4 wedge_g2 = float4(((float3(other.e3215) * this.group1.xyz) + (other.group1.zxy * this.group2.yzx) + float3(float2(0.0), other.e4235 * this.e25 * -1.0) - (float3(this.scalar) * other.group0.xyz) - float3((other.group1.yz * this.group2.zx), 0.0)), other.e3215 * this.scalar);
        float3 wedge_g3_xyz = float3(this.scalar * -1.0) * other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3_xyz[0]) * other.group1.yzxx) + float4(float3(0.0), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_orthogonally_onto(Line other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        float wedge_g2_w = -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43);
        float4 wedge_g3 = float4(float3(0.0), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), other.e415 * this.e15);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) + float4((other.group1.yzx * wedge_g3.zxy), 0.0) - (float4(wedge_g3[1], wedge_g3[2], wedge_g3[0], wedge_g1_xyz[0] * other.e415) * float4(other.group1.zxy, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float4 wedge_g1 = float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e45);
        float4 wedge_g2 = (float4(right_anti_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) + float4((right_anti_dual_g1_xyz * float3(this.scalar)), 0.0);
        float4 wedge_g3 = (right_anti_dual_g0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyw * float4(right_anti_dual_g1_xyz.yzx, other.e5));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2[3]) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group1.yz), 0.0) - float3((wedge_g3.yz * other.group1.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45);
        float4 wedge_g1 = right_anti_dual_g1 * float4(this.scalar);
        float wedge_g2 = other.e3215 * this.scalar;
        float4 wedge_g3 = (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * float4(this.e15, this.e25, this.e35, this.e45));
        float3 wedge_g4 = (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz);
        float3 wedge_g5 = (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz);
        float4 wedge_g6 = (right_anti_dual_g6 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12)) + float4((float3(right_anti_dual_g1[3]) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g1[1] * this.e41) - (right_anti_dual_g1[0] * this.e42)) + float3((right_anti_dual_g1.zx * this.group0.yz), 0.0) - float3((right_anti_dual_g1.yz * this.group0.zx), 0.0);
        float3 wedge_g8 = (right_anti_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + float3((right_anti_dual_g1.yz * this.group2.zx), 0.0) - float3((right_anti_dual_g1.zx * this.group2.yz), 0.0);
        float4 wedge_g9 = (float4(right_anti_dual_g0[0]) * this.group3) + float4(float3(0.0), -(right_anti_dual_g5[1] * this.e25) - (right_anti_dual_g5[2] * this.e35) - (right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12) - (other.e5 * this.scalar)) + float4((right_anti_dual_g5 * float3(this.e45)), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((other.group7.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g3.zxy * this.group0.yzx), 0.0) - (right_anti_dual_g3.yzxx * float4(this.group0.zxy, this.e23)) - (this.group2.yzxx * float4(other.group7.zxy, right_anti_dual_g5[0]));
        float wedge_g10 = (right_anti_dual_g0[0] * this.e1234) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e4 * this.scalar) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_anti_dual_g0[0] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e12345)) + (wedge_g3 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + (other.group9.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4(float3(0.0), -(wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412) - (wedge_g9[1] * other.e42) - (wedge_g9[2] * other.e43)) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(wedge_g9[3]) * other.group4.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g4[1] * other.e4315) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (float4(wedge_g10) * other.group3) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * float3(other.e3215)), wedge_g7[0] * other.e415) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g7[1] * other.e425) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g7[2] * other.e435) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) + float4(float3(0.0), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((float3(wedge_g9[3]) * other.group6.xyz), 0.0) + float4((float3(other.e3215) * wedge_g6.xyz), 0.0) + float4((wedge_g8.yzx * other.group9.zxy), 0.0) + float4((other.group8.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - (other.group9.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + (wedge_g9.yzxw * float4(other.group9.zxy, other.e1234)) - (float4(wedge_g9[2], wedge_g9[0], wedge_g9[1], wedge_g10) * other.group9.yzxw), 
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
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) + float4(float3(0.0), -(other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group0.zxyx * float4(this.group0.yzx, this.e4235));
        float4 wedge_g1 = (other.group0.xyzx * this.group1.wwwx) + (other.group0.wwwy * float4(this.group0.xyz, this.e31)) + float4(float3(0.0), other.e4125 * this.e12);
        float4 wedge_g2 = float4(((float3(other.e3215) * this.group1.xyz) + (other.group0.zxy * this.group2.yzx) + float3(float2(0.0), other.e4235 * this.e25 * -1.0) - float3((other.group0.yz * this.group2.zx), 0.0)), other.e3215 * this.scalar);
        float3 wedge_g3_xyz = float3(this.scalar * -1.0) * other.group0.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3_xyz[0]) * other.group0.yzxx) + float4(float3(0.0), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125)) - float4((wedge_g0.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((float3(other.e3215) * wedge_g0.xyz), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - float4((wedge_g2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (other.e4 * other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = (float4(right_anti_dual_g0[3]) * float4(this.group1.xyz, this.e3215)) + (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (other.e3215 * this.e1234)) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0);
        float4 wedge_g1 = float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)) + float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g0.yzx * this.group2.zxy) + float3(float2(0.0), (right_anti_dual_g0[1] * this.e15) * -1.0) - float3((right_anti_dual_g0.zx * this.group2.yz), 0.0);
        float4 wedge_g3 = right_anti_dual_g0 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], other.e3215 * this.scalar)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group0.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215)) - float4((wedge_g0.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(other.e3215) * wedge_g0.xyz), 0.0) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group0.zxy) - (wedge_g2_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        float4 wedge_g0 = float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), right_anti_dual_g0_w * this.scalar);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1);
        float4 wedge_g2 = (right_anti_dual_g2 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group2) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43));
        float4 wedge_g3 = (float4(right_anti_dual_g0_w) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e25) - (right_anti_dual_g1[2] * this.e35) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12) - (other.e5 * this.scalar)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((float3(this.e45) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0) + float4((right_anti_dual_g2.zxy * this.group0.yzx), 0.0) - (right_anti_dual_g2.yzxx * float4(this.group0.zxy, this.e23)) - (this.group2.yzxx * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.e12345)) + (float4(wedge_g2[3]) * float4(other.group1.xyz, other.e5)) + (wedge_g3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - (float4(wedge_g3[2], wedge_g3[0], wedge_g3[1], wedge_g2[0]) * other.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4(float3(0.0), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((float3(wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(wedge_g3[3]) * other.group0.xyz), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + float3(float2(0.0), (wedge_g3[1] * other.e235) - (wedge_g3[0] * other.e315)) + float3((wedge_g3.zx * other.group2.yz), 0.0) - float3((wedge_g3.yz * other.group2.zx), 0.0)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 wedge_g0 = (right_anti_dual_g0 * float4(this.scalar)) + (float4(right_anti_dual_g3[3]) * float4(this.group1.xyz, this.e3215)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (other.e3215 * this.e1234) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45)) - (float4(right_anti_dual_g3[1], right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g2_xyz[0]) * this.group0.zxyx);
        float4 wedge_g1 = (right_anti_dual_g1 * float4(this.scalar)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e31) - (right_anti_dual_g3[2] * this.e12)) + float4((float3(right_anti_dual_g3[3]) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + float3((right_anti_dual_g3.yz * this.group2.zx), 0.0) - float3((right_anti_dual_g3.zx * this.group2.yz), 0.0);
        float4 wedge_g3 = right_anti_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e1234) * float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], other.e3215 * this.scalar)) + (float4(wedge_g0[2], wedge_g0[0], wedge_g0[1], wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g2_xyz * float3(other.e1234)), 0.0) + float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
