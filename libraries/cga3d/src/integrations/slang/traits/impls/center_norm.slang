implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.center_norm();
}
public extension AntiCircleRotor {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.center_norm();
}
public extension AntiDipoleInversion {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.center_norm();
}
public extension Circle {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.center_norm();
}
public extension CircleRotor {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.center_norm();
}
public extension Dipole {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.center_norm();
}
public extension DipoleInversion {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.center_norm();
}
public extension MultiVector {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321 - this.e12345 * this.e12345 - this.e45 * this.e45 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.center_norm();
}
public extension VersorEven {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e12345 * this.e12345 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, CenterNormPrefixOrPostfix rhs)  {
    return lhs.center_norm();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.center_norm();
}
public extension VersorOdd {
    public Scalar center_norm() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
