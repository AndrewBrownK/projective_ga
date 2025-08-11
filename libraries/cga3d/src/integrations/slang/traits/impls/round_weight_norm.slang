implementing cga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_weight_norm();
}
public extension AntiCircleRotor {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_weight_norm();
}
public extension AntiDipoleInversion {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e4 * this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_weight_norm();
}
public extension Circle {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_weight_norm();
}
public extension CircleRotor {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_weight_norm();
}
public extension Dipole {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_weight_norm();
}
public extension DipoleInversion {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_weight_norm();
}
public extension MultiVector {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e4 * this.e4 + this.e1234 * this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_weight_norm();
}
public extension RoundPoint {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.round_weight_norm();
}
public extension Sphere {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_weight_norm();
}
public extension VersorEven {
    public AntiScalar round_weight_norm() {
        float3 wedge_g1_xyz = this.group0.xyz;
        return AntiScalar(
            /* e12345 */
            wedge_g1_xyz[0] * wedge_g1_xyz[0] + wedge_g1_xyz[1] * wedge_g1_xyz[1] + wedge_g1_xyz[2] * wedge_g1_xyz[2] + this.e4 * this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_weight_norm();
}
public extension VersorOdd {
    public AntiScalar round_weight_norm() {
        float3 sub_type_g0 = this.group0.xyz;
        return AntiScalar(
            /* e12345 */
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] + this.e1234 * this.e1234
        );
    }
}
