// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 44
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

impl From<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_on_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            from_anti_circle_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_aligning_origin.group0().with_w(from_anti_circle_rotor_aligning_origin[scalar]),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
        )
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_aligning_origin_at_infinity[scalar]),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
        )
    }
}

impl From<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_on_origin.group0(),
            // e23, e31, e12, e3215
            from_anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_dual_num[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e1234]),
        )
    }
}

impl From<AntiLine> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line: AntiLine) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
        )
    }
}

impl From<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            from_anti_line_on_origin.group0().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMotor> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor[scalar]),
            // e23, e31, e12, e3215
            from_anti_motor.group0().xyz().with_w(from_anti_motor[e3215]),
            // e15, e25, e35, e1234
            from_anti_motor.group1().xyz().with_w(0.0),
        )
    }
}

impl From<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor_on_origin[scalar]),
            // e23, e31, e12, e3215
            from_anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_versor_even_on_origin.group0(),
            // e23, e31, e12, e3215
            from_anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_versor_even_on_origin[e1234]),
        )
    }
}

impl From<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_at_origin.group1().with_w(0.0),
        )
    }
}

impl From<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_origin[e3215]),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
        )
    }
}

impl From<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([
                from_dipole_inversion_orthogonal_origin[e23],
                from_dipole_inversion_orthogonal_origin[e31],
                from_dipole_inversion_orthogonal_origin[e12],
                from_dipole_inversion_orthogonal_origin[e3215],
            ]),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
        )
    }
}

impl From<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_dipole_orthogonal_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            from_dipole_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_orthogonal_origin.group2().with_w(0.0),
        )
    }
}

impl From<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flat_point_at_infinity.group0().with_w(0.0),
        )
    }
}

impl From<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
            // e15, e25, e35, e1234
            from_flector_at_infinity.group0().xyz().with_w(0.0),
        )
    }
}

impl From<Horizon> for VersorOddOrthogonalOrigin {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_dipole_inversion_at_origin[e1234]),
        )
    }
}

impl From<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_sphere_at_origin[e1234]),
        )
    }
}

impl From<Scalar> for VersorOddOrthogonalOrigin {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}

impl From<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e1234]),
        )
    }
}

impl TryFrom<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            anti_circle_rotor.group0().with_w(anti_circle_rotor[scalar]),
            // e23, e31, e12, e3215
            anti_circle_rotor.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(anti_circle_rotor_at_infinity[scalar]),
            // e23, e31, e12, e3215
            anti_circle_rotor_at_infinity.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            anti_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(anti_mystery_circle_rotor[scalar]),
            // e23, e31, e12, e3215
            anti_mystery_circle_rotor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Dipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole.group0().with_w(0.0),
            // e23, e31, e12, e3215
            dipole.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            dipole.group2().with_w(0.0),
        ))
    }
}

impl TryFrom<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            dipole_aligning_origin.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            dipole_at_infinity.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            dipole_at_infinity.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion.group0().with_w(0.0),
            // e23, e31, e12, e3215
            dipole_inversion.group1().xyz().with_w(dipole_inversion[e3215]),
            // e15, e25, e35, e1234
            dipole_inversion.group2(),
        ))
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(dipole_inversion_aligning_origin[e3215]),
            // e15, e25, e35, e1234
            dipole_inversion_aligning_origin.group1(),
        ))
    }
}

impl TryFrom<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            dipole_inversion_at_infinity.group0().xyz().with_w(dipole_inversion_at_infinity[e3215]),
            // e15, e25, e35, e1234
            dipole_inversion_at_infinity.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(dipole_inversion_on_origin[e1234]),
        ))
    }
}

impl TryFrom<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            dipole_on_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<FlatPoint> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            flat_point.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<Flector> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(flector[e3215]),
            // e15, e25, e35, e1234
            flector.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            multi_vector.group3().xyz().with_w(multi_vector[scalar]),
            // e23, e31, e12, e3215
            multi_vector.group5().with_w(multi_vector[e3215]),
            // e15, e25, e35, e1234
            multi_vector.group4().with_w(multi_vector[e1234]),
        ))
    }
}

impl TryFrom<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole: MysteryDipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            mystery_dipole.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole_inversion: MysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            mystery_dipole_inversion.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(mystery_versor_odd[scalar]),
            // e23, e31, e12, e3215
            mystery_versor_odd.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Plane> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(plane: Plane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Plane do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(plane[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Sphere> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x3::from(0.0).with_w(sphere[e3215]),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(sphere[e1234]),
        ))
    }
}

impl TryFrom<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(sphere_on_origin[e1234]),
        ))
    }
}

impl TryFrom<VersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            versor_odd.group0(),
            // e23, e31, e12, e3215
            versor_odd.group1().xyz().with_w(versor_odd[e3215]),
            // e15, e25, e35, e1234
            versor_odd.group2(),
        ))
    }
}

impl TryFrom<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(versor_odd_at_infinity[scalar]),
            // e23, e31, e12, e3215
            versor_odd_at_infinity.group1().xyz().with_w(versor_odd_at_infinity[e3215]),
            // e15, e25, e35, e1234
            versor_odd_at_infinity.group0().yzw().with_w(0.0),
        ))
    }
}
