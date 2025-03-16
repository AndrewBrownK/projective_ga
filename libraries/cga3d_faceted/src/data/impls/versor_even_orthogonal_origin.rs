// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 44
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

impl From<AntiDipoleInversionOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_anti_dipole_inversion_on_origin.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
        )
    }
}

impl From<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_anti_dipole_on_origin.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e5
            from_anti_flat_point.group0().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlector> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125], from_anti_flector[e5]]),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
        )
    }
}

impl From<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
        )
    }
}

impl From<AntiPlane> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_anti_plane[e5]),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
        )
    }
}

impl From<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
        )
    }
}

impl From<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_circle_at_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_at_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_circle_orthogonal_origin.group0(),
            // e235, e315, e125, e5
            from_circle_orthogonal_origin.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<Infinity> for VersorEvenOrthogonalOrigin {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_line_at_infinity.group0().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_motor_at_infinity.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_circle_at_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
        )
    }
}

impl From<Origin> for VersorEvenOrthogonalOrigin {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
        )
    }
}

impl From<RoundPoint> for VersorEvenOrthogonalOrigin {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point[e5]),
            // e1, e2, e3, e4
            from_round_point.group0(),
        )
    }
}

impl From<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
        )
    }
}

impl From<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            from_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_versor_even_at_origin.group1(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
        )
    }
}

impl TryFrom<AntiDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([anti_dipole_inversion[e423], anti_dipole_inversion[e431], anti_dipole_inversion[e412], anti_dipole_inversion[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], anti_dipole_inversion[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3], anti_dipole_inversion[e4]]),
        ))
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_at_infinity[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
                anti_dipole_inversion_at_infinity[e5],
            ]),
            // e1, e2, e3, e4
            anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                anti_dipole_inversion_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_orthogonal_origin[e4]),
        ))
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(anti_mystery_dipole_inversion[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            anti_mystery_dipole_inversion.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<Circle> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle[e423], circle[e431], circle[e412], circle[e321]]),
            // e235, e315, e125, e5
            circle.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_aligning_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            circle_aligning_origin.group2().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_at_infinity[e321]),
            // e235, e315, e125, e5
            circle_at_infinity.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412], circle_rotor[e321]]),
            // e235, e315, e125, e5
            circle_rotor.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_aligning_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(circle_rotor_at_infinity[e321]),
            // e235, e315, e125, e5
            circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<CircleRotorOnOrigin> for VersorEvenOrthogonalOrigin {
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
        let el = circle_rotor_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DualNum> for VersorEvenOrthogonalOrigin {
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
            let mut error = "Elements from DualNum do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(dual_num[e4]),
        ))
    }
}

impl TryFrom<Line> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            line.group1().with_w(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Motor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            motor.group1(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for VersorEvenOrthogonalOrigin {
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
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
            let mut error = "Elements from MultiVector do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            multi_vector.group1(),
        ))
    }
}

impl TryFrom<MysteryCircle> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle: MysteryCircle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircle do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryCircleRotor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_circle_rotor[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryVersorEven> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(mystery_versor_even[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
        ))
    }
}

impl TryFrom<VersorEven> for VersorEvenOrthogonalOrigin {
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
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even[e423], versor_even[e431], versor_even[e412], versor_even[e321]]),
            // e235, e315, e125, e5
            versor_even.group2(),
            // e1, e2, e3, e4
            versor_even.group3(),
        ))
    }
}

impl TryFrom<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
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
        let el = versor_even_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_aligning_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            versor_even_aligning_origin.group2(),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(versor_even_aligning_origin[e4]),
        ))
    }
}

impl TryFrom<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
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
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(versor_even_at_infinity[e321]),
            // e235, e315, e125, e5
            versor_even_at_infinity.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
        ))
    }
}

impl TryFrom<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
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
        let el = versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            versor_even_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(versor_even_on_origin[e4]),
        ))
    }
}
