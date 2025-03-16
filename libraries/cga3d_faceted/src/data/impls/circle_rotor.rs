// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 40
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

impl From<AntiDipoleOnOrigin> for CircleRotor {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            from_anti_dipole_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatOrigin> for CircleRotor {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiFlatPoint> for CircleRotor {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e235, e315, e125, e12345
            from_anti_flat_point.group0().xyz().with_w(0.0),
        )
    }
}

impl From<AntiScalar> for CircleRotor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(from_anti_scalar[e12345]),
        )
    }
}

impl From<Circle> for CircleRotor {
    fn from(from_circle: Circle) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle.group0(),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e235, e315, e125, e12345
            from_circle.group2().with_w(0.0),
        )
    }
}

impl From<CircleAligningOrigin> for CircleRotor {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e12345
            from_circle_aligning_origin.group2().with_w(0.0),
        )
    }
}

impl From<CircleAtInfinity> for CircleRotor {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e235, e315, e125, e12345
            from_circle_at_infinity.group1().with_w(0.0),
        )
    }
}

impl From<CircleAtOrigin> for CircleRotor {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            from_circle_at_origin.group1().with_w(0.0),
        )
    }
}

impl From<CircleOnOrigin> for CircleRotor {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl From<CircleOrthogonalOrigin> for CircleRotor {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e235, e315, e125, e12345
            from_circle_orthogonal_origin.group1().with_w(0.0),
        )
    }
}

impl From<CircleRotorAligningOrigin> for CircleRotor {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_rotor_aligning_origin.group0(),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e235, e315, e125, e12345
            from_circle_rotor_aligning_origin.group2(),
        )
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e235, e315, e125, e12345
            from_circle_rotor_aligning_origin_at_infinity.group1(),
        )
    }
}

impl From<CircleRotorAtInfinity> for CircleRotor {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_at_infinity.group0(),
            // e235, e315, e125, e12345
            from_circle_rotor_at_infinity.group1(),
        )
    }
}

impl From<CircleRotorOnOrigin> for CircleRotor {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            from_circle_rotor_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(from_circle_rotor_on_origin[e12345]),
        )
    }
}

impl From<Line> for CircleRotor {
    fn from(from_line: Line) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e235, e315, e125, e12345
            from_line.group1().with_w(0.0),
        )
    }
}

impl From<LineAtInfinity> for CircleRotor {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            from_line_at_infinity.group0().with_w(0.0),
        )
    }
}

impl From<LineOnOrigin> for CircleRotor {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl From<MotorOnOrigin> for CircleRotor {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(from_motor_on_origin[e12345]),
        )
    }
}

impl From<MysteryCircle> for CircleRotor {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryCircleRotor> for CircleRotor {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle_rotor.group0(),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(from_mystery_circle_rotor[e12345]),
        )
    }
}

impl From<NullCircleAtOrigin> for CircleRotor {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}

impl TryFrom<AntiDipoleInversion> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
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
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            anti_dipole_inversion.group0(),
            // e415, e425, e435, e321
            anti_dipole_inversion.group1(),
            // e235, e315, e125, e12345
            anti_dipole_inversion.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = anti_dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            anti_dipole_inversion_at_infinity.group0(),
            // e235, e315, e125, e12345
            anti_dipole_inversion_at_infinity.group1().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
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
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            anti_dipole_inversion_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(anti_dipole_inversion_on_origin[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            anti_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e235, e315, e125, e12345
            anti_dipole_inversion_orthogonal_origin.group2().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiFlector> for CircleRotor {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(anti_flector[e321]),
            // e235, e315, e125, e12345
            anti_flector.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<AntiFlectorOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlectorOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(anti_flector_on_origin[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for CircleRotor {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            anti_mystery_dipole_inversion.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DualNum> for CircleRotor {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(dual_num[e12345]),
        ))
    }
}

impl TryFrom<Motor> for CircleRotor {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            motor.group0().xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e12345]]),
        ))
    }
}

impl TryFrom<MotorAtInfinity> for CircleRotor {
    type Error = String;
    fn try_from(motor_at_infinity: MotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            motor_at_infinity.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for CircleRotor {
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
            let mut error = "Elements from MultiVector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            multi_vector.group7(),
            // e415, e425, e435, e321
            multi_vector.group6(),
            // e235, e315, e125, e12345
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e12345]]),
        ))
    }
}

impl TryFrom<MysteryVersorEven> for CircleRotor {
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
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            mystery_versor_even.group1(),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(mystery_versor_even[e12345]),
        ))
    }
}

impl TryFrom<NullVersorEvenAtOrigin> for CircleRotor {
    type Error = String;
    fn try_from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullVersorEvenAtOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            null_versor_even_at_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<VersorEven> for CircleRotor {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            versor_even.group0().xyz(),
            // e415, e425, e435, e321
            versor_even.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e12345]]),
        ))
    }
}

impl TryFrom<VersorEvenAligningOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            versor_even_aligning_origin.group0().xyz(),
            // e415, e425, e435, e321
            versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e12345],
            ]),
        ))
    }
}

impl TryFrom<VersorEvenAtInfinity> for CircleRotor {
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
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            versor_even_at_infinity.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_at_infinity[e235],
                versor_even_at_infinity[e315],
                versor_even_at_infinity[e125],
                versor_even_at_infinity[e12345],
            ]),
        ))
    }
}

impl TryFrom<VersorEvenAtOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            versor_even_at_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            versor_even_at_origin.group1().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<VersorEvenOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            versor_even_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            versor_even_on_origin.group1().xyz().with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(versor_even_on_origin[e12345]),
        ))
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(CircleRotor::from_groups(
            // e423, e431, e412
            versor_even_orthogonal_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(versor_even_orthogonal_origin[e321]),
            // e235, e315, e125, e12345
            versor_even_orthogonal_origin.group1().xyz().with_w(0.0),
        ))
    }
}
