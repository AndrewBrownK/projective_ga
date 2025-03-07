implementing cga3d;
public extension AntiCircleRotor {
    public AntiCircleRotor zero() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion zero() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension AntiDualNum {
    public AntiDualNum zero() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlatPoint zero() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(0.0)
        );
    }
}
public extension AntiFlector {
    public AntiFlector zero() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension AntiLine {
    public AntiLine zero() {
        return AntiLine(
            /* e23, e31, e12 */
            float3(0.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor zero() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiPlane {
    public AntiPlane zero() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar zero() {
        return AntiScalar(
            /* e12345 */
            0.0
        );
    }
}
public extension Circle {
    public Circle zero() {
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor zero() {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(0.0)
        );
    }
}
public extension Dipole {
    public Dipole zero() {
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
public extension DipoleInversion {
    public DipoleInversion zero() {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public DualNum zero() {
        return DualNum(
            /* e5, e12345 */
            float2(0.0)
        );
    }
}
public extension FlatPoint {
    public FlatPoint zero() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(0.0)
        );
    }
}
public extension Flector {
    public Flector zero() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension Line {
    public Line zero() {
        return Line(
            /* e415, e425, e435 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
public extension Motor {
    public Motor zero() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector zero() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
public extension Plane {
    public Plane zero() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension RoundPoint {
    public RoundPoint zero() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0
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
public extension Sphere {
    public Sphere zero() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension VersorEven {
    public VersorEven zero() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension VersorOdd {
    public VersorOdd zero() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
