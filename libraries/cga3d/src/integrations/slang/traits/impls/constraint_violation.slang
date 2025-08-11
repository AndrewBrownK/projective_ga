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
            float4((float3(2.0)*(this.group0.zxy * this.group2.yzx) - float3(2.0)*(float3(this.e45) * this.group1.xyz) - float3(2.0)*(this.group0.yzx * this.group2.zxy)), 0.0), 
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
            (this.group1.zxyw * this.group3.yzxw) + float4((float3(2.0)*(this.group0.xyx * float3(float2(this.e5), this.e315)) + float3(2.0)*(this.group0.yzz * float3(this.group2.zx, this.e5)) - float3(2.0)*(float3(this.e4) * this.group2.xyz) - float3(2.0)*(this.group0.zxy * this.group2.yzx)), 2.0*(this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) - 2.0 * (this.e415 * this.e235)) - float4(2.0)*(this.group1.xyxy * float4(float2(this.e321), this.e2, this.e315)) - float4(2.0)*(this.group1.yzzz * float4(this.group3.zx, this.e321, this.e125)), 
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
            float4((float3(2.0)*(this.group0.yzx * this.group2.zxy) - float3(2.0)*(float3(this.e321) * this.group1.xyz) - float3(2.0)*(this.group0.zxy * this.group2.yzx)), 0.0), 
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
            float4((float3(2.0)*(this.group0.yzx * this.group2.zxy) - float3(2.0)*(float3(this.e321) * this.group1.xyz) - float3(2.0)*(this.group0.zxy * this.group2.yzx)), 0.0), 
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
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((reverse_g2.zxy * this.group0.yzx) + (this.group0.zxy * this.group2.yzx) - (reverse_g2.yzx * this.group0.zxy) - (this.group0.yzx * this.group2.zxy) - float3(2.0)*(float3(this.e45) * this.group1.xyz)), 0.0), 
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
            (this.group2.xyzy * float4(float3(this.e1234), this.e4315)) + (this.group3.yzxx * float4(this.group1.zxy, this.e15)) + float4((float3(2.0)*(this.group0.zxy * this.group2.yzx) + float3((float2(-2.0)*(float2(this.e45) * this.group1.xy) - float2(2.0)*(this.group1.yz * this.group3.zx)), -2.0*(this.e23 * this.e4315) - 2.0 * (this.e12 * this.e45)) - float3(2.0)*(this.group0.xyx * float3(float2(this.e3215), this.e25)) - float3(2.0)*(this.group0.yzz * float3(this.group2.zx, this.e3215))), this.e45 * this.e3215), 
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
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, 2.0*(this.scalar * this.e12345) + 2.0 * (this.e41 * this.e235) + 2.0 * (this.e42 * this.e315) + 2.0 * (this.e43 * this.e125) + 2.0 * (this.e23 * this.e415) + 2.0 * (this.e31 * this.e425) + 2.0 * (this.e12 * this.e435) + 2.0 * (this.e423 * this.e15) + 2.0 * (this.e431 * this.e25) + 2.0 * (this.e412 * this.e35) + 2.0 * (this.e1 * this.e4235) + 2.0 * (this.e45 * this.e321) + 2.0 * (this.e5 * this.e1234) + (this.e2 * this.e4315) + (this.e3 * this.e4125) + (this.e4 * this.e3215)), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.scalar) * this.group1) + (this.group1.yzxy * float4(this.group5.zxy, this.e42)) + float4((float3(2.0)*(float3(this.e45) * this.group6.xyz) + float3(2.0)*(this.group4.xyx * float3(float2(this.e5), this.e315)) + float3(2.0)*(this.group4.yzz * float3(this.group8.xyz.zx, this.e5)) + float3(2.0)*(this.group7.xyx * float3(float2(this.e3215), this.e25)) + float3(2.0)*(this.group7.yzz * float3(this.group3.zx, this.e3215)) + float3((float2(2.0)*(this.group6.yz * this.group9.zx) - float2(2.0)*(this.group6.zx * this.group9.yz)), 2.0*(this.e415 * this.e4315) - 2.0 * (this.e425 * this.e4235)) - float3(2.0)*(float3(this.e12345) * this.group9.xyz) - float3(2.0)*(float3(this.e4) * this.group3.xyz) - float3(2.0)*(float3(this.e1234) * this.group8.xyz) - float3(2.0)*(this.group4.zxy * this.group8.yzx) - float3(2.0)*(this.group5.xyx * float3(float2(this.e321), this.e2)) - float3(2.0)*(this.group5.yzz * float3(this.group1.zx, this.e321)) - float3(2.0)*(this.group7.zxy * this.group3.yzx)), 2.0*(this.e12345 * this.e1234) + (this.e41 * this.e1) + (this.e43 * this.e3) + (this.e321 * this.e1234)), 
            /* e5 */
            2.0*(this.scalar * this.e5) + 2.0 * (this.e12345 * this.e3215) + 2.0 * (this.e23 * this.e235) + 2.0 * (this.e31 * this.e315) + 2.0 * (this.e12 * this.e125) + 2.0 * (this.e235 * this.e4235) + 2.0 * (this.e315 * this.e4315) + 2.0 * (this.e125 * this.e4125) + 2.0 * (this.e15 * this.e415) + 2.0 * (this.e25 * this.e425) + 2.0 * (this.e35 * this.e435) + 2.0 * (this.e45 * this.e5) - 2.0 * (this.e1 * this.e15) - 2.0 * (this.e2 * this.e25) - 2.0 * (this.e3 * this.e35) - 2.0 * (this.e321 * this.e3215), 
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
            float4(2.0)*(float4(this.scalar) * this.group9) + (this.group3 * float4(float3(this.e1234), this.e3215)) + (this.group1.xyzx * float4(float3(this.e12345), this.e235)) + float4((float3(2.0)*(this.group4.zxy * this.group3.yzx) + float3(2.0)*(this.group5.zxy * this.group9.yzx) + float3(2.0)*(this.group7.xyx * float3(float2(this.e5), this.e315)) + float3(2.0)*(this.group7.yzz * float3(this.group8.xyz.zx, this.e5)) + float3((float2(2.0)*(this.group1.yz * this.group6.zx) - float2(2.0)*(this.group1.zx * this.group6.yz)), 2.0*(this.e1 * this.e425) - 2.0 * (this.e2 * this.e415)) - float3(2.0)*(float3(this.e4) * this.group8.xyz) - float3(2.0)*(float3(this.e321) * this.group6.xyz) - float3(2.0)*(this.group4.xyx * float3(float2(this.e3215), this.e25)) - float3(2.0)*(this.group4.yzz * float3(this.group3.zx, this.e3215)) - float3(2.0)*(this.group5.xyx * float3(float2(this.e45), this.e4315)) - float3(2.0)*(this.group5.yzz * float3(this.group9.zx, this.e45)) - float3(2.0)*(this.group7.zxy * this.group8.yzx)), (this.e315 * this.e2) + (this.e125 * this.e3) + (this.e15 * this.e4235) + (this.e25 * this.e4315) + (this.e35 * this.e4125) - 2.0 * (this.e125 * this.e435)), 
            /* e1234 */
            2.0*(this.scalar * this.e1234) + 2.0 * (this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) - 2.0 * (this.e41 * this.e4235) - 2.0 * (this.e42 * this.e4315) - 2.0 * (this.e43 * this.e4125) - 2.0 * (this.e423 * this.e1) - 2.0 * (this.e423 * this.e415) - 2.0 * (this.e431 * this.e2) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e412 * this.e3) - 2.0 * (this.e412 * this.e435) - 2.0 * (this.e4 * this.e321) - 2.0 * (this.e45 * this.e1234) - 2.0 * (this.e12345 * this.e4)
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
            float4(2.0)*(this.group2.wwyw * float4(this.group0.xyx, this.e321)) + float4(2.0)*(this.group3.xyzy * float4(float3(this.e12345), this.e315)) + float4(2.0)*(this.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((float3(float2(2.0)*(this.group0.yz * this.group2.zx), this.e412 * this.e5 * 2.0) - float3(2.0)*(float3(this.e4) * this.group2.xyz)), 2.0*(this.e235 * this.e1) - 2.0 * (this.e435 * this.e125)) - float4(2.0)*(this.group0.zxyw * this.group2.yzxw) - float4(2.0)*(this.group1.xyxx * float4(float2(this.e321), this.e2, this.e235)) - float4(2.0)*(this.group1.yzzy * float4(this.group3.zx, this.e321, this.e315)), 
            /* e1234 */
            -2.0*(this.e423 * this.e415) - 2.0 * (this.e423 * this.e1) - 2.0 * (this.e431 * this.e425) - 2.0 * (this.e431 * this.e2) - 2.0 * (this.e412 * this.e435) - 2.0 * (this.e412 * this.e3) - 2.0 * (this.e12345 * this.e4) - 2.0 * (this.e321 * this.e4)
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
            float4(2.0)*(this.group0.zxyw * float4(this.group2.yzx, this.e3215)) + (this.group3 * float4(float3(this.scalar), this.e45)) + (this.group3.yzxy * float4(this.group1.zxy, this.e25)) + float4((float3(2.0)*(float3(this.e1234) * this.group2.xyz) + float3((float2(-2.0)*(float2(this.e45) * this.group1.xy) - float2(2.0)*(float2(this.e3215) * this.group0.xy) - float2(2.0)*(this.group0.yz * this.group2.zx) - float2(2.0)*(this.group1.yz * this.group3.zx)), -2.0*(this.e41 * this.e25) - 2.0 * (this.e43 * this.e3215) - 2.0 * (this.e23 * this.e4315) - 2.0 * (this.e12 * this.e45))), this.e15 * this.e4235), 
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) + 2.0 * (this.scalar * this.e1234) - 2.0 * (this.e41 * this.e4235) - 2.0 * (this.e42 * this.e4315) - 2.0 * (this.e43 * this.e4125) - 2.0 * (this.e45 * this.e1234)
        );
    }
}
