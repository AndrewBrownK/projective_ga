implementing cga3d;
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_circle_rotor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_circle_rotor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_circle_rotor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(0.0, anti_circle_rotor.scalar)
        );
    }
}
public extension AntiCircleRotor {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            anti_circle_rotor.group1.xyz, 
            /* e15, e25, e35 */
            anti_circle_rotor.group2.xyz
        );
    }
}
public extension AntiCircleRotor {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(anti_circle_rotor.e23, anti_circle_rotor.e31, anti_circle_rotor.e12, anti_circle_rotor.scalar), 
            /* e15, e25, e35, e3215 */
            float4(anti_circle_rotor.group2.xyz, 0.0)
        );
    }
}
public extension AntiCircleRotor {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_scalar = anti_circle_rotor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            anti_circle_rotor.group0.xyz, 
            /* e23, e31, e12, e45 */
            anti_circle_rotor.group1, 
            /* e15, e25, e35 */
            anti_circle_rotor.group2.xyz
        );
    }
}
public extension AntiCircleRotor {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension DipoleInversion {
    public static Optional<DipoleInversion> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_scalar = anti_circle_rotor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_circle_rotor.group0.xyz, 
            /* e23, e31, e12, e45 */
            anti_circle_rotor.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_circle_rotor.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiCircleRotor {
    public Optional<DipoleInversion> try_into_DipoleInversion() {
        return DipoleInversion.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(anti_circle_rotor.e15, anti_circle_rotor.e25, anti_circle_rotor.e35, anti_circle_rotor.e45)
        );
    }
}
public extension AntiCircleRotor {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_circle_rotor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_circle_rotor.e15, anti_circle_rotor.e25, anti_circle_rotor.e35, anti_circle_rotor.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiCircleRotor {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(AntiCircleRotor anti_circle_rotor) {

        float disallowed_e41 = anti_circle_rotor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = anti_circle_rotor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = anti_circle_rotor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = anti_circle_rotor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_circle_rotor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_circle_rotor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = anti_circle_rotor.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_circle_rotor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_circle_rotor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_circle_rotor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            anti_circle_rotor.scalar
        );
    }
}
public extension AntiCircleRotor {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e321)
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e321), 
            /* e1, e2, e3, e5 */
            anti_dipole_inversion.group3
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            anti_dipole_inversion.group3
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            anti_dipole_inversion.group0.xyz, 
            /* e415, e425, e435, e321 */
            anti_dipole_inversion.group1, 
            /* e235, e315, e125 */
            anti_dipole_inversion.group2.xyz
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            anti_dipole_inversion.group0.xyz, 
            /* e415, e425, e435, e321 */
            anti_dipole_inversion.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_dipole_inversion.group2.xyz, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(anti_dipole_inversion.e5, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_dipole_inversion.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            anti_dipole_inversion.group1.xyz, 
            /* e235, e315, e125 */
            anti_dipole_inversion.group2.xyz
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e4 = anti_dipole_inversion.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_dipole_inversion.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_dipole_inversion.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_dipole_inversion.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_dipole_inversion.group1.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_dipole_inversion.e235, anti_dipole_inversion.e315, anti_dipole_inversion.e125, anti_dipole_inversion.e5)
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(AntiDipoleInversion anti_dipole_inversion) {

        float disallowed_e423 = anti_dipole_inversion.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = anti_dipole_inversion.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = anti_dipole_inversion.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = anti_dipole_inversion.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = anti_dipole_inversion.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = anti_dipole_inversion.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_dipole_inversion.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = anti_dipole_inversion.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_dipole_inversion.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_dipole_inversion.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_dipole_inversion.e1, anti_dipole_inversion.e2, anti_dipole_inversion.e3, anti_dipole_inversion.e4), 
            /* e5 */
            anti_dipole_inversion.e5
        );
    }
}
public extension AntiDipoleInversion {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(AntiDualNum anti_dual_num) {

