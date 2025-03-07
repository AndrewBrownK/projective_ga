implementing cga3d;
public extension AntiCircleRotor {
    public AntiCircleRotor one() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum one() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(0.0, 1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor one() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector one() {
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, 0.0), 
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
public extension Scalar {
    public Scalar one() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}
public extension VersorOdd {
    public VersorOdd one() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), 1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
