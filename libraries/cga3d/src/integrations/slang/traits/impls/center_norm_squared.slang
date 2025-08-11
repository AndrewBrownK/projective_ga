implementing cga3d;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar center_norm_squared() {
        float3 sub_type_g0_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + this.scalar * this.scalar - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar center_norm_squared() {
        float3 sub_type_g1_xyz = this.group3.xyz;
        float3 sub_type_g0 = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e321 * this.e321 - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.center_norm_squared();
}
public extension Circle {
    public Scalar center_norm_squared() {
        float3 sub_type_g0 = this.group1.xyz;
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension CircleRotor {
    public Scalar center_norm_squared() {
        float3 sub_type_g0_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.center_norm_squared();
}
public extension Dipole {
    public Scalar center_norm_squared() {
        float3 sub_type_g0 = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension DipoleInversion {
    public Scalar center_norm_squared() {
        float3 sub_type_g0 = this.group1.xyz;
        float3 sub_type_g1_xyz = this.group3.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - sub_type_g1_xyz[0] * sub_type_g1_xyz[0] - sub_type_g1_xyz[1] * sub_type_g1_xyz[1] - sub_type_g1_xyz[2] * sub_type_g1_xyz[2] - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.center_norm_squared();
}
public extension MultiVector {
    public Scalar center_norm_squared() {
        float3 sub_type_g1_xyz = this.group1.xyz;
        float3 sub_type_g6_xyz = this.group6.xyz;
        float3 sub_type_g9_xyz = this.group9.xyz;
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321 - this.e12345 * this.e12345 - sub_type_g6_xyz[0] * sub_type_g6_xyz[0] - sub_type_g6_xyz[1] * sub_type_g6_xyz[1] - sub_type_g6_xyz[2] * sub_type_g6_xyz[2] - sub_type_g9_xyz[0] * sub_type_g9_xyz[0] - sub_type_g9_xyz[1] * sub_type_g9_xyz[1] - sub_type_g9_xyz[2] * sub_type_g9_xyz[2] - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.center_norm_squared();
}
public extension VersorEven {
    public Scalar center_norm_squared() {
        float3 sub_type_g1_xyz = this.group3.xyz;
        float3 sub_type_g0_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e321 * this.e321 - sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.center_norm_squared();
}
public extension VersorOdd {
    public Scalar center_norm_squared() {
        float3 sub_type_g0_xyz = this.group1.xyz;
        float3 sub_type_g1_xyz = this.group3.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + this.scalar * this.scalar - sub_type_g1_xyz[0] * sub_type_g1_xyz[0] - sub_type_g1_xyz[1] * sub_type_g1_xyz[1] - sub_type_g1_xyz[2] * sub_type_g1_xyz[2] - this.e45 * this.e45
        );
    }
}
