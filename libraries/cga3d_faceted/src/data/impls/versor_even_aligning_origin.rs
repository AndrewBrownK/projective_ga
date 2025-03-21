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

impl From<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAligningOrigin {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(from_anti_dipole_inversion_orthogonal_origin[e4]),
            // e235, e315, e125, e5
            from_anti_dipole_inversion_orthogonal_origin.group2().xyz().with_w(from_anti_dipole_inversion_orthogonal_origin[e5]),
        )
    }
}

impl From<AntiScalar> for VersorEvenAligningOrigin {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleAligningOrigin> for VersorEvenAligningOrigin {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_circle_aligning_origin.group0().with_w(0.0),
            // e415, e425, e435, e4
            from_circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_aligning_origin.group2().with_w(0.0),
        )
    }
}

impl From<CircleAtOrigin> for VersorEvenAligningOrigin {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_circle_at_origin.group1().with_w(0.0),
        )
    }
}

impl From<CircleOnOrigin> for VersorEvenAligningOrigin {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_circle_on_origin.group0().with_w(0.0),
            // e415, e425, e435, e4
            from_circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleRotorAligningOrigin> for VersorEvenAligningOrigin {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor_aligning_origin.group0().with_w(from_circle_rotor_aligning_origin[e12345]),
            // e415, e425, e435, e4
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
        )
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for VersorEvenAligningOrigin {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_aligning_origin_at_infinity[e12345]),
            // e415, e425, e435, e4
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
        )
    }
}

impl From<CircleRotorOnOrigin> for VersorEvenAligningOrigin {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_circle_rotor_on_origin.group0(),
            // e415, e425, e435, e4
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<DualNum> for VersorEvenAligningOrigin {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_dual_num[e12345]),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(from_dual_num[e4]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Infinity> for VersorEvenAligningOrigin {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_infinity[e5]),
        )
    }
}

impl From<Line> for VersorEvenAligningOrigin {
    fn from(from_line: Line) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e5
            from_line.group1().with_w(0.0),
        )
    }
}

impl From<LineAtInfinity> for VersorEvenAligningOrigin {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_line_at_infinity.group0().with_w(0.0),
        )
    }
}

impl From<LineOnOrigin> for VersorEvenAligningOrigin {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Motor> for VersorEvenAligningOrigin {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor[e12345]),
            // e415, e425, e435, e4
            from_motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            from_motor.group1(),
        )
    }
}

impl From<MotorAtInfinity> for VersorEvenAligningOrigin {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            from_motor_at_infinity.group0(),
        )
    }
}

impl From<MotorOnOrigin> for VersorEvenAligningOrigin {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(from_motor_on_origin[e12345]),
            // e415, e425, e435, e4
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullCircleAtOrigin> for VersorEvenAligningOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_null_circle_at_origin.group0().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEvenAligningOrigin {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_null_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<Origin> for VersorEvenAligningOrigin {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl From<RoundPointAtOrigin> for VersorEvenAligningOrigin {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e5]),
        )
    }
}

impl From<VersorEvenAtOrigin> for VersorEvenAligningOrigin {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_at_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
            // e235, e315, e125, e5
            from_versor_even_at_origin.group1(),
        )
    }
}

impl From<VersorEvenOnOrigin> for VersorEvenAligningOrigin {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            from_versor_even_on_origin.group0(),
            // e415, e425, e435, e4
            from_versor_even_on_origin.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}

impl TryFrom<AntiDipoleInversion> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            anti_dipole_inversion.group0().with_w(0.0),
            // e415, e425, e435, e4
            anti_dipole_inversion.group1().xyz().with_w(anti_dipole_inversion[e4]),
            // e235, e315, e125, e5
            anti_dipole_inversion.group2().xyz().with_w(anti_dipole_inversion[e5]),
        ))
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            anti_dipole_inversion_at_infinity.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            anti_dipole_inversion_at_infinity.group1().with_w(anti_dipole_inversion_at_infinity[e5]),
        ))
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            anti_dipole_inversion_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_on_origin[e4]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiDipoleOnOrigin> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            anti_dipole_on_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiFlatPoint> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            anti_flat_point.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiFlector> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            anti_flector.group0().xyz().with_w(anti_flector[e5]),
        ))
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            anti_mystery_dipole_inversion.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiPlane> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(anti_plane[e5]),
        ))
    }
}

impl TryFrom<AntiSphereOnOrigin> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(anti_sphere_on_origin[e4]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Circle> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            circle.group0().with_w(0.0),
            // e415, e425, e435, e4
            circle.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            circle.group2().with_w(0.0),
        ))
    }
}

impl TryFrom<CircleAtInfinity> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            circle_at_infinity.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            circle_at_infinity.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<CircleOrthogonalOrigin> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            circle_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            circle_orthogonal_origin.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<CircleRotor> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            circle_rotor.group0().with_w(circle_rotor[e12345]),
            // e415, e425, e435, e4
            circle_rotor.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            circle_rotor.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<CircleRotorAtInfinity> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(circle_rotor_at_infinity[e12345]),
            // e415, e425, e435, e4
            circle_rotor_at_infinity.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            circle_rotor_at_infinity.group1().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for VersorEvenAligningOrigin {
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
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from MultiVector do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            multi_vector.group7().with_w(multi_vector[e12345]),
            // e415, e425, e435, e4
            multi_vector.group6().xyz().with_w(multi_vector[e4]),
            // e235, e315, e125, e5
            multi_vector.group8().with_w(multi_vector[e5]),
        ))
    }
}

impl TryFrom<MysteryCircle> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(mystery_circle: MysteryCircle) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircle do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            mystery_circle.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryCircleRotor> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(mystery_circle_rotor[e12345]),
            // e415, e425, e435, e4
            mystery_circle_rotor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MysteryVersorEven> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(mystery_versor_even[e12345]),
            // e415, e425, e435, e4
            mystery_versor_even.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<RoundPoint> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(round_point[e4]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(round_point[e5]),
        ))
    }
}

impl TryFrom<VersorEven> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            versor_even.group0(),
            // e415, e425, e435, e4
            versor_even.group1().xyz().with_w(versor_even[e4]),
            // e235, e315, e125, e5
            versor_even.group2(),
        ))
    }
}

impl TryFrom<VersorEvenAtInfinity> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(versor_even_at_infinity[e12345]),
            // e415, e425, e435, e4
            versor_even_at_infinity.group1().xyz().with_w(0.0),
            // e235, e315, e125, e5
            versor_even_at_infinity.group2(),
        ))
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for VersorEvenAligningOrigin {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into VersorEvenAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            versor_even_orthogonal_origin.group0().xyz().with_w(0.0),
            // e415, e425, e435, e4
            Simd32x3::from(0.0).with_w(versor_even_orthogonal_origin[e4]),
            // e235, e315, e125, e5
            versor_even_orthogonal_origin.group1(),
        ))
    }
}
