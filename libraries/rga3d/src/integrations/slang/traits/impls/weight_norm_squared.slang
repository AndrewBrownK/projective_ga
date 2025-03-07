implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.weight_norm_squared();
}
public extension AntiScalar {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.weight_norm_squared();
}
public extension DualNum {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.weight_norm_squared();
}
public extension Flector {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.weight_norm_squared();
}
public extension Line {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.weight_norm_squared();
}
public extension Motor {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.weight_norm_squared();
}
public extension MultiVector {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * this.e1234 + this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Origin lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Origin rhs) {
    return rhs.weight_norm_squared();
}
public extension Origin {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.weight_norm_squared();
}
public extension Plane {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.weight_norm_squared();
}
public extension Point {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4
        );
    }
}