        float disallowed_e3215 = anti_dual_num.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), anti_dual_num.scalar)
        );
    }
}
public extension AntiDualNum {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension DipoleInversion {
    public static Optional<DipoleInversion> try_from(AntiDualNum anti_dual_num) {

        float disallowed_scalar = anti_dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
public extension AntiDualNum {
    public Optional<DipoleInversion> try_into_DipoleInversion() {
        return DipoleInversion.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(AntiDualNum anti_dual_num) {

        float disallowed_scalar = anti_dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
public extension AntiDualNum {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(AntiDualNum anti_dual_num) {

        float disallowed_scalar = anti_dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215)
        );
    }
}
public extension AntiDualNum {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(AntiDualNum anti_dual_num) {

        float disallowed_e3215 = anti_dual_num.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            anti_dual_num.scalar
        );
    }
}
public extension AntiDualNum {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(AntiDualNum anti_dual_num) {

        float disallowed_scalar = anti_dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_dual_num.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiDualNum {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(AntiFlatPoint anti_flat_point) {

        float disallowed_e321 = anti_flat_point.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            float3(0.0), 
            /* e235, e315, e125 */
            anti_flat_point.group0.xyz
        );
    }
}
public extension AntiFlatPoint {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(AntiFlatPoint anti_flat_point) {

        float disallowed_e321 = anti_flat_point.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_flat_point.group0.xyz, 0.0)
        );
    }
}
public extension AntiFlatPoint {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(AntiFlector anti_flector) {

        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            anti_flector.group0
        );
    }
}
public extension AntiFlector {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(AntiFlector anti_flector) {

        float disallowed_e235 = anti_flector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            anti_flector.group1
        );
    }
}
public extension AntiFlector {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(AntiFlector anti_flector) {

        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_flector.e321), 
            /* e235, e315, e125 */
            anti_flector.group0.xyz
        );
    }
}
public extension AntiFlector {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(AntiFlector anti_flector) {

        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_flector.e321), 
            /* e235, e315, e125, e12345 */
            float4(anti_flector.group0.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(AntiFlector anti_flector) {

        float disallowed_e235 = anti_flector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(anti_flector.e5, 0.0)
        );
    }
}
public extension AntiFlector {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(AntiFlector anti_flector) {

        float disallowed_e321 = anti_flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e5 = anti_flector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            float3(0.0), 
            /* e235, e315, e125 */
            anti_flector.group0.xyz
        );
    }
}
public extension AntiFlector {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(AntiFlector anti_flector) {

        float disallowed_e321 = anti_flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e1 = anti_flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(anti_flector.e235, anti_flector.e315, anti_flector.e125, anti_flector.e5)
        );
    }
}
public extension AntiFlector {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(AntiFlector anti_flector) {

        float disallowed_e235 = anti_flector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = anti_flector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = anti_flector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e321 = anti_flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_flector.group1.xyz, 0.0), 
            /* e5 */
            anti_flector.e5
        );
    }
}
public extension AntiFlector {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(AntiLine anti_line) {

        float disallowed_e23 = anti_line.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_line.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_line.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(anti_line.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(AntiLine anti_line) {

        float disallowed_e23 = anti_line.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_line.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_line.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_line.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension AntiLine {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(AntiMotor anti_motor) {

        float disallowed_e3215 = anti_motor.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, scalar */
            float4(anti_motor.e15, anti_motor.e25, anti_motor.e35, anti_motor.scalar)
        );
    }
}
public extension AntiMotor {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(anti_motor.e3215, anti_motor.scalar)
        );
    }
}
public extension AntiMotor {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(AntiMotor anti_motor) {

        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            anti_motor.group0.xyz, 
            /* e15, e25, e35 */
            anti_motor.group1.xyz
        );
    }
}
public extension AntiMotor {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(AntiMotor anti_motor) {

        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35 */
            anti_motor.group1.xyz
        );
    }
}
public extension AntiMotor {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension DipoleInversion {
    public static Optional<DipoleInversion> try_from(AntiMotor anti_motor) {

        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(anti_motor.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
public extension AntiMotor {
    public Optional<DipoleInversion> try_into_DipoleInversion() {
        return DipoleInversion.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(anti_motor.group1.xyz, 0.0)
        );
    }
}
public extension AntiMotor {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_motor.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
public extension AntiMotor {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215)
        );
    }
}
public extension AntiMotor {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e3215 = anti_motor.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            anti_motor.scalar
        );
    }
}
public extension AntiMotor {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(AntiMotor anti_motor) {

