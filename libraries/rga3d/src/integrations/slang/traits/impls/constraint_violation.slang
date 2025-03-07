implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiScalar operator /(ConstraintViolationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.constraint_violation();
}
public extension DualNum {
    public AntiScalar constraint_violation() {
        return AntiScalar(
            /* e1234 */
            this.scalar * this.e1234 * 2.0
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiScalar operator /(ConstraintViolationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.constraint_violation();
}
public extension Flector {
    public AntiScalar constraint_violation() {
        float4 reverse_g1 = this.group1 * float4(-1.0);
        return AntiScalar(
            /* e1234 */
            (reverse_g1[0] * this.e1) + (reverse_g1[1] * this.e2) + (reverse_g1[2] * this.e3) + (reverse_g1[3] * this.e4) - (this.e1 * this.e423) - (this.e2 * this.e431) - (this.e3 * this.e412) - (this.e4 * this.e321)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiScalar operator /(ConstraintViolationPrefixOrPostfix lhs, Line rhs) {
    return rhs.constraint_violation();
}
public extension Line {
    public AntiScalar constraint_violation() {
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return AntiScalar(
            /* e1234 */
            -(reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, ConstraintViolationPrefixOrPostfix rhs)  {
    return lhs.constraint_violation();
    }
// Fancy prefix trick
public AntiScalar operator /(ConstraintViolationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.constraint_violation();
}
public extension Motor {
    public AntiScalar constraint_violation() {
        float4 reverse_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 reverse_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e1234 */
            (reverse_g0[3] * this.scalar) + (reverse_g1[3] * this.e1234) - (reverse_g0[0] * this.e23) - (reverse_g0[1] * this.e31) - (reverse_g0[2] * this.e12) - (reverse_g1[0] * this.e41) - (reverse_g1[1] * this.e42) - (reverse_g1[2] * this.e43)
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
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (reverse_g4[0] * this.e1) + (reverse_g4[1] * this.e2) + (reverse_g4[2] * this.e3) + (reverse_g4[3] * this.e4) + 2.0 * (this.scalar * this.e1234) - (reverse_g2[0] * this.e23) - (reverse_g2[1] * this.e31) - (reverse_g2[2] * this.e12) - (reverse_g3[0] * this.e41) - (reverse_g3[1] * this.e42) - (reverse_g3[2] * this.e43) - (this.group4.wxzw[1] * this.e1) - (this.e2 * this.e431) - (this.e3 * this.e412) - (this.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * this.group1) + (float4(reverse_g4[3], this.e3, this.e1, this.e2) * float4(this.group3.xxy, this.e42)) + (float4(this.e2, reverse_g4[3], reverse_g4[3], this.e3) * float4(this.group3.zyz, this.e43)) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(this.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, this.e1) * float4(reverse_g3.yzz, this.e41)) + (float4(this.group4.ww, this.e2, this.e4) * float4(reverse_g3.xyx, this.scalar)) + float4(float3(0.0), -(reverse_g2[1] * this.e2) - (reverse_g2[2] * this.e3) - (reverse_g3[0] * this.e423) - (reverse_g3[1] * this.e431) - (reverse_g3[2] * this.e412) - (reverse_g4[0] * this.e23) - (reverse_g4[1] * this.e31) - (reverse_g4[2] * this.e12)) - float4((reverse_g3.zxy * this.group1.yzx), reverse_g4[3] * this.e1234) - float4((this.group3.yzx * this.group1.zxy), reverse_g2[0] * this.e1), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
