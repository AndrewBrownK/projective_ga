implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.weight_norm();
}
public extension AntiScalar {
    public AntiScalar weight_norm() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.weight_norm();
}
public extension DualNum {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.weight_norm();
}
public extension Flector {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.weight_norm();
}
public extension Line {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.weight_norm();
}
public extension Motor {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.weight_norm();
}
public extension MultiVector {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * this.e1234 + this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Origin lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Origin rhs) {
    return rhs.weight_norm();
}
public extension Origin {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.weight_norm();
}
public extension Plane {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.weight_norm();
}
public extension Point {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
