// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 46
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

impl From<AntiCircleOnOrigin> for DipoleInversion {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_anti_circle_on_origin.group0(),
            // e23, e31, e12, e45
            from_anti_circle_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLine> for DipoleInversion {
    fn from(from_anti_line: AntiLine) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLineOnOrigin> for DipoleInversion {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_anti_line_on_origin.group0().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Dipole> for DipoleInversion {
    fn from(from_dipole: Dipole) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole.group0(),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAligningOrigin> for DipoleInversion {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_aligning_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_aligning_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAtInfinity> for DipoleInversion {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_dipole_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAtOrigin> for DipoleInversion {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_at_origin.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_at_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleInversionAligningOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_aligning_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_inversion_aligning_origin.group1(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_aligning_origin.group2(),
        )
    }
}

impl From<DipoleInversionAtInfinity> for DipoleInversion {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_at_infinity.group2(),
        )
    }
}

impl From<DipoleInversionAtOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_origin[e3215]),
        )
    }
}

impl From<DipoleInversionOnOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_dipole_inversion_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                from_dipole_inversion_on_origin[e4235],
                from_dipole_inversion_on_origin[e4315],
                from_dipole_inversion_on_origin[e4125],
                0.0,
            ]),
        )
    }
}

impl From<DipoleInversionOrthogonalOrigin> for DipoleInversion {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e23, e31, e12, e45
            from_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_orthogonal_origin[e3215]),
        )
    }
}

impl From<DipoleOnOrigin> for DipoleInversion {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleOrthogonalOrigin> for DipoleInversion {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_dipole_orthogonal_origin.group0(),
            // e23, e31, e12, e45
            from_dipole_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_orthogonal_origin.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatOrigin> for DipoleInversion {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPoint> for DipoleInversion {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPointAtInfinity> for DipoleInversion {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flat_point_at_infinity.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for DipoleInversion {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        )
    }
}

impl From<FlectorAtInfinity> for DipoleInversion {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flector_at_infinity.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
        )
    }
}

impl From<FlectorOnOrigin> for DipoleInversion {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125], 0.0]),
        )
    }
}

impl From<Horizon> for DipoleInversion {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
        )
    }
}

impl From<MysteryDipole> for DipoleInversion {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryDipoleInversion> for DipoleInversion {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole_inversion.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_mystery_dipole_inversion.group1().with_w(0.0),
        )
    }
}

impl From<NullDipoleAtOrigin> for DipoleInversion {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_null_dipole_at_origin.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullDipoleInversionAtOrigin> for DipoleInversion {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            from_null_dipole_inversion_at_origin.group0().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_dipole_inversion_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullSphereAtOrigin> for DipoleInversion {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Plane> for DipoleInversion {
    fn from(from_plane: Plane) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        )
    }
}

impl From<PlaneOnOrigin> for DipoleInversion {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<Sphere> for DipoleInversion {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        )
    }
}

impl From<SphereAtOrigin> for DipoleInversion {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e3215]),
        )
    }
}

impl From<SphereOnOrigin> for DipoleInversion {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere_on_origin.group0().xyz().with_w(0.0),
        )
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor.group0(),
            // e23, e31, e12, e45
            anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor_aligning_origin.group0(),
            // e23, e31, e12, e45
            anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_circle_rotor_at_infinity.group0(),
            // e15, e25, e35, e1234
            anti_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_circle_rotor_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiDualNum> for DipoleInversion {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(anti_dual_num[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiMotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(anti_motor[e3215]),
        ))
    }
}

impl TryFrom<AntiMotorOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotorOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiMysteryCircleRotor> for DipoleInversion {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            anti_mystery_circle_rotor.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            anti_versor_even_on_origin.group0().xyz(),
            // e23, e31, e12, e45
            anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(anti_versor_even_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for DipoleInversion {
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
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            multi_vector.group3().xyz(),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ))
    }
}

impl TryFrom<MysteryVersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            mystery_versor_odd.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        ))
    }
}

impl TryFrom<VersorOdd> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            versor_odd.group0().xyz(),
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e15, e25, e35, e1234
            versor_odd.group2(),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ))
    }
}

impl TryFrom<VersorOddAtInfinity> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            versor_odd_at_infinity.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            versor_odd_at_infinity.group2(),
        ))
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for DipoleInversion {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into DipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(DipoleInversion::from_groups(
            // e41, e42, e43
            versor_odd_orthogonal_origin.group0().xyz(),
            // e23, e31, e12, e45
            versor_odd_orthogonal_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            versor_odd_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(versor_odd_orthogonal_origin[e3215]),
        ))
    }
}
