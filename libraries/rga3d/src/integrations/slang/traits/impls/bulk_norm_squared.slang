implementing rga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(Motor lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.bulk_norm_squared();
}
public extension Motor {
    public Scalar bulk_norm_squared() {
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
        return Scalar(
            /* scalar */
            pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e321, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, BulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(BulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.bulk_norm_squared();
}
public extension MultiVector {
    public Scalar bulk_norm_squared() {
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
        return Scalar(
            /* scalar */
            pow(wedge.scalar, 2) + pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) + pow(wedge.e321, 2)
        );
    }
}