        float disallowed_e23 = anti_motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = anti_motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = anti_motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = anti_motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e15 = anti_motor.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = anti_motor.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = anti_motor.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_motor.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(AntiPlane anti_plane) {

        float disallowed_e1 = anti_plane.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_plane.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_plane.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(anti_plane.e5, 0.0)
        );
    }
}
public extension AntiPlane {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(AntiPlane anti_plane) {

        float disallowed_e1 = anti_plane.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = anti_plane.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = anti_plane.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), anti_plane.e5)
        );
    }
}
public extension AntiPlane {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(Circle circle) {

        float disallowed_e423 = circle.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(circle.e235, circle.e315, circle.e125, circle.e321)
        );
    }
}
public extension Circle {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(Circle circle) {

        float disallowed_e423 = circle.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(circle.e235, circle.e315, circle.e125, circle.e321), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension Circle {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(Circle circle) {

        float disallowed_e423 = circle.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            circle.group1.xyz, 
            /* e235, e315, e125 */
            circle.group2.xyz
        );
    }
}
public extension Circle {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(Circle circle) {

        float disallowed_e423 = circle.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(circle.group1.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(circle.group2.xyz, 0.0)
        );
    }
}
public extension Circle {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension AntiDipoleInversion {
    public static Optional<AntiDipoleInversion> try_from(CircleRotor circle_rotor) {

        float disallowed_e12345 = circle_rotor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            circle_rotor.group0.xyz, 
            /* e415, e425, e435, e321 */
            circle_rotor.group1, 
            /* e235, e315, e125, e4 */
            float4(circle_rotor.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public Optional<AntiDipoleInversion> try_into_AntiDipoleInversion() {
        return AntiDipoleInversion.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(circle_rotor.e235, circle_rotor.e315, circle_rotor.e125, circle_rotor.e321)
        );
    }
}
public extension CircleRotor {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(circle_rotor.e235, circle_rotor.e315, circle_rotor.e125, circle_rotor.e321), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = circle_rotor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = circle_rotor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = circle_rotor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e12345 */
            circle_rotor.e12345
        );
    }
}
public extension CircleRotor {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(CircleRotor circle_rotor) {

        float disallowed_e12345 = circle_rotor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            circle_rotor.group0.xyz, 
            /* e415, e425, e435, e321 */
            circle_rotor.group1, 
            /* e235, e315, e125 */
            circle_rotor.group2.xyz
        );
    }
}
public extension CircleRotor {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = circle_rotor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = circle_rotor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = circle_rotor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = circle_rotor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = circle_rotor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = circle_rotor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(0.0, circle_rotor.e12345)
        );
    }
}
public extension CircleRotor {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e12345 = circle_rotor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            circle_rotor.group1.xyz, 
            /* e235, e315, e125 */
            circle_rotor.group2.xyz
        );
    }
}
public extension CircleRotor {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(CircleRotor circle_rotor) {

        float disallowed_e423 = circle_rotor.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = circle_rotor.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = circle_rotor.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = circle_rotor.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(circle_rotor.e415, circle_rotor.e425, circle_rotor.e435, circle_rotor.e12345), 
            /* e235, e315, e125, e5 */
            float4(circle_rotor.group2.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(Dipole dipole) {

        float disallowed_e41 = dipole.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            dipole.group1.xyz, 
            /* e15, e25, e35 */
            dipole.group2.xyz
        );
    }
}
public extension Dipole {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(Dipole dipole) {

        float disallowed_e41 = dipole.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(dipole.group1.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(dipole.group2.xyz, 0.0)
        );
    }
}
public extension Dipole {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(Dipole dipole) {

        float disallowed_e41 = dipole.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(dipole.e15, dipole.e25, dipole.e35, dipole.e45)
        );
    }
}
public extension Dipole {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(Dipole dipole) {

