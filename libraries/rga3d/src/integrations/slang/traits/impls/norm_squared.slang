implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(NormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.norm_squared();
}
public extension Flector {
    public AntiScalar norm_squared() {
        float3 sub_type_g1_xyz = this.group1.xyz;
        return AntiScalar(
            /* e1234 */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e4 * this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(NormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.norm_squared();
}
public extension Line {
    public AntiScalar norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(NormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.norm_squared();
}
public extension Motor {
    public AntiScalar norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(NormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.norm_squared();
}
public extension MultiVector {
    public AntiScalar norm_squared() {
        float3 sub_type_g4_xyz = this.group4.xyz;
        return AntiScalar(
            /* e1234 */
            sub_type_g4_xyz[0] * sub_type_g4_xyz[0] + sub_type_g4_xyz[1] * sub_type_g4_xyz[1] + sub_type_g4_xyz[2] * sub_type_g4_xyz[2] + this.e1234 * this.e1234 + this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, NormSquaredPrefixOrPostfix rhs)  {
    return lhs.norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(NormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.norm_squared();
}
public extension Point {
    public AntiScalar norm_squared() {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4
        );
    }
}
