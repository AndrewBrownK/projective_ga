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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((this.group0.zxy * this.group2.yzx), 0.0) + float4(float3(0.0), (this.e43 * this.e12) * -2.0) - float4(2.0)*(this.group1.xyzx * float4(float3(this.e45), this.e41)) - float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e42 * this.e31), 
            /* e5 */
            -2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)
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
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float4 anti_reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (anti_reverse_g1 * float4(float3(this.e321), this.e4)) + (float4(anti_reverse_g2[1], anti_reverse_g2[2], anti_reverse_g2[0], this.e412 * this.e435) * float4(this.group0.zxy, 1.0)) + (this.group1.xyzx * float4(float3(anti_reverse_g1[3]), this.e423)) + (this.group1.yzxy * float4(this.group3.zxy, this.e431)) + float4(float3(0.0), (this.e431 * this.e2) * -2.0) + float4((float3(anti_reverse_g2[3]) * this.group2.xyz), 0.0) + float4((this.group0.yzx * this.group2.zxy), 0.0) + float4((anti_reverse_g1.zxy * this.group3.yzx), 0.0) - float4(anti_reverse_g1[1] * this.e3, this.e415 * this.e3, this.e425 * this.e1, 0.0) - float4(this.e435 * this.e2, anti_reverse_g1[2] * this.e1, 0.0, this.e423 * this.e1) - (anti_reverse_g2 * float4(float3(this.e4), this.e321)) - (float4(this.e423) * float4(this.e5, this.e125, anti_reverse_g2[1], anti_reverse_g1[0])) - (float4(this.e431) * float4(anti_reverse_g2[2], this.e5, this.e235, anti_reverse_g1[1])) - (float4(this.e412) * float4(this.e315, anti_reverse_g2[0], this.e5, this.e3)) - float4(float2(0.0), anti_reverse_g1[0] * this.e2, anti_reverse_g1[2] * this.e412), 
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
        return DualNum(
            /* e5, e12345 */
            float2(2.0*(this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e125 * this.e3) + 2.0 * (this.e321 * this.e5), 0.0)
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
        return DualNum(
            /* e5, e12345 */
            float2(-2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35), 0.0)
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
        return DualNum(
            /* e5, e12345 */
            float2(-2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35) - 2.0 * (this.scalar * this.e3215), 0.0)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e412 * this.e435) + float4(float3(0.0), 2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425)) - float4(2.0)*float4((float3(this.e321) * this.group1.xyz), 0.0) - float4(2.0)*float4((this.group0.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e412 * this.e435) + float4(float3(0.0), 2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425)) - float4(2.0)*float4((float3(this.e321) * this.group1.xyz), 0.0) - float4(2.0)*float4((this.group0.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((this.group0.zxy * this.group2.yzx), 0.0) + float4(float3(0.0), (this.e43 * this.e12) * -2.0) - float4(2.0)*(this.group1.xyzx * float4(float3(this.e45), this.e41)) - float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e42 * this.e31), 
            /* e5 */
            -2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)
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
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((float3(this.e25, this.e3215, this.e3215) * this.group0.zyz), 0.0) + (anti_reverse_g1 * float4(float3(this.e45), this.e1234)) + (float4(this.e3215, this.e35, this.e15, anti_reverse_g1[2] * this.e43) * float4(this.group0.xxy, 1.0)) + (anti_reverse_g1.zxyx * float4(this.group3.yzx, this.e41)) + (anti_reverse_g1.wwwy * float4(this.group1.xyz, this.e42)) + float4(float3(0.0), -(this.e43 * this.e12) - 2.0 * (this.e43 * this.e4125)) + float4((this.group1.yzx * this.group3.zxy), 0.0) - float4(0.0, this.e23 * this.e4125, this.e31 * this.e4235, this.e42 * this.e4315) - float4(anti_reverse_g1[1] * this.e4125, anti_reverse_g1[2] * this.e4235, 0.0, this.e41 * this.e23) - float4(this.e42 * this.e35, 0.0, anti_reverse_g1[0] * this.e4315, this.e41 * this.e4235) - float4(this.e12 * this.e4315, this.e43 * this.e15, this.e41 * this.e25, 0.0) - (this.group2 * float4(float3(this.e1234), this.e45)) - float4(float3(0.0), this.e42 * this.e31), 
            /* e5 */
            (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35) + (this.e45 * this.e3215) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) - (anti_reverse_g1[3] * this.e3215) - (this.e23 * this.e15) - (this.e31 * this.e25) - (this.e12 * this.e35)
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
        return DualNum(
            /* e5, e12345 */
            float2(2.0*(this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) + 2.0 * (this.e45 * this.e3215), 0.0)
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
        return DualNum(
            /* e5, e12345 */
            float2(2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125), 0.0)
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
        return DualNum(
            /* e5, e12345 */
            float2(2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125) + 2.0 * (this.e12345 * this.e5), 0.0)
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
        float4 anti_reverse_g6 = this.group6 * float4(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(this.e1 * this.e4235) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) + 2.0 * (this.e5 * this.e1234) + 2.0 * (this.e41 * this.e235) + 2.0 * (this.e42 * this.e315) + 2.0 * (this.e43 * this.e125) + (this.scalar * this.e12345) + (this.e2 * this.e4315) + (this.e15 * this.e423) + (this.e25 * this.e431) + (this.e35 * this.e412) + (this.e23 * this.e415) + (this.e31 * this.e425) + (this.e12 * this.e435) - (anti_reverse_g3[0] * this.e423) - (anti_reverse_g3[1] * this.e431) - (anti_reverse_g3[2] * this.e412) - (anti_reverse_g3[3] * this.e321) - (anti_reverse_g6[0] * this.e23) - (anti_reverse_g6[1] * this.e31) - (anti_reverse_g6[2] * this.e12) - (anti_reverse_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.e12345) * this.group1) + float4(2.0)*float4((this.group5.yzx * this.group9.zxy), 0.0) + float4((this.e35 * this.e42) * -1.0, (this.e423 * this.e125) * -1.0, (this.e431 * this.e235) * -1.0, -(this.e2 * this.e431) - (this.e4 * this.e321) - (this.e42 * this.e4315) - 2.0 * (this.e3 * this.e412) - 2.0 * (this.e41 * this.e23) - 2.0 * (this.e43 * this.e12) - 2.0 * (this.e43 * this.e4125)) + (anti_reverse_g3 * float4(this.e1234)) + (anti_reverse_g6 * float4(float3(this.e321), this.e4)) + (float4(anti_reverse_g3[2], anti_reverse_g3[0], anti_reverse_g3[1], this.e435 * this.e412) * float4(this.group4.yzx, 1.0)) + (this.group6.xyzx * float4(float3(anti_reverse_g6[3]), this.e423)) + (this.group6.yzxy * float4(this.group1.zxy, this.e431)) + float4((float3(anti_reverse_g3[3]) * this.group5.xyz), 0.0) + float4((float3(this.scalar) * this.group9.xyz), 0.0) + float4((float3(this.e4) * this.group8.xyz), 0.0) + float4((float3(this.e3215) * this.group4.xyz), 0.0) + float4((this.group4.zxy * this.group3.yzx), 0.0) + float4((this.group7.yzx * this.group8.zxy), 0.0) + float4((anti_reverse_g6.zxy * this.group1.yzx), 0.0) - float4(this.e412 * this.e315, anti_reverse_g3[2] * this.e41, anti_reverse_g3[0] * this.e42, 0.0) - (float4(this.e1234) * this.group3) - (float4(anti_reverse_g3[1], this.e15, this.e25, anti_reverse_g6[1] * this.e431) * float4(this.group4.zzx, 1.0)) - (float4(this.e5, this.e5, this.e5, anti_reverse_g6[2] * this.e412) * float4(this.group7.xyz, 1.0)) - (anti_reverse_g6.yzxx * float4(this.group1.zxy, this.e423)) - (this.group1.yzxx * float4(this.group6.zxy, this.e423)) - (this.group9.yzxx * float4(this.group5.zxy, this.e41)) - float4(float3(0.0), this.e42 * this.e31) - float4((float3(this.e45) * this.group5.xyz), this.scalar * this.e1234), 
            /* e5 */
            2.0*(this.e12345 * this.e5) + 2.0 * (this.e1 * this.e235) + 2.0 * (this.e2 * this.e315) + 2.0 * (this.e3 * this.e125) + (anti_reverse_g3[0] * this.e23) + (anti_reverse_g3[1] * this.e31) + (anti_reverse_g3[2] * this.e12) + (this.e5 * this.e321) + (this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) + (this.e45 * this.e3215) + (this.e415 * this.e235) + (this.e425 * this.e315) + (this.e435 * this.e125) - (anti_reverse_g3[0] * this.e4235) - (anti_reverse_g3[1] * this.e4315) - (anti_reverse_g3[2] * this.e4125) - (anti_reverse_g3[3] * this.e3215) - (anti_reverse_g6[0] * this.e235) - (anti_reverse_g6[1] * this.e315) - (anti_reverse_g6[2] * this.e125) - (anti_reverse_g6[3] * this.e5) - (this.e15 * this.e23) - (this.e25 * this.e31) - (this.e35 * this.e12) - 2.0 * (this.scalar * this.e3215), 
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
            float4(2.0)*(float4(this.e12345) * this.group9) + float4(2.0)*float4((float3(this.e3215) * this.group7.xyz), this.e31 * this.e315) + float4(2.0)*float4((this.group4.zxy * this.group8.yzx), this.e23 * this.e235) + float4(anti_reverse_g6[2] * this.e4315, anti_reverse_g6[0] * this.e4125, anti_reverse_g6[1] * this.e4235, 0.0) + (anti_reverse_g3 * float4(this.e4, this.e4, this.e4, this.e5)) + (float4(this.e5) * float4(this.group4.xyz, this.scalar)) + (float4(anti_reverse_g3[2], anti_reverse_g3[0], anti_reverse_g3[1], this.e12 * this.e125) * float4(this.group7.yzx, 1.0)) + (float4(anti_reverse_g6[0], anti_reverse_g6[1], anti_reverse_g6[2], this.e1) * this.group3.wwwx) + (this.group6 * float4(float3(anti_reverse_g3[3]), this.e3215)) + (this.group1.yzxy * float4(this.group5.zxy, this.e25)) + float4(float3(0.0), this.e3 * this.e35) + float4(float3(0.0), -2.0*(this.e235 * this.e4235) - 2.0 * (this.e315 * this.e4315) - 2.0 * (this.e125 * this.e4125)) + float4((float3(this.e321) * this.group5.xyz), 0.0) + float4((this.group7.zxy * this.group3.yzx), 0.0) + float4((this.group6.yzx * this.group9.zxy), 0.0) - (float4(anti_reverse_g6[3]) * float4(this.group5.xyz, this.e3215)) - (float4(this.e4, this.e4, this.e4, this.e5) * this.group3) - (float4(this.e1234, this.e1234, this.e1234, anti_reverse_g6[2] * this.e35) * float4(this.group8.xyz, 1.0)) - (anti_reverse_g3.yzxx * float4(this.group7.zxy, this.e1)) - (anti_reverse_g6.yzxx * float4(this.group9.zxy, this.e15)) - (this.group1.xyzy * float4(float3(this.scalar), anti_reverse_g3[1])) - (this.group1.zxyz * float4(this.group5.yzx, anti_reverse_g3[2])) - (this.group3.zxyy * float4(this.group7.yzx, anti_reverse_g6[1])) - (this.group6.zxyx * float4(this.group9.yzx, anti_reverse_g3[0])) - float4(float3(0.0), anti_reverse_g3[2] * this.e435) - float4((this.group4.yzx * this.group8.zxy), anti_reverse_g3[1] * this.e425), 
            /* e1234 */
            2.0*(this.scalar * this.e4) + 2.0 * (this.e12345 * this.e1234) + 2.0 * (this.e423 * this.e4235) + 2.0 * (this.e431 * this.e4315) + 2.0 * (this.e412 * this.e4125) + (anti_reverse_g6[3] * this.e1234) + (this.e4 * this.e45) + (this.e41 * this.e415) + (this.e42 * this.e425) + (this.e43 * this.e435) + 2.0 * (this.e23 * this.e423) + 2.0 * (this.e31 * this.e431) + 2.0 * (this.e12 * this.e412) - (anti_reverse_g3[3] * this.e4) - (anti_reverse_g6[0] * this.e41) - (anti_reverse_g6[1] * this.e42) - (anti_reverse_g6[2] * this.e43) - (this.e321 * this.e1234) - 2.0 * (this.e1 * this.e41) - 2.0 * (this.e2 * this.e42) - 2.0 * (this.e3 * this.e43)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*float4((float3(this.e4) * this.group2.xyz), 0.0) + (anti_reverse_g0 * float4(float3(this.e5), this.e4)) + (this.group3 * float4(float3(anti_reverse_g0[3]), this.e12345)) + (anti_reverse_g0.zxyx * float4(this.group2.yzx, this.e1)) + (this.group0.yzxx * float4(this.group2.zxy, this.e415)) + (this.group3.xyzy * float4(float3(this.e12345), anti_reverse_g0[1])) + (this.group3.zxyz * float4(this.group1.yzx, anti_reverse_g0[2])) + float4(float3(0.0), this.e431 * this.e425) + float4(float3(0.0), (this.e412 * this.e435) - (this.e412 * this.e3)) - (anti_reverse_g0.yzxx * float4(this.group2.zxy, this.e415)) - (this.group0.xxyx * float4(this.group2.wzx, this.e1)) - (this.group0.zyzy * float4(this.group2.yww, this.e2)) - (this.group1.zxyy * float4(this.group3.yzx, anti_reverse_g0[1])) - float4(float3(0.0), anti_reverse_g0[2] * this.e435) - float4(2.0)*(this.group1 * float4(float3(this.e321), this.e4)), 
            /* e5 */
            (anti_reverse_g0[3] * this.e5) + (this.e12345 * this.e5) + 2.0 * (this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125) + 2.0 * (this.e321 * this.e5) + 2.0 * (this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e125 * this.e3)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (anti_reverse_g1 * float4(float3(this.e45), this.e1234)) + (float4(this.e4235) * float4(anti_reverse_g0[3], this.e12, anti_reverse_g1[1], anti_reverse_g0[0])) + (float4(this.e4315) * float4(anti_reverse_g1[2], anti_reverse_g0[3], this.e23, anti_reverse_g0[1])) + (float4(this.e4125) * float4(this.e31, anti_reverse_g1[0], anti_reverse_g0[3], anti_reverse_g0[2])) + (float4(this.e25, this.e3215, this.e3215, anti_reverse_g1[1]) * this.group0.zyzy) + (float4(this.e3215, this.e35, this.e15, anti_reverse_g1[0]) * this.group0.xxyx) + (anti_reverse_g0.yzxx * float4(this.group2.zxy, this.e23)) + (this.group0.wwwz * float4(this.group3.xyz, anti_reverse_g1[2])) + (this.group1.xyzy * float4(float3(anti_reverse_g1[3]), anti_reverse_g0[1])) + float4(float3(0.0), anti_reverse_g0[2] * this.e12) - (anti_reverse_g0 * float4(float3(this.e3215), this.e1234)) - (float4(anti_reverse_g1[1], anti_reverse_g1[2], anti_reverse_g1[0], this.e41) * this.group3.zxyx) - (this.group2 * float4(float3(this.e1234), this.scalar)) - (this.group0.yzxy * float4(this.group2.zxy, this.e4315)) - (this.group3.yzxz * float4(this.group1.zxy, this.e43)) - float4(float3(0.0), this.e45 * this.e1234) - float4((anti_reverse_g0.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            (anti_reverse_g1[0] * this.e15) + (anti_reverse_g1[1] * this.e25) + (anti_reverse_g1[2] * this.e35) + (this.e45 * this.e3215) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) - (anti_reverse_g0[3] * this.e3215) - (anti_reverse_g1[3] * this.e3215) - (this.scalar * this.e3215) - (this.e23 * this.e15) - (this.e31 * this.e25) - (this.e12 * this.e35)
        );
    }
}