        float disallowed_e41 = dipole.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(dipole.e15, dipole.e25, dipole.e35, dipole.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
public extension Dipole {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            dipole_inversion.group0.xyz, 
            /* e23, e31, e12, e45 */
            dipole_inversion.group1, 
            /* e15, e25, e35, scalar */
            float4(dipole_inversion.group2.xyz, 0.0)
        );
    }
}
public extension DipoleInversion {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(dipole_inversion.e3215, 0.0)
        );
    }
}
public extension DipoleInversion {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            dipole_inversion.group1.xyz, 
            /* e15, e25, e35 */
            dipole_inversion.group2.xyz
        );
    }
}
public extension DipoleInversion {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(dipole_inversion.group1.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e3215)
        );
    }
}
public extension DipoleInversion {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            dipole_inversion.group0.xyz, 
            /* e23, e31, e12, e45 */
            dipole_inversion.group1, 
            /* e15, e25, e35 */
            dipole_inversion.group2.xyz
        );
    }
}
public extension DipoleInversion {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = dipole_inversion.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = dipole_inversion.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = dipole_inversion.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = dipole_inversion.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e45)
        );
    }
}
public extension DipoleInversion {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(dipole_inversion.e15, dipole_inversion.e25, dipole_inversion.e35, dipole_inversion.e45), 
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3
        );
    }
}
public extension DipoleInversion {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e1234 = dipole_inversion.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3
        );
    }
}
public extension DipoleInversion {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(DipoleInversion dipole_inversion) {

        float disallowed_e41 = dipole_inversion.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = dipole_inversion.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = dipole_inversion.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = dipole_inversion.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = dipole_inversion.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = dipole_inversion.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = dipole_inversion.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = dipole_inversion.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = dipole_inversion.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = dipole_inversion.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            dipole_inversion.group3, 
            /* e1234 */
            dipole_inversion.e1234
        );
    }
}
public extension DipoleInversion {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
public extension AntiDipoleInversion {
    public static Optional<AntiDipoleInversion> try_from(DualNum dual_num) {

        float disallowed_e12345 = dual_num.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
public extension DualNum {
    public Optional<AntiDipoleInversion> try_into_AntiDipoleInversion() {
        return AntiDipoleInversion.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(DualNum dual_num) {

        float disallowed_e12345 = dual_num.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
public extension DualNum {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(DualNum dual_num) {

        float disallowed_e12345 = dual_num.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), dual_num.e5)
        );
    }
}
public extension DualNum {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(DualNum dual_num) {

        float disallowed_e5 = dual_num.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e12345 */
            dual_num.e12345
        );
    }
}
public extension DualNum {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(DualNum dual_num) {

        float disallowed_e5 = dual_num.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), dual_num.e12345)
        );
    }
}
public extension DualNum {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(DualNum dual_num) {

        float disallowed_e12345 = dual_num.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            dual_num.e5
        );
    }
}
public extension DualNum {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(FlatPoint flat_point) {

        float disallowed_e45 = flat_point.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            float3(0.0), 
            /* e15, e25, e35 */
            flat_point.group0.xyz
        );
    }
}
public extension FlatPoint {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(FlatPoint flat_point) {

        float disallowed_e45 = flat_point.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(flat_point.group0.xyz, 0.0)
        );
    }
}
public extension FlatPoint {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(Flector flector) {

        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), flector.e45), 
            /* e15, e25, e35, scalar */
            float4(flector.group0.xyz, 0.0)
        );
    }
}
public extension Flector {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(Flector flector) {

        float disallowed_e15 = flector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(flector.e3215, 0.0)
        );
    }
}
public extension Flector {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(Flector flector) {

        float disallowed_e45 = flector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            float3(0.0), 
            /* e15, e25, e35 */
            flector.group0.xyz
        );
    }
}
public extension Flector {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(Flector flector) {

        float disallowed_e45 = flector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(flector.e15, flector.e25, flector.e35, flector.e3215)
        );
    }
}
public extension Flector {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(Flector flector) {

        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), flector.e45), 
            /* e15, e25, e35 */
            flector.group0.xyz
        );
    }
}
public extension Flector {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(Flector flector) {

        float disallowed_e4235 = flector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = flector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = flector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = flector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            flector.group0
        );
    }
}
public extension Flector {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(Flector flector) {

        float disallowed_e15 = flector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            flector.group1
        );
    }
}
public extension Flector {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(Flector flector) {

