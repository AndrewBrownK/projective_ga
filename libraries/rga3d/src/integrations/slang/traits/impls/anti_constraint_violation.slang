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
            (this.scalar * this.e1234) * 2.0
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
        return Scalar(
            /* scalar */
            -2.0*(this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
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
        return Scalar(
            /* scalar */
            2.0*(this.e1234 * this.scalar) - 2.0 * (this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12)
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
            /* scalar, e1234 */
            float2(2.0*(this.scalar * this.e1234) - 2.0 * (this.e41 * this.e23) - 2.0 * (this.e42 * this.e31) - 2.0 * (this.e43 * this.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(2.0)*(float4(this.e1234) * this.group4) + float4(this.e4 * this.e41 * 2.0, this.e4 * this.e42 * 2.0, this.e4 * this.e43 * 2.0, -2.0*(this.e1 * this.e41) - 2.0 * (this.e2 * this.e42) - 2.0 * (this.e3 * this.e43))
        );
    }
}
