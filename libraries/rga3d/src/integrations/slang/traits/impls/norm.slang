implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.norm();
}
public extension Flector {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Line rhs) {
    return rhs.norm();
}
public extension Line {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.norm();
}
public extension Motor {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.norm();
}
public extension MultiVector {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * this.e1234 + this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Point rhs) {
    return rhs.norm();
}
public extension Point {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