        float disallowed_e15 = flector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = flector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = flector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = flector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            flector.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension Flector {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(Line line) {

        float disallowed_e415 = line.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = line.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = line.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(line.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(Line line) {

        float disallowed_e415 = line.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = line.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = line.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(line.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
public extension Line {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiDipoleInversion {
    public static Optional<AntiDipoleInversion> try_from(Motor motor) {

        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(motor.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
public extension Motor {
    public Optional<AntiDipoleInversion> try_into_AntiDipoleInversion() {
        return AntiDipoleInversion.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(motor.group1.xyz, 0.0)
        );
    }
}
public extension Motor {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(motor.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
public extension Motor {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), motor.e5)
        );
    }
}
public extension Motor {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e12345 */
            motor.e12345
        );
    }
}
public extension Motor {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(Motor motor) {

        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125 */
            motor.group1.xyz
        );
    }
}
public extension Motor {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(Motor motor) {

        float disallowed_e5 = motor.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(motor.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(motor.e235, motor.e315, motor.e125, motor.e12345)
        );
    }
}
public extension Motor {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(motor.e5, motor.e12345)
        );
    }
}
public extension Motor {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(Motor motor) {

        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e5 = motor.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            motor.group0.xyz, 
            /* e235, e315, e125 */
            motor.group1.xyz
        );
    }
}
public extension Motor {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(Motor motor) {

        float disallowed_e415 = motor.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = motor.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = motor.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e12345 = motor.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e235 = motor.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = motor.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = motor.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            motor.e5
        );
    }
}
public extension Motor {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(MultiVector multi_vector) {

        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            multi_vector.group4.xyz, 
            /* e23, e31, e12, e45 */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.e45), 
            /* e15, e25, e35, scalar */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.scalar)
        );
    }
}
public extension MultiVector {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension AntiDipoleInversion {
    public static Optional<AntiDipoleInversion> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            multi_vector.group7.xyz, 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e4 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e4), 
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
public extension MultiVector {
    public Optional<AntiDipoleInversion> try_into_AntiDipoleInversion() {
        return AntiDipoleInversion.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(MultiVector multi_vector) {

        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(multi_vector.e3215, multi_vector.scalar)
        );
    }
}
public extension MultiVector {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e321)
        );
    }
}
public extension MultiVector {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e321), 
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
public extension MultiVector {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            multi_vector.group5.xyz, 
            /* e15, e25, e35 */
            multi_vector.group3.xyz
        );
    }
}
public extension MultiVector {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(MultiVector multi_vector) {

        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.scalar), 
            /* e15, e25, e35, e3215 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e3215)
        );
    }
}
public extension MultiVector {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(multi_vector.e1, multi_vector.e2, multi_vector.e3, multi_vector.e5)
        );
    }
}
public extension MultiVector {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e12345 */
            multi_vector.e12345
        );
    }
}
public extension MultiVector {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            multi_vector.group7.xyz, 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125 */
            multi_vector.group8.xyz
        );
    }
}
public extension MultiVector {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            multi_vector.group7.xyz, 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e12345 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e12345)
        );
    }
}
public extension MultiVector {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            multi_vector.group4.xyz, 
            /* e23, e31, e12, e45 */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.e45), 
            /* e15, e25, e35 */
            multi_vector.group3.xyz
        );
    }
}
public extension MultiVector {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension DipoleInversion {
    public static Optional<DipoleInversion> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return DipoleInversion(
            /* e41, e42, e43 */
            multi_vector.group4.xyz, 
            /* e23, e31, e12, e45 */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.e45), 
            /* e15, e25, e35, e1234 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e1234), 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
public extension MultiVector {
    public Optional<DipoleInversion> try_into_DipoleInversion() {
        return DipoleInversion.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(multi_vector.e5, multi_vector.e12345)
        );
    }
}
public extension MultiVector {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            multi_vector.group3
        );
    }
}
public extension MultiVector {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            multi_vector.group3, 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
public extension MultiVector {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            multi_vector.group6.xyz, 
            /* e235, e315, e125 */
            multi_vector.group8.xyz
        );
    }
}
public extension MultiVector {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(multi_vector.e415, multi_vector.e425, multi_vector.e435, multi_vector.e12345), 
            /* e235, e315, e125, e5 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e5)
        );
    }
}
public extension MultiVector {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
public extension MultiVector {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            multi_vector.group1, 
            /* e5 */
            multi_vector.e5
        );
    }
}
public extension MultiVector {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(MultiVector multi_vector) {

        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            multi_vector.scalar
        );
    }
}
public extension MultiVector {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9, 
            /* e1234 */
            multi_vector.e1234
        );
    }
}
public extension MultiVector {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
public extension VersorEven {
    public static Optional<VersorEven> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e15 = multi_vector.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = multi_vector.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = multi_vector.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e45 = multi_vector.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e4235 = multi_vector.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = multi_vector.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = multi_vector.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = multi_vector.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(multi_vector.e423, multi_vector.e431, multi_vector.e412, multi_vector.e12345), 
            /* e415, e425, e435, e321 */
            multi_vector.group6, 
            /* e235, e315, e125, e5 */
            float4(multi_vector.e235, multi_vector.e315, multi_vector.e125, multi_vector.e5), 
            /* e1, e2, e3, e4 */
            multi_vector.group1
        );
    }
}
public extension MultiVector {
    public Optional<VersorEven> try_into_VersorEven() {
        return VersorEven.try_from(this);
    }
}
public extension VersorOdd {
    public static Optional<VersorOdd> try_from(MultiVector multi_vector) {

        float disallowed_e12345 = multi_vector.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e5 = multi_vector.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e415 = multi_vector.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = multi_vector.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = multi_vector.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e235 = multi_vector.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = multi_vector.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = multi_vector.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(multi_vector.e41, multi_vector.e42, multi_vector.e43, multi_vector.scalar), 
            /* e23, e31, e12, e45 */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.e45), 
            /* e15, e25, e35, e1234 */
            float4(multi_vector.e15, multi_vector.e25, multi_vector.e35, multi_vector.e1234), 
            /* e4235, e4315, e4125, e3215 */
            multi_vector.group9
        );
    }
}
public extension MultiVector {
    public Optional<VersorOdd> try_into_VersorOdd() {
        return VersorOdd.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(Plane plane) {

        float disallowed_e4235 = plane.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = plane.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = plane.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(plane.e3215, 0.0)
        );
    }
}
public extension Plane {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(Plane plane) {

