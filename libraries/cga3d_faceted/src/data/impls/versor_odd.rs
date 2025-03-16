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

impl From<AntiCircleOnOrigin> for VersorOdd {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_on_origin.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_anti_circle_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotor> for VersorOdd {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_anti_circle_rotor[e41], from_anti_circle_rotor[e42], from_anti_circle_rotor[e43], from_anti_circle_rotor[scalar]]),
            // e23, e31, e12, e45
            from_anti_circle_rotor.group1(),
            // e15, e25, e35, e1234
            from_anti_circle_rotor.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin[e41],
                from_anti_circle_rotor_aligning_origin[e42],
                from_anti_circle_rotor_aligning_origin[e43],
                from_anti_circle_rotor_aligning_origin[scalar],
            ]),
            // e23, e31, e12, e45
            from_anti_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin.group2().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_aligning_origin_at_infinity[scalar]),
            // e23, e31, e12, e45
            from_anti_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorAtInfinity> for VersorOdd {
    fn from(from_anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_at_infinity[scalar]),
            // e23, e31, e12, e45
            from_anti_circle_rotor_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_anti_circle_rotor_at_infinity.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiCircleRotorOnOrigin> for VersorOdd {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_on_origin.group0(),
            // e23, e31, e12, e45
            from_anti_circle_rotor_on_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiDualNum> for VersorOdd {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_dual_num[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_dual_num[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLine> for VersorOdd {
    fn from(from_anti_line: AntiLine) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_anti_line.group0().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_line.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiLineOnOrigin> for VersorOdd {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_anti_line_on_origin.group0().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMotor> for VersorOdd {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor[scalar]),
            // e23, e31, e12, e45
            from_anti_motor.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            from_anti_motor.group1().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_anti_motor[e3215]),
        )
    }
}

impl From<AntiMotorOnOrigin> for VersorOdd {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_motor_on_origin[scalar]),
            // e23, e31, e12, e45
            from_anti_motor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiMysteryCircleRotor> for VersorOdd {
    fn from(from_anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_anti_mystery_circle_rotor[scalar]),
            // e23, e31, e12, e45
            from_anti_mystery_circle_rotor.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<AntiVersorEvenOnOrigin> for VersorOdd {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_anti_versor_even_on_origin.group0(),
            // e23, e31, e12, e45
            from_anti_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_anti_versor_even_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Dipole> for VersorOdd {
    fn from(from_dipole: Dipole) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole.group1(),
            // e15, e25, e35, e1234
            from_dipole.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAligningOrigin> for VersorOdd {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_aligning_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAtInfinity> for VersorOdd {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_dipole_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleAtOrigin> for VersorOdd {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_at_origin.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleInversion> for VersorOdd {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion.group1(),
            // e15, e25, e35, e1234
            from_dipole_inversion.group2(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion.group3(),
        )
    }
}

impl From<DipoleInversionAligningOrigin> for VersorOdd {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_aligning_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_aligning_origin[e45]),
            // e15, e25, e35, e1234
            from_dipole_inversion_aligning_origin.group1(),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_aligning_origin.group2(),
        )
    }
}

impl From<DipoleInversionAtInfinity> for VersorOdd {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion_at_infinity.group0(),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_infinity.group1().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_dipole_inversion_at_infinity.group2(),
        )
    }
}

impl From<DipoleInversionAtOrigin> for VersorOdd {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_origin[e3215]),
        )
    }
}

impl From<DipoleInversionOnOrigin> for VersorOdd {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_on_origin.group0().xyz().with_w(0.0),
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

impl From<DipoleInversionOrthogonalOrigin> for VersorOdd {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_dipole_inversion_orthogonal_origin[e3215]),
        )
    }
}

impl From<DipoleOnOrigin> for VersorOdd {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_on_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_dipole_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<DipoleOrthogonalOrigin> for VersorOdd {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_dipole_orthogonal_origin.group0().with_w(0.0),
            // e23, e31, e12, e45
            from_dipole_orthogonal_origin.group1().with_w(0.0),
            // e15, e25, e35, e1234
            from_dipole_orthogonal_origin.group2().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatOrigin> for VersorOdd {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPoint> for VersorOdd {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35, e1234
            from_flat_point.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<FlatPointAtInfinity> for VersorOdd {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flat_point_at_infinity.group0().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Flector> for VersorOdd {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35, e1234
            from_flector.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        )
    }
}

impl From<FlectorAtInfinity> for VersorOdd {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            from_flector_at_infinity.group0().xyz().with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_flector_at_infinity[e3215]),
        )
    }
}

impl From<FlectorOnOrigin> for VersorOdd {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125], 0.0]),
        )
    }
}

impl From<Horizon> for VersorOdd {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_horizon[e3215]),
        )
    }
}

impl From<MysteryDipole> for VersorOdd {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<MysteryDipoleInversion> for VersorOdd {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            from_mystery_dipole_inversion.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_mystery_dipole_inversion.group1().with_w(0.0),
        )
    }
}

impl From<MysteryVersorOdd> for VersorOdd {
    fn from(from_mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_mystery_versor_odd[scalar]),
            // e23, e31, e12, e45
            from_mystery_versor_odd.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_mystery_versor_odd[e4235], from_mystery_versor_odd[e4315], from_mystery_versor_odd[e4125], 0.0]),
        )
    }
}

impl From<NullDipoleAtOrigin> for VersorOdd {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_at_origin.group0().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullDipoleInversionAtOrigin> for VersorOdd {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_null_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_dipole_inversion_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<NullSphereAtOrigin> for VersorOdd {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_null_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Plane> for VersorOdd {
    fn from(from_plane: Plane) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        )
    }
}

impl From<PlaneOnOrigin> for VersorOdd {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane_on_origin.group0().with_w(0.0),
        )
    }
}

impl From<Scalar> for VersorOdd {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_scalar[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}

impl From<Sphere> for VersorOdd {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere.group0(),
        )
    }
}

impl From<SphereAtOrigin> for VersorOdd {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_sphere_at_origin[e3215]),
        )
    }
}

impl From<SphereOnOrigin> for VersorOdd {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x3::from(0.0).with_w(from_sphere_on_origin[e1234]),
            // e4235, e4315, e4125, e3215
            from_sphere_on_origin.group0().xyz().with_w(0.0),
        )
    }
}

impl From<VersorOddAtInfinity> for VersorOdd {
    fn from(from_versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(from_versor_odd_at_infinity[scalar]),
            // e23, e31, e12, e45
            from_versor_odd_at_infinity.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([from_versor_odd_at_infinity[e15], from_versor_odd_at_infinity[e25], from_versor_odd_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            from_versor_odd_at_infinity.group2(),
        )
    }
}

impl From<VersorOddOrthogonalOrigin> for VersorOdd {
    fn from(from_versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            from_versor_odd_orthogonal_origin.group0(),
            // e23, e31, e12, e45
            from_versor_odd_orthogonal_origin.group1().xyz().with_w(0.0),
            // e15, e25, e35, e1234
            from_versor_odd_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(from_versor_odd_orthogonal_origin[e3215]),
        )
    }
}

impl TryFrom<MultiVector> for VersorOdd {
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
            let mut error = "Elements from MultiVector do not fit into VersorOdd { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        Ok(VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ))
    }
}
