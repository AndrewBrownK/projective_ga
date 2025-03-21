implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_round_norm_squared() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_round_norm_squared() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension Circle {
    public float unitized_round_norm_squared() {
        return (this.e423 * this.e423 * this.e321 * this.e321) + (this.e431 * this.e431 * this.e321 * this.e321) + (this.e412 * this.e412 * this.e321 * this.e321);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension CircleRotor {
    public float unitized_round_norm_squared() {
        return (this.e423 * this.e423 * this.e321 * this.e321) + (this.e431 * this.e431 * this.e321 * this.e321) + (this.e412 * this.e412 * this.e321 * this.e321);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension Dipole {
    public float unitized_round_norm_squared() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension DipoleInversion {
    public float unitized_round_norm_squared() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension MultiVector {
    public float unitized_round_norm_squared() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension RoundPoint {
    public float unitized_round_norm_squared() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return (sub_type_g0_xyz[0] * sub_type_g0_xyz[0] /  (this.e4 * this.e4)) + (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] /  (this.e4 * this.e4)) + (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] /  (this.e4 * this.e4));
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension VersorEven {
    public float unitized_round_norm_squared() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_round_norm_squared();
}
public extension VersorOdd {
    public float unitized_round_norm_squared() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
