implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.weight_norm();
}
public extension AntiScalar {
    public AntiScalar weight_norm() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.weight_norm();
}
public extension DualNum {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.weight_norm();
}
public extension Flector {
    public AntiScalar weight_norm() {
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
public AntiScalar operator /(Line lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.weight_norm();
}
public extension Line {
    public AntiScalar weight_norm() {
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
public AntiScalar operator /(Motor lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.weight_norm();
}
public extension Motor {
    public AntiScalar weight_norm() {
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
public AntiScalar operator /(MultiVector lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.weight_norm();
}
public extension MultiVector {
    public AntiScalar weight_norm() {
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
public AntiScalar operator /(Origin lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Origin rhs) {
    return rhs.weight_norm();
}
public extension Origin {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.weight_norm();
}
public extension Plane {
    public AntiScalar weight_norm() {
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
public AntiScalar operator /(Point lhs, WeightNormPrefixOrPostfix rhs)  {
    return lhs.weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightNormPrefixOrPostfix lhs, Point rhs) {
    return rhs.weight_norm();
}
public extension Point {
    public AntiScalar weight_norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
