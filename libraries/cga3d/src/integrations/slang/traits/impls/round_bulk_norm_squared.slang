implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiDualNum {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiFlatPoint {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiFlector {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiLine {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiMotor {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiPlane {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Circle {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension CircleRotor {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Dipole {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension DipoleInversion {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension MultiVector {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension RoundPoint {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Scalar {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension VersorEven {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension VersorOdd {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
