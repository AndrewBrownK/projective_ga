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
        float3 sub_type_g0_xyz = this.group1.xyz;
        return (sub_type_g0_xyz[0] * sub_type_g0_xyz[0] * this.e41 * this.e41) + (sub_type_g0_xyz[1] * sub_type_g0_xyz[1] * this.e41 * this.e41) + (sub_type_g0_xyz[2] * sub_type_g0_xyz[2] * this.e41 * this.e41) + (this.e41 * this.e41 * this.scalar * this.scalar);
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
        float3 sub_type_g1_xyz = this.group3.xyz;
        return (sub_type_g1_xyz[0] * sub_type_g1_xyz[0] * this.e4 * this.e4) + (sub_type_g1_xyz[1] * sub_type_g1_xyz[1] * this.e4 * this.e4) + (sub_type_g1_xyz[2] * sub_type_g1_xyz[2] * this.e4 * this.e4) + (this.e321 * this.e321 * this.e4 * this.e4);
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
        return this.e423 * this.e423 * this.e321 * this.e321;
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
        return this.e423 * this.e423 * this.e321 * this.e321;
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
        float3 sub_type_g0 = this.group1.xyz;
        return (sub_type_g0[0] * sub_type_g0[0] * this.e41 * this.e41) + (sub_type_g0[1] * sub_type_g0[1] * this.e41 * this.e41) + (sub_type_g0[2] * sub_type_g0[2] * this.e41 * this.e41);
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
        float3 sub_type_g0 = this.group1.xyz;
        return (sub_type_g0[0] * sub_type_g0[0] * this.e41 * this.e41) + (sub_type_g0[1] * sub_type_g0[1] * this.e41 * this.e41) + (sub_type_g0[2] * sub_type_g0[2] * this.e41 * this.e41);
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
        float3 sub_type_g1_xyz = this.group3.xyz;
        return (sub_type_g1_xyz[0] * sub_type_g1_xyz[0] * this.e4 * this.e4) + (sub_type_g1_xyz[1] * sub_type_g1_xyz[1] * this.e4 * this.e4) + (sub_type_g1_xyz[2] * sub_type_g1_xyz[2] * this.e4 * this.e4) + (this.e321 * this.e321 * this.e4 * this.e4);
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
        float3 sub_type_g0_xyz = this.group1.xyz;
        float3 sub_type_g0 = this.group0.xyz;
        return (sub_type_g0[0] * sub_type_g0[0] * sub_type_g0_xyz[0] * sub_type_g0_xyz[0]) + (sub_type_g0[0] * sub_type_g0[0] * sub_type_g0_xyz[1] * sub_type_g0_xyz[1]) + (sub_type_g0[0] * sub_type_g0[0] * sub_type_g0_xyz[2] * sub_type_g0_xyz[2]) + (sub_type_g0[0] * sub_type_g0[0] * this.scalar * this.scalar);
    }
}
