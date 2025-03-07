implementing cga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Circle {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Dipole {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension DualNum {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(FlatPoint lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Flector {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Line {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Motor {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension MultiVector {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * this.e12345 + this.e45 * this.e45 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Plane {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Sphere {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension VersorEven {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * this.e12345 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125
        );
    }
}
