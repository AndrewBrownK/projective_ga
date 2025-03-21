// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 46
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       0       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       0       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       0       0       0
//  Average:         0       0       0       0
//  Maximum:         0       0       0       0

impl From<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e235, e315, e125, e4
            from_anti_dipole_inversion_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e5
            from_anti_dipole_inversion_at_infinity.group2(),
        )
    }
}

impl From<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_anti_dipole_inversion_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e321]),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e4]),
            // e1, e2, e3, e5
            from_anti_dipole_inversion_on_origin.group1().yzw().with_w(0.0),
        )
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e235, e315, e125, e4
            from_anti_dipole_inversion_orthogonal_origin.group2(),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_orthogonal_origin[e5]),
        )
    }
}

impl From<AntiDipoleOnOrigin> for AntiDipoleInversion {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_anti_dipole_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatOrigin> for AntiDipoleInversion {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatPoint> for AntiDipoleInversion {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e4
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlector> for AntiDipoleInversion {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e4
            from_anti_flector.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            from_anti_flector.group1(),
        )
    }
}

impl From<AntiFlectorOnOrigin> for AntiDipoleInversion {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            from_anti_flector_on_origin.group0().yzw().with_w(0.0),
        )
    }
}

impl From<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
        )
    }
}

impl From<AntiPlane> for AntiDipoleInversion {
    fn from(from_anti_plane: AntiPlane) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            from_anti_plane.group0(),
        )
    }
}

impl From<AntiPlaneOnOrigin> for AntiDipoleInversion {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            from_anti_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<AntiSphereOnOrigin> for AntiDipoleInversion {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_anti_sphere_on_origin[e4]),
            // e1, e2, e3, e5
            from_anti_sphere_on_origin.group0().xyz().with_w(0.0),
        )
    }
}

impl From<Circle> for AntiDipoleInversion {
    fn from(from_circle: Circle) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle.group0(),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e4
            from_circle.group2().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAligningOrigin> for AntiDipoleInversion {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e4
            from_circle_aligning_origin.group2().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAtInfinity> for AntiDipoleInversion {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e235, e315, e125, e4
            from_circle_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAtOrigin> for AntiDipoleInversion {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            from_circle_at_origin.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOnOrigin> for AntiDipoleInversion {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOrthogonalOrigin> for AntiDipoleInversion {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_circle_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e235, e315, e125, e4
            from_circle_orthogonal_origin.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Infinity> for AntiDipoleInversion {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
        )
    }
}

impl From<Line> for AntiDipoleInversion {
    fn from(from_line: Line) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e4
            from_line.group1().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<LineAtInfinity> for AntiDipoleInversion {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            from_line_at_infinity.group0().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<LineOnOrigin> for AntiDipoleInversion {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<MotorAtInfinity> for AntiDipoleInversion {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            from_motor_at_infinity.group0().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_motor_at_infinity[e5]),
        )
    }
}

impl From<MysteryCircle> for AntiDipoleInversion {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullCircleAtOrigin> for AntiDipoleInversion {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_null_versor_even_at_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Origin> for AntiDipoleInversion {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<RoundPoint> for AntiDipoleInversion {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_round_point[e4]),
            // e1, e2, e3, e5
            from_round_point.group0().xyz().with_w(from_round_point[e5]),
        )
    }
}

impl From<RoundPointAtOrigin> for AntiDipoleInversion {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
        )
    }
}

impl From<VersorEvenAtOrigin> for AntiDipoleInversion {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_versor_even_at_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                from_versor_even_at_origin[e235],
                from_versor_even_at_origin[e315],
                from_versor_even_at_origin[e125],
                from_versor_even_at_origin[e4],
            ]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e5]),
        )
    }
}

impl From<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            from_versor_even_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_versor_even_orthogonal_origin[e321]),
            // e235, e315, e125, e4
            from_versor_even_orthogonal_origin.group1().xyz().with_w(from_versor_even_orthogonal_origin[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([
                from_versor_even_orthogonal_origin[e1],
                from_versor_even_orthogonal_origin[e2],
                from_versor_even_orthogonal_origin[e3],
                from_versor_even_orthogonal_origin[e5],
            ]),
        )
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            circle_rotor.group0(),
            // e415, e425, e435, e321
            circle_rotor.group1(),
            // e235, e315, e125, e4
            circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            circle_rotor_aligning_origin.group0(),
            // e415, e425, e435, e321
            circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e4
            circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125, e4
            circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            circle_rotor_at_infinity.group0(),
            // e235, e315, e125, e4
            circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            circle_rotor_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DualNum> for AntiDipoleInversion {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(dual_num[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Motor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e4
            motor.group1().xyz().with_w(0.0),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(motor[e5]),
        ))
    }
}

impl TryFrom<MotorOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(motor_on_origin: MotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversion {
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
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            multi_vector.group7(),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e4
            multi_vector.group8().with_w(multi_vector[e4]),
            // e1, e2, e3, e5
            multi_vector.group1().xyz().with_w(multi_vector[e5]),
        ))
    }
}

impl TryFrom<MysteryCircleRotor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            mystery_circle_rotor.group0(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryVersorEven> for AntiDipoleInversion {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            mystery_versor_even.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            mystery_versor_even.group0().yzw().with_w(0.0),
        ))
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            versor_even.group0().xyz(),
            // e415, e425, e435, e321
            versor_even.group1(),
            // e235, e315, e125, e4
            versor_even.group2().xyz().with_w(versor_even[e4]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e5]]),
        ))
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            versor_even_aligning_origin.group0().xyz(),
            // e415, e425, e435, e321
            versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e4],
            ]),
            // e1, e2, e3, e5
            Simd32x3::from(0.0).with_w(versor_even_aligning_origin[e5]),
        ))
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            versor_even_at_infinity.group1(),
            // e235, e315, e125, e4
            versor_even_at_infinity.group2().xyz().with_w(0.0),
            // e1, e2, e3, e5
            versor_even_at_infinity.group0().yzw().with_w(versor_even_at_infinity[e5]),
        ))
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            versor_even_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            versor_even_on_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e4
            Simd32x3::from(0.0).with_w(versor_even_on_origin[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ))
    }
}
