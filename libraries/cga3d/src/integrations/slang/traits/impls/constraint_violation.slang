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
            float4(2.0*(this.e42 * this.e35) + 2.0 * (this.e23 * this.e45) - (this.group2.yzx[0] * this.e43) - (this.e43 * this.e25), 2.0*(this.e43 * this.e15) + 2.0 * (this.e31 * this.e45) - (this.group2.yzx[1] * this.e41) - (this.e41 * this.e35), 2.0*(this.e41 * this.e25) + 2.0 * (this.e12 * this.e45) - (this.group2.yzx[2] * this.e42) - (this.e42 * this.e15), -2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)), 
            /* e1234 */
            -2.0*(this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
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
            float4(2.0*(this.e412 * this.e315) + 2.0 * (this.e415 * this.e321) - 2.0 * (this.e431 * this.e125), 2.0*(this.e423 * this.e125) + 2.0 * (this.e425 * this.e321) - 2.0 * (this.e412 * this.e235), (this.group1.xyxy[2] * this.e2) + (this.group1.xyzx[2] * this.group1.wwww[2]) + (this.group1.yzzz[2] * this.e321) + (this.group2.xyxy[2] * this.e431) + (this.group2.yzzz[2] * this.e4) + (this.e431 * this.e235) - (this.e415 * this.e2) - (this.e125 * this.e4) - 2.0 * (this.e423 * this.e315), (this.group1.xyzx[3] * this.e235) + (this.group1.yzzz[3] * this.e125) + (this.group2.xyxy[3] * this.e425) + (this.group2.yzzz[3] * this.e435) + (this.group3.yzxx[3] * this.e235) + (this.e415 * this.e235) + (this.e315 * this.e2) + (this.e125 * this.e3) - (this.group3.yzxy[3] * this.e315) - (this.e235 * this.e1)) + (float4(this.group1.zxy[0], this.group1.zxy[1], this.group1.zxy[2] * this.group3.yzxx[2], this.group1.xyxy[3] * this.e315) * float4(this.group3.yz, 1.0, 1.0)) - (float4(this.group1.zxy[0], this.group1.zxy[1], this.group1.zxy[2] * this.group3.yzxy[2], this.group2.zxyz[3] * this.e3) * float4(this.group3.yz, 1.0, 1.0)), 
            /* e1234 */
            2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425) + 2.0 * (this.e412 * this.e435)
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
            float2(this.e3215 * this.scalar, 0.0) * float2(2.0, 0.0)
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
            float2(-2.0*(this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35), 0.0)
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
            float2(2.0*(this.scalar * this.e3215) - 2.0 * (this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35), 0.0)
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
            float4(2.0)*(this.group1.xyzz * float4(this.group1.www, this.e125)) + float4(2.0)*float4((this.group0.zxy * this.group2.yzx), this.e425 * this.e315) + float4(this.e431 * this.e125 * -2.0, this.e412 * this.e235 * -2.0, this.e423 * this.e315 * -2.0, (this.group1.wwwx[3] * this.e235) + (this.e415 * this.e235)), 
            /* e1234 */
            2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425) + 2.0 * (this.e412 * this.e435)
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
            float4(2.0*(this.e412 * this.e315) + 2.0 * (this.e415 * this.e321) - 2.0 * (this.e431 * this.e125), 2.0*(this.e423 * this.e125) + 2.0 * (this.e425 * this.e321) - 2.0 * (this.e412 * this.e235), 2.0*(this.e431 * this.e235) + 2.0 * (this.e435 * this.e321) - 2.0 * (this.e423 * this.e315), (this.group1.xyzy[3] * this.e315) + (this.group1.wwwz[3] * this.e125) + (this.group2.yzxx[3] * this.e415) + (this.e415 * this.e235) + (this.e425 * this.e315) + (this.e435 * this.e125)), 
            /* e1234 */
            2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425) + 2.0 * (this.e412 * this.e435)
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
            float4(2.0*(this.e42 * this.e35) + 2.0 * (this.e23 * this.e45), 2.0*(this.e43 * this.e15) + 2.0 * (this.e31 * this.e45), 2.0*(this.e41 * this.e25) + 2.0 * (this.e12 * this.e45), -2.0*(this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)) - float4(2.0)*float4((this.group0.zxy * this.group2.yzx), this.e23 * this.e15), 
            /* e1234 */
            -2.0*(this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
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
            float4(2.0*(this.e42 * this.e35) + (this.group1.xyx[0] * this.e45) + (this.group1.yzz[0] * this.e4125) + (this.e23 * this.e45) + (this.e12 * this.e4315) + (this.e15 * this.e1234) - (this.group2.yzz[0] * this.e43) - (this.group3.yzx[0] * this.e12) - (this.e43 * this.e25), 2.0*(this.e43 * this.e15) + (this.group1.xyx[1] * this.e45) + (this.group1.yzz[1] * this.e4235) + (this.e23 * this.e4125) + (this.e31 * this.e45) + (this.e25 * this.e1234) - (this.group2.yzz[1] * this.e41) - (this.group3.yzx[1] * this.e23) - (this.e41 * this.e35), 2.0*(this.e41 * this.e25) + (this.group1.xyx[2] * this.e4315) + (this.group1.yzz[2] * this.e45) + (this.e31 * this.e4235) + (this.e12 * this.e45) + (this.e35 * this.e1234) - (this.group2.yzz[2] * this.e1234) - (this.group3.yzx[2] * this.group1.zxyy[2]) - (this.e42 * this.e15), -2.0*(this.e23 * this.e15) - 2.0 * (this.e12 * this.e35)) - (float4(this.group1.yzx[0], this.group1.yzx[1], this.group1.yzx[2] * this.e4315, this.group1.zxyy[3] * this.e25) * float4(this.group3.zx, 1.0, 1.0)) - (float4(this.group2.xyx[0], this.group2.xyx[1], this.group2.xyx[2] * this.e42, this.e31 * this.e25) * float4(this.group2.ww, 1.0, 1.0)), 
            /* e1234 */
            -2.0*(this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
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
            float2(this.e5 * this.e12345, 0.0) * float2(-2.0, 0.0)
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
            float2(2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125), 0.0)
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
            float2(2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125) - 2.0 * (this.e12345 * this.e5), 0.0)
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
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, 2.0*(this.group9.wxzw[1] * this.e1) + 2.0 * (this.scalar * this.e12345) + 2.0 * (this.e2 * this.e4315) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) + 2.0 * (this.e5 * this.e1234) - 2.0 * (this.e15 * this.e423) - 2.0 * (this.e25 * this.e431) - 2.0 * (this.e35 * this.e412) - 2.0 * (this.e45 * this.e321) - 2.0 * (this.e41 * this.e235) - 2.0 * (this.e42 * this.e315) - 2.0 * (this.e43 * this.e125) - 2.0 * (this.e23 * this.e415) - 2.0 * (this.e31 * this.e425) - 2.0 * (this.e12 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.scalar) * this.group1) + float4(2.0)*float4((this.group4.zxy * this.group8.yzx), this.e12345 * this.e1234) + float4(2.0*(this.e23 * this.e321) + (this.group3.xyx[0] * this.e4) + (this.group3.yzx[0] * this.e412) + (this.group3.yzz[0] * this.e412) + (this.e3 * this.e31) + (this.e425 * this.e4125) + (this.e435 * this.e4315) - (this.group1.zxy[0] * this.e31) - (this.group3.zxy[0] * this.e431) - (this.group6.xyx[0] * this.e45) - (this.group6.yzz[0] * this.e4125) - (this.group6.zxy[0] * this.group9.yzx[0]) - (this.e4 * this.e15) - (this.e35 * this.e431) - (this.e45 * this.e415), 2.0*(this.e31 * this.e321) + (this.group3.xyx[1] * this.e4) + (this.group3.yzx[1] * this.e423) + (this.group3.yzz[1] * this.e423) + (this.e1 * this.e12) + (this.e415 * this.e4125) + (this.e435 * this.e4235) - (this.group1.zxy[1] * this.e12) - (this.group3.zxy[1] * this.e412) - (this.group6.xyx[1] * this.e45) - (this.group6.yzz[1] * this.e4235) - (this.group6.zxy[1] * this.group9.yzx[1]) - (this.e4 * this.e25) - (this.e15 * this.e412) - (this.e45 * this.e425), 2.0*(this.e12 * this.e321) + (this.group3.xyx[2] * this.e431) + (this.group3.yzx[2] * this.e431) + (this.group3.yzz[2] * this.e4) + (this.e2 * this.e23) + (this.e415 * this.e4315) + (this.e425 * this.e4235) - (this.group1.zxy[2] * this.e23) - (this.group3.zxy[2] * this.e423) - (this.group6.xyx[2] * this.e4315) - (this.group6.yzz[2] * this.e45) - (this.group6.zxy[2] * this.group9.yzx[2]) - (this.group1.wwww[2] * this.e35) - (this.group3.wwww[2] * this.e435) - (this.e25 * this.e423), -2.0*(this.e43 * this.e435) - 2.0 * (this.e23 * this.e423) - 2.0 * (this.e31 * this.e431) - 2.0 * (this.e12 * this.e412)) - float4(2.0)*(float4(this.group0.yy, this.e12345, this.e42 * this.e425) * float4(this.group9.xyz, 1.0)) - float4(2.0)*float4((this.group4.yzx * this.group8.zxy), this.e41 * this.e415), 
            /* e5 */
            2.0*(this.scalar * this.e5) + 2.0 * (this.e12345 * this.e3215) - 2.0 * (this.e15 * this.e415) - 2.0 * (this.e25 * this.e425) - 2.0 * (this.e35 * this.e435) - 2.0 * (this.e23 * this.e235) - 2.0 * (this.e31 * this.e315) - 2.0 * (this.e12 * this.e125), 
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
            float4(2.0)*(float4(this.scalar) * this.group9) + float4(2.0)*(float4(this.group0.yy, this.e12345, this.e415 * this.e235) * float4(this.group1.xyz, 1.0)) + float4(2.0)*float4((this.group5.xyz.xy * this.group3.ww), this.e431 * this.e235, this.e425 * this.e315) + float4(2.0*(this.e412 * this.e315) + (this.group1.yzx[0] * this.group6.zxy[0]) + (this.group6.xyx[0] * this.e321) + (this.group6.yzz[0] * this.e3) + (this.e15 * this.e1234) + (this.e35 * this.e42) + (this.e12 * this.e4315) + (this.e415 * this.e321) - (this.group1.yzx[0] * this.group6.zxy[0]) - (this.group1.zxy[0] * this.group6.yzx[0]) - (this.group3.xyx[0] * this.e1234) - 2.0 * (this.e431 * this.e125), 2.0*(this.e423 * this.e125) + (this.group1.yzx[1] * this.group6.zxy[1]) + (this.group6.xyx[1] * this.e321) + (this.group6.yzz[1] * this.e1) + (this.e15 * this.e43) + (this.e25 * this.e1234) + (this.e23 * this.e4125) + (this.e425 * this.e321) - (this.group1.yzx[1] * this.group6.zxy[1]) - (this.group1.zxy[1] * this.group6.yzx[1]) - (this.group3.xyx[1] * this.e1234) - 2.0 * (this.e412 * this.e235), (this.group3.zxy[2] * this.e41) + (this.group6.xyx[2] * this.e2) + (this.group6.yzz[2] * this.e321) + (this.group3.wwww[2] * this.e12) + (this.group6.wwww[2] * this.e435) + (this.e4 * this.e125) + (this.e25 * this.e41) + (this.e35 * this.e1234) + (this.e45 * this.e12) + (this.e43 * this.e3215) + (this.e23 * this.e4315) + (this.e31 * this.e4235) - (this.group1.yzx[2] * this.group6.zxy[2]) - (this.group1.zxy[2] * this.group6.yzx[2]) - (this.group3.xyx[2] * this.e42) - (this.group9.yzx[2] * this.e31) - (this.group1.wwww[2] * this.e125) - (this.group3.yzzx[2] * this.e1234) - (this.group9.wwww[2] * this.e43) - (this.e15 * this.e42), 2.0*(this.e435 * this.e125) - (this.group9.zxyz[3] * this.e35) - (this.e15 * this.e23) - 2.0 * (this.e12345 * this.e5) - 2.0 * (this.e35 * this.e12)) + (float4(this.group3.zxy[0], this.group3.zxy[1], this.group1.yzx[2] * this.group6.zxy[2], this.e35 * this.e4125) * float4(this.group4.xyz.yz, 1.0, 1.0)) - (float4(this.group9.yzx[0], this.group9.yzx[1], this.group5.yzx[2] * this.group9.zxyz[2], this.group3.yzzx[3] * this.e23) * float4(this.group5.xyz.zx, 1.0, 1.0)) - float4(2.0)*float4((this.group4.xyz.zx * this.group3.yz), this.e423 * this.e315, this.e25 * this.e31), 
            /* e1234 */
            2.0*(this.scalar * this.e1234) + 2.0 * (this.e415 * this.e423) + 2.0 * (this.e425 * this.e431) + 2.0 * (this.e435 * this.e412) - 2.0 * (this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12) - 2.0 * (this.e12345 * this.e4)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(2.0)*float4((this.group3.xyz * this.group0.www), this.e415 * this.e235) + float4(2.0*(this.e415 * this.e321) - (this.group1.zxy[0] * this.e2), 2.0*(this.e425 * this.e321) - (this.group1.zxy[1] * this.e3), (this.e412 * this.e5) + (this.e415 * this.e2) + (this.e435 * this.e321) - (this.group1.zxy[2] * this.e1) - (this.e423 * this.e315), (this.group3.yzxz[3] * this.e125) + (this.e425 * this.e315) + (this.e435 * this.e125) - 2.0 * (this.e12345 * this.e5)) + (float4(this.group0.zxy[0], this.group0.zxy[1], this.group0.zxy[2], this.group1.xyzy[3]) * this.group2.yzxy) + (float4(this.group1.yzz[0], this.group1.yzz[1], this.group1.yzz[2] * this.e321, this.group2.xyxz[3] * this.e435) * float4(this.group3.zx, 1.0, 1.0)) + (float4(this.group1.zxy[0], this.group1.zxy[1], this.group1.zxy[2] * this.group3.yzxz[2], this.group2.yzzw[3] * this.e321) * float4(this.group3.yz, 1.0, 1.0)) + (this.group2.yzxy * float4(this.group0.zxy, this.group3.xyzy[3])) - (float4(this.group0.yzx[0], this.group0.yzx[1], this.group0.yzx[2] * this.group2.zxyy[2], this.group2.zxyy[3] * this.e2) * float4(this.group2.zx, 1.0, 1.0)) - (float4(this.group0.yzz[0], this.group0.yzz[1], this.group0.yzz[2], this.group3.zxyz[3]) * this.group2.zxwz) - (float4(this.group1.yzx[0], this.group1.yzx[1], this.group1.yzx[2] * this.group3.zxyz[2], this.e321 * this.e5) * float4(this.group3.zx, 1.0, 1.0)), 
            /* e1234 */
            2.0*(this.e423 * this.e415) + 2.0 * (this.e431 * this.e425) + 2.0 * (this.e412 * this.e435) - 2.0 * (this.e12345 * this.e4)
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
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group1.yzz[0] * this.e4125) + (this.group0.wwww[0] * this.e4235) + (this.group1.wwww[0] * this.e23) + (this.e42 * this.e35) + (this.scalar * this.e4235) + (this.e12 * this.e4315) + (this.e15 * this.e1234) - (this.group2.xyx[0] * this.e1234) - (this.group2.yzz[0] * this.e43) - (this.group3.yzx[0] * this.e12), (this.group1.yzz[1] * this.e4235) + (this.group0.wwww[1] * this.e4315) + (this.group1.wwww[1] * this.e31) + (this.e43 * this.e15) + (this.scalar * this.e4315) + (this.e23 * this.e4125) + (this.e25 * this.e1234) - (this.group2.xyx[1] * this.e1234) - (this.group2.yzz[1] * this.e41) - (this.group3.yzx[1] * this.e23), 2.0*(this.e41 * this.e25) + (this.group1.yzz[2] * this.e45) + (this.group0.wwww[2] * this.e4125) + (this.group1.wwww[2] * this.e12) + (this.scalar * this.e4125) + (this.e31 * this.e4235) + (this.e35 * this.e1234) - (this.group2.xyx[2] * this.e42) - (this.group2.yzz[2] * this.e1234) - (this.group3.yzx[2] * this.group1.zxyy[2]) - (this.e43 * this.e3215), -2.0*(this.e23 * this.e15) - 2.0 * (this.e12 * this.e35)) + (float4(this.group0.yzz[0], this.group0.yzz[1], this.group0.yzz[2] * this.e3215, this.group0.xyxw[3] * this.e3215) * float4(this.group2.zx, 1.0, 1.0)) + (float4(this.group1.xyx[0], this.group1.xyx[1], this.group1.xyx[2] * this.e4315, this.scalar * this.e3215) * float4(this.group1.ww, 1.0, 1.0)) - (float4(this.group0.zxy[0], this.group0.zxy[1], this.group0.zxy[2], this.group1.zxyy[3]) * this.group2.yzxy) - (float4(this.group1.yzx[0], this.group1.yzx[1], this.group1.yzx[2] * this.e4315, this.e31 * this.e25) * float4(this.group3.zx, 1.0, 1.0)), 
            /* e1234 */
            2.0*(this.scalar * this.e1234) - 2.0 * (this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
        );
    }
}
