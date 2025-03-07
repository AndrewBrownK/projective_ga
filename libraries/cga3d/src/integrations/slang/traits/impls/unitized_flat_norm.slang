implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_flat_norm();
}
public extension AntiCircleRotor {
    public float unitized_flat_norm() {
        float3 wedge_g0 = this.group2.xyz;
        return -(wedge_g0[0] * wedge_g0[0] / (this.e45)) - (wedge_g0[1] * wedge_g0[1] / (this.e45)) - (wedge_g0[2] * wedge_g0[2] / (this.e45));
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_flat_norm();
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e5, 2);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_flat_norm();
}
public extension Circle {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e235, 2);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_flat_norm();
}
public extension CircleRotor {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e235, 2);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_flat_norm();
}
public extension Dipole {
    public float unitized_flat_norm() {
        float3 wedge_g0 = this.group2.xyz;
        return -(wedge_g0[0] * wedge_g0[0] / (this.e45)) - (wedge_g0[1] * wedge_g0[1] / (this.e45)) - (wedge_g0[2] * wedge_g0[2] / (this.e45));
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_flat_norm();
}
public extension DipoleInversion {
    public float unitized_flat_norm() {
        return this.e45 * this.e45 * this.e15 * -1.0;
    }
}
// Fancy postfix trick
public float operator /(DualNum lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitized_flat_norm();
}
public extension DualNum {
    public float unitized_flat_norm() {
        return this.e5 / (this.e12345);
    }
}
// Fancy postfix trick
public float operator /(FlatPoint lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.unitized_flat_norm();
}
public extension FlatPoint {
    public float unitized_flat_norm() {
        float3 wedge_g0 = this.group0.xyz;
        return -(wedge_g0[0] * wedge_g0[0] / (this.e45)) - (wedge_g0[1] * wedge_g0[1] / (this.e45)) - (wedge_g0[2] * wedge_g0[2] / (this.e45));
    }
}
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_flat_norm();
}
public extension Flector {
    public float unitized_flat_norm() {
        return this.e45 * this.e45 * this.e15 * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_flat_norm();
}
public extension Line {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e235, 2);
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_flat_norm();
}
public extension Motor {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e5, 2);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_flat_norm();
}
public extension MultiVector {
    public float unitized_flat_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Plane lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitized_flat_norm();
}
public extension Plane {
    public float unitized_flat_norm() {
        return -(this.e4235 * this.e4235 * this.e3215) - (this.e4315 * this.e4315 * this.e3215) - (this.e4125 * this.e4125 * this.e3215);
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_flat_norm();
}
public extension Sphere {
    public float unitized_flat_norm() {
        return -(this.e4235 * this.e4235 * this.e3215) - (this.e4315 * this.e4315 * this.e3215) - (this.e4125 * this.e4125 * this.e3215);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_flat_norm();
}
public extension VersorEven {
    public float unitized_flat_norm() {
        return this.e415 * this.e415 * pow(this.e5, 2);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedFlatNormPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_flat_norm();
}
public extension VersorOdd {
    public float unitized_flat_norm() {
        return this.e45 * this.e45 * this.e15 * -1.0;
    }
}
