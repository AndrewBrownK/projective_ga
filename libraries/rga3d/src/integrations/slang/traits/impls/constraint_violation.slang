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
        return AntiScalar(
            /* e1234 */
            -2.0*(this.e1 * this.e423) - 2.0 * (this.e2 * this.e431) - 2.0 * (this.e3 * this.e412) - 2.0 * (this.e4 * this.e321)
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
        return AntiScalar(
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12)
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
        return AntiScalar(
            /* e1234 */
            2.0*(this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) + 2.0 * (this.e1234 * this.scalar)
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
            /* scalar, e1234 */
            float2(0.0, (this.scalar * this.e1234) + 2.0 * (this.e41 * this.e23) + 2.0 * (this.e42 * this.e31) + 2.0 * (this.e43 * this.e12) - 2.0 * (this.e1 * this.e423) - 2.0 * (this.e2 * this.e431) - 2.0 * (this.e3 * this.e412) - 2.0 * (this.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(2.0)*(float4(this.scalar) * this.group1) + float4(2.0)*(this.group1.yzxx * float4(this.group3.zxy, this.e41)) + float4(float3(0.0), 2.0*(this.e1234 * this.e321) + 2.0 * (this.e2 * this.e42) + 2.0 * (this.e3 * this.e43) + 2.0 * (this.e23 * this.e423) + 2.0 * (this.e31 * this.e431) + 2.0 * (this.e12 * this.e412)) - float4(2.0)*float4((float3(this.e321) * this.group3.xyz), 0.0) - float4(2.0)*float4((this.group3.yzx * this.group1.zxy), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
