implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_center_norm();
}
public extension AntiCircleRotor {
    public float unitized_center_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_center_norm();
}
public extension AntiDipoleInversion {
    public float unitized_center_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_center_norm();
}
public extension Circle {
    public float unitized_center_norm() {
        return this.e423 * this.e423 * this.e321 * this.e321;
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_center_norm();
}
public extension CircleRotor {
    public float unitized_center_norm() {
        return this.e423 * this.e423 * this.e321 * this.e321;
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_center_norm();
}
public extension Dipole {
    public float unitized_center_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_center_norm();
}
public extension DipoleInversion {
    public float unitized_center_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_center_norm();
}
public extension MultiVector {
    public float unitized_center_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_center_norm();
}
public extension VersorEven {
    public float unitized_center_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedCenterNormPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_center_norm();
}
public extension VersorOdd {
    public float unitized_center_norm() {
        return this.e41 * this.e41 * this.e23 * this.e23;
    }
}
