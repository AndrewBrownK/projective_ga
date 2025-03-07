implementing cga3d;
public extension AntiScalar {
    public AntiScalar anti_one() {
        return AntiScalar(
            /* e12345 */
            1.0
        );
    }
}
public extension CircleRotor {
    public CircleRotor anti_one() {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), 1.0)
        );
    }
}
public extension DualNum {
    public DualNum anti_one() {
        return DualNum(
            /* e5, e12345 */
            float2(0.0, 1.0)
        );
    }
}
public extension Motor {
    public Motor anti_one() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), 1.0), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector anti_one() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension VersorEven {
    public VersorEven anti_one() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), 1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
