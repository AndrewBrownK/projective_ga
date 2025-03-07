implementing rga3d;
public extension AntiScalar {
    public AntiScalar zero() {
        return AntiScalar(
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public DualNum zero() {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0)
        );
    }
}
public extension Flector {
    public Flector zero() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Horizon {
    public Horizon zero() {
        return Horizon(
            /* e321 */
            0.0
        );
    }
}
public extension Line {
    public Line zero() {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    public Motor zero() {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector zero() {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
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
public extension Origin {
    public Origin zero() {
        return Origin(
            /* e4 */
            0.0
        );
    }
}
public extension Plane {
    public Plane zero() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public Point zero() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public Scalar zero() {
        return Scalar(
            /* scalar */
            0.0
        );
    }
}
