implementing rga3d;
public extension DualNum {
    public DualNum one() {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0, 0.0)
        );
    }
}
public extension Motor {
    public Motor one() {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector one() {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public Scalar one() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}
