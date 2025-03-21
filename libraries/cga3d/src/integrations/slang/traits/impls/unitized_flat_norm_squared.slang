implementing cga3d;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_flat_norm_squared() {
        float3 wedge_g0 = this.group2.xyz;
        return -(wedge_g0[0] * wedge_g0[0] /  (this.e45 * this.e45)) - (wedge_g0[1] * wedge_g0[1] /  (this.e45 * this.e45)) - (wedge_g0[2] * wedge_g0[2] /  (this.e45 * this.e45));
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e5 * this.e5;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Circle {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e235 * this.e235 * -1.0;
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension CircleRotor {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e235 * this.e235 * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Dipole {
    public float unitized_flat_norm_squared() {
        return -(this.e15 * this.e15 /  (this.e45 * this.e45)) - (this.e25 * this.e25 /  (this.e45 * this.e45)) - (this.e35 * this.e35 /  (this.e45 * this.e45));
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DipoleInversion {
    public float unitized_flat_norm_squared() {
        return this.e45 * this.e45 * this.group2[0] * -1.0;
    }
}
// Fancy postfix trick
public float operator /(DualNum lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DualNum {
    public float unitized_flat_norm_squared() {
        return this.e5 * this.e5 /  (this.e12345 * this.e12345);
    }
}
// Fancy postfix trick
public float operator /(FlatPoint lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension FlatPoint {
    public float unitized_flat_norm_squared() {
        float3 wedge_g0 = this.group0.xyz;
        return -(wedge_g0[0] * wedge_g0[0] /  (this.e45 * this.e45)) - (wedge_g0[1] * wedge_g0[1] /  (this.e45 * this.e45)) - (wedge_g0[2] * wedge_g0[2] /  (this.e45 * this.e45));
    }
}
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Flector {
    public float unitized_flat_norm_squared() {
        return this.e45 * this.e45 * this.group0[0] * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Line {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e235 * this.e235 * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Motor {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e5 * this.e5;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Plane lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Plane {
    public float unitized_flat_norm_squared() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return -(sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * this.e3215 * this.e3215) - (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * this.e3215 * this.e3215) - (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * this.e3215 * this.e3215);
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Sphere {
    public float unitized_flat_norm_squared() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return -(sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * this.e3215 * this.e3215) - (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * this.e3215 * this.e3215) - (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * this.e3215 * this.e3215);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorEven {
    public float unitized_flat_norm_squared() {
        return this.e415 * this.e415 * this.e5 * this.e5;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorOdd {
    public float unitized_flat_norm_squared() {
        return this.e45 * this.e45 * this.group2[0] * -1.0;
    }
}
