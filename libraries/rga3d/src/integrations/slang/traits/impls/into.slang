implementing rga3d;
public extension DualNum {
    public static DualNum from(AntiScalar from_anti_scalar) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar.e1234)
        );
    }
}
public extension AntiScalar {
    public DualNum into_DualNum() {
        return DualNum.from(this);
    }
}
public extension Motor {
    public static Motor from(AntiScalar from_anti_scalar) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_anti_scalar.e1234), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar {
    public Motor into_Motor() {
        return Motor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiScalar from_anti_scalar) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar.e1234), 
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
public extension AntiScalar {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Motor {
    public static Motor from(DualNum from_dual_num) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_dual_num.e1234), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_dual_num.scalar)
        );
    }
}
public extension DualNum {
    public Motor into_Motor() {
        return Motor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(DualNum from_dual_num) {
        return MultiVector(
            /* scalar, e1234 */
            from_dual_num.group0.xy, 
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
public extension DualNum {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Flector from_flector) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_flector.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            from_flector.group1
        );
    }
}
public extension Flector {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Flector {
    public static Flector from(Horizon from_horizon) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon {
    public Flector into_Flector() {
        return Flector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Horizon from_horizon) {
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
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Plane {
    public static Plane from(Horizon from_horizon) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon {
    public Plane into_Plane() {
        return Plane.from(this);
    }
}
public extension Motor {
    public static Motor from(Line from_line) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(from_line.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public Motor into_Motor() {
        return Motor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Line from_line) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_line.group0.xyz, 
            /* e23, e31, e12 */
            from_line.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Motor from_motor) {
        return MultiVector(
            /* scalar, e1234 */
            float2(from_motor.scalar, from_motor.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_motor.group0.xyz, 
            /* e23, e31, e12 */
            from_motor.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Flector {
    public static Flector from(Origin from_origin) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Flector into_Flector() {
        return Flector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Origin from_origin) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
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
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Point {
    public static Point from(Origin from_origin) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4)
        );
    }
}
public extension Origin {
    public Point into_Point() {
        return Point.from(this);
    }
}
public extension Flector {
    public static Flector from(Plane from_plane) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0
        );
    }
}
public extension Plane {
    public Flector into_Flector() {
        return Flector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Plane from_plane) {
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
            from_plane.group0
        );
    }
}
public extension Plane {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Flector {
    public static Flector from(Point from_point) {
        return Flector(
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public Flector into_Flector() {
        return Flector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Point from_point) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension DualNum {
    public static DualNum from(Scalar from_scalar) {
        return DualNum(
            /* scalar, e1234 */
            float2(from_scalar.scalar, 0.0)
        );
    }
}
public extension Scalar {
    public DualNum into_DualNum() {
        return DualNum.from(this);
    }
}
public extension Motor {
    public static Motor from(Scalar from_scalar) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
public extension Scalar {
    public Motor into_Motor() {
        return Motor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Scalar from_scalar) {
        return MultiVector(
            /* scalar, e1234 */
            float2(from_scalar.scalar, 0.0), 
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
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
