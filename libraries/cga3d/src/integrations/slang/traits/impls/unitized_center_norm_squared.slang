implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_center_norm_squared() {
        return (this.e41 * this.e41 * pow(this.e23, 2)) + (this.e41 * this.e41 * pow(this.e31, 2)) + (this.e41 * this.e41 * pow(this.e12, 2)) + (this.e41 * this.e41 * pow(this.scalar, 2));
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_center_norm_squared() {
        return (this.e321 * this.e321 * pow(this.e4, 2)) + (this.e4 * this.e4 * pow(this.e1, 2)) + (this.e4 * this.e4 * pow(this.e2, 2)) + (this.e4 * this.e4 * pow(this.e3, 2));
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Circle {
    public float unitized_center_norm_squared() {
        return this.e423 * this.e423 * pow(this.e321, 2);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension CircleRotor {
    public float unitized_center_norm_squared() {
        return this.e423 * this.e423 * pow(this.e321, 2);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Dipole {
    public float unitized_center_norm_squared() {
        return (this.e41 * this.e41 * pow(this.e23, 2)) + (this.e41 * this.e41 * pow(this.e31, 2)) + (this.e41 * this.e41 * pow(this.e12, 2));
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension DipoleInversion {
    public float unitized_center_norm_squared() {
        return (this.e41 * this.e41 * pow(this.e23, 2)) + (this.e41 * this.e41 * pow(this.e31, 2)) + (this.e41 * this.e41 * pow(this.e12, 2));
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension MultiVector {
    public float unitized_center_norm_squared() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorEven {
    public float unitized_center_norm_squared() {
        return (this.e321 * this.e321 * pow(this.e4, 2)) + (this.e1 * this.e1 * pow(this.e4, 2)) + (this.e2 * this.e2 * pow(this.e4, 2)) + (this.e3 * this.e3 * pow(this.e4, 2));
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorOdd {
    public float unitized_center_norm_squared() {
        return (this.e41 * this.e41 * pow(this.scalar, 2)) + (this.e41 * this.e41 * pow(this.e23, 2)) + (this.e41 * this.e41 * pow(this.e31, 2)) + (this.e41 * this.e41 * pow(this.e12, 2));
    }
}
