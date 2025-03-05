implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.weight_norm_squared();
}
public extension AntiScalar {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            pow(this.e1234, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.weight_norm_squared();
}
public extension DualNum {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            pow(this.e1234, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.weight_norm_squared();
}
public extension Flector {
    public AntiScalar weight_norm_squared() {
        Flector sub_type = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type.e4, 2) + pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.weight_norm_squared();
}
public extension Line {
    public AntiScalar weight_norm_squared() {
        Line sub_type = Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.weight_norm_squared();
}
public extension Motor {
    public AntiScalar weight_norm_squared() {
        Motor sub_type = Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2) + pow(sub_type.e1234, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.weight_norm_squared();
}
public extension MultiVector {
    public AntiScalar weight_norm_squared() {
        MultiVector sub_type = MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type.e1234, 2) + pow(sub_type.e4, 2) + pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2) + pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Origin lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Origin rhs) {
    return rhs.weight_norm_squared();
}
public extension Origin {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            pow(this.e4, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.weight_norm_squared();
}
public extension Plane {
    public AntiScalar weight_norm_squared() {
        Plane sub_type = Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, WeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.weight_norm_squared();
}
public extension Point {
    public AntiScalar weight_norm_squared() {
        return AntiScalar(
            /* e1234 */
            pow(this.e4, 2)
        );
    }
}
