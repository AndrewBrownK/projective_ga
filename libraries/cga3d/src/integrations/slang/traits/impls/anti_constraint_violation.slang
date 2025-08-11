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
            float4((float3(2.0)*(this.group0.zxy * this.group2.yzx) - float3(2.0)*(float3(this.e45) * this.group1.xyz) - float3(2.0)*(this.group0.yzx * this.group2.zxy)), 0.0), 
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, 2.0*(this.e415 * this.e2) - 2.0 * (this.e425 * this.e1) - 2.0 * (this.e435 * this.e321), 0.0) + float4((float3(2.0)*(float3(this.e4) * this.group2.xyz) + float3(2.0)*(this.group0.yzx * this.group2.zxy) + float3((float2(2.0)*(this.group1.yz * this.group3.zx) - float2(2.0)*(float2(this.e415) * float2(this.e321, this.e3)) - float2(2.0)*(float2(this.e2, this.e321) * this.group1.zy)), 0.0) - float3(2.0)*(float3(this.e315, this.e5, this.e5) * this.group0.zyz) - float3(2.0)*(float3(this.e5, this.e125, this.e235) * this.group0.xxy)), 0.0), 
            /* e5 */
            2.0*(this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125) + 2.0 * (this.e321 * this.e5) + 2.0 * (this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e125 * this.e3)
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
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g0.zxy * this.group2.yzx) + (this.group0.yzx * this.group2.zxy) - (anti_reverse_g0.yzx * this.group2.zxy) - (this.group0.zxy * this.group2.yzx) - float3(2.0)*(float3(this.e321) * this.group1.xyz)), 0.0), 
            /* e5 */
            2.0*(this.e235 * this.e415) + 2.0 * (this.e315 * this.e425) + 2.0 * (this.e125 * this.e435)
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
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g0.zxy * this.group2.yzx) + (this.group0.yzx * this.group2.zxy) - (anti_reverse_g0.yzx * this.group2.zxy) - (this.group0.zxy * this.group2.yzx) - float3(2.0)*(float3(this.e321) * this.group1.xyz)), 0.0), 
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
            float4((float3(2.0)*(this.group0.zxy * this.group2.yzx) - float3(2.0)*(float3(this.e45) * this.group1.xyz) - float3(2.0)*(this.group0.yzx * this.group2.zxy)), 0.0), 
            /* e5 */
            -2.0*(this.e15 * this.e23) - 2.0 * (this.e25 * this.e31) - 2.0 * (this.e35 * this.e12)
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
            float4(0.0, 0.0, 2.0*(this.e23 * this.e4315) - 2.0 * (this.e31 * this.e4235) - 2.0 * (this.e12 * this.e45), 0.0) + float4((float3(2.0)*(float3(this.e25, this.e3215, this.e3215) * this.group0.zyz) + float3(2.0)*(float3(this.e3215, this.e35, this.e15) * this.group0.xxy) + float3((float2(2.0)*(this.group1.yz * this.group3.zx) - float2(2.0)*(float2(this.e23) * float2(this.e45, this.e4125)) - float2(2.0)*(float2(this.e4315, this.e45) * this.group1.zy)), 0.0) - float3(2.0)*(float3(this.e1234) * this.group2.xyz) - float3(2.0)*(this.group0.yzx * this.group2.zxy)), 0.0), 
            /* e5 */
            2.0*(this.e45 * this.e3215) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) - 2.0 * (this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)
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
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(this.scalar * this.e12345) + 2.0 * (this.e41 * this.e235) + 2.0 * (this.e42 * this.e315) + 2.0 * (this.e43 * this.e125) + 2.0 * (this.e23 * this.e415) + 2.0 * (this.e31 * this.e425) + 2.0 * (this.e12 * this.e435) + 2.0 * (this.e423 * this.e15) + 2.0 * (this.e431 * this.e25) + 2.0 * (this.e412 * this.e35) + 2.0 * (this.e1 * this.e4235) + 2.0 * (this.e2 * this.e4315) + 2.0 * (this.e3 * this.e4125) + 2.0 * (this.e4 * this.e3215) + 2.0 * (this.e45 * this.e321) + 2.0 * (this.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(2.0)*(float3(this.scalar) * this.group9.xyz) + float3(2.0)*(float3(this.e12345) * this.group1.xyz) + float3(2.0)*(float3(this.e4) * this.group8.xyz) + float3(2.0)*(float3(this.e25, this.e3215, this.e3215) * this.group4.zyz) + float3(2.0)*(float3(this.e3215, this.e35, this.e15) * this.group4.xxy) + float3(2.0)*(this.group5.yzx * this.group9.zxy) + float3(2.0)*(this.group7.yzx * this.group8.zxy) + float3((float2(2.0)*(this.group1.zx * this.group6.yz) - float2(2.0)*(this.group1.yz * this.group6.zx)), 2.0*(this.e2 * this.e415) - 2.0 * (this.e1 * this.e425)) - float3(2.0)*(float3(this.e321) * this.group6.xyz) - float3(2.0)*(float3(this.e1234) * this.group3.xyz) - float3(2.0)*(float3(this.e315, this.e5, this.e5) * this.group7.zyz) - float3(2.0)*(float3(this.e45, this.e4125, this.e4235) * this.group5.xxy) - float3(2.0)*(float3(this.e4315, this.e45, this.e45) * this.group5.zyz) - float3(2.0)*(float3(this.e5, this.e125, this.e235) * this.group7.xxy) - float3(2.0)*(this.group4.yzx * this.group3.zxy)), this.e12345 * this.e4 * 2.0), 
            /* e5 */
            2.0*(this.e12345 * this.e5) + 2.0 * (this.e235 * this.e1) + 2.0 * (this.e235 * this.e415) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e315 * this.e425) + 2.0 * (this.e125 * this.e3) + 2.0 * (this.e125 * this.e435) + 2.0 * (this.e321 * this.e5) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) + 2.0 * (this.e45 * this.e3215) - 2.0 * (this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35) - 2.0 * (this.scalar * this.e3215), 
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
            float4((float3(2.0)*(float3(this.e12345) * this.group9.xyz) + float3(2.0)*(float3(this.e315, this.e5, this.e5) * this.group4.zyz) + float3(2.0)*(float3(this.e2, this.e321, this.e321) * this.group5.zyz) + float3(2.0)*(float3(this.e25, this.e3215, this.e3215) * this.group7.zyz) + float3(2.0)*(float3(this.e321, this.e3, this.e1) * this.group5.xxy) + float3(2.0)*(float3(this.e3215, this.e35, this.e15) * this.group7.xxy) + float3(2.0)*(float3(this.e5, this.e125, this.e235) * this.group4.xxy) + float3(((this.group6.yz * this.group9.zx) - float2(2.0)*(this.group6.zx * this.group9.yz)), (this.e415 * this.e4315) - 2.0 * (this.e425 * this.e4235)) - float3(2.0)*(float3(this.scalar) * this.group1.xyz) - float3(2.0)*(float3(this.e4) * this.group3.xyz) - float3(2.0)*(float3(this.e45) * this.group6.xyz) - float3(2.0)*(float3(this.e1234) * this.group8.xyz) - float3(2.0)*(this.group4.yzx * this.group8.zxy) - float3(2.0)*(this.group5.yzx * this.group1.zxy) - float3(2.0)*(this.group7.yzx * this.group3.zxy)), 2.0*(this.scalar * this.e5) + 2.0 * (this.e12345 * this.e3215) + (this.e1 * this.e15) + (this.e2 * this.e25) + (this.e3 * this.e35) + (this.e321 * this.e3215)), 
            /* e1234 */
            2.0*(this.scalar * this.e4) + 2.0 * (this.e12345 * this.e1234) + 2.0 * (this.e41 * this.e415) + 2.0 * (this.e42 * this.e425) + 2.0 * (this.e43 * this.e435) + 2.0 * (this.e23 * this.e423) + 2.0 * (this.e31 * this.e431) + 2.0 * (this.e12 * this.e412) + 2.0 * (this.e423 * this.e4235) + 2.0 * (this.e431 * this.e4315) + 2.0 * (this.e412 * this.e4125) + 2.0 * (this.e4 * this.e45) - 2.0 * (this.e41 * this.e1) - 2.0 * (this.e42 * this.e2) - 2.0 * (this.e43 * this.e3) - 2.0 * (this.e321 * this.e1234)
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
            float4(2.0)*(this.group0.yzxw * float4(this.group2.zxy, this.e4)) + float4(0.0, 0.0, 2.0*(this.e415 * this.e2) - 2.0 * (this.e431 * this.e235) - 2.0 * (this.e412 * this.e5) - 2.0 * (this.e425 * this.e1) - 2.0 * (this.e435 * this.e321), 0.0) + float4((float3(2.0)*(float3(this.e12345) * this.group3.xyz) + float3(2.0)*(float3(this.e4) * this.group2.xyz) + float3((float2(2.0)*(this.group1.yz * this.group3.zx) - float2(2.0)*(float2(this.e423) * this.group2.wz) - float2(2.0)*(float2(this.e415) * float2(this.e321, this.e3)) - float2(2.0)*(float2(this.e2, this.e321) * this.group1.zy) - float2(2.0)*(this.group0.zy * this.group2.yw)), 0.0)), 0.0), 
            /* e5 */
            2.0*(this.e12345 * this.e5) + 2.0 * (this.e415 * this.e235) + 2.0 * (this.e425 * this.e315) + 2.0 * (this.e435 * this.e125) + 2.0 * (this.e321 * this.e5) + 2.0 * (this.e235 * this.e1) + 2.0 * (this.e315 * this.e2) + 2.0 * (this.e125 * this.e3)
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
            float4(0.0, 0.0, 2.0*(this.e42 * this.e15) + 2.0 * (this.e43 * this.e3215) + 2.0 * (this.e23 * this.e4315) - 2.0 * (this.e41 * this.e25) - 2.0 * (this.e31 * this.e4235) - 2.0 * (this.e12 * this.e45), 0.0) + float4((float3(2.0)*(float3(this.scalar) * this.group3.xyz) + float3((float2(2.0)*(float2(this.e41) * float2(this.e3215, this.e35)) + float2(2.0)*(float2(this.e25, this.e3215) * this.group0.zy) + float2(2.0)*(this.group1.yz * this.group3.zx) - float2(2.0)*(float2(this.e23) * float2(this.e45, this.e4125)) - float2(2.0)*(float2(this.e4315, this.e45) * this.group1.zy) - float2(2.0)*(this.group0.yz * this.group2.zx)), 0.0) - float3(2.0)*(float3(this.e1234) * this.group2.xyz)), 0.0), 
            /* e5 */
            2.0*(this.e45 * this.e3215) + 2.0 * (this.e15 * this.e4235) + 2.0 * (this.e25 * this.e4315) + 2.0 * (this.e35 * this.e4125) - 2.0 * (this.scalar * this.e3215) - 2.0 * (this.e23 * this.e15) - 2.0 * (this.e31 * this.e25) - 2.0 * (this.e12 * this.e35)
        );
    }
}
