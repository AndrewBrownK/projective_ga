implementing rga3d;
using traits;
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_norm();
}
public extension Flector {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_norm();
}
public extension Line {
    public float unitized_norm() {
        Plane wedge = Plane(
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
        Line sub_type_2 = Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return (pow(sub_type_2.e41, 2) * wedge.e321) + (pow(sub_type_2.e42, 2) * wedge.e321) + (pow(sub_type_2.e43, 2) * wedge.e321);
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_norm();
}
public extension Motor {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_norm();
}
public extension MultiVector {
    public float unitized_norm() {
        return 0.0;
    }
}
// Fancy postfix trick
public float operator /(Point lhs, UnitizedNormPrefixOrPostfix rhs)  {
    return lhs.unitized_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.unitized_norm();
}
public extension Point {
    public float unitized_norm() {
        return 0.0;
    }
}
