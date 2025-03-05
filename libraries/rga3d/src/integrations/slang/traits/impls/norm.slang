implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.norm();
}
public extension Flector {
    public AntiScalar norm() {
        Flector sub_type_2 = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
        return AntiScalar(
            /* e1234 */
            pow(sub_type_2.e4, 2) + pow(sub_type_2.e423, 2) + pow(sub_type_2.e431, 2) + pow(sub_type_2.e412, 2)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Line lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Line rhs) {
    return rhs.norm();
}
public extension Line {
    public DualNum norm() {
        Line sub_type_2 = Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, pow(sub_type_2.e41, 2) + pow(sub_type_2.e42, 2) + pow(sub_type_2.e43, 2))
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Motor lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.norm();
}
public extension Motor {
    public DualNum norm() {
        Motor sub_type = Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
        Origin other = Origin(
            /* e4 */
            1.0
        );
        Flector wedge = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), sub_type.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * sub_type.group1.xyz), 0.0)
        );
        Motor sub_type_2 = Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return DualNum(
            /* scalar, e1234 */
            float2(pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e321, 2), pow(sub_type_2.e41, 2) + pow(sub_type_2.e42, 2) + pow(sub_type_2.e43, 2) + pow(sub_type_2.e1234, 2))
        );
    }
}
// Fancy postfix trick
public DualNum operator /(MultiVector lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public DualNum operator /(NormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.norm();
}
public extension MultiVector {
    public DualNum norm() {
        MultiVector sub_type = MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
        Origin other = Origin(
            /* e4 */
            1.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e1234 */
            float2(1.0, sub_type.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), sub_type.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * sub_type.group3.xyz), 0.0)
        );
        MultiVector sub_type_2 = MultiVector(
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
        return DualNum(
            /* scalar, e1234 */
            float2(pow(wedge.scalar, 2) + pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) + pow(wedge.e321, 2), pow(sub_type_2.e1234, 2) + pow(sub_type_2.e4, 2) + pow(sub_type_2.e41, 2) + pow(sub_type_2.e42, 2) + pow(sub_type_2.e43, 2) + pow(sub_type_2.e423, 2) + pow(sub_type_2.e431, 2) + pow(sub_type_2.e412, 2))
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, NormPrefixOrPostfix rhs)  {
    return lhs.norm();
    }
// Fancy prefix trick
public AntiScalar operator /(NormPrefixOrPostfix lhs, Point rhs) {
    return rhs.norm();
}
public extension Point {
    public AntiScalar norm() {
        return AntiScalar(
            /* e1234 */
            this.e4
        );
    }
}
