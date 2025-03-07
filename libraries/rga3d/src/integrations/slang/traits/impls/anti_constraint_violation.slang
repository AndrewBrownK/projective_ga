implementing rga3d;
// Fancy postfix trick
public Scalar operator /(DualNum lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_constraint_violation();
}
public extension DualNum {
    public Scalar anti_constraint_violation() {
        return Scalar(
            /* scalar */
            this.scalar * this.e1234 * 2.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_constraint_violation();
}
public extension Flector {
    public Scalar anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0);
        return Scalar(
            /* scalar */
            (this.e1 * this.e423) + (this.e2 * this.e431) + (this.e3 * this.e412) + (this.e4 * this.e321) - (anti_reverse_g0[0] * this.e423) - (anti_reverse_g0[1] * this.e431) - (anti_reverse_g0[2] * this.e412) - (anti_reverse_g0[3] * this.e321)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_constraint_violation();
}
public extension Line {
    public Scalar anti_constraint_violation() {
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(anti_reverse_g0[0] * this.e23) - (anti_reverse_g0[1] * this.e31) - (anti_reverse_g0[2] * this.e12) - (anti_reverse_g1[0] * this.e41) - (anti_reverse_g1[1] * this.e42) - (anti_reverse_g1[2] * this.e43)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, AntiConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.anti_constraint_violation();
    }
// Fancy prefix trick
public Scalar operator /(AntiConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_constraint_violation();
}
public extension Motor {
    public Scalar anti_constraint_violation() {
        float4 anti_reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (anti_reverse_g0[3] * this.scalar) + (anti_reverse_g1[3] * this.e1234) - (anti_reverse_g0[0] * this.e23) - (anti_reverse_g0[1] * this.e31) - (anti_reverse_g0[2] * this.e12) - (anti_reverse_g1[0] * this.e41) - (anti_reverse_g1[1] * this.e42) - (anti_reverse_g1[2] * this.e43)
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
        float4 anti_reverse_g1 = this.group1 * float4(-1.0);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(2.0*(this.scalar * this.e1234) + (this.e1 * this.e423) + (this.e2 * this.e431) + (this.e3 * this.e412) + (this.e4 * this.e321) - (anti_reverse_g2[0] * this.e23) - (anti_reverse_g2[1] * this.e31) - (anti_reverse_g2[2] * this.e12) - (anti_reverse_g3[0] * this.e41) - (anti_reverse_g3[1] * this.e42) - (anti_reverse_g3[2] * this.e43) - (anti_reverse_g1[1] * this.e431) - (anti_reverse_g1[2] * this.e412) - (anti_reverse_g1[3] * this.e321) - (anti_reverse_g1.xwzw[0] * this.e423), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, this.e321) * float4(anti_reverse_g2.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(anti_reverse_g2.zyz, anti_reverse_g3[0])) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(this.group4.xyz, anti_reverse_g1[3])) + (float4(anti_reverse_g1.ww, this.e431, this.e431) * float4(this.group2.xyx, anti_reverse_g3[1])) + (float4(this.group4.zx, anti_reverse_g1[3], this.e412) * float4(this.group2.yzz, anti_reverse_g3[2])) + float4(float3(0.0), -(anti_reverse_g2[1] * this.e2) - (anti_reverse_g2[2] * this.e3) - (anti_reverse_g1[0] * this.e41) - (anti_reverse_g1[1] * this.e42) - (anti_reverse_g1[2] * this.e43) - (this.e23 * this.e423) - (this.e31 * this.e431) - (this.e12 * this.e412)) - float4((anti_reverse_g2.yzx * this.group4.zxy), this.scalar * this.e4) - float4((this.group2.zxy * this.group4.yzx), anti_reverse_g2[0] * this.e1)
        );
    }
}