        float disallowed_e4235 = plane.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = plane.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = plane.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), plane.e3215)
        );
    }
}
public extension Plane {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(RoundPoint round_point) {

        float disallowed_e4 = round_point.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(round_point.e1, round_point.e2, round_point.e3, round_point.e5)
        );
    }
}
public extension RoundPoint {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(RoundPoint round_point) {

        float disallowed_e4 = round_point.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(round_point.e1, round_point.e2, round_point.e3, round_point.e5)
        );
    }
}
public extension RoundPoint {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(RoundPoint round_point) {

        float disallowed_e1 = round_point.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = round_point.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = round_point.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = round_point.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(round_point.e5, 0.0)
        );
    }
}
public extension RoundPoint {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(RoundPoint round_point) {

        float disallowed_e1 = round_point.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = round_point.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = round_point.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = round_point.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), round_point.e5)
        );
    }
}
public extension RoundPoint {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(Sphere sphere) {

        float disallowed_e4235 = sphere.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = sphere.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = sphere.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = sphere.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(sphere.e3215, 0.0)
        );
    }
}
public extension Sphere {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(Sphere sphere) {

        float disallowed_e4235 = sphere.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = sphere.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = sphere.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e1234 = sphere.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), sphere.e3215)
        );
    }
}
public extension Sphere {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(Sphere sphere) {

        float disallowed_e1234 = sphere.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            sphere.group0
        );
    }
}
public extension Sphere {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(Sphere sphere) {

