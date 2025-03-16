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
            float4(2.0)*(this.group1.xyzx * float4(this.group1.www, this.e41)) + float4(this.e43 * this.e25 * -2.0, this.e41 * this.e35 * -2.0, this.e42 * this.e15 * -2.0, (this.e42 * this.e31) + (this.e43 * this.e12)) + (float4(this.group2.zxy[0], this.group2.zxy[1], this.group2.zxy[2] * this.e41, this.group1.xyzz[3] * this.e43) * float4(this.group0.xyz.yz, 1.0, 1.0)) + (float4(this.group2.zxy[0], this.group2.zxy[1], this.group2.zxy[2] * this.e41, this.group1.wwwy[3] * this.e42) * float4(this.group0.xyz.yz, 1.0, 1.0)), 
            /* e5 */
            2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0*(this.e412 * this.e315) + (this.group1.zyzw[0] * this.e2) + (this.e415 * this.e321) + (this.e425 * this.e3) + (this.e235 * this.e4) - (this.group1.yzx[0] * this.group3.zxy[0]) - (this.group1.zxy[0] * this.group3.yzx[0]) - (this.group2.zxy[0] * this.e431) - (this.group2.zyz[0] * this.e431), 2.0*(this.e423 * this.e125) + (this.group1.xxy[1] * this.e3) + (this.e425 * this.e321) + (this.e435 * this.e1) + (this.e315 * this.e4) - (this.group1.yzx[1] * this.group3.zxy[1]) - (this.group1.zxy[1] * this.group3.yzx[1]) - (this.group2.xxy[1] * this.e412) - (this.group2.zxy[1] * this.e412), 2.0*(this.e431 * this.e235) + (this.group1.zyzw[2] * this.e321) + (this.e412 * this.e5) + (this.e415 * this.e2) + (this.e435 * this.e321) + (this.e125 * this.e4) - (this.group1.zxy[2] * this.group3.yzx[2]) - (this.group2.xxy[2] * this.e423) - (this.group2.zxy[2] * this.e423) - (this.group2.zyz[2] * this.e4) - (this.group3.wwww[2] * this.e412), -2.0*(this.e423 * this.e415) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e412 * this.e435)) + (float4(this.group1.xxy[0], this.group1.zyzw[1], this.group1.xxy[2] * this.e1, this.group1.zyzw[3] * this.e4) * float4(this.group1.ww, 1.0, 1.0)) - (float4(this.group2.xxy[0], this.group2.zyz[1], this.group1.yzx[2] * this.group3.zxy[2], this.e321 * this.e4) * float4(this.group2.ww, 1.0, 1.0)), 
            /* e5 */
            -2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125)
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
            float2(this.e3215 * this.scalar, 0.0) * float2(-2.0, 0.0)
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
            float2(2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35), 0.0)
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
            float2(2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35) - 2.0 * (this.scalar * this.e3215), 0.0)
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
            float4(2.0*(this.e412 * this.e315) + 2.0 * (this.e415 * this.e321), 2.0*(this.e423 * this.e125) + 2.0 * (this.e425 * this.e321), 2.0*(this.e431 * this.e235) + 2.0 * (this.e435 * this.e321), -2.0*(this.e431 * this.e425) - 2.0 * (this.e412 * this.e435)) - float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e423 * this.e415), 
            /* e5 */
            -2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125)
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
            float4(2.0*(this.e412 * this.e315) + 2.0 * (this.e415 * this.e321) - (this.group2.zxy[0] * this.e431) - (this.group2.zxy[0] * this.e431), 2.0*(this.e423 * this.e125) + 2.0 * (this.e425 * this.e321) - (this.group2.zxy[1] * this.e412) - (this.group2.zxy[1] * this.e412), 2.0*(this.e431 * this.e235) + 2.0 * (this.e435 * this.e321) - (this.group2.zxy[2] * this.e423) - (this.group2.zxy[2] * this.e423), -2.0*(this.e423 * this.e415) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e412 * this.e435)), 
            /* e5 */
            -2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125)
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
            float4(2.0)*(this.group1.xyzz * float4(this.group1.www, this.e43)) + float4(2.0)*float4((this.group0.yzx * this.group2.zxy), this.e42 * this.e31) + float4(this.e43 * this.e25 * -2.0, this.e41 * this.e35 * -2.0, this.e42 * this.e15 * -2.0, (this.group1.wwwx[3] * this.e41) + (this.e41 * this.e23)), 
            /* e5 */
            2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.group1.yzx[0] * this.group3.zxy[0]) + (this.group1.zyz[0] * this.e4315) + (this.group2.xxy[0] * this.e1234) + (this.group2.zxy[0] * this.e42) + (this.group2.zyz[0] * this.e42) + (this.e23 * this.e45) - (this.group2.yzx[0] * this.e43) - (this.e43 * this.e25) - (this.e12 * this.e4315) - (this.e15 * this.e1234), (this.group1.xxy[1] * this.e4125) + (this.group1.yzx[1] * this.group3.zxy[1]) + (this.group2.xxy[1] * this.e43) + (this.group2.zxy[1] * this.e43) + (this.group2.zyz[1] * this.e1234) + (this.e31 * this.e45) - (this.group2.yzx[1] * this.e41) - (this.e41 * this.e35) - (this.e23 * this.e4125) - (this.e25 * this.e1234), (this.group1.yzx[2] * this.group3.zxy[2]) + (this.group1.zyz[2] * this.e45) + (this.group2.xxy[2] * this.e41) + (this.group2.zxy[2] * this.e41) + (this.group2.zyz[2] * this.e1234) + (this.group1.wwww[2] * this.e12) + (this.group3.wwww[2] * this.e43) - (this.group2.yzx[2] * this.e42) - (this.e42 * this.e15) - (this.e43 * this.e3215) - (this.e31 * this.e4235) - (this.e35 * this.e1234), 2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12)) + (float4(this.group1.xxy[0], this.group1.zyz[1], this.group1.xxy[2] * this.e4235, this.e42 * this.e4315) * float4(this.group1.ww, 1.0, 1.0)) - (float4(this.group1.yzx[0], this.group1.yzx[1], this.group1.yzx[2] * this.group3.zxyy[2], this.group3.zxyy[3] * this.e42) * float4(this.group3.zx, 1.0, 1.0)), 
            /* e5 */
            2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35)
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
            float2(this.e5 * this.e12345, 0.0) * float2(2.0, 0.0)
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
            float2(-2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125), 0.0)
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
            float2(2.0*(this.e12345 * this.e5) - 2.0 * (this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125), 0.0)
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
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(this.group1.xwzw[0] * this.e4235) + 2.0 * (this.scalar * this.e12345) + 2.0 * (this.e2 * this.e4315) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) + 2.0 * (this.e5 * this.e1234) - (this.group3.xwzw[0] * this.e423) - (this.e15 * this.e423) - 2.0 * (this.e25 * this.e431) - 2.0 * (this.e35 * this.e412) - 2.0 * (this.e45 * this.e321) - 2.0 * (this.e41 * this.e235) - 2.0 * (this.e42 * this.e315) - 2.0 * (this.e43 * this.e125) - 2.0 * (this.e23 * this.e415) - 2.0 * (this.e31 * this.e425) - 2.0 * (this.e12 * this.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.group0.xx, this.scalar, this.e12345 * this.e4) * float4(this.group9.xyz, 1.0)) + float4(2.0)*(float4(this.group0.yy, this.e12345, this.e41 * this.e23) * float4(this.group1.xyz, 1.0)) + float4(2.0)*float4((this.group5.xyz.xy * this.group3.ww), this.e431 * this.e235, this.e42 * this.e31) + float4(2.0*(this.e412 * this.e315) + (this.group1.zxy[0] * this.group6.yzx[0]) + (this.group3.xxy[0] * this.e1234) + (this.group6.zyz[0] * this.e2) + (this.e31 * this.e4125) + (this.e415 * this.e321) - (this.group9.zxy[0] * this.e31) - (this.e15 * this.e1234) - (this.e25 * this.e43), 2.0*(this.e423 * this.e125) + (this.group1.zxy[1] * this.group6.yzx[1]) + (this.group3.zyz[1] * this.e1234) + (this.group6.xxy[1] * this.e3) + (this.e12 * this.e4235) + (this.e425 * this.e321) - (this.group9.zxy[1] * this.e12) - (this.e25 * this.e1234) - (this.e35 * this.e41), (this.group3.zyz[2] * this.e1234) + (this.group6.xxy[2] * this.e1) + (this.group6.zyz[2] * this.e321) + (this.group3.wwww[2] * this.e12) + (this.group6.wwww[2] * this.e435) + (this.e45 * this.e12) + (this.e23 * this.e4315) - (this.group9.zxy[2] * this.e23) - (this.e15 * this.e42) - (this.e35 * this.e1234), 2.0*(this.e43 * this.e12) - 2.0 * (this.scalar * this.e1234) - 2.0 * (this.e425 * this.e431) - 2.0 * (this.e435 * this.e412)) + (float4(this.group3.zxy[0], this.group3.xxy[1], this.group1.zxy[2] * this.group6.yzx[2], this.e2 * this.e431) * float4(this.group4.xyz.yz, 1.0, 1.0)) + (float4(this.group3.zyz[0], this.group3.zxy[1], this.group3.xxy[2] * this.e41, this.e3 * this.e412) * float4(this.group4.xyz.yz, 1.0, 1.0)) + (float4(this.group6.xxy[0], this.group6.zyz[1], this.group3.zxy[2] * this.e41, this.e41 * this.e4235) * float4(this.group6.ww, 1.0, 1.0)) - (float4(this.group3.yzx[0], this.group3.yzx[1], this.group3.yzx[2] * this.e42, this.group1.yzxz[3] * this.e412) * float4(this.group4.xyz.zx, 1.0, 1.0)) - (float4(this.group6.yzx[0], this.group6.yzx[1], this.group6.yzx[2] * this.group1.zxyy[2], this.group1.zxyy[3] * this.e431) * float4(this.group1.zx, 1.0, 1.0)) - (float4(this.group6.zxy[0], this.group6.zxy[1], this.group6.zxy[2] * this.group1.yzxz[2], this.group9.yzxx[3] * this.e41) * float4(this.group1.yz, 1.0, 1.0)) - float4(2.0)*float4((this.group7.yzx * this.group8.zxy), this.e415 * this.e423), 
            /* e5 */
            2.0*(this.e12345 * this.e5) + 2.0 * (this.e15 * this.e23) + 2.0 * (this.e25 * this.e31) + 2.0 * (this.e35 * this.e12) - 2.0 * (this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125) - 2.0 * (this.scalar * this.e3215), 
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
            float4(2.0)*(float4(this.group0.yy, this.e12345, this.scalar * this.e5) * float4(this.group9.xyz, 1.0)) + float4(2.0)*float4((this.group4.yzx * this.group8.zxy), this.e12345 * this.e3215) + float4((this.group3.xxy[0] * this.e4) + (this.group3.zxy[0] * this.e431) + (this.group3.zyz[0] * this.e431) + (this.group3.wwww[0] * this.e415) + (this.e45 * this.e415) + (this.e425 * this.e4125) + (this.e435 * this.e4315) - (this.group6.yzx[0] * this.group9.zxy[0]) - (this.group6.zxy[0] * this.group9.yzx[0]) - 2.0 * (this.scalar * this.e1), (this.group3.xxy[1] * this.e412) + (this.group3.zxy[1] * this.e412) + (this.group3.zyz[1] * this.e4) + (this.group3.wwww[1] * this.e425) + (this.e45 * this.e425) + (this.e415 * this.e4125) + (this.e435 * this.e4235) - (this.group6.yzx[1] * this.group9.zxy[1]) - (this.group6.zxy[1] * this.group9.yzx[1]) - 2.0 * (this.scalar * this.e2), (this.group3.xxy[2] * this.e423) + (this.group3.zxy[2] * this.e423) + (this.group3.zyz[2] * this.e4) + (this.group3.wwww[2] * this.e435) + (this.e45 * this.e435) + (this.e415 * this.e4315) + (this.e425 * this.e4235) - (this.group1.wwww[2] * this.e35) - (this.group6.wwww[2] * this.e12) - (this.e2 * this.e23) - (this.e12 * this.e321), -(this.e15 * this.e415) - 2.0 * (this.e31 * this.e315) - 2.0 * (this.e12 * this.e125)) + (float4(this.group1.zxy[0], this.group1.zxy[1], this.group1.zxy[2] * this.e23, this.e3 * this.e35) * float4(this.group5.xyz.yz, 1.0, 1.0)) - float4((this.group5.xyz.yz * this.group1.zx), this.group6.yzx[2] * this.group9.zxy[2], this.group1.zxyz[3] * this.e35) - float4((this.group3.xy * this.group1.ww), this.group6.zxy[2] * this.group9.yzx[2], this.group3.yzxx[3] * this.e415) - float4(2.0)*(this.group3.yzxy * float4(this.group7.zxy, this.e425)) - float4(2.0)*float4((this.group5.xyz.xy * this.group6.ww), this.scalar * this.e3, this.e23 * this.e235) - float4(2.0)*float4((this.group4.zxy * this.group8.yzx), this.e35 * this.e435), 
            /* e1234 */
            2.0*(this.scalar * this.e4) + 2.0 * (this.e12345 * this.e1234) - 2.0 * (this.e41 * this.e415) - 2.0 * (this.e42 * this.e425) - 2.0 * (this.e43 * this.e435) - 2.0 * (this.e23 * this.e423) - 2.0 * (this.e31 * this.e431) - 2.0 * (this.e12 * this.e412)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0)*(this.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4(2.0*(this.e12345 * this.e1) + (this.e235 * this.e4) - (this.group1.yzx[0] * this.group3.zxy[0]) - (this.group1.zxy[0] * this.group3.yzx[0]), 2.0*(this.e12345 * this.e2) + (this.e315 * this.e4) - (this.group1.yzx[1] * this.group3.zxy[1]) - (this.group1.zxy[1] * this.group3.yzx[1]), 2.0*(this.e12345 * this.e3) + (this.e415 * this.e2) + (this.e125 * this.e4) - (this.group2.zxy[2] * this.group0.yzxx[2]) - (this.group2.zyz[2] * this.e4) - (this.group0.xyzy[2] * this.group2.wwww[2]), -(this.e423 * this.e1) - (this.e431 * this.e425) - (this.e431 * this.e2) - (this.e412 * this.e3) - (this.e321 * this.e4) - 2.0 * (this.e412 * this.e435)) + (float4(this.group1.xxy[0], this.group1.zyzw[1], this.group1.xxy[2] * this.e1, this.group0.xxyx[3] * this.e1) * float4(this.group1.ww, 1.0, 1.0)) + (float4(this.group1.zyzw[0], this.group1.xxy[1], this.group1.zyzw[2] * this.e321, this.group0.zyzy[3] * this.e2) * float4(this.group3.yz, 1.0, 1.0)) + float4((this.group1.yz * this.group3.zx), this.e412 * this.e5, this.group1.zyzw[3] * this.e4) + float4((this.group1.xyz * this.group1.www), this.group0.wwwz[3] * this.group3.xyzz[3]) - (float4(this.group2.xxy[0], this.group2.zyz[1], this.group1.yzx[2] * this.group3.zxy[2], this.group0.xyzy[3] * this.e425) * float4(this.group3.ww, 1.0, 1.0)) - (float4(this.group2.zxy[0], this.group2.xxy[1], this.group1.zxy[2] * this.group3.yzx[2], this.group0.yzxx[3] * this.e415) * float4(this.group0.yz, 1.0, 1.0)) - (float4(this.group2.zyz[0], this.group2.zxy[1], this.group2.xxy[2], this.e415) * this.group0.yzxx), 
            /* e5 */
            2.0*(this.e12345 * this.e5) - 2.0 * (this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(2.0*(this.scalar * this.e4235) + (this.group1.yzx[0] * this.group3.zxy[0]) - (this.group1.yzx[0] * this.group3.zxy[0]) - (this.e15 * this.e1234), 2.0*(this.scalar * this.e4315) + (this.group1.yzx[1] * this.group3.zxy[1]) - (this.group1.yzx[1] * this.group3.zxy[1]) - (this.e25 * this.e1234), (this.group0.wwwx[2] * this.group3.xyzx[2]) + (this.group0.wwwz[2] * this.e4125) + (this.group1.xyzz[2] * this.group1.wwww[2]) - (this.group0.xxyw[2] * this.e15) - (this.group2.wwww[2] * this.e35), this.e43 * this.e4125) + (float4(this.group1.xxy[0], this.group1.zyz[1], this.group1.xxy[2] * this.e4235, this.group0.xyzy[3] * this.e31) * float4(this.group1.ww, 1.0, 1.0)) + (float4(this.group1.zyz[0], this.group1.xxy[1], this.group1.yzx[2] * this.group3.zxy[2], this.group0.yzxx[3] * this.e23) * float4(this.group3.yz, 1.0, 1.0)) + (float4(this.group2.xxy[0], this.group2.zyz[1], this.group1.zyz[2] * this.e45, this.group0.wwwx[3] * this.group3.xyzx[3]) * float4(this.group2.ww, 1.0, 1.0)) + (float4(this.group2.zxy[0], this.group2.xxy[1], this.group2.xxy[2] * this.e41, this.group0.wwwz[3] * this.e12) * float4(this.group0.yz, 1.0, 1.0)) + (float4(this.group2.zyz[0], this.group2.zxy[1], this.group2.zxy[2] * this.group0.yzxx[2], this.group1.xyzz[3] * this.e43) * float4(this.group0.yz, 1.0, 1.0)) + float4((this.group0.xy * this.group3.ww), this.group2.zyz[2] * this.e1234, this.e41 * this.e23) + float4((this.group1.xy * this.group1.ww), this.group0.xyzy[2] * this.group3.wwww[2], this.e42 * this.e31) - (float4(this.group0.zyz[0], this.group0.xxyw[1], this.group0.zyz[2] * this.e3215, this.group0.xxyw[3] * this.e1234) * float4(this.group2.yz, 1.0, 1.0)) - (float4(this.group1.zxy[0], this.group0.zyz[1], this.group1.yzx[2] * this.group3.zxy[2], this.group0.zxyx[3] * this.e4235) * float4(this.group3.yw, 1.0, 1.0)) - (float4(this.group2.yzx[0], this.group2.yzx[1], this.group1.zxy[2] * this.group3.yzxz[2], this.group3.yzxz[3] * this.e43) * float4(this.group0.zx, 1.0, 1.0)) - (float4(this.group0.xxyw[0], this.group1.zxy[1], this.group2.yzx[2] * this.group0.zxyx[2], this.scalar * this.e1234) * float4(this.group3.wz, 1.0, 1.0)), 
            /* e5 */
            2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35) - 2.0 * (this.scalar * this.e3215)
        );
    }
}
