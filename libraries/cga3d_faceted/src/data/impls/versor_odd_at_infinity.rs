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

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().wxyz(),
            // e23, e31, e12, e45
            from_anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn from(from_anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            from_anti_circle_rotor_at_infinity.group1().wxyz(),
            // e23, e31, e12, e45
            from_anti_circle_rotor_at_infinity.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLine> for VersorOddAtInfinity {
    fn from(from_anti_line: AntiLine) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_anti_line[e15], from_anti_line[e25], from_anti_line[e35]]),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLineOnOrigin> for VersorOddAtInfinity {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_anti_line_on_origin.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMotor> for VersorOddAtInfinity {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([from_anti_motor[scalar], from_anti_motor[e15], from_anti_motor[e25], from_anti_motor[e35]]),
            // e23, e31, e12, e45
            from_anti_motor.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_motor[e3215]),
        )
    }
}

impl From<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([from_anti_motor_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            from_anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn from(from_anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([from_anti_mystery_circle_rotor[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            from_anti_mystery_circle_rotor.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAtInfinity> for VersorOddAtInfinity {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_dipole_at_infinity[e15], from_dipole_at_infinity[e25], from_dipole_at_infinity[e35]]),
            // e23, e31, e12, e45
            from_dipole_at_infinity.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                0.0,
                from_dipole_inversion_at_infinity[e15],
                from_dipole_inversion_at_infinity[e25],
                from_dipole_inversion_at_infinity[e35],
            ]),
            // e23, e31, e12, e45
            from_dipole_inversion_at_infinity.group0(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_at_infinity.group2(),
        )
    }
}

impl From<FlatOrigin> for VersorOddAtInfinity {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPoint> for VersorOddAtInfinity {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_flat_point[e15], from_flat_point[e25], from_flat_point[e35]]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPointAtInfinity> for VersorOddAtInfinity {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_flat_point_at_infinity[e15], from_flat_point_at_infinity[e25], from_flat_point_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for VersorOddAtInfinity {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_flector[e15], from_flector[e25], from_flector[e35]]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        )
    }
}

impl From<FlectorAtInfinity> for VersorOddAtInfinity {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, from_flector_at_infinity[e15], from_flector_at_infinity[e25], from_flector_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
        )
    }
}

impl From<FlectorOnOrigin> for VersorOddAtInfinity {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e4235, e4315, e4125, e3215
            from_flector_on_origin.group0().yzw().with_w(0.0),
        )
    }
}

impl From<Horizon> for VersorOddAtInfinity {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
        )
    }
}

impl From<MysteryDipole> for VersorOddAtInfinity {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryDipoleInversion> for VersorOddAtInfinity {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole_inversion.group0(),
            // e4235, e4315, e4125, e3215
            from_mystery_dipole_inversion.group1().with_w(0.0),
        )
    }
}

impl From<MysteryVersorOdd> for VersorOddAtInfinity {
    fn from(from_mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([from_mystery_versor_odd[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            from_mystery_versor_odd.group1(),
            // e4235, e4315, e4125, e3215
            from_mystery_versor_odd.group0().yzw().with_w(0.0),
        )
    }
}

impl From<Plane> for VersorOddAtInfinity {
    fn from(from_plane: Plane) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        )
    }
}

impl From<PlaneOnOrigin> for VersorOddAtInfinity {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<Scalar> for VersorOddAtInfinity {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([from_scalar[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl TryFrom<AntiCircleOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            anti_circle_on_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotor> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            anti_circle_rotor.group2().wxyz(),
            // e23, e31, e12, e45
            anti_circle_rotor.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            anti_circle_rotor_aligning_origin.group2().wxyz(),
            // e23, e31, e12, e45
            anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_circle_rotor_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiDualNum> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_dual_num[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_versor_even_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<Dipole> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole[e15], dipole[e25], dipole[e35]]),
            // e23, e31, e12, e45
            dipole.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DipoleAligningOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(dipole_aligning_origin[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DipoleAtOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DipoleInversion> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35]]),
            // e23, e31, e12, e45
            dipole_inversion.group1(),
            // e4235, e4315, e4125, e3215
            dipole_inversion.group3(),
        ))
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion_aligning_origin[e15], dipole_inversion_aligning_origin[e25], dipole_inversion_aligning_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(dipole_inversion_aligning_origin[e45]),
            // e4235, e4315, e4125, e3215
            dipole_inversion_aligning_origin.group2(),
        ))
    }
}

impl TryFrom<DipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion_at_origin[e15], dipole_inversion_at_origin[e25], dipole_inversion_at_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(dipole_inversion_at_origin[e3215]),
        ))
    }
}

impl TryFrom<DipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(dipole_inversion_on_origin[e45]),
            // e4235, e4315, e4125, e3215
            dipole_inversion_on_origin.group1().yzw().with_w(0.0),
        ))
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                0.0,
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
            ]),
            // e23, e31, e12, e45
            dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(dipole_inversion_orthogonal_origin[e3215]),
        ))
    }
}

impl TryFrom<DipoleOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(dipole_on_origin[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35]]),
            // e23, e31, e12, e45
            dipole_orthogonal_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ))
    }
}

impl TryFrom<MultiVector> for VersorOddAtInfinity {
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([multi_vector[scalar], multi_vector[e15], multi_vector[e25], multi_vector[e35]]),
            // e23, e31, e12, e45
            multi_vector.group5().with_w(multi_vector[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ))
    }
}

impl TryFrom<Sphere> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            sphere.group0(),
        ))
    }
}

impl TryFrom<SphereAtOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(sphere_at_origin: SphereAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(sphere_at_origin[e3215]),
        ))
    }
}

impl TryFrom<SphereOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            sphere_on_origin.group0().xyz().with_w(0.0),
        ))
    }
}

impl TryFrom<VersorOdd> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([versor_odd[scalar], versor_odd[e15], versor_odd[e25], versor_odd[e35]]),
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ))
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                versor_odd_orthogonal_origin[scalar],
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
            ]),
            // e23, e31, e12, e45
            versor_odd_orthogonal_origin.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(versor_odd_orthogonal_origin[e3215]),
        ))
    }
}
