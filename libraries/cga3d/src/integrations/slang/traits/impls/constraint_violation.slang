implementing cga3d;
// Fancy postfix trick
public Sphere operator /(AntiCircleRotor lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.constraint_violation();
}
public extension AntiCircleRotor {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((reverse_g0[1] * this.e35) + (reverse_g1[0] * this.e45) + (reverse_g1[3] * this.e23) + (reverse_g2[2] * this.e42), (reverse_g0[2] * this.e15) + (reverse_g1[1] * this.e45) + (reverse_g1[3] * this.e31) + (reverse_g2[0] * this.e43), (reverse_g0[0] * this.e25) + (reverse_g1[2] * this.e45) + (reverse_g1[3] * this.e12) + (reverse_g2[1] * this.e41), -(reverse_g1[2] * this.e35) - (reverse_g2[0] * this.e23) - (reverse_g2[1] * this.e31) - (reverse_g2[2] * this.e12)) - (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) - float4((this.group0.zxy * reverse_g2.yzx), reverse_g1[1] * this.e25), 
            /* e1234 */
            -(reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(AntiDipoleInversion lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.constraint_violation();
}
public extension AntiDipoleInversion {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse_g2[3] * this.e235) - (this.e425 * this.e3), -(reverse_g2[3] * this.e315) - (this.e435 * this.e1), -(reverse_g2[3] * this.e125) - (this.e415 * this.e2), (this.e321 * this.e5) + (this.e125 * this.e3)) + (float4(reverse_g2[1], this.e5, this.e5, this.e125) * float4(this.group0.zyz, reverse_g1[2])) + (float4(this.e5, reverse_g2[2], reverse_g2[0], this.e315) * float4(this.group0.xxy, reverse_g1[1])) + (this.group1.xyzz * float4(reverse_g1.www, reverse_g2[2])) + (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) + (this.group2.wwwx * float4(reverse_g2.xyz, this.e1)) + (this.group3.yzxy * float4(this.group1.zxy, this.e315)) + (float4(this.group1.ww, this.e2, this.e415) * float4(reverse_g1.xyx, reverse_g2[0])) + (float4(this.group3.zx, this.e321, this.e425) * float4(reverse_g1.yzz, reverse_g2[1])) - (this.group3.yzxz * float4(reverse_g1.zxy, reverse_g2[2])) - (float4(this.group2.zx, this.e5, this.e1) * float4(reverse_g0.yzz, reverse_g2[0])) - (float4(this.group3.ww, this.e315, this.e5) * float4(reverse_g0.xyx, reverse_g1[3])) - float4((this.group0.yzx * reverse_g2.zxy), reverse_g2[1] * this.e2), 
            /* e1234 */
            (reverse_g0[0] * this.e415) + (reverse_g0[0] * this.e1) + (reverse_g0[1] * this.e425) + (reverse_g0[1] * this.e2) + (reverse_g0[2] * this.e435) + (reverse_g0[2] * this.e3) + (reverse_g1[0] * this.e423) + (reverse_g1[1] * this.e431) + (reverse_g1[2] * this.e412) + (reverse_g1[3] * this.e4) - (reverse_g2[3] * this.e321) - (this.e423 * this.e1) - (this.e431 * this.e2) - (this.e412 * this.e3)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.constraint_violation();
}
public extension AntiDualNum {
    public AntiDualNum constraint_violation() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215 * this.scalar * 2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiFlector lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.constraint_violation();
}
public extension AntiFlector {
    public AntiDualNum constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * this.e1) + (this.e315 * this.e2) + (this.e125 * this.e3) + (this.e321 * this.e5) - (reverse_g0[0] * this.e1) - (reverse_g0[1] * this.e2) - (reverse_g0[2] * this.e3) - (reverse_g0[3] * this.e5), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiLine lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.constraint_violation();
}
public extension AntiLine {
    public AntiDualNum constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(reverse_g0[0] * this.e15) - (reverse_g0[1] * this.e25) - (reverse_g0[2] * this.e35) - (reverse_g1[0] * this.e23) - (reverse_g1[1] * this.e31) - (reverse_g1[2] * this.e12), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiMotor lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.constraint_violation();
}
public extension AntiMotor {
    public AntiDualNum constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse_g0[3] * this.e3215) + (reverse_g1[3] * this.scalar) - (reverse_g0[0] * this.e15) - (reverse_g0[1] * this.e25) - (reverse_g0[2] * this.e35) - (reverse_g1[0] * this.e23) - (reverse_g1[1] * this.e31) - (reverse_g1[2] * this.e12), 0.0)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Circle lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, Circle rhs) {
    return rhs.constraint_violation();
}
public extension Circle {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse_g0[1] * this.e125) - (reverse_g2[2] * this.e431), -(reverse_g0[2] * this.e235) - (reverse_g2[0] * this.e412), -(reverse_g0[0] * this.e315) - (reverse_g2[1] * this.e423), (reverse_g1[1] * this.e315) + (reverse_g1[2] * this.e125)) + (this.group1.wwwz * float4(reverse_g1.xyz, reverse_g2[2])) + float4((reverse_g0.zxy * this.group2.yzx), reverse_g2[0] * this.e415) + float4((reverse_g2.yzx * this.group0.zxy), reverse_g2[1] * this.e425) + float4((this.group1.xyz * reverse_g1.www), reverse_g1[0] * this.e235), 
            /* e1234 */
            (reverse_g0[0] * this.e415) + (reverse_g0[1] * this.e425) + (reverse_g0[2] * this.e435) + (reverse_g1[0] * this.e423) + (reverse_g1[1] * this.e431) + (reverse_g1[2] * this.e412)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(CircleRotor lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.constraint_violation();
}
public extension CircleRotor {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(reverse_g0[1] * this.e125) - (reverse_g2[2] * this.e431), -(reverse_g0[2] * this.e235) - (reverse_g2[0] * this.e412), -(reverse_g0[0] * this.e315) - (reverse_g2[1] * this.e423), (reverse_g2[1] * this.e425) + (reverse_g2[2] * this.e435)) + (reverse_g1.xyzz * float4(this.group1.www, this.e125)) + (this.group1.xyzx * float4(reverse_g1.www, reverse_g2[0])) + (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) + float4((this.group0.zxy * reverse_g2.yzx), reverse_g1[1] * this.e315), 
            /* e1234 */
            (reverse_g0[0] * this.e415) + (reverse_g0[1] * this.e425) + (reverse_g0[2] * this.e435) + (reverse_g1[0] * this.e423) + (reverse_g1[1] * this.e431) + (reverse_g1[2] * this.e412)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Dipole lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.constraint_violation();
}
public extension Dipole {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((reverse_g0[1] * this.e35) + (reverse_g2[2] * this.e42) + (reverse_g1[0] * this.e45) + (reverse_g1[3] * this.e23), (reverse_g0[2] * this.e15) + (reverse_g2[0] * this.e43) + (reverse_g1[1] * this.e45) + (reverse_g1[3] * this.e31), (reverse_g0[0] * this.e25) + (reverse_g2[1] * this.e41) + (reverse_g1[2] * this.e45) + (reverse_g1[3] * this.e12), -(reverse_g2[2] * this.e12) - (reverse_g1[0] * this.e15) - (reverse_g1[1] * this.e25) - (reverse_g1[2] * this.e35)) - float4((reverse_g0.zxy * this.group2.yzx), reverse_g2[0] * this.e23) - float4((reverse_g2.yzx * this.group0.zxy), reverse_g2[1] * this.e31), 
            /* e1234 */
            -(reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(DipoleInversion lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.constraint_violation();
}
public extension DipoleInversion {
    public Sphere constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((reverse_g1[1] * this.e4125) + (reverse_g1[3] * this.e23) + (reverse_g2[3] * this.e15) + (this.e12 * this.e4315), (reverse_g1[2] * this.e4235) + (reverse_g1[3] * this.e31) + (reverse_g2[3] * this.e25) + (this.e23 * this.e4125), (reverse_g1[2] * this.e45) + (reverse_g1[3] * this.e12) + (reverse_g2[3] * this.e35) + (this.e31 * this.e4235), -(reverse_g2[1] * this.e31) - (reverse_g2[1] * this.e4315) - (reverse_g2[2] * this.e12) - (reverse_g2[2] * this.e4125)) + (float4(this.group1.ww, this.e4315, this.e45) * float4(reverse_g1.xyx, this.e3215)) + (float4(this.group2.zx, this.e3215, this.e25) * float4(reverse_g0.yzz, this.e4315)) + (float4(this.group3.ww, this.e25, this.e15) * float4(reverse_g0.xyx, this.e4235)) + float4((this.group0.yzx * reverse_g2.zxy), this.e35 * this.e4125) - (float4(reverse_g2[1], this.e3215, this.e3215, this.e35) * float4(this.group0.zyz, reverse_g1[2])) - (float4(this.e3215, reverse_g2[2], reverse_g2[0], this.e25) * float4(this.group0.xxy, reverse_g1[1])) - (reverse_g1.zxyw * this.group3.yzxw) - (reverse_g2.xyzx * float4(this.group2.www, this.e23)) - (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) - float4((this.group1.yzx * this.group3.zxy), reverse_g2[0] * this.e4235), 
            /* e1234 */
            (reverse_g0[0] * this.e4235) + (reverse_g0[1] * this.e4315) + (reverse_g0[2] * this.e4125) + (reverse_g1[3] * this.e1234) - (reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43) - (reverse_g2[3] * this.e45) - (this.e41 * this.e4235) - (this.e42 * this.e4315) - (this.e43 * this.e4125)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(DualNum lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.constraint_violation();
}
public extension DualNum {
    public AntiDualNum constraint_violation() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * this.e12345 * -2.0, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Flector lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.constraint_violation();
}
public extension Flector {
    public AntiDualNum constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) + (this.e45 * this.e3215) - (reverse_g0[0] * this.e4235) - (reverse_g0[1] * this.e4315) - (reverse_g0[2] * this.e4125) - (reverse_g0[3] * this.e3215), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Line lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.constraint_violation();
}
public extension Line {
    public AntiDualNum constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse_g0[0] * this.e235) + (reverse_g0[1] * this.e315) + (reverse_g0[2] * this.e125) + (reverse_g1[0] * this.e415) + (reverse_g1[1] * this.e425) + (reverse_g1[2] * this.e435), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Motor lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.constraint_violation();
}
public extension Motor {
    public AntiDualNum constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((reverse_g0[0] * this.e235) + (reverse_g0[1] * this.e315) + (reverse_g0[2] * this.e125) + (reverse_g1[0] * this.e415) + (reverse_g1[1] * this.e425) + (reverse_g1[2] * this.e435) - (reverse_g0[3] * this.e5) - (reverse_g1[3] * this.e12345), 0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public MultiVector operator /(ConstraintViolationPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.constraint_violation();
}
public extension MultiVector {
    public MultiVector constraint_violation() {
        float4 reverse_g3 = this.group3 * float4(-1.0);
        float3 reverse_g4 = this.group4.xyz * float3(-1.0);
        float3 reverse_g5 = this.group5.xyz * float3(-1.0);
        float4 reverse_g6 = this.group6 * float4(-1.0);
        float3 reverse_g7 = this.group7.xyz * float3(-1.0);
        float3 reverse_g8 = this.group8.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, 2.0*(this.e5 * this.e1234) + (this.group9.wxzw[1] * this.e1) + 2.0 * (this.scalar * this.e12345) + (this.e1 * this.e4235) + 2.0 * (this.e2 * this.e4315) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) - (reverse_g4[0] * this.e235) - (reverse_g4[1] * this.e315) - (reverse_g4[2] * this.e125) - (reverse_g5[0] * this.e415) - (reverse_g5[1] * this.e425) - (reverse_g5[2] * this.e435) - (reverse_g7[0] * this.e15) - (reverse_g7[1] * this.e25) - (reverse_g7[2] * this.e35) - (reverse_g8[0] * this.e41) - (reverse_g8[1] * this.e42) - (reverse_g8[2] * this.e43) - (reverse_g3[0] * this.e423) - (reverse_g3[1] * this.e431) - (reverse_g3[2] * this.e412) - (reverse_g3[3] * this.e321) - (reverse_g6[1] * this.e31) - (reverse_g6[2] * this.e12) - (reverse_g6[3] * this.e45) - (reverse_g6.wxzw[1] * this.e23)), 
            /* e1, e2, e3, e4 */
            float4((reverse_g3[0] * this.e4) + (reverse_g3[1] * this.e412) + (reverse_g6[2] * this.e4315) + (this.e425 * this.e4125), (reverse_g3[1] * this.e4) + (reverse_g6[0] * this.e4125) + (this.e435 * this.e4235) + (this.e431 * this.e3215), (reverse_g3[2] * this.e4) + (reverse_g6[1] * this.e4235) + (this.e415 * this.e4315) + (this.e412 * this.e3215), -(reverse_g6[3] * this.e1234) - (this.e4 * this.e45) - (this.e431 * this.e4315) - (this.e412 * this.e4125)) + (float4(this.scalar) * this.group1) + (float4(reverse_g6[3], this.e3, this.e1, this.e3) * float4(this.group5.xxy, this.e43)) + (float4(this.e2, reverse_g6[3], reverse_g6[3], this.e4) * float4(this.group5.zyz, reverse_g3[3])) + (float4(this.e3215, reverse_g3[2], reverse_g3[0], this.e1234) * float4(this.group7.xxy, this.e321)) + (float4(this.e1234, this.e1234, this.e42, this.e4125) * float4(reverse_g8.xyx, reverse_g7[2])) + (float4(this.group0.xx, this.scalar, this.e12345) * float4(this.group1.xyz, this.e1234)) + (float4(this.group4.xyz.zx, this.e1234, this.e1) * float4(reverse_g8.yzz, this.e41)) + (float4(this.group1.zx, this.e321, this.e4235) * float4(reverse_g5.yzz, reverse_g7[0])) + (float4(this.group6.ww, this.e2, this.e1234) * float4(reverse_g5.xyx, this.e12345)) + float4((float3(this.e5) * this.group4.xyz), this.e2 * this.e42) + float4((reverse_g4.zxy * this.group8.yzx), this.scalar * this.e4) + float4((reverse_g7.zxy * this.group3.yzx), reverse_g7[1] * this.e4315) - (float4(this.e5, this.e5, this.e315, this.e2) * float4(reverse_g4.xyx, reverse_g4[1])) - (this.group1.yzxz * float4(reverse_g5.zxy, reverse_g4[2])) - (float4(this.group0.yy, this.e12345, reverse_g4[0]) * float4(this.group9.xyz, this.e1)) - (float4(this.group0.yy, this.e12345, reverse_g4[0]) * float4(this.group9.xyz, this.e415)) - (float4(this.group8.xyz.zx, this.e5, this.e425) * float4(reverse_g4.yzz, reverse_g4[1])) - (float4(this.group3.zx, this.e3215, this.e423) * float4(reverse_g7.yzz, reverse_g5[0])) - (float4(this.group3.ww, this.e4315, reverse_g6[1]) * float4(reverse_g6.xyx, this.e42)) - (float4(this.group9.zx, this.e45, reverse_g6[2]) * float4(reverse_g6.yzz, this.e43)) - (float4(this.group9.ww, this.e25, this.e435) * float4(reverse_g7.xyx, reverse_g4[2])) - float4((float3(this.e1234) * this.group8.xyz), reverse_g7[1] * this.e31) - float4((reverse_g8.zxy * this.group4.yzx), reverse_g5[1] * this.e431) - float4((this.group5.yzx * this.group1.zxy), reverse_g5[2] * this.e412) - float4((this.group7.yzx * reverse_g3.zxy), reverse_g7[0] * this.e23) - float4((this.group6.zxy * this.group9.yzx), this.e423 * this.e4235) - float4((this.group3.xyz * this.group1.www), reverse_g7[2] * this.e12) - float4((this.group6.xyz * reverse_g3.www), reverse_g6[0] * this.e41), 
            /* e5 */
            (reverse_g3[0] * this.e1) + (reverse_g3[1] * this.e2) + (reverse_g3[2] * this.e3) + (reverse_g6[3] * this.e3215) + 2.0 * (this.scalar * this.e5) + 2.0 * (this.e12345 * this.e3215) + (this.e5 * this.e45) + (this.e235 * this.e4235) + (this.e315 * this.e4315) + (this.e125 * this.e4125) - (reverse_g5[0] * this.e235) - (reverse_g5[1] * this.e315) - (reverse_g5[2] * this.e125) - (reverse_g8[0] * this.e23) - (reverse_g8[0] * this.e4235) - (reverse_g8[1] * this.e31) - (reverse_g8[1] * this.e4315) - (reverse_g8[2] * this.e12) - (reverse_g8[2] * this.e4125) - (reverse_g3[0] * this.e415) - (reverse_g3[1] * this.e425) - (reverse_g3[2] * this.e435) - (reverse_g3[3] * this.e5) - (reverse_g6[0] * this.e15) - (reverse_g6[1] * this.e25) - (reverse_g6[2] * this.e35) - (this.e1 * this.e15) - (this.e2 * this.e25) - (this.e3 * this.e35) - (this.e321 * this.e3215), 
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
            float4((reverse_g6[0] * this.e321) + (reverse_g6[1] * this.e3) + (reverse_g6[3] * this.e415) + (this.e15 * this.e1234), (reverse_g6[1] * this.e321) + (reverse_g6[2] * this.e1) + (reverse_g6[3] * this.e425) + (this.e25 * this.e1234), (reverse_g6[0] * this.e2) + (reverse_g6[2] * this.e321) + (reverse_g6[3] * this.e435) + (this.e35 * this.e1234), -(reverse_g3[1] * this.e4315) - (reverse_g3[2] * this.e4125) - (reverse_g3[3] * this.e3215) - (reverse_g6[3] * this.e5)) + (float4(this.scalar) * this.group9) + (float4(reverse_g3[3], this.e4125, this.e4235, this.e25) * float4(this.group5.xxy, this.e4315)) + (float4(this.e4315, reverse_g3[3], reverse_g3[3], this.e35) * float4(this.group5.zyz, this.e4125)) + (float4(this.group0.xx, this.scalar, reverse_g8[0]) * float4(this.group9.xyz, this.e415)) + (float4(this.group0.yy, this.e12345, reverse_g8[1]) * float4(this.group1.xyz, this.e425)) + (float4(this.group0.yy, this.e12345, this.scalar) * float4(this.group1.xyz, this.e3215)) + (float4(this.group7.xyz.zx, this.e4, reverse_g6[2]) * float4(reverse_g8.yzz, this.e125)) + (float4(this.group1.ww, this.e431, this.e3) * float4(reverse_g8.xyx, this.e125)) + (float4(this.group3.zx, this.e3215, this.e1) * float4(reverse_g4.yzz, this.e235)) + (float4(this.group3.ww, this.e4315, reverse_g6[0]) * float4(reverse_g5.xyx, this.e235)) + (float4(this.group9.zx, this.e45, this.e2) * float4(reverse_g5.yzz, this.e315)) + (float4(this.group9.ww, this.e25, this.e435) * float4(reverse_g4.xyx, reverse_g8[2])) + float4((float3(this.e5) * this.group7.xyz), this.e45 * this.e3215) + float4((reverse_g7.zxy * this.group8.yzx), reverse_g6[1] * this.e315) + float4((this.group4.yzx * reverse_g3.zxy), this.e15 * this.e4235) + float4((this.group1.yzx * this.group6.zxy), this.e5 * this.e321) - (float4(reverse_g3[1], this.e3215, this.e3215, this.e2) * float4(this.group4.zyz, reverse_g8[1])) - (float4(this.e5, this.e5, this.e315, this.e15) * float4(reverse_g7.xyx, reverse_g5[0])) - (float4(this.e3215, reverse_g3[2], reverse_g3[0], this.e1) * float4(this.group4.xxy, reverse_g8[0])) - (float4(this.group8.xyz.zx, this.e5, this.e25) * float4(reverse_g7.yzz, reverse_g5[1])) - float4((float3(this.e1234) * reverse_g3.xyz), reverse_g3[2] * this.e12) - float4((this.group8.xyz * this.group1.www), reverse_g3[0] * this.e23) - float4((reverse_g4.zxy * this.group3.yzx), this.e12345 * this.e5) - float4((reverse_g5.zxy * this.group9.yzx), this.e12345 * this.e5) - float4((reverse_g8.zxy * this.group7.yzx), reverse_g5[2] * this.e35) - float4((this.group5.yzx * this.group9.zxy), reverse_g8[2] * this.e3) - float4((reverse_g6.zxy * this.group1.yzx), reverse_g3[0] * this.e4235) - float4((this.group1.zxy * this.group6.yzx), reverse_g3[1] * this.e31), 
            /* e1234 */
            (reverse_g4[0] * this.e4235) + (reverse_g4[1] * this.e4315) + (reverse_g4[2] * this.e4125) + (reverse_g7[0] * this.e1) + (reverse_g7[0] * this.e415) + (reverse_g7[1] * this.e2) + (reverse_g7[1] * this.e425) + (reverse_g7[2] * this.e3) + (reverse_g7[2] * this.e435) + (reverse_g3[3] * this.e1234) + (reverse_g6[0] * this.e423) + (reverse_g6[1] * this.e431) + (reverse_g6[2] * this.e412) + (reverse_g6[3] * this.e4) + 2.0 * (this.scalar * this.e1234) - (reverse_g4[0] * this.e23) - (reverse_g4[1] * this.e31) - (reverse_g4[2] * this.e12) - (reverse_g5[0] * this.e41) - (reverse_g5[1] * this.e42) - (reverse_g5[2] * this.e43) - 2.0 * (this.e12345 * this.e4) - (this.e1 * this.e423) - (this.e2 * this.e431) - (this.e3 * this.e412) - (this.e4 * this.e321) - (this.e45 * this.e1234) - (this.e41 * this.e4235) - (this.e42 * this.e4315) - (this.e43 * this.e4125)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(VersorEven lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.constraint_violation();
}
public extension VersorEven {
    public Sphere constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (reverse_g1.xyxz * float4(this.group1.ww, this.e2, this.e125)) + (reverse_g2.xyxz * float4(this.group3.ww, this.e431, this.e435)) + (reverse_g2.yzzw * float4(this.group0.zx, this.e4, this.e321)) + (this.group1.xyzy * float4(reverse_g1.www, reverse_g2[1])) + (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) + (this.group3.xyxy * float4(this.group0.ww, this.e425, this.e315)) + (this.group3.yzzz * float4(this.group1.zx, this.e12345, this.e125)) + (float4(this.group3.zx, this.e321, this.e415) * float4(reverse_g1.yzz, reverse_g2[0])) + float4((this.group0.xyz * reverse_g2.www), this.e235 * this.e1) + float4((this.group3.xyz * reverse_g0.www), reverse_g1[1] * this.e315) - (reverse_g0.xyxw * this.group2.wwyw) - (reverse_g2.zxyy * float4(this.group0.yzx, this.e2)) - (this.group2.zxww * float4(reverse_g0.yzz, reverse_g1[3])) - (this.group3.yzxx * float4(reverse_g1.zxy, reverse_g2[0])) - float4((this.group1.yzx * this.group3.zxy), reverse_g2[2] * this.e3) - float4((this.group2.xyz * this.group3.www), reverse_g2[3] * this.e12345), 
            /* e1234 */
            (reverse_g0[0] * this.e415) + (reverse_g0[0] * this.e1) + (reverse_g0[1] * this.e425) + (reverse_g0[1] * this.e2) + (reverse_g0[2] * this.e435) + (reverse_g0[2] * this.e3) + (reverse_g1[0] * this.e423) + (reverse_g1[1] * this.e431) + (reverse_g1[2] * this.e412) + (reverse_g1[3] * this.e4) - (reverse_g0[3] * this.e4) - (this.e423 * this.e1) - (this.e431 * this.e2) - (this.e412 * this.e3) - (this.e12345 * this.e4) - (this.e321 * this.e4)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(VersorOdd lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public Sphere operator /(ConstraintViolationPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.constraint_violation();
}
public extension VersorOdd {
    public Sphere constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0);
        float4 reverse_g2 = this.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((reverse_g2[2] * this.e42) + (reverse_g2[3] * this.e15) + (this.scalar * this.e4235) + (this.e12 * this.e4315), (reverse_g2[0] * this.e43) + (reverse_g2[3] * this.e25) + (this.scalar * this.e4315) + (this.e23 * this.e4125), (reverse_g2[1] * this.e41) + (reverse_g2[3] * this.e35) + (this.scalar * this.e4125) + (this.e31 * this.e4235), -(reverse_g2[1] * this.e31) - (reverse_g2[1] * this.e4315) - (reverse_g2[2] * this.e12) - (reverse_g2[2] * this.e4125)) + (this.group1 * float4(reverse_g1.www, this.e3215)) + (reverse_g0.xyxw * float4(this.group3.ww, this.e25, this.e3215)) + (float4(this.group1.ww, this.e4315, this.e35) * float4(reverse_g1.xyx, this.e4125)) + (float4(this.group2.zx, this.e3215, this.e15) * float4(reverse_g0.yzz, this.e4235)) + (float4(this.group3.zx, this.e45, this.scalar) * float4(reverse_g1.yzz, this.e3215)) + float4((this.group3.xyz * reverse_g0.www), this.e25 * this.e4315) - (reverse_g1.zxyy * float4(this.group3.yzx, this.e25)) - (this.group1.yzxx * float4(this.group3.zxy, reverse_g2[0])) - (this.group2.yzxx * float4(reverse_g0.zxy, reverse_g1[0])) - (float4(this.group0.zx, this.e1234, this.e3215) * float4(reverse_g2.yzz, reverse_g1[3])) - (float4(this.group2.ww, this.e42, this.e35) * float4(reverse_g2.xyx, reverse_g1[2])) - float4((this.group0.xyz * this.group3.www), reverse_g2[0] * this.e4235), 
            /* e1234 */
            (reverse_g0[0] * this.e4235) + (reverse_g0[1] * this.e4315) + (reverse_g0[2] * this.e4125) + (reverse_g0[3] * this.e1234) + (reverse_g1[3] * this.e1234) + (reverse_g2[3] * this.scalar) - (reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43) - (reverse_g2[3] * this.e45) - (this.e41 * this.e4235) - (this.e42 * this.e4315) - (this.e43 * this.e4125)
        );
    }
}
