implementing cga3d;
public extension MultiVector {
    public static MultiVector from(AntiCircleRotor from_anti_circle_rotor) {
        return MultiVector(
            /* scalar, e12345 */
            float2(from_anti_circle_rotor.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_anti_circle_rotor.group2.xyz, from_anti_circle_rotor.e45), 
            /* e41, e42, e43 */
            from_anti_circle_rotor.group0.xyz, 
            /* e23, e31, e12 */
            from_anti_circle_rotor.group1.xyz, 
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
public extension AntiCircleRotor {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(AntiCircleRotor from_anti_circle_rotor) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(from_anti_circle_rotor.group0.xyz, from_anti_circle_rotor.scalar), 
            /* e23, e31, e12, e45 */
            from_anti_circle_rotor.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_anti_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiCircleRotor {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiDipoleInversion from_anti_dipole_inversion) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.group3.xyz, from_anti_dipole_inversion.e4), 
            /* e5 */
            from_anti_dipole_inversion.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e423, e431, e412 */
            from_anti_dipole_inversion.group0.xyz, 
            /* e235, e315, e125 */
            from_anti_dipole_inversion.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(AntiDipoleInversion from_anti_dipole_inversion) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(from_anti_dipole_inversion.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_anti_dipole_inversion.group1, 
            /* e235, e315, e125, e5 */
            float4(from_anti_dipole_inversion.group2.xyz, from_anti_dipole_inversion.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_dipole_inversion.group3.xyz, from_anti_dipole_inversion.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiMotor {
    public static AntiMotor from(AntiDualNum from_anti_dual_num) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_anti_dual_num.scalar), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), from_anti_dual_num.e3215)
        );
    }
}
public extension AntiDualNum {
    public AntiMotor into_AntiMotor() {
        return AntiMotor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiDualNum from_anti_dual_num) {
        return MultiVector(
            /* scalar, e12345 */
            float2(from_anti_dual_num.scalar, 0.0), 
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
            float4(float3(0.0), from_anti_dual_num.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDualNum {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(AntiDualNum from_anti_dual_num) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_anti_dual_num.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_dual_num.e3215)
        );
    }
}
public extension AntiDualNum {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(AntiFlatPoint from_anti_flat_point) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e4 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension AntiFlector {
    public static AntiFlector from(AntiFlatPoint from_anti_flat_point) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            from_anti_flat_point.group0, 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension AntiFlatPoint {
    public AntiFlector into_AntiFlector() {
        return AntiFlector.from(this);
    }
}
public extension Circle {
    public static Circle from(AntiFlatPoint from_anti_flat_point) {
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125 */
            from_anti_flat_point.group0.xyz
        );
    }
}
public extension AntiFlatPoint {
    public Circle into_Circle() {
        return Circle.from(this);
    }
}
public extension CircleRotor {
    public static CircleRotor from(AntiFlatPoint from_anti_flat_point) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e12345 */
            float4(from_anti_flat_point.group0.xyz, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public CircleRotor into_CircleRotor() {
        return CircleRotor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiFlatPoint from_anti_flat_point) {
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
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            from_anti_flat_point.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlatPoint {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(AntiFlatPoint from_anti_flat_point) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flat_point.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flat_point.group0.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension AntiFlatPoint {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(AntiFlector from_anti_flector) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e235, e315, e125, e4 */
            float4(from_anti_flector.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            from_anti_flector.group1
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiFlector from_anti_flector) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0), 
            /* e5 */
            from_anti_flector.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            from_anti_flector.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(AntiFlector from_anti_flector) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), from_anti_flector.e321), 
            /* e235, e315, e125, e5 */
            float4(from_anti_flector.group0.xyz, from_anti_flector.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_flector.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiCircleRotor {
    public static AntiCircleRotor from(AntiLine from_anti_line) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, scalar */
            float4(from_anti_line.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiCircleRotor into_AntiCircleRotor() {
        return AntiCircleRotor.from(this);
    }
}
public extension AntiMotor {
    public static AntiMotor from(AntiLine from_anti_line) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(from_anti_line.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public AntiMotor into_AntiMotor() {
        return AntiMotor.from(this);
    }
}
public extension Dipole {
    public static Dipole from(AntiLine from_anti_line) {
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35 */
            from_anti_line.group1.xyz
        );
    }
}
public extension AntiLine {
    public Dipole into_Dipole() {
        return Dipole.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(AntiLine from_anti_line) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiLine {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiLine from_anti_line) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            from_anti_line.group0.xyz, 
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
public extension AntiLine {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(AntiLine from_anti_line) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(from_anti_line.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiLine {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiMotor from_anti_motor) {
        return MultiVector(
            /* scalar, e12345 */
            float2(from_anti_motor.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_anti_motor.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            from_anti_motor.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_motor.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(AntiMotor from_anti_motor) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_anti_motor.scalar), 
            /* e23, e31, e12, e45 */
            float4(from_anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(from_anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), from_anti_motor.e3215)
        );
    }
}
public extension AntiMotor {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(AntiPlane from_anti_plane) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            from_anti_plane.group0
        );
    }
}
public extension AntiPlane {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension AntiFlector {
    public static AntiFlector from(AntiPlane from_anti_plane) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            from_anti_plane.group0
        );
    }
}
public extension AntiPlane {
    public AntiFlector into_AntiFlector() {
        return AntiFlector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(AntiPlane from_anti_plane) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0), 
            /* e5 */
            from_anti_plane.e5, 
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
public extension AntiPlane {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension RoundPoint {
    public static RoundPoint from(AntiPlane from_anti_plane) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0), 
            /* e5 */
            from_anti_plane.e5
        );
    }
}
public extension AntiPlane {
    public RoundPoint into_RoundPoint() {
        return RoundPoint.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(AntiPlane from_anti_plane) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_anti_plane.e5), 
            /* e1, e2, e3, e4 */
            float4(from_anti_plane.group0.xyz, 0.0)
        );
    }
}
public extension AntiPlane {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension CircleRotor {
    public static CircleRotor from(AntiScalar from_anti_scalar) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345)
        );
    }
}
public extension AntiScalar {
    public CircleRotor into_CircleRotor() {
        return CircleRotor.from(this);
    }
}
public extension DualNum {
    public static DualNum from(AntiScalar from_anti_scalar) {
        return DualNum(
            /* e5, e12345 */
            float2(0.0, from_anti_scalar.e12345)
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
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345), 
            /* e235, e315, e125, e5 */
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
            /* scalar, e12345 */
            float2(0.0, from_anti_scalar.e12345), 
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
public extension AntiScalar {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(AntiScalar from_anti_scalar) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_anti_scalar.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension AntiScalar {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(Circle from_circle) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            from_circle.group0.xyz, 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e4 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension Circle {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension CircleRotor {
    public static CircleRotor from(Circle from_circle) {
        return CircleRotor(
            /* e423, e431, e412 */
            from_circle.group0.xyz, 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e12345 */
            float4(from_circle.group2.xyz, 0.0)
        );
    }
}
public extension Circle {
    public CircleRotor into_CircleRotor() {
        return CircleRotor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Circle from_circle) {
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
            from_circle.group1, 
            /* e423, e431, e412 */
            from_circle.group0.xyz, 
            /* e235, e315, e125 */
            from_circle.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Circle {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(Circle from_circle) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(from_circle.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            from_circle.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Circle {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(CircleRotor from_circle_rotor) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, from_circle_rotor.e12345), 
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
            from_circle_rotor.group1, 
            /* e423, e431, e412 */
            from_circle_rotor.group0.xyz, 
            /* e235, e315, e125 */
            from_circle_rotor.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(CircleRotor from_circle_rotor) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(from_circle_rotor.group0.xyz, from_circle_rotor.e12345), 
            /* e415, e425, e435, e321 */
            from_circle_rotor.group1, 
            /* e235, e315, e125, e5 */
            float4(from_circle_rotor.group2.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiCircleRotor {
    public static AntiCircleRotor from(Dipole from_dipole) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            from_dipole.group0.xyz, 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, scalar */
            float4(from_dipole.group2.xyz, 0.0)
        );
    }
}
public extension Dipole {
    public AntiCircleRotor into_AntiCircleRotor() {
        return AntiCircleRotor.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(Dipole from_dipole) {
        return DipoleInversion(
            /* e41, e42, e43 */
            from_dipole.group0.xyz, 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_dipole.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension Dipole {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Dipole from_dipole) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_dipole.group2.xyz, from_dipole.e45), 
            /* e41, e42, e43 */
            from_dipole.group0.xyz, 
            /* e23, e31, e12 */
            from_dipole.group1.xyz, 
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
public extension Dipole {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(Dipole from_dipole) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(from_dipole.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole.group1, 
            /* e15, e25, e35, e1234 */
            float4(from_dipole.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension Dipole {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(DipoleInversion from_dipole_inversion) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_dipole_inversion.group2.xyz, from_dipole_inversion.e45), 
            /* e41, e42, e43 */
            from_dipole_inversion.group0.xyz, 
            /* e23, e31, e12 */
            from_dipole_inversion.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_dipole_inversion.group3, 
            /* e1234 */
            from_dipole_inversion.e1234
        );
    }
}
public extension DipoleInversion {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(DipoleInversion from_dipole_inversion) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(from_dipole_inversion.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            from_dipole_inversion.group1, 
            /* e15, e25, e35, e1234 */
            from_dipole_inversion.group2, 
            /* e4235, e4315, e4125, e3215 */
            from_dipole_inversion.group3
        );
    }
}
public extension DipoleInversion {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension Motor {
    public static Motor from(DualNum from_dual_num) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), from_dual_num.e12345), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_dual_num.e5)
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
            /* scalar, e12345 */
            float2(0.0, from_dual_num.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            from_dual_num.e5, 
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
public extension DualNum {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(DualNum from_dual_num) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_dual_num.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_dual_num.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiCircleRotor {
    public static AntiCircleRotor from(FlatPoint from_flat_point) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, scalar */
            float4(from_flat_point.group0.xyz, 0.0)
        );
    }
}
public extension FlatPoint {
    public AntiCircleRotor into_AntiCircleRotor() {
        return AntiCircleRotor.from(this);
    }
}
public extension Dipole {
    public static Dipole from(FlatPoint from_flat_point) {
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35 */
            from_flat_point.group0.xyz
        );
    }
}
public extension FlatPoint {
    public Dipole into_Dipole() {
        return Dipole.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(FlatPoint from_flat_point) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension FlatPoint {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension Flector {
    public static Flector from(FlatPoint from_flat_point) {
        return Flector(
            /* e15, e25, e35, e45 */
            from_flat_point.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension FlatPoint {
    public Flector into_Flector() {
        return Flector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(FlatPoint from_flat_point) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            from_flat_point.group0, 
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
public extension FlatPoint {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(FlatPoint from_flat_point) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flat_point.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flat_point.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension FlatPoint {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(Flector from_flector) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flector.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1
        );
    }
}
public extension Flector {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Flector from_flector) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            from_flector.group0, 
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
            from_flector.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(Flector from_flector) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), from_flector.e45), 
            /* e15, e25, e35, e1234 */
            float4(from_flector.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_flector.group1
        );
    }
}
public extension Flector {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(Line from_line) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(from_line.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension Line {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension Circle {
    public static Circle from(Line from_line) {
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125 */
            from_line.group1.xyz
        );
    }
}
public extension Line {
    public Circle into_Circle() {
        return Circle.from(this);
    }
}
public extension CircleRotor {
    public static CircleRotor from(Line from_line) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public CircleRotor into_CircleRotor() {
        return CircleRotor.from(this);
    }
}
public extension Motor {
    public static Motor from(Line from_line) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
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
            float4(from_line.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            from_line.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Line {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(Line from_line) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_line.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(from_line.group1.xyz, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Line {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Motor from_motor) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, from_motor.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            from_motor.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            from_motor.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Motor {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(Motor from_motor) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), from_motor.e12345), 
            /* e415, e425, e435, e321 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            from_motor.group1, 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(Plane from_plane) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}
public extension Plane {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension Flector {
    public static Flector from(Plane from_plane) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
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
            from_plane.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension Sphere {
    public static Sphere from(Plane from_plane) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension Plane {
    public Sphere into_Sphere() {
        return Sphere.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(Plane from_plane) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_plane.group0
        );
    }
}
public extension Plane {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension AntiDipoleInversion {
    public static AntiDipoleInversion from(RoundPoint from_round_point) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), from_round_point.e4), 
            /* e1, e2, e3, e5 */
            float4(from_round_point.group0.xyz, from_round_point.e5)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion into_AntiDipoleInversion() {
        return AntiDipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(RoundPoint from_round_point) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_round_point.group0, 
            /* e5 */
            from_round_point.e5, 
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
public extension RoundPoint {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorEven {
    public static VersorEven from(RoundPoint from_round_point) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), from_round_point.e5), 
            /* e1, e2, e3, e4 */
            from_round_point.group0
        );
    }
}
public extension RoundPoint {
    public VersorEven into_VersorEven() {
        return VersorEven.from(this);
    }
}
public extension AntiCircleRotor {
    public static AntiCircleRotor from(Scalar from_scalar) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
public extension Scalar {
    public AntiCircleRotor into_AntiCircleRotor() {
        return AntiCircleRotor.from(this);
    }
}
public extension AntiDualNum {
    public static AntiDualNum from(Scalar from_scalar) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(0.0, from_scalar.scalar)
        );
    }
}
public extension Scalar {
    public AntiDualNum into_AntiDualNum() {
        return AntiDualNum.from(this);
    }
}
public extension AntiMotor {
    public static AntiMotor from(Scalar from_scalar) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public AntiMotor into_AntiMotor() {
        return AntiMotor.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Scalar from_scalar) {
        return MultiVector(
            /* scalar, e12345 */
            float2(from_scalar.scalar, 0.0), 
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
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(Scalar from_scalar) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), from_scalar.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension DipoleInversion {
    public static DipoleInversion from(Sphere from_sphere) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), from_sphere.e1234), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion into_DipoleInversion() {
        return DipoleInversion.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(Sphere from_sphere) {
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
            from_sphere.group0, 
            /* e1234 */
            from_sphere.e1234
        );
    }
}
public extension Sphere {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension VersorOdd {
    public static VersorOdd from(Sphere from_sphere) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), from_sphere.e1234), 
            /* e4235, e4315, e4125, e3215 */
            from_sphere.group0
        );
    }
}
public extension Sphere {
    public VersorOdd into_VersorOdd() {
        return VersorOdd.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(VersorEven from_versor_even) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, from_versor_even.e12345), 
            /* e1, e2, e3, e4 */
            from_versor_even.group3, 
            /* e5 */
            from_versor_even.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            from_versor_even.group1, 
            /* e423, e431, e412 */
            from_versor_even.group0.xyz, 
            /* e235, e315, e125 */
            from_versor_even.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension VersorEven {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
public extension MultiVector {
    public static MultiVector from(VersorOdd from_versor_odd) {
        return MultiVector(
            /* scalar, e12345 */
            float2(from_versor_odd.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(from_versor_odd.group2.xyz, from_versor_odd.e45), 
            /* e41, e42, e43 */
            from_versor_odd.group0.xyz, 
            /* e23, e31, e12 */
            from_versor_odd.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            from_versor_odd.group3, 
            /* e1234 */
            from_versor_odd.e1234
        );
    }
}
public extension VersorOdd {
    public MultiVector into_MultiVector() {
        return MultiVector.from(this);
    }
}
