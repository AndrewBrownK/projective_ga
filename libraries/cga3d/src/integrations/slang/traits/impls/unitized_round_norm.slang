implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_round_norm();
}
public extension AntiCircleRotor {
    public float unitized_round_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_round_norm();
}
public extension AntiDipoleInversion {
    public float unitized_round_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_round_norm();
}
public extension Circle {
    public float unitized_round_norm() {
        return (this.e423 * this.e423 * this.e321) + (this.e431 * this.e431 * this.e321) + (this.e412 * this.e412 * this.e321);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_round_norm();
}
public extension CircleRotor {
    public float unitized_round_norm() {
        return (this.e423 * this.e423 * this.e321) + (this.e431 * this.e431 * this.e321) + (this.e412 * this.e412 * this.e321);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_round_norm();
}
public extension Dipole {
    public float unitized_round_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_round_norm();
}
public extension DipoleInversion {
    public float unitized_round_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_round_norm();
}
public extension MultiVector {
    public float unitized_round_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_round_norm();
}
public extension RoundPoint {
    public float unitized_round_norm() {
        return (this.e1 * this.e1 / this.e4) + (this.e2 * this.e2 / this.e4) + (this.e3 * this.e3 / this.e4);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_round_norm();
}
public extension VersorEven {
    public float unitized_round_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_round_norm();
}
public extension VersorOdd {
    public float unitized_round_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