        float disallowed_e1234 = sphere.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            sphere.group0
        );
    }
}
public extension Sphere {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension AntiDipoleInversion {
    public static Optional<AntiDipoleInversion> try_from(VersorEven versor_even) {

        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            versor_even.group0.xyz, 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, e4 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e4), 
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
public extension VersorEven {
    public Optional<AntiDipoleInversion> try_into_AntiDipoleInversion() {
        return AntiDipoleInversion.try_from(this);
    }
}
public extension AntiFlatPoint {
    public static Optional<AntiFlatPoint> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e321)
        );
    }
}
public extension VersorEven {
    public Optional<AntiFlatPoint> try_into_AntiFlatPoint() {
        return AntiFlatPoint.try_from(this);
    }
}
public extension AntiFlector {
    public static Optional<AntiFlector> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e321), 
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
public extension VersorEven {
    public Optional<AntiFlector> try_into_AntiFlector() {
        return AntiFlector.try_from(this);
    }
}
public extension AntiPlane {
    public static Optional<AntiPlane> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(versor_even.e1, versor_even.e2, versor_even.e3, versor_even.e5)
        );
    }
}
public extension VersorEven {
    public Optional<AntiPlane> try_into_AntiPlane() {
        return AntiPlane.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e12345 */
            versor_even.e12345
        );
    }
}
public extension VersorEven {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension Circle {
    public static Optional<Circle> try_from(VersorEven versor_even) {

        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Circle(
            /* e423, e431, e412 */
            versor_even.group0.xyz, 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125 */
            versor_even.group2.xyz
        );
    }
}
public extension VersorEven {
    public Optional<Circle> try_into_Circle() {
        return Circle.try_from(this);
    }
}
public extension CircleRotor {
    public static Optional<CircleRotor> try_from(VersorEven versor_even) {

        float disallowed_e5 = versor_even.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return CircleRotor(
            /* e423, e431, e412 */
            versor_even.group0.xyz, 
            /* e415, e425, e435, e321 */
            versor_even.group1, 
            /* e235, e315, e125, e12345 */
            float4(versor_even.e235, versor_even.e315, versor_even.e125, versor_even.e12345)
        );
    }
}
public extension VersorEven {
    public Optional<CircleRotor> try_into_CircleRotor() {
        return CircleRotor.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return DualNum(
            /* e5, e12345 */
            float2(versor_even.e5, versor_even.e12345)
        );
    }
}
public extension VersorEven {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e5 = versor_even.e5;
        if (disallowed_e5 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Line(
            /* e415, e425, e435 */
            versor_even.group1.xyz, 
            /* e235, e315, e125 */
            versor_even.group2.xyz
        );
    }
}
public extension VersorEven {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e1 = versor_even.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = versor_even.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = versor_even.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = versor_even.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(versor_even.e415, versor_even.e425, versor_even.e435, versor_even.e12345), 
            /* e235, e315, e125, e5 */
            versor_even.group2
        );
    }
}
public extension VersorEven {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension RoundPoint {
    public static Optional<RoundPoint> try_from(VersorEven versor_even) {

        float disallowed_e423 = versor_even.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = versor_even.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = versor_even.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e12345 = versor_even.e12345;
        if (disallowed_e12345 != 0.0) {
            return none;
        }
        float disallowed_e415 = versor_even.e415;
        if (disallowed_e415 != 0.0) {
            return none;
        }
        float disallowed_e425 = versor_even.e425;
        if (disallowed_e425 != 0.0) {
            return none;
        }
        float disallowed_e435 = versor_even.e435;
        if (disallowed_e435 != 0.0) {
            return none;
        }
        float disallowed_e321 = versor_even.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        float disallowed_e235 = versor_even.e235;
        if (disallowed_e235 != 0.0) {
            return none;
        }
        float disallowed_e315 = versor_even.e315;
        if (disallowed_e315 != 0.0) {
            return none;
        }
        float disallowed_e125 = versor_even.e125;
        if (disallowed_e125 != 0.0) {
            return none;
        }
        return RoundPoint(
            /* e1, e2, e3, e4 */
            versor_even.group3, 
            /* e5 */
            versor_even.e5
        );
    }
}
public extension VersorEven {
    public Optional<RoundPoint> try_into_RoundPoint() {
        return RoundPoint.try_from(this);
    }
}
public extension AntiCircleRotor {
    public static Optional<AntiCircleRotor> try_from(VersorOdd versor_odd) {

        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiCircleRotor(
            /* e41, e42, e43 */
            versor_odd.group0.xyz, 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, scalar */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.scalar)
        );
    }
}
public extension VersorOdd {
    public Optional<AntiCircleRotor> try_into_AntiCircleRotor() {
        return AntiCircleRotor.try_from(this);
    }
}
public extension AntiDualNum {
    public static Optional<AntiDualNum> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiDualNum(
            /* e3215, scalar */
            float2(versor_odd.e3215, versor_odd.scalar)
        );
    }
}
public extension VersorOdd {
    public Optional<AntiDualNum> try_into_AntiDualNum() {
        return AntiDualNum.try_from(this);
    }
}
public extension AntiLine {
    public static Optional<AntiLine> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return AntiLine(
            /* e23, e31, e12 */
            versor_odd.group1.xyz, 
            /* e15, e25, e35 */
            versor_odd.group2.xyz
        );
    }
}
public extension VersorOdd {
    public Optional<AntiLine> try_into_AntiLine() {
        return AntiLine.try_from(this);
    }
}
public extension AntiMotor {
    public static Optional<AntiMotor> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(versor_odd.e23, versor_odd.e31, versor_odd.e12, versor_odd.scalar), 
            /* e15, e25, e35, e3215 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e3215)
        );
    }
}
public extension VersorOdd {
    public Optional<AntiMotor> try_into_AntiMotor() {
        return AntiMotor.try_from(this);
    }
}
public extension Dipole {
    public static Optional<Dipole> try_from(VersorOdd versor_odd) {

        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Dipole(
            /* e41, e42, e43 */
            versor_odd.group0.xyz, 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35 */
            versor_odd.group2.xyz
        );
    }
}
public extension VersorOdd {
    public Optional<Dipole> try_into_Dipole() {
        return Dipole.try_from(this);
    }
}
public extension DipoleInversion {
    public static Optional<DipoleInversion> try_from(VersorOdd versor_odd) {

        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return DipoleInversion(
            /* e41, e42, e43 */
            versor_odd.group0.xyz, 
            /* e23, e31, e12, e45 */
            versor_odd.group1, 
            /* e15, e25, e35, e1234 */
            versor_odd.group2, 
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
public extension VersorOdd {
    public Optional<DipoleInversion> try_into_DipoleInversion() {
        return DipoleInversion.try_from(this);
    }
}
public extension FlatPoint {
    public static Optional<FlatPoint> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e45)
        );
    }
}
public extension VersorOdd {
    public Optional<FlatPoint> try_into_FlatPoint() {
        return FlatPoint.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Flector(
            /* e15, e25, e35, e45 */
            float4(versor_odd.e15, versor_odd.e25, versor_odd.e35, versor_odd.e45), 
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
public extension VersorOdd {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3
        );
    }
}
public extension VersorOdd {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        float disallowed_e1234 = versor_odd.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e4235 = versor_odd.e4235;
        if (disallowed_e4235 != 0.0) {
            return none;
        }
        float disallowed_e4315 = versor_odd.e4315;
        if (disallowed_e4315 != 0.0) {
            return none;
        }
        float disallowed_e4125 = versor_odd.e4125;
        if (disallowed_e4125 != 0.0) {
            return none;
        }
        float disallowed_e3215 = versor_odd.e3215;
        if (disallowed_e3215 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            versor_odd.scalar
        );
    }
}
public extension VersorOdd {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Sphere {
    public static Optional<Sphere> try_from(VersorOdd versor_odd) {

        float disallowed_e41 = versor_odd.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = versor_odd.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = versor_odd.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_scalar = versor_odd.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e23 = versor_odd.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = versor_odd.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = versor_odd.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e45 = versor_odd.e45;
        if (disallowed_e45 != 0.0) {
            return none;
        }
        float disallowed_e15 = versor_odd.e15;
        if (disallowed_e15 != 0.0) {
            return none;
        }
        float disallowed_e25 = versor_odd.e25;
        if (disallowed_e25 != 0.0) {
            return none;
        }
        float disallowed_e35 = versor_odd.e35;
        if (disallowed_e35 != 0.0) {
            return none;
        }
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            versor_odd.group3, 
            /* e1234 */
            versor_odd.e1234
        );
    }
}
public extension VersorOdd {
    public Optional<Sphere> try_into_Sphere() {
        return Sphere.try_from(this);
    }
}
