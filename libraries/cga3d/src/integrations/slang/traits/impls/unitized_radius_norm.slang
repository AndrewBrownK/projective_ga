implementing cga3d;
using traits;
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
        float anti_dot_product_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.scalar * this.scalar - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return -(this.e41 * this.e41 * pow(anti_dot_product_g0, 0.5)) - (this.e42 * this.e42 * pow(anti_dot_product_g0, 0.5)) - (this.e43 * this.e43 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return -(this.e423 * this.e423 * pow(anti_dot_product_g0, 0.5)) - (this.e431 * this.e431 * pow(anti_dot_product_g0, 0.5)) - (this.e412 * this.e412 * pow(anti_dot_product_g0, 0.5)) - (this.e4 * this.e4 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321;
        return -(this.e423 * this.e423 * pow(anti_dot_product_g0, 0.5)) - (this.e431 * this.e431 * pow(anti_dot_product_g0, 0.5)) - (this.e412 * this.e412 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345 - this.e321 * this.e321;
        return -(this.e423 * this.e423 * pow(anti_dot_product_g0, 0.5)) - (this.e431 * this.e431 * pow(anti_dot_product_g0, 0.5)) - (this.e412 * this.e412 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return -(this.e41 * this.e41 * pow(anti_dot_product_g0, 0.5)) - (this.e42 * this.e42 * pow(anti_dot_product_g0, 0.5)) - (this.e43 * this.e43 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return -(this.e41 * this.e41 * pow(anti_dot_product_g0, 0.5)) - (this.e42 * this.e42 * pow(anti_dot_product_g0, 0.5)) - (this.e43 * this.e43 * pow(anti_dot_product_g0, 0.5)) - (this.e1234 * this.e1234 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = 2.0*(this.e4 * this.e5) + 2.0 * (this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e12345 * this.e12345 + this.e45 * this.e45 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.e321 * this.e321 - 2.0 * (this.e15 * this.e41) - 2.0 * (this.e25 * this.e42) - 2.0 * (this.e35 * this.e43) - 2.0 * (this.e3215 * this.e1234);
        return -(this.e4 * this.e4 * pow(anti_dot_product_g0, 0.5)) - (this.e41 * this.e41 * pow(anti_dot_product_g0, 0.5)) - (this.e42 * this.e42 * pow(anti_dot_product_g0, 0.5)) - (this.e43 * this.e43 * pow(anti_dot_product_g0, 0.5)) - (this.e423 * this.e423 * pow(anti_dot_product_g0, 0.5)) - (this.e431 * this.e431 * pow(anti_dot_product_g0, 0.5)) - (this.e412 * this.e412 * pow(anti_dot_product_g0, 0.5)) - (this.e1234 * this.e1234 * pow(anti_dot_product_g0, 0.5));
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
        return (this.e1 * this.e1 / (this.e4)) + (this.e2 * this.e2 / (this.e4)) + (this.e3 * this.e3 / (this.e4)) - 2.0 * this.e5;
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
        return 2.0*this.e3215 - (this.e4235 * this.e4235 / (this.e1234)) - (this.e4315 * this.e4315 / (this.e1234)) - (this.e4125 * this.e4125 / (this.e1234));
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
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + this.e12345 * this.e12345 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return -(this.e423 * this.e423 * pow(anti_dot_product_g0, 0.5)) - (this.e431 * this.e431 * pow(anti_dot_product_g0, 0.5)) - (this.e412 * this.e412 * pow(anti_dot_product_g0, 0.5)) - (this.e4 * this.e4 * pow(anti_dot_product_g0, 0.5));
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
        float anti_dot_product_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return -(this.e41 * this.e41 * pow(anti_dot_product_g0, 0.5)) - (this.e42 * this.e42 * pow(anti_dot_product_g0, 0.5)) - (this.e43 * this.e43 * pow(anti_dot_product_g0, 0.5)) - (this.e1234 * this.e1234 * pow(anti_dot_product_g0, 0.5));
    }
}
