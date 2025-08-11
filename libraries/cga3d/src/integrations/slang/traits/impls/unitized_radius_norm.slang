implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_radius_norm();
}
public extension AntiCircleRotor {
    public float unitized_radius_norm() {
        return pow(this.e41, 3) * this.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_radius_norm();
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_radius_norm();
}
public extension Circle {
    public float unitized_radius_norm() {
        return this.e423 * this.e423 * this.e321 * this.e321;
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_radius_norm();
}
public extension CircleRotor {
    public float unitized_radius_norm() {
        return this.e423 * this.e423 * this.e321 * this.e321;
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_radius_norm();
}
public extension Dipole {
    public float unitized_radius_norm() {
        return pow(this.e41, 3) * this.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_radius_norm();
}
public extension DipoleInversion {
    public float unitized_radius_norm() {
        return pow(this.e41, 3) * this.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_radius_norm();
}
public extension MultiVector {
    public float unitized_radius_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_radius_norm();
}
public extension RoundPoint {
    public float unitized_radius_norm() {
        return (this.e1 * this.e1 / this.e4) + (this.e2 * this.e2 / this.e4) + (this.e3 * this.e3 / this.e4) - 2.0 * this.e5;
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_radius_norm();
}
public extension Sphere {
    public float unitized_radius_norm() {
        return 2.0*this.e3215 - (this.e4235 * this.e4235 / this.e1234) - (this.e4315 * this.e4315 / this.e1234) - (this.e4125 * this.e4125 / this.e1234);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_radius_norm();
}
public extension VersorEven {
    public float unitized_radius_norm() {
        return this.e321 * this.e321 * this.e4 * this.e4;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRadiusNormPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_radius_norm();
}
public extension VersorOdd {
    public float unitized_radius_norm() {
        return pow(this.e41, 3) * this.e15 * 2.0;
    }
}
