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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*(this.group2.yzxy * float4(this.group0.zxy, this.e31)) + float4(float3(0.0), 2.0*(this.e23 * this.e15) + 2.0 * (this.e12 * this.e35)) - float4(2.0)*float4((float3(this.e45) * this.group1.xyz), 0.0) - float4(2.0)*float4((this.group0.yzx * this.group2.zxy), 0.0), 
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*(float4(this.e5) * float4(this.group0.xyz, this.e321)) + float4(2.0)*(this.group2.zxyx * float4(this.group0.yzx, this.e1)) + float4(2.0)*(this.group3.yzxy * float4(this.group1.zxy, this.e315)) + float4(float3(0.0), (this.e125 * this.e3) * 2.0) - float4(2.0)*(float4(this.e412, this.e423, this.e431, this.e415) * this.group2.yzxx) - float4(2.0)*(this.group1.xyxy * float4(float2(this.e321), this.e2, this.e315)) - float4(2.0)*(this.group1.yzzz * float4(this.group3.zx, this.e321, this.e125)) - float4(2.0)*float4((float3(this.e4) * this.group2.xyz), 0.0), 
            /* e1234 */
            -2.0*(this.e423 * this.e415) - 2.0 * (this.e423 * this.e1) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e431 * this.e2) - 2.0 * (this.e412 * this.e435) - 2.0 * (this.e412 * this.e3) - 2.0 * (this.e321 * this.e4)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(2.0*(this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e125 * this.e3) + 2.0 * (this.e321 * this.e5), 0.0)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35), 0.0)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25) + 2.0 * (this.e12 * this.e35) + 2.0 * (this.scalar * this.e3215), 0.0)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*float4((this.group0.yzx * this.group2.zxy), 0.0) + float4(float3(0.0), (this.e435 * this.e125) * -2.0) - float4(2.0)*(float4(this.e321, this.e321, this.e321, this.e235) * this.group1.xyzx) - float4(2.0)*float4((this.group0.zxy * this.group2.yzx), this.e425 * this.e315), 
            /* e1234 */
            -2.0*(this.e423 * this.e415) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e412 * this.e435)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*float4((this.group0.yzx * this.group2.zxy), 0.0) + float4(float3(0.0), (this.e435 * this.e125) * -2.0) - float4(2.0)*(float4(this.e412, this.e423, this.e431, this.e415) * this.group2.yzxx) - float4(2.0)*(float4(this.e321, this.e321, this.e321, this.e315) * this.group1.xyzy), 
            /* e1234 */
            -2.0*(this.e423 * this.e415) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e412 * this.e435)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*float4((this.group0.zxy * this.group2.yzx), this.e12 * this.e35) + float4(float3(0.0), 2.0*(this.e23 * this.e15) + 2.0 * (this.e31 * this.e25)) - float4(2.0)*float4((float3(this.e45) * this.group1.xyz), 0.0) - float4(2.0)*float4((this.group0.yzx * this.group2.zxy), 0.0), 
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*(float4(this.e43, this.e41, this.e42, this.e23) * this.group2.yzxx) + float4(2.0)*(this.group1.zxyy * float4(this.group3.yzx, this.e25)) + float4(2.0)*(this.group2.xyzz * float4(float3(this.e1234), this.e12)) + float4(float3(0.0), 2.0*(this.e45 * this.e3215) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125)) - float4(2.0)*float4((this.group0.xyx * float3(float2(this.e3215), this.e25)), 0.0) - float4(2.0)*float4((this.group0.yzz * float3(this.group2.zx, this.e3215)), 0.0) - float4(2.0)*float4((this.group1.xyx * float3(float2(this.e45), this.e4315)), 0.0) - float4(2.0)*float4((this.group1.yzz * float3(this.group3.zx, this.e45)), 0.0), 
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) - 2.0 * (this.e41 * this.e4235) - 2.0 * (this.e42 * this.e4315) - 2.0 * (this.e43 * this.e4125) - 2.0 * (this.e45 * this.e1234)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(2.0*(this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) + 2.0 * (this.e45 * this.e3215), 0.0)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(-2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125), 0.0)
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
        return AntiDualNum(
            /* e3215, scalar */
            float2(-2.0*(this.e415 * this.e235) - 2.0 * (this.e425 * this.e315) - 2.0 * (this.e435 * this.e125) - 2.0 * (this.e12345 * this.e5), 0.0)
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
        float4 reverse_g6 = this.group6 * float4(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, 2.0*(this.e1 * this.e4235) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) + 2.0 * (this.e5 * this.e1234) + (this.scalar * this.e12345) + (this.e2 * this.e4315) + (this.e15 * this.e423) + (this.e25 * this.e431) + (this.e35 * this.e412) + 2.0 * (this.e41 * this.e235) + 2.0 * (this.e42 * this.e315) + 2.0 * (this.e43 * this.e125) + (this.e23 * this.e415) + (this.e31 * this.e425) + (this.e12 * this.e435) - (reverse_g3[0] * this.e423) - (reverse_g3[1] * this.e431) - (reverse_g3[2] * this.e412) - (reverse_g3[3] * this.e321) - (reverse_g6[0] * this.e23) - (reverse_g6[1] * this.e31) - (reverse_g6[2] * this.e12) - (reverse_g6[3] * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.scalar) * this.group1) + float4(2.0)*(float4(this.e5, this.e5, this.e5, this.e2 * this.e42) * float4(this.group4.xyz, 1.0)) + float4(2.0)*(this.group1.yzxx * float4(this.group5.zxy, this.e41)) + float4(0.0, this.e43 * this.e235, reverse_g6[3] * this.e12, this.e42 * this.e425) + float4(reverse_g6[2] * this.e4315, reverse_g3[2] * this.e423, reverse_g3[0] * this.e431, 0.0) + float4(reverse_g6[3] * this.e23, reverse_g6[0] * this.e4125, 0.0, this.e12345 * this.e1234) + float4(this.e42 * this.e125, reverse_g6[3] * this.e31, reverse_g6[1] * this.e4235, 0.0) + float4(this.e423 * this.e3215, 0.0, this.e41 * this.e315, this.e43 * this.e435) + (reverse_g3 * float4(this.e4)) + (float4(this.e412) * float4(reverse_g3[1], this.e15, this.e3215, this.e12)) + (float4(this.e35, this.e3215, this.e25, this.e3 * this.e43) * float4(this.group7.yyx, 1.0)) + (this.group6.yzxx * float4(this.group9.zxy, this.e41)) + float4(float3(0.0), this.e23 * this.e423) + float4(float3(0.0), 2.0*(this.e31 * this.e431) + (this.e321 * this.e1234)) - float4(0.0, reverse_g6[2] * this.e4235, this.e2 * this.e23, reverse_g6[0] * this.e41) - float4(0.0, this.e315 * this.e1234, this.e42 * this.e235, 0.0) - float4(reverse_g3[2] * this.e431, this.e1 * this.e12, 0.0, reverse_g6[1] * this.e42) - float4(reverse_g3[3] * this.e415, 0.0, this.e15 * this.e431, reverse_g6[2] * this.e43) - float4(this.e12345 * this.e4235, reverse_g3[0] * this.e412, reverse_g3[1] * this.e423, 0.0) - float4(this.e23 * this.e321, 0.0, 0.0, 0.0) - float4(this.e235 * this.e1234, reverse_g3[3] * this.e425, reverse_g3[3] * this.e435, 0.0) - (reverse_g6 * float4(float3(this.e45), this.e1234)) - (float4(this.e4) * this.group3) - (float4(this.e4315) * float4(this.e435, this.e12345, reverse_g6[0], this.e431)) - (float4(this.e4125) * float4(reverse_g6[1], this.e415, this.e12345, this.e412)) - (float4(this.e3, this.e321, this.e321, this.e423 * this.e4235) * float4(this.group5.yyz, 1.0)) - float4(float2(0.0), this.e125 * this.e1234, 0.0) - float4(float3((this.group3.yz * this.group7.xyz.zx), 0.0), 0.0) - float4(float3((this.group4.xyz.zx * this.group8.xyz.yz), this.e425 * this.e4235), 0.0), 
            /* e5 */
            2.0*(this.scalar * this.e5) + 2.0 * (this.e12345 * this.e3215) + (reverse_g3[0] * this.e1) + (reverse_g3[1] * this.e2) + (reverse_g3[2] * this.e3) + (reverse_g6[3] * this.e3215) + (this.e5 * this.e45) + 2.0 * (this.e23 * this.e235) + 2.0 * (this.e31 * this.e315) + 2.0 * (this.e12 * this.e125) + 2.0 * (this.e235 * this.e4235) + 2.0 * (this.e315 * this.e4315) + 2.0 * (this.e125 * this.e4125) - (reverse_g3[0] * this.e415) - (reverse_g3[1] * this.e425) - (reverse_g3[2] * this.e435) - (reverse_g3[3] * this.e5) - (reverse_g6[0] * this.e15) - (reverse_g6[1] * this.e25) - (reverse_g6[2] * this.e35) - (this.e1 * this.e15) - (this.e2 * this.e25) - (this.e3 * this.e35) - (this.e321 * this.e3215), 
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
            float4(2.0)*(float4(this.scalar) * this.group9) + float4(0.0, reverse_g6[1] * this.e321, reverse_g6[2] * this.e321, reverse_g6[0] * this.e235) + float4(this.e12345 * this.e1, reverse_g3[0] * this.e43, reverse_g3[1] * this.e41, 0.0) + float4(this.e5 * this.e423, reverse_g3[3] * this.e31, reverse_g3[3] * this.e12, 0.0) + float4((reverse_g3[2] * this.e42) + (reverse_g3[3] * this.e23), (this.e412 * this.e235) * 2.0, (this.e423 * this.e315) * 2.0, (this.e25 * this.e4315) + (this.e35 * this.e12) + (this.e35 * this.e4125)) + (float4(this.e2) * float4(this.e435, this.e12345, reverse_g6[0], this.e315)) + (float4(this.e3) * float4(reverse_g6[1], this.e415, this.e12345, this.e125)) + (float4(this.e43, this.e41, this.e42, this.e23) * this.group3.yzxx) + (float4(this.e321, this.e1, this.e1 * this.e425, this.e1 * this.e235) * float4(reverse_g6.xz, 1.0, 1.0)) + (float4(this.e125, this.e5, this.e5, reverse_g6[1] * this.e315) * float4(this.group7.yyz, 1.0)) + (float4(this.e1234, this.e1234, this.e1234, this.e3215) * this.group3) + (this.group6 * float4(float3(reverse_g6[3]), this.e5)) + (this.group9.yzxx * float4(this.group5.zxy, this.e15)) + float4(float3(0.0), reverse_g6[2] * this.e125) + float4(float3(0.0), this.e25 * this.e31) - (reverse_g3 * float4(this.e1234, this.e1234, this.e1234, this.e3215)) - (float4(this.e43, this.e41, this.e42, this.e23) * reverse_g3.yzxx) - (reverse_g6.zxyw * float4(this.group1.yzx, this.e5)) - (this.group6.yzxx * float4(this.group1.zxy, this.e235)) - (this.group9.zxyy * float4(this.group5.yzx, reverse_g3[1])) - (this.group9.wwwx * float4(this.group4.xyz, reverse_g3[0])) - float4(float3(0.0), reverse_g3[1] * this.e31) - float4(float3(0.0), reverse_g3[2] * this.e12) - float4(float3(0.0), this.e425 * this.e315) - float4((float3(this.e4) * this.group8.xyz), reverse_g3[2] * this.e4125) - float4((float3(this.e45) * this.group5.xyz), this.e12345 * this.e5) - float4((this.group4.yzx * this.group3.zxy), this.e435 * this.e125) - float4((this.group7.zxy * this.group8.yzx), 0.0), 
            /* e1234 */
            2.0*(this.scalar * this.e1234) + (reverse_g3[3] * this.e1234) + (reverse_g6[0] * this.e423) + (reverse_g6[1] * this.e431) + (reverse_g6[2] * this.e412) + (reverse_g6[3] * this.e4) + 2.0 * (this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) - 2.0 * (this.e1 * this.e423) - 2.0 * (this.e2 * this.e431) - 2.0 * (this.e3 * this.e412) - (this.e4 * this.e321) - (this.e45 * this.e1234) - 2.0 * (this.e41 * this.e4235) - 2.0 * (this.e42 * this.e4315) - 2.0 * (this.e43 * this.e4125) - (this.e415 * this.e423) - (this.e425 * this.e431) - (this.e435 * this.e412) - 2.0 * (this.e12345 * this.e4)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e431 * this.e125, reverse_g0[0] * this.e125, reverse_g0[1] * this.e235, 0.0) + float4(this.e435 * this.e2, this.e415 * this.e3, this.e425 * this.e1, 0.0) + (float4(this.e5) * float4(this.group0.xyz, this.e321)) + (float4(reverse_g0[2], this.e412, this.e423, this.e1) * this.group2.yxyx) + (this.group3.xyzy * float4(float3(this.e12345), this.e315)) + float4(float3(0.0), this.e125 * this.e3) + float4((float3(reverse_g0[3]) * this.group3.xyz), 0.0) - float4(reverse_g0[1] * this.e125, reverse_g0[2] * this.e235, reverse_g0[0] * this.e315, 0.0) - float4(this.e415 * this.e321, this.e425 * this.e321, this.e415 * this.e2, 0.0) - float4(this.e425 * this.e3, this.e435 * this.e1, this.e435 * this.e321, 0.0) - (reverse_g0 * float4(this.e5)) - (this.group2 * float4(float3(this.e4), this.e12345)) - (this.group2.yzxy * float4(this.group0.zxy, this.e425)) - float4(float3(0.0), this.e415 * this.e235) - float4(float3(0.0), this.e435 * this.e125), 
            /* e1234 */
            (reverse_g0[0] * this.e415) + (reverse_g0[0] * this.e1) + (reverse_g0[1] * this.e425) + (reverse_g0[1] * this.e2) + (reverse_g0[2] * this.e435) + (reverse_g0[2] * this.e3) - (reverse_g0[3] * this.e4) - (this.e423 * this.e415) - (this.e423 * this.e1) - (this.e431 * this.e425) - (this.e431 * this.e2) - (this.e412 * this.e435) - (this.e412 * this.e3) - (this.e12345 * this.e4) - 2.0 * (this.e321 * this.e4)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, this.e41 * this.e35, this.e42 * this.e15, this.e12 * this.e35) + float4(reverse_g0[1] * this.e35, this.e25 * this.e1234, this.e35 * this.e1234, 0.0) + float4(this.e43 * this.e25, 0.0, 0.0, this.e31 * this.e25) + float4(this.e12 * this.e4315, reverse_g0[2] * this.e15, reverse_g0[0] * this.e25, 0.0) + float4(this.e15 * this.e1234, this.e23 * this.e4125, this.e31 * this.e4235, 0.0) + (reverse_g0 * float4(this.e3215)) + (this.group3 * float4(float3(reverse_g0[3]), this.scalar)) + (this.group3 * float4(float3(this.scalar), this.e45)) + float4(float3(0.0), this.e23 * this.e15) + float4(float3(0.0), 2.0*(this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125)) - float4((reverse_g0.zxy * this.group2.yzx), 0.0) - float4((this.group0.xyx * float3(float2(this.e3215), this.e25)), 0.0) - float4((this.group0.yzz * float3(this.group2.zx, this.e3215)), 0.0) - float4(2.0)*float4((this.group1.xyx * float3(float2(this.e45), this.e4315)), 0.0) - float4(2.0)*float4((this.group1.yzz * float3(this.group3.zx, this.e45)), 0.0), 
            /* e1234 */
            (reverse_g0[0] * this.e4235) + (reverse_g0[1] * this.e4315) + (reverse_g0[2] * this.e4125) + (reverse_g0[3] * this.e1234) + (this.e41 * this.e23) + (this.e42 * this.e31) + (this.e43 * this.e12) + (this.scalar * this.e1234) - (reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (this.e41 * this.e4235) - (this.e42 * this.e4315) - (this.e43 * this.e4125) - 2.0 * (this.e45 * this.e1234)
        );
    }
}
