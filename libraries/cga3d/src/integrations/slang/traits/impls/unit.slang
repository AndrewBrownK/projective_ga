implementing cga3d;
public extension AntiCircleRotor {
    public AntiCircleRotor unit() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, scalar */
            float4(1.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion unit() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e4 */
            float4(1.0), 
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum unit() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(1.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint unit() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(1.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector unit() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(1.0), 
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
public extension AntiLine {
    public AntiLine unit() {
        return AntiLine(
            /* e23, e31, e12 */
            float3(1.0), 
            /* e15, e25, e35 */
            float3(1.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor unit() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(1.0), 
            /* e15, e25, e35, e3215 */
            float4(1.0)
        );
    }
}
public extension AntiPlane {
    public AntiPlane unit() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar unit() {
        return AntiScalar(
            /* e12345 */
            1.0
        );
    }
}
public extension Circle {
    public Circle unit() {
        return Circle(
            /* e423, e431, e412 */
            float3(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125 */
            float3(1.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor unit() {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e12345 */
            float4(1.0)
        );
    }
}
public extension Dipole {
    public Dipole unit() {
        return Dipole(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35 */
            float3(1.0)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion unit() {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, e1234 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
public extension DualNum {
    public DualNum unit() {
        return DualNum(
            /* e5, e12345 */
            float2(1.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint unit() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(1.0)
        );
    }
}
public extension Flector {
    public Flector unit() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
public extension Line {
    public Line unit() {
        return Line(
            /* e415, e425, e435 */
            float3(1.0), 
            /* e235, e315, e125 */
            float3(1.0)
        );
    }
}
public extension Motor {
    public Motor unit() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(1.0), 
            /* e235, e315, e125, e5 */
            float4(1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector unit() {
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0), 
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e5 */
            1.0, 
            /* e15, e25, e35, e45 */
            float4(1.0), 
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e423, e431, e412 */
            float3(1.0), 
            /* e235, e315, e125 */
            float3(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0), 
            /* e1234 */
            1.0
        );
    }
}
public extension Plane {
    public Plane unit() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
public extension RoundPoint {
    public RoundPoint unit() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e5 */
            1.0
        );
    }
}
public extension Scalar {
    public Scalar unit() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}
public extension Sphere {
    public Sphere unit() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(1.0), 
            /* e1234 */
            1.0
        );
    }
}
public extension VersorEven {
    public VersorEven unit() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e5 */
            float4(1.0), 
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
public extension VersorOdd {
    public VersorOdd unit() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(1.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, e1234 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
