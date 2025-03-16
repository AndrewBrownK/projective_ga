// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 47
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0

impl From<AntiDipoleInversion> for VersorEven {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion[e235],
                from_anti_dipole_inversion[e315],
                from_anti_dipole_inversion[e125],
                from_anti_dipole_inversion[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
        )
    }
}

impl From<AntiDipoleInversionAtInfinity> for VersorEven {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion_at_infinity[e235],
                from_anti_dipole_inversion_at_infinity[e315],
                from_anti_dipole_inversion_at_infinity[e125],
                from_anti_dipole_inversion_at_infinity[e5],
            ]),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
        )
    }
}

impl From<AntiDipoleInversionOnOrigin> for VersorEven {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
        )
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                from_anti_dipole_inversion_orthogonal_origin[e235],
                from_anti_dipole_inversion_orthogonal_origin[e315],
                from_anti_dipole_inversion_orthogonal_origin[e125],
                from_anti_dipole_inversion_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_orthogonal_origin[e4]),
        )
    }
}

impl From<AntiDipoleOnOrigin> for VersorEven {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatOrigin> for VersorEven {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e5
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125], from_anti_flector[e5]]),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
        )
    }
}

impl From<AntiFlectorOnOrigin> for VersorEven {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
        )
    }
}

impl From<AntiMysteryDipoleInversion> for VersorEven {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
        )
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_anti_plane[e5]),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
        )
    }
}

impl From<AntiPlaneOnOrigin> for VersorEven {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiSphereOnOrigin> for VersorEven {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
        )
    }
}

impl From<Circle> for VersorEven {
    fn from(from_circle: Circle) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e5
            from_circle.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAligningOrigin> for VersorEven {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_aligning_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_aligning_origin.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAtInfinity> for VersorEven {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e235, e315, e125, e5
            from_circle_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAtOrigin> for VersorEven {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_circle_at_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOnOrigin> for VersorEven {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_on_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOrthogonalOrigin> for VersorEven {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e235, e315, e125, e5
            from_circle_orthogonal_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_circle_rotor[e423], from_circle_rotor[e431], from_circle_rotor[e412], from_circle_rotor[e12345]]),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e235, e315, e125, e5
            from_circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotorAligningOrigin> for VersorEven {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                from_circle_rotor_aligning_origin[e423],
                from_circle_rotor_aligning_origin[e431],
                from_circle_rotor_aligning_origin[e412],
                from_circle_rotor_aligning_origin[e12345],
            ]),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_aligning_origin_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotorAtInfinity> for VersorEven {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_circle_rotor_at_infinity.group0(),
            // e235, e315, e125, e5
            from_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotorOnOrigin> for VersorEven {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor_on_origin.group0(),
            // e415, e425, e435, e321
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for VersorEven {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_dual_num[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_dual_num[e4]),
        )
    }
}

impl From<Infinity> for VersorEven {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Line> for VersorEven {
    fn from(from_line: Line) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_line.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<LineAtInfinity> for VersorEven {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_line_at_infinity.group0().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<LineOnOrigin> for VersorEven {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Motor> for VersorEven {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor[e12345]),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MotorAtInfinity> for VersorEven {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_motor_at_infinity.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MotorOnOrigin> for VersorEven {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor_on_origin[e12345]),
            // e415, e425, e435, e321
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryCircle> for VersorEven {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryCircleRotor> for VersorEven {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_mystery_circle_rotor[e12345]),
            // e415, e425, e435, e321
            from_mystery_circle_rotor.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryVersorEven> for VersorEven {
    fn from(from_mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_mystery_versor_even[e12345]),
            // e415, e425, e435, e321
            from_mystery_versor_even.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_mystery_versor_even[e1], from_mystery_versor_even[e2], from_mystery_versor_even[e3], 0.0]),
        )
    }
}

impl From<NullCircleAtOrigin> for VersorEven {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_null_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEven {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
        )
    }
}

impl From<Origin> for VersorEven {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
        )
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point[e5]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        )
    }
}

impl From<RoundPointAtOrigin> for VersorEven {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
        )
    }
}

impl From<VersorEvenAligningOrigin> for VersorEven {
    fn from(from_versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_aligning_origin.group0(),
            // e415, e425, e435, e321
            from_versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_versor_even_aligning_origin.group2(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_aligning_origin[e4]),
        )
    }
}

impl From<VersorEvenAtInfinity> for VersorEven {
    fn from(from_versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_versor_even_at_infinity[e12345]),
            // e415, e425, e435, e321
            from_versor_even_at_infinity.group1(),
            // e235, e315, e125, e5
            from_versor_even_at_infinity.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([from_versor_even_at_infinity[e1], from_versor_even_at_infinity[e2], from_versor_even_at_infinity[e3], 0.0]),
        )
    }
}

impl From<VersorEvenAtOrigin> for VersorEven {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_versor_even_at_origin.group1(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
        )
    }
}

impl From<VersorEvenOnOrigin> for VersorEven {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_on_origin.group0(),
            // e415, e425, e435, e321
            from_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_on_origin[e4]),
        )
    }
}

impl From<VersorEvenOrthogonalOrigin> for VersorEven {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_versor_even_orthogonal_origin[e321]),
            // e235, e315, e125, e5
            from_versor_even_orthogonal_origin.group1(),
            // e1, e2, e3, e4
            from_versor_even_orthogonal_origin.group2(),
        )
    }
}

impl TryFrom<MultiVector> for VersorEven {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e12345]]),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ))
    }
}
