implementing cga3d;
// Fancy postfix trick
public RoundPoint operator /(AntiCircleRotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiCircleRotor {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse_g0[2] * this.e25) - (anti_reverse_g2[1] * this.e43), -(anti_reverse_g0[0] * this.e35) - (anti_reverse_g2[2] * this.e41), -(anti_reverse_g0[1] * this.e15) - (anti_reverse_g2[0] * this.e42), (anti_reverse_g1[1] * this.e42) + (anti_reverse_g1[2] * this.e43)) + (this.group1.wwwz * float4(anti_reverse_g1.xyz, anti_reverse_g0[2])) + float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e23) + float4((this.group0.yzx * anti_reverse_g2.zxy), anti_reverse_g0[1] * this.e31) + float4((this.group1.xyz * anti_reverse_g1.www), anti_reverse_g1[0] * this.e41), 
            /* e5 */
            (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35) + (anti_reverse_g2[0] * this.e23) + (anti_reverse_g2[1] * this.e31) + (anti_reverse_g2[2] * this.e12)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(AntiDipoleInversion lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiDipoleInversion {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((anti_reverse_g1[2] * this.e2) + (anti_reverse_g1[3] * this.e415) + (anti_reverse_g2[3] * this.e235) + (this.e425 * this.e3), (anti_reverse_g1[1] * this.e321) + (anti_reverse_g1[3] * this.e425) + (anti_reverse_g2[3] * this.e315) + (this.e435 * this.e1), (anti_reverse_g1[2] * this.e321) + (anti_reverse_g1[3] * this.e435) + (anti_reverse_g2[3] * this.e125) + (this.e415 * this.e2), -(anti_reverse_g1[2] * this.e412) - (anti_reverse_g2[3] * this.e321) - (this.e431 * this.e2) - (this.e412 * this.e3)) + (float4(this.e321, this.e3, this.e1, this.e4) * anti_reverse_g1.xxyw) + (float4(this.e315, this.e5, this.e5, this.e2) * float4(anti_reverse_g0.zyz, anti_reverse_g0[1])) + (float4(this.e5, this.e125, this.e235, this.e1) * float4(anti_reverse_g0.xxy, anti_reverse_g0[0])) + float4((this.group0.zxy * anti_reverse_g2.yzx), anti_reverse_g0[2] * this.e3) - (float4(anti_reverse_g2.zx, this.e5, this.e435) * float4(this.group0.yzz, anti_reverse_g0[2])) - (float4(this.group3.ww, anti_reverse_g2[1], this.e425) * float4(this.group0.xyx, anti_reverse_g0[1])) - float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e415) - float4((anti_reverse_g1.yzx * this.group3.zxy), anti_reverse_g1[0] * this.e423) - float4((this.group1.zxy * this.group3.yzx), anti_reverse_g1[1] * this.e431) - float4((anti_reverse_g2.xyz * this.group2.www), this.e423 * this.e1), 
            /* e5 */
            (this.e321 * this.e5) + (this.e235 * this.e1) + (this.e315 * this.e2) + (this.e125 * this.e3) - (anti_reverse_g1[0] * this.e235) - (anti_reverse_g1[1] * this.e315) - (anti_reverse_g1[2] * this.e125) - (anti_reverse_g1[3] * this.e5) - (anti_reverse_g2[0] * this.e415) - (anti_reverse_g2[0] * this.e1) - (anti_reverse_g2[1] * this.e425) - (anti_reverse_g2[1] * this.e2) - (anti_reverse_g2[2] * this.e435) - (anti_reverse_g2[2] * this.e3)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiDualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiDualNum {
    public DualNum anti_constraint_violation() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * this.scalar * -2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiFlector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiFlector {
    public DualNum anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2((this.e235 * this.e1) + (this.e315 * this.e2) + (this.e125 * this.e3) + (this.e321 * this.e5) - (anti_reverse_g0[0] * this.e1) - (anti_reverse_g0[1] * this.e2) - (anti_reverse_g0[2] * this.e3) - (anti_reverse_g0[3] * this.e5), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiLine lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiLine {
    public DualNum anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse_g0[0] * this.e15) + (anti_reverse_g0[1] * this.e25) + (anti_reverse_g0[2] * this.e35) + (anti_reverse_g1[0] * this.e23) + (anti_reverse_g1[1] * this.e31) + (anti_reverse_g1[2] * this.e12), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiMotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension AntiMotor {
    public DualNum anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse_g0[0] * this.e15) + (anti_reverse_g0[1] * this.e25) + (anti_reverse_g0[2] * this.e35) + (anti_reverse_g1[0] * this.e23) + (anti_reverse_g1[1] * this.e31) + (anti_reverse_g1[2] * this.e12) - (anti_reverse_g0[3] * this.e3215) - (anti_reverse_g1[3] * this.scalar), 0.0)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Circle lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_constraint_violation();
}
public extension Circle {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((anti_reverse_g0[2] * this.e315) + (anti_reverse_g2[1] * this.e412) + (anti_reverse_g1[0] * this.e321) + (anti_reverse_g1[3] * this.e415), (anti_reverse_g0[0] * this.e125) + (anti_reverse_g2[2] * this.e423) + (anti_reverse_g1[1] * this.e321) + (anti_reverse_g1[3] * this.e425), (anti_reverse_g0[1] * this.e235) + (anti_reverse_g2[0] * this.e431) + (anti_reverse_g1[2] * this.e321) + (anti_reverse_g1[3] * this.e435), -(anti_reverse_g0[2] * this.e435) - (anti_reverse_g1[0] * this.e423) - (anti_reverse_g1[1] * this.e431) - (anti_reverse_g1[2] * this.e412)) - float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e415) - float4((anti_reverse_g2.zxy * this.group0.yzx), anti_reverse_g0[1] * this.e425), 
            /* e5 */
            -(anti_reverse_g2[0] * this.e415) - (anti_reverse_g2[1] * this.e425) - (anti_reverse_g2[2] * this.e435) - (anti_reverse_g1[0] * this.e235) - (anti_reverse_g1[1] * this.e315) - (anti_reverse_g1[2] * this.e125)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(CircleRotor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_constraint_violation();
}
public extension CircleRotor {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((anti_reverse_g0[2] * this.e315) + (anti_reverse_g1[0] * this.e321) + (anti_reverse_g1[3] * this.e415) + (anti_reverse_g2[1] * this.e412), (anti_reverse_g0[0] * this.e125) + (anti_reverse_g1[1] * this.e321) + (anti_reverse_g1[3] * this.e425) + (anti_reverse_g2[2] * this.e423), (anti_reverse_g0[1] * this.e235) + (anti_reverse_g1[2] * this.e321) + (anti_reverse_g1[3] * this.e435) + (anti_reverse_g2[0] * this.e431), -(anti_reverse_g0[2] * this.e435) - (anti_reverse_g1[0] * this.e423) - (anti_reverse_g1[1] * this.e431) - (anti_reverse_g1[2] * this.e412)) - float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e415) - float4((this.group0.yzx * anti_reverse_g2.zxy), anti_reverse_g0[1] * this.e425), 
            /* e5 */
            -(anti_reverse_g1[0] * this.e235) - (anti_reverse_g1[1] * this.e315) - (anti_reverse_g1[2] * this.e125) - (anti_reverse_g2[0] * this.e415) - (anti_reverse_g2[1] * this.e425) - (anti_reverse_g2[2] * this.e435)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Dipole lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_constraint_violation();
}
public extension Dipole {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse_g0[2] * this.e25) - (anti_reverse_g2[1] * this.e43), -(anti_reverse_g0[0] * this.e35) - (anti_reverse_g2[2] * this.e41), -(anti_reverse_g0[1] * this.e15) - (anti_reverse_g2[0] * this.e42), (anti_reverse_g1[1] * this.e42) + (anti_reverse_g1[2] * this.e43)) + (this.group1.wwwz * float4(anti_reverse_g1.xyz, anti_reverse_g0[2])) + float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e23) + float4((anti_reverse_g2.zxy * this.group0.yzx), anti_reverse_g0[1] * this.e31) + float4((this.group1.xyz * anti_reverse_g1.www), anti_reverse_g1[0] * this.e41), 
            /* e5 */
            (anti_reverse_g2[0] * this.e23) + (anti_reverse_g2[1] * this.e31) + (anti_reverse_g2[2] * this.e12) + (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(DipoleInversion lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_constraint_violation();
}
public extension DipoleInversion {
    public RoundPoint anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(anti_reverse_g2[3] * this.e15) - (this.e12 * this.e4315), -(anti_reverse_g2[3] * this.e25) - (this.e23 * this.e4125), -(anti_reverse_g2[3] * this.e35) - (this.e31 * this.e4235), (anti_reverse_g1[2] * this.e43) + (anti_reverse_g1[3] * this.e1234)) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse_g1.xxy, anti_reverse_g0[1])) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse_g1.zyz, anti_reverse_g0[2])) + (float4(anti_reverse_g2.zx, this.e3215, this.e31) * float4(this.group0.yzz, anti_reverse_g0[1])) + (float4(this.group3.ww, anti_reverse_g2[1], this.e4235) * float4(this.group0.xyx, anti_reverse_g0[0])) + float4((anti_reverse_g0.yzx * this.group2.zxy), anti_reverse_g0[0] * this.e23) + float4((this.group1.yzx * this.group3.zxy), anti_reverse_g1[1] * this.e42) + float4((anti_reverse_g2.xyz * this.group2.www), anti_reverse_g1[0] * this.e41) + float4((this.group1.xyz * anti_reverse_g1.www), anti_reverse_g0[2] * this.e4125) - (float4(this.e25, this.e3215, this.e3215, this.e4315) * float4(anti_reverse_g0.zyz, this.e42)) - (float4(this.e3215, this.e35, this.e15, this.e4235) * float4(anti_reverse_g0.xxy, this.e41)) - float4((this.group0.zxy * anti_reverse_g2.yzx), this.e43 * this.e4125) - float4((anti_reverse_g1.yzx * this.group3.zxy), anti_reverse_g2[3] * this.e45), 
            /* e5 */
            (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35) + (anti_reverse_g2[0] * this.e23) + (anti_reverse_g2[1] * this.e31) + (anti_reverse_g2[2] * this.e12) + (this.e45 * this.e3215) + (this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) - (anti_reverse_g1[3] * this.e3215) - (anti_reverse_g2[0] * this.e4235) - (anti_reverse_g2[1] * this.e4315) - (anti_reverse_g2[2] * this.e4125)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension DualNum {
    public DualNum anti_constraint_violation() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5 * this.e12345 * 2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Flector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_constraint_violation();
}
public extension Flector {
    public DualNum anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) + (this.e45 * this.e3215) - (anti_reverse_g0[0] * this.e4235) - (anti_reverse_g0[1] * this.e4315) - (anti_reverse_g0[2] * this.e4125) - (anti_reverse_g0[3] * this.e3215), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Line lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_constraint_violation();
}
public extension Line {
    public DualNum anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2(-(anti_reverse_g0[0] * this.e235) - (anti_reverse_g0[1] * this.e315) - (anti_reverse_g0[2] * this.e125) - (anti_reverse_g1[0] * this.e415) - (anti_reverse_g1[1] * this.e425) - (anti_reverse_g1[2] * this.e435), 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Motor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public DualNum operator /(AntiConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_constraint_violation();
}
public extension Motor {
    public DualNum anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DualNum(
            /* e5, e12345 */
            float2((anti_reverse_g0[3] * this.e5) + (anti_reverse_g1[3] * this.e12345) - (anti_reverse_g0[0] * this.e235) - (anti_reverse_g0[1] * this.e315) - (anti_reverse_g0[2] * this.e125) - (anti_reverse_g1[0] * this.e415) - (anti_reverse_g1[1] * this.e425) - (anti_reverse_g1[2] * this.e435), 0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public MultiVector operator /(AntiConstraintViolationPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_constraint_violation();
}
public extension MultiVector {
    public MultiVector anti_constraint_violation() {
        float4 anti_reverse_g3 = this.group3 * float4(-1.0);
        float3 anti_reverse_g4 = this.group4.xyz * float3(-1.0);
        float3 anti_reverse_g5 = this.group5.xyz * float3(-1.0);
        float4 anti_reverse_g6 = this.group6 * float4(-1.0);
        float3 anti_reverse_g7 = this.group7.xyz * float3(-1.0);
        float3 anti_reverse_g8 = this.group8.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(this.e5 * this.e1234) + (this.group9.yx[0] * this.e2) + 2.0 * (this.group1.xwzw[0] * this.e4235) + (this.group9.zyzw[0] * this.e3) + (this.group9.wzzw[0] * this.e4) + 2.0 * (this.scalar * this.e12345) + (this.e2 * this.e4315) + (this.e3 * this.e4125) + (this.e4 * this.e3215) - (anti_reverse_g4[0] * this.e235) - (anti_reverse_g4[1] * this.e315) - (anti_reverse_g4[2] * this.e125) - (anti_reverse_g5[0] * this.e415) - (anti_reverse_g5[1] * this.e425) - (anti_reverse_g5[2] * this.e435) - (anti_reverse_g7[0] * this.group3.xwzw[0]) - (anti_reverse_g7[1] * this.e25) - (anti_reverse_g7[2] * this.e35) - (anti_reverse_g8[0] * this.e41) - (anti_reverse_g8[1] * this.e42) - (anti_reverse_g8[2] * this.e43) - (anti_reverse_g3[0] * this.e423) - (anti_reverse_g3[1] * this.e431) - (anti_reverse_g3[2] * this.e412) - (anti_reverse_g3[3] * this.e321) - (anti_reverse_g6[0] * this.e23) - (anti_reverse_g6[1] * this.e31) - (anti_reverse_g6[2] * this.e12) - (anti_reverse_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_reverse_g3[0] * this.e1234) + (anti_reverse_g6[0] * this.e321) + (anti_reverse_g6[2] * this.e2) + (anti_reverse_g6[3] * this.e415), (anti_reverse_g3[1] * this.e1234) + (anti_reverse_g6[0] * this.e3) + (anti_reverse_g6[1] * this.e321) + (anti_reverse_g6[3] * this.e425), (anti_reverse_g3[2] * this.e1234) + (anti_reverse_g6[1] * this.e1) + (anti_reverse_g6[2] * this.e321) + (anti_reverse_g6[3] * this.e435), -(anti_reverse_g6[2] * this.e412) - (this.e3 * this.e412) - (this.e4 * this.e321) - (this.e45 * this.e1234)) + (float4(this.e5, this.e125, this.e235, this.e4125) * float4(anti_reverse_g7.xxy, anti_reverse_g4[2])) + (float4(this.e45, this.e4125, this.e4235, this.e4315) * float4(anti_reverse_g5.xxy, anti_reverse_g4[1])) + (float4(this.e315, this.e5, this.e5, this.e41) * float4(anti_reverse_g7.zyz, anti_reverse_g5[0])) + (float4(this.e4315, this.e45, this.e45, this.e12) * float4(anti_reverse_g5.zyz, anti_reverse_g4[2])) + (float4(this.group0.xx, this.scalar, anti_reverse_g4[0]) * float4(this.group9.xyz, this.e23)) + (float4(this.group0.xx, this.scalar, this.e12345) * float4(this.group9.xyz, this.e4)) + (float4(this.group0.yy, this.e12345, anti_reverse_g4[0]) * float4(this.group1.xyz, this.e4235)) + (float4(this.group0.yy, this.e12345, this.e12345) * float4(this.group1.xyz, this.e4)) + (float4(anti_reverse_g3.zx, this.e3215, this.e1) * float4(this.group4.yzz, anti_reverse_g7[0])) + (float4(anti_reverse_g3.ww, this.e4315, this.e2) * float4(this.group5.xyx, anti_reverse_g7[1])) + (float4(this.group9.zx, anti_reverse_g3[3], this.e3) * float4(this.group5.yzz, anti_reverse_g7[2])) + (float4(this.group9.ww, anti_reverse_g3[1], this.e43) * float4(this.group4.xyx, anti_reverse_g5[2])) + float4((this.group8.xyz * this.group1.www), anti_reverse_g3[3] * this.e1234) + float4((anti_reverse_g4.yzx * this.group3.zxy), anti_reverse_g4[1] * this.e31) + float4((anti_reverse_g8.yzx * this.group7.zxy), anti_reverse_g5[1] * this.e42) + float4((this.group1.zxy * this.group6.yzx), anti_reverse_g6[3] * this.e4) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse_g8.xxy, anti_reverse_g7[2])) - (float4(this.e25, this.e3215, this.e3215, this.e1234) * float4(anti_reverse_g4.zyz, this.scalar)) - (float4(this.e431, this.e4, this.e4, this.e4235) * float4(anti_reverse_g8.zyz, this.e41)) - (float4(this.e3215, this.e35, this.e15, this.e1234) * float4(anti_reverse_g4.xxy, this.scalar)) - (this.group9.yzxz * float4(this.group5.zxy, this.e43)) - float4((float3(this.e5) * this.group7.xyz), this.e1 * this.e423) - float4((float3(this.e1234) * this.group3.xyz), anti_reverse_g6[1] * this.e431) - float4((anti_reverse_g5.yzx * this.group9.zxy), anti_reverse_g7[0] * this.e415) - float4((anti_reverse_g7.yzx * this.group8.zxy), anti_reverse_g7[1] * this.e425) - float4((this.group4.zxy * anti_reverse_g3.yzx), this.e42 * this.e4315) - float4((anti_reverse_g6.yzx * this.group1.zxy), this.e2 * this.e431) - float4((this.group1.yzx * this.group6.zxy), anti_reverse_g6[0] * this.e423), 
            /* e5 */
            (anti_reverse_g5[0] * this.e15) + (anti_reverse_g5[1] * this.e25) + (anti_reverse_g5[2] * this.e35) + (anti_reverse_g3[0] * this.e23) + (anti_reverse_g3[1] * this.e31) + (anti_reverse_g3[2] * this.e12) + 2.0 * (this.e12345 * this.e5) + (this.e1 * this.e235) + (this.e2 * this.e315) + (this.e3 * this.e125) + (this.e5 * this.e321) + (this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) + (this.e45 * this.e3215) - (anti_reverse_g8[0] * this.e1) - (anti_reverse_g8[0] * this.e415) - (anti_reverse_g8[1] * this.e2) - (anti_reverse_g8[1] * this.e425) - (anti_reverse_g8[2] * this.e3) - (anti_reverse_g8[2] * this.e435) - (anti_reverse_g3[0] * this.e4235) - (anti_reverse_g3[1] * this.e4315) - (anti_reverse_g3[2] * this.e4125) - (anti_reverse_g3[3] * this.e3215) - (anti_reverse_g6[0] * this.e235) - (anti_reverse_g6[1] * this.e315) - (anti_reverse_g6[2] * this.e125) - (anti_reverse_g6[3] * this.e5) - 2.0 * (this.scalar * this.e3215), 
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
            float4((anti_reverse_g3[3] * this.e415) + (anti_reverse_g6[0] * this.e45) + (anti_reverse_g6[2] * this.e4315) + (this.e425 * this.e4125), (anti_reverse_g3[3] * this.e425) + (anti_reverse_g6[0] * this.e4125) + (anti_reverse_g6[1] * this.e45) + (this.e435 * this.e4235), (anti_reverse_g3[3] * this.e435) + (anti_reverse_g6[1] * this.e4235) + (anti_reverse_g6[2] * this.e45) + (this.e415 * this.e4315), -(anti_reverse_g6[1] * this.e25) - (anti_reverse_g6[2] * this.e35) - (anti_reverse_g6[3] * this.e3215) - (this.e5 * this.e45)) + (float4(this.e42, this.e1234, this.e1234, this.e4125) * float4(anti_reverse_g8.zyz, anti_reverse_g8[2])) + (float4(this.e1234, this.e43, this.e41, this.e4315) * float4(anti_reverse_g8.xxy, anti_reverse_g8[1])) + (float4(this.group0.yy, this.e12345, this.scalar) * float4(this.group9.xyz, this.e5)) + (float4(this.group0.yy, this.e12345, this.e12345) * float4(this.group9.xyz, this.e3215)) + (float4(anti_reverse_g3.zx, this.e3215, this.e5) * float4(this.group7.yzz, anti_reverse_g3[3])) + (float4(this.group9.ww, anti_reverse_g3[1], this.e35) * float4(this.group7.xyx, this.e3)) + float4((float3(this.e5) * this.group4.xyz), this.e1 * this.e15) + float4((anti_reverse_g4.yzx * this.group8.zxy), this.scalar * this.e5) + float4((anti_reverse_g5.yzx * this.group1.zxy), this.e12345 * this.e3215) + float4((anti_reverse_g7.yzx * this.group3.zxy), anti_reverse_g8[0] * this.e4235) + float4((this.group5.zxy * this.group1.yzx), this.e2 * this.e25) + float4((anti_reverse_g3.xyz * this.group1.www), this.e321 * this.e3215) - (float4(this.e2, this.e321, this.e321, this.e12) * float4(anti_reverse_g5.zyz, anti_reverse_g8[2])) - (float4(this.e5, this.e125, this.e235, this.e125) * float4(anti_reverse_g4.xxy, anti_reverse_g5[2])) - (float4(this.e25, this.e3215, this.e3215, this.e4315) * float4(anti_reverse_g7.zyz, this.e315)) - (float4(this.e321, this.e3, this.e1, this.e31) * float4(anti_reverse_g5.xxy, anti_reverse_g8[1])) - (float4(this.e315, this.e5, this.e5, this.e23) * float4(anti_reverse_g4.zyz, anti_reverse_g8[0])) - (float4(this.e3215, this.e35, this.e15, this.e4235) * float4(anti_reverse_g7.xxy, this.e235)) - (float4(this.group0.xx, this.scalar, anti_reverse_g5[0]) * float4(this.group1.xyz, this.e235)) - (float4(this.group0.xx, this.scalar, anti_reverse_g5[1]) * float4(this.group1.xyz, this.e315)) - (float4(anti_reverse_g6.ww, this.e2, this.e1) * float4(this.group5.xyx, anti_reverse_g3[0])) - (float4(this.group1.zx, anti_reverse_g6[3], this.e415) * float4(this.group5.yzz, anti_reverse_g3[0])) - float4((float3(this.e1234) * this.group8.xyz), anti_reverse_g3[1] * this.e425) - float4((anti_reverse_g8.yzx * this.group4.zxy), this.e125 * this.e4125) - float4((this.group7.zxy * anti_reverse_g3.yzx), anti_reverse_g3[1] * this.e2) - float4((anti_reverse_g6.yzx * this.group9.zxy), anti_reverse_g3[2] * this.e435) - float4((this.group6.zxy * this.group9.yzx), anti_reverse_g6[0] * this.e15) - float4((this.group3.xyz * this.group1.www), anti_reverse_g3[2] * this.e3), 
            /* e1234 */
            (anti_reverse_g4[0] * this.e1) + (anti_reverse_g4[1] * this.e2) + (anti_reverse_g4[2] * this.e3) + (anti_reverse_g6[3] * this.e1234) + 2.0 * (this.scalar * this.e4) + 2.0 * (this.e12345 * this.e1234) + (this.e4 * this.e45) + (this.e423 * this.e4235) + (this.e431 * this.e4315) + (this.e412 * this.e4125) - (anti_reverse_g4[0] * this.e415) - (anti_reverse_g4[1] * this.e425) - (anti_reverse_g4[2] * this.e435) - (anti_reverse_g5[0] * this.e423) - (anti_reverse_g5[1] * this.e431) - (anti_reverse_g5[2] * this.e412) - (anti_reverse_g7[0] * this.e23) - (anti_reverse_g7[0] * this.e4235) - (anti_reverse_g7[1] * this.e31) - (anti_reverse_g7[1] * this.e4315) - (anti_reverse_g7[2] * this.e12) - (anti_reverse_g7[2] * this.e4125) - (anti_reverse_g3[3] * this.e4) - (anti_reverse_g6[0] * this.e41) - (anti_reverse_g6[1] * this.e42) - (anti_reverse_g6[2] * this.e43) - (this.e1 * this.e41) - (this.e2 * this.e42) - (this.e3 * this.e43) - (this.e321 * this.e1234)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(VersorEven lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_constraint_violation();
}
public extension VersorEven {
    public RoundPoint anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((anti_reverse_g2[1] * this.e412) + (this.e12345 * this.e1) + (this.e425 * this.e3) + (this.e235 * this.e4), (anti_reverse_g2[2] * this.e423) + (this.e12345 * this.e2) + (this.e435 * this.e1) + (this.e315 * this.e4), (anti_reverse_g2[0] * this.e431) + (this.e12345 * this.e3) + (this.e415 * this.e2) + (this.e125 * this.e4), -(this.e423 * this.e1) - (this.e431 * this.e2) - (this.e412 * this.e3) - (this.e321 * this.e4)) + (float4(this.e321, this.e3, this.e1, this.e4) * float4(anti_reverse_g1.xxy, anti_reverse_g0[3])) + (float4(this.e2, this.e321, this.e321, this.e4) * anti_reverse_g1.zyzw) + (anti_reverse_g0.xxyx * float4(this.group2.wzx, this.e1)) + (anti_reverse_g0.zyzy * float4(this.group2.yww, this.e2)) + (anti_reverse_g0.wwwz * this.group3.xyzz) + float4((this.group1.xyz * anti_reverse_g1.www), this.e12345 * this.e4) - (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse_g2.zyz, anti_reverse_g1[0])) - (float4(this.e4, this.e412, this.e423, this.e435) * float4(anti_reverse_g2.xxy, anti_reverse_g0[2])) - (anti_reverse_g0.yzxx * float4(this.group2.zxy, this.e415)) - (this.group0.xyzy * float4(anti_reverse_g2.www, anti_reverse_g1[1])) - float4((anti_reverse_g1.yzx * this.group3.zxy), anti_reverse_g0[1] * this.e425) - float4((this.group1.zxy * this.group3.yzx), anti_reverse_g1[2] * this.e412), 
            /* e5 */
            (anti_reverse_g0[3] * this.e5) + (anti_reverse_g2[3] * this.e12345) + (anti_reverse_g2[3] * this.e321) + (this.e235 * this.e1) + (this.e315 * this.e2) + (this.e125 * this.e3) - (anti_reverse_g1[0] * this.e235) - (anti_reverse_g1[1] * this.e315) - (anti_reverse_g1[2] * this.e125) - (anti_reverse_g1[3] * this.e5) - (anti_reverse_g2[0] * this.e415) - (anti_reverse_g2[0] * this.e1) - (anti_reverse_g2[1] * this.e425) - (anti_reverse_g2[1] * this.e2) - (anti_reverse_g2[2] * this.e435) - (anti_reverse_g2[2] * this.e3)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(VersorOdd lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiConstraintViolationPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_constraint_violation();
}
public extension VersorOdd {
    public RoundPoint anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e42, this.e1234, this.e1234, this.e41) * float4(anti_reverse_g2.zyz, anti_reverse_g1[0])) + (float4(this.scalar, this.e12, this.e23, this.e42) * float4(this.group3.xxy, anti_reverse_g1[1])) + (float4(this.e31, this.scalar, this.scalar, this.e43) * float4(this.group3.zyz, anti_reverse_g1[2])) + (float4(this.e45, this.e4125, this.e4235, this.e31) * float4(anti_reverse_g1.xxy, anti_reverse_g0[1])) + (float4(this.e1234, this.e43, this.e41, this.e4125) * float4(anti_reverse_g2.xxy, anti_reverse_g0[2])) + (float4(this.e4315, this.e45, this.e45, this.e4315) * float4(anti_reverse_g1.zyz, anti_reverse_g0[1])) + (anti_reverse_g0.yzxx * float4(this.group2.zxy, this.e23)) + (anti_reverse_g0.wwwx * this.group3.xyzx) + (this.group1.xyzz * float4(anti_reverse_g1.www, anti_reverse_g0[2])) + float4((this.group0.xyz * this.group3.www), anti_reverse_g1[3] * this.e1234) - (float4(this.e25, this.e3215, this.e3215, this.scalar) * float4(anti_reverse_g0.zyz, anti_reverse_g2[3])) - (float4(this.e3215, this.e35, this.e15, this.e1234) * anti_reverse_g0.xxyw) - (this.group0.zxyx * float4(anti_reverse_g2.yzx, this.e4235)) - (this.group3.yzxz * float4(this.group1.zxy, this.e43)) - float4((anti_reverse_g1.yzx * this.group3.zxy), anti_reverse_g2[3] * this.e45) - float4((this.group2.xyz * anti_reverse_g2.www), this.e42 * this.e4315), 
            /* e5 */
            (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35) + (anti_reverse_g2[0] * this.e23) + (anti_reverse_g2[1] * this.e31) + (anti_reverse_g2[2] * this.e12) + (this.e45 * this.e3215) + (this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) - (anti_reverse_g0[3] * this.e3215) - (anti_reverse_g1[3] * this.e3215) - (anti_reverse_g2[0] * this.e4235) - (anti_reverse_g2[1] * this.e4315) - (anti_reverse_g2[2] * this.e4125) - (this.scalar * this.e3215)
        );
    }
}
