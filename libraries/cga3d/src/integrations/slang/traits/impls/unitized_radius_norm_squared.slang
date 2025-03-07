implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.scalar * this.scalar - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return -(this.e41 * this.e41 * anti_dot_product_g0) - (this.e42 * this.e42 * anti_dot_product_g0) - (this.e43 * this.e43 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return -(this.e423 * this.e423 * anti_dot_product_g0) - (this.e431 * this.e431 * anti_dot_product_g0) - (this.e412 * this.e412 * anti_dot_product_g0) - (this.e4 * this.e4 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Circle {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321;
        return -(this.e423 * this.e423 * anti_dot_product_g0) - (this.e431 * this.e431 * anti_dot_product_g0) - (this.e412 * this.e412 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension CircleRotor {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345 - this.e321 * this.e321;
        return -(this.e423 * this.e423 * anti_dot_product_g0) - (this.e431 * this.e431 * anti_dot_product_g0) - (this.e412 * this.e412 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Dipole {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return -(this.e41 * this.e41 * anti_dot_product_g0) - (this.e42 * this.e42 * anti_dot_product_g0) - (this.e43 * this.e43 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension DipoleInversion {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return -(this.e41 * this.e41 * anti_dot_product_g0) - (this.e42 * this.e42 * anti_dot_product_g0) - (this.e43 * this.e43 * anti_dot_product_g0) - (this.e1234 * this.e1234 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension MultiVector {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = 2.0*(this.e4 * this.e5) + 2.0 * (this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e12345 * this.e12345 + this.e45 * this.e45 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.e321 * this.e321 - 2.0 * (this.e15 * this.e41) - 2.0 * (this.e25 * this.e42) - 2.0 * (this.e35 * this.e43) - 2.0 * (this.e3215 * this.e1234);
        return -(this.e4 * this.e4 * anti_dot_product_g0) - (this.e41 * this.e41 * anti_dot_product_g0) - (this.e42 * this.e42 * anti_dot_product_g0) - (this.e43 * this.e43 * anti_dot_product_g0) - (this.e423 * this.e423 * anti_dot_product_g0) - (this.e431 * this.e431 * anti_dot_product_g0) - (this.e412 * this.e412 * anti_dot_product_g0) - (this.e1234 * this.e1234 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension RoundPoint {
    public float unitized_radius_norm_squared() {
        return (this.e1 * this.e1 * pow(this.e4, -2)) + (this.e2 * this.e2 * pow(this.e4, -2)) + (this.e3 * this.e3 * pow(this.e4, -2)) - 2.0 * (this.e5 / (this.e4));
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Sphere {
    public float unitized_radius_norm_squared() {
        return 2.0*(this.e3215 / (this.e1234)) - (this.e4235 * this.e4235 * pow(this.e1234, -2)) - (this.e4315 * this.e4315 * pow(this.e1234, -2)) - (this.e4125 * this.e4125 * pow(this.e1234, -2));
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorEven {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + this.e12345 * this.e12345 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return -(this.e423 * this.e423 * anti_dot_product_g0) - (this.e431 * this.e431 * anti_dot_product_g0) - (this.e412 * this.e412 * anti_dot_product_g0) - (this.e4 * this.e4 * anti_dot_product_g0);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorOdd {
    public float unitized_radius_norm_squared() {
        float anti_dot_product_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return -(this.e41 * this.e41 * anti_dot_product_g0) - (this.e42 * this.e42 * anti_dot_product_g0) - (this.e43 * this.e43 * anti_dot_product_g0) - (this.e1234 * this.e1234 * anti_dot_product_g0);
    }
}
