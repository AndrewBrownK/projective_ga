// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 94
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

impl From<AntiCircleOnOrigin> for MultiVector {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_on_origin.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_on_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiCircleRotor> for MultiVector {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_anti_circle_rotor[e41], from_anti_circle_rotor[e42], from_anti_circle_rotor[e43], from_anti_circle_rotor[e45]]),
            // e15, e25, e35
            from_anti_circle_rotor.group2().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiCircleRotorAligningOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_rotor_aligning_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_anti_circle_rotor_aligning_origin.group2().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_circle_rotor_aligning_origin_at_infinity.group1().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin_at_infinity.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiCircleRotorAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_anti_circle_rotor_at_infinity[e45]),
            // e15, e25, e35
            from_anti_circle_rotor_at_infinity.group1().xyz(),
            // e23, e31, e12
            from_anti_circle_rotor_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiCircleRotorOnOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_circle_rotor_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_rotor_on_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDipoleInversion> for MultiVector {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
            // e5
            from_anti_dipole_inversion[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e423, e431, e412
            from_anti_dipole_inversion.group0(),
            // e235, e315, e125
            from_anti_dipole_inversion.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDipoleInversionAtInfinity> for MultiVector {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_at_infinity.group2().xyz().with_w(0.0),
            // e5
            from_anti_dipole_inversion_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_dipole_inversion_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDipoleInversionOnOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_dipole_inversion_on_origin.group1().yzwx(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_on_origin[e321]),
            // e423, e431, e412
            from_anti_dipole_inversion_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_anti_dipole_inversion_orthogonal_origin[e4]),
            // e5
            from_anti_dipole_inversion_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_orthogonal_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_anti_dipole_inversion_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_anti_dipole_inversion_orthogonal_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDipoleOnOrigin> for MultiVector {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_dipole_on_origin[e321]),
            // e423, e431, e412
            from_anti_dipole_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiDualNum> for MultiVector {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_dual_num[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_dual_num[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        )
    }
}

impl From<AntiFlatOrigin> for MultiVector {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_origin[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiFlatPoint> for MultiVector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flat_point[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flat_point.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_flector.group1().xyz().with_w(0.0),
            // e5
            from_anti_flector[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_flector.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiFlectorOnOrigin> for MultiVector {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_anti_flector_on_origin[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiLine> for MultiVector {
    fn from(from_anti_line: AntiLine) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_line.group1(),
            // e23, e31, e12
            from_anti_line.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiLineOnOrigin> for MultiVector {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line_on_origin.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiMotor> for MultiVector {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_motor.group1().xyz(),
            // e23, e31, e12
            from_anti_motor.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_anti_motor[e3215],
        )
    }
}

impl From<AntiMotorOnOrigin> for MultiVector {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_motor_on_origin.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiMysteryCircleRotor> for MultiVector {
    fn from(from_anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_mystery_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_anti_mystery_circle_rotor[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_mystery_circle_rotor.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiMysteryDipoleInversion> for MultiVector {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_mystery_dipole_inversion.group1().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiPlane> for MultiVector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane.group0().xyz().with_w(0.0),
            // e5
            from_anti_plane[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiPlaneOnOrigin> for MultiVector {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_plane_on_origin.group0().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_anti_scalar[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiSphereOnOrigin> for MultiVector {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<AntiVersorEvenOnOrigin> for MultiVector {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_versor_even_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_anti_versor_even_on_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_versor_even_on_origin.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_versor_even_on_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        )
    }
}

impl From<Circle> for MultiVector {
    fn from(from_circle: Circle) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle.group1(),
            // e423, e431, e412
            from_circle.group0(),
            // e235, e315, e125
            from_circle.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleAligningOrigin> for MultiVector {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_aligning_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e235, e315, e125
            from_circle_aligning_origin.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleAtInfinity> for MultiVector {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleAtOrigin> for MultiVector {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e235, e315, e125
            from_circle_at_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleOnOrigin> for MultiVector {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_on_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleOrthogonalOrigin> for MultiVector {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_circle_orthogonal_origin[e321]),
            // e423, e431, e412
            from_circle_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_circle_orthogonal_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor.group1(),
            // e423, e431, e412
            from_circle_rotor.group0(),
            // e235, e315, e125
            from_circle_rotor.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleRotorAligningOrigin> for MultiVector {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_rotor_aligning_origin.group0(),
            // e235, e315, e125
            from_circle_rotor_aligning_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_aligning_origin_at_infinity.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_rotor_aligning_origin_at_infinity.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleRotorAtInfinity> for MultiVector {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_rotor_at_infinity.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<CircleRotorOnOrigin> for MultiVector {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_circle_rotor_on_origin.group1().with_w(0.0),
            // e423, e431, e412
            from_circle_rotor_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Dipole> for MultiVector {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole[e41], from_dipole[e42], from_dipole[e43], from_dipole[e45]]),
            // e15, e25, e35
            from_dipole.group2(),
            // e23, e31, e12
            from_dipole.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleAligningOrigin> for MultiVector {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_aligning_origin.group0(),
            // e15, e25, e35
            from_dipole_aligning_origin.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleAtInfinity> for MultiVector {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_dipole_at_infinity[e45]),
            // e15, e25, e35
            from_dipole_at_infinity.group1(),
            // e23, e31, e12
            from_dipole_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleAtOrigin> for MultiVector {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_at_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_dipole_at_origin.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleInversion> for MultiVector {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole_inversion[e41], from_dipole_inversion[e42], from_dipole_inversion[e43], from_dipole_inversion[e45]]),
            // e15, e25, e35
            from_dipole_inversion.group2().xyz(),
            // e23, e31, e12
            from_dipole_inversion.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion[e1234], from_dipole_inversion[e4235], from_dipole_inversion[e4315], from_dipole_inversion[e4125]]),
            // e3215
            from_dipole_inversion[e3215],
        )
    }
}

impl From<DipoleInversionAligningOrigin> for MultiVector {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_aligning_origin.group0(),
            // e15, e25, e35
            from_dipole_inversion_aligning_origin.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                from_dipole_inversion_aligning_origin[e1234],
                from_dipole_inversion_aligning_origin[e4235],
                from_dipole_inversion_aligning_origin[e4315],
                from_dipole_inversion_aligning_origin[e4125],
            ]),
            // e3215
            from_dipole_inversion_aligning_origin[e3215],
        )
    }
}

impl From<DipoleInversionAtInfinity> for MultiVector {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_dipole_inversion_at_infinity[e45]),
            // e15, e25, e35
            from_dipole_inversion_at_infinity.group1(),
            // e23, e31, e12
            from_dipole_inversion_at_infinity.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                0.0,
                from_dipole_inversion_at_infinity[e4235],
                from_dipole_inversion_at_infinity[e4315],
                from_dipole_inversion_at_infinity[e4125],
            ]),
            // e3215
            from_dipole_inversion_at_infinity[e3215],
        )
    }
}

impl From<DipoleInversionAtOrigin> for MultiVector {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_dipole_inversion_at_origin.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_at_origin[e3215],
        )
    }
}

impl From<DipoleInversionOnOrigin> for MultiVector {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_on_origin.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            from_dipole_inversion_on_origin.group1(),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_orthogonal_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_dipole_inversion_orthogonal_origin.group2().xyz(),
            // e23, e31, e12
            from_dipole_inversion_orthogonal_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_orthogonal_origin[e3215],
        )
    }
}

impl From<DipoleOnOrigin> for MultiVector {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_on_origin.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DipoleOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_orthogonal_origin.group0().with_w(0.0),
            // e15, e25, e35
            from_dipole_orthogonal_origin.group2(),
            // e23, e31, e12
            from_dipole_orthogonal_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_dual_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_dual_num[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<FlatOrigin> for MultiVector {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flat_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<FlatPoint> for MultiVector {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flat_point[e45]),
            // e15, e25, e35
            from_flat_point.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<FlatPointAtInfinity> for MultiVector {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flat_point_at_infinity.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flector[e45]),
            // e15, e25, e35
            from_flector.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_flector[e4235], from_flector[e4315], from_flector[e4125]]),
            // e3215
            from_flector[e3215],
        )
    }
}

impl From<FlectorAtInfinity> for MultiVector {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flector_at_infinity.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_flector_at_infinity[e3215],
        )
    }
}

impl From<FlectorOnOrigin> for MultiVector {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_flector_on_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125]]),
            // e3215
            0.0,
        )
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_horizon[e3215],
        )
    }
}

impl From<Infinity> for MultiVector {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<LineAtInfinity> for MultiVector {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line_at_infinity.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<LineOnOrigin> for MultiVector {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_line_on_origin.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_motor.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MotorAtInfinity> for MultiVector {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_motor_at_infinity.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MotorOnOrigin> for MultiVector {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_motor_on_origin.group0().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryCircle> for MultiVector {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryCircleRotor> for MultiVector {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_circle_rotor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_circle_rotor.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryDipole> for MultiVector {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_dipole[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_dipole.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryDipoleInversion> for MultiVector {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_dipole_inversion[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_dipole_inversion.group0().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_dipole_inversion[e4235], from_mystery_dipole_inversion[e4315], from_mystery_dipole_inversion[e4125]]),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryVersorEven> for MultiVector {
    fn from(from_mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_mystery_versor_even[e1], from_mystery_versor_even[e2], from_mystery_versor_even[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_versor_even.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<MysteryVersorOdd> for MultiVector {
    fn from(from_mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_mystery_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_mystery_versor_odd[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_mystery_versor_odd.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_versor_odd[e4235], from_mystery_versor_odd[e4315], from_mystery_versor_odd[e4125]]),
            // e3215
            0.0,
        )
    }
}

impl From<NullCircleAtOrigin> for MultiVector {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<NullDipoleAtOrigin> for MultiVector {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_null_dipole_at_origin.group0().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<NullDipoleInversionAtOrigin> for MultiVector {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_null_dipole_inversion_at_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_null_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        )
    }
}

impl From<NullSphereAtOrigin> for MultiVector {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_null_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        )
    }
}

impl From<NullVersorEvenAtOrigin> for MultiVector {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_null_versor_even_at_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_null_versor_even_at_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane[e4235], from_plane[e4315], from_plane[e4125]]),
            // e3215
            from_plane[e3215],
        )
    }
}

impl From<PlaneOnOrigin> for MultiVector {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane_on_origin[e4235], from_plane_on_origin[e4315], from_plane_on_origin[e4125]]),
            // e3215
            0.0,
        )
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_round_point.group0(),
            // e5
            from_round_point[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<RoundPointAtOrigin> for MultiVector {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_round_point_at_origin[e4]),
            // e5
            from_round_point_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<Sphere> for MultiVector {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere[e1234], from_sphere[e4235], from_sphere[e4315], from_sphere[e4125]]),
            // e3215
            from_sphere[e3215],
        )
    }
}

impl From<SphereAtOrigin> for MultiVector {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_sphere_at_origin[e3215],
        )
    }
}

impl From<SphereOnOrigin> for MultiVector {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            from_sphere_on_origin.group0().wxyz(),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEven> for MultiVector {
    fn from(from_versor_even: VersorEven) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even[e12345]]),
            // e1, e2, e3, e4
            from_versor_even.group3(),
            // e5
            from_versor_even[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even.group1(),
            // e423, e431, e412
            from_versor_even.group0().xyz(),
            // e235, e315, e125
            from_versor_even.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEvenAligningOrigin> for MultiVector {
    fn from(from_versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_aligning_origin[e4]),
            // e5
            from_versor_even_aligning_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_aligning_origin.group1().xyz().with_w(0.0),
            // e423, e431, e412
            from_versor_even_aligning_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_aligning_origin.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEvenAtInfinity> for MultiVector {
    fn from(from_versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_versor_even_at_infinity[e1], from_versor_even_at_infinity[e2], from_versor_even_at_infinity[e3], 0.0]),
            // e5
            from_versor_even_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_at_infinity.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_versor_even_at_infinity.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEvenAtOrigin> for MultiVector {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_at_origin[e4]),
            // e5
            from_versor_even_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_versor_even_at_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_at_origin.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEvenOnOrigin> for MultiVector {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(from_versor_even_on_origin[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_on_origin.group1().xyz().with_w(0.0),
            // e423, e431, e412
            from_versor_even_on_origin.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorEvenOrthogonalOrigin> for MultiVector {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_versor_even_orthogonal_origin.group2(),
            // e5
            from_versor_even_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(from_versor_even_orthogonal_origin[e321]),
            // e423, e431, e412
            from_versor_even_orthogonal_origin.group0().xyz(),
            // e235, e315, e125
            from_versor_even_orthogonal_origin.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(from_versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_versor_odd[e41], from_versor_odd[e42], from_versor_odd[e43], from_versor_odd[e45]]),
            // e15, e25, e35
            from_versor_odd.group2().xyz(),
            // e23, e31, e12
            from_versor_odd.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd[e1234], from_versor_odd[e4235], from_versor_odd[e4315], from_versor_odd[e4125]]),
            // e3215
            from_versor_odd[e3215],
        )
    }
}

impl From<VersorOddAtInfinity> for MultiVector {
    fn from(from_versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(from_versor_odd_at_infinity[e45]),
            // e15, e25, e35
            Simd32x3::from([from_versor_odd_at_infinity[e15], from_versor_odd_at_infinity[e25], from_versor_odd_at_infinity[e35]]),
            // e23, e31, e12
            from_versor_odd_at_infinity.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_versor_odd_at_infinity[e4235], from_versor_odd_at_infinity[e4315], from_versor_odd_at_infinity[e4125]]),
            // e3215
            from_versor_odd_at_infinity[e3215],
        )
    }
}

impl From<VersorOddOrthogonalOrigin> for MultiVector {
    fn from(from_versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_orthogonal_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_versor_odd_orthogonal_origin.group0().xyz().with_w(0.0),
            // e15, e25, e35
            from_versor_odd_orthogonal_origin.group2().xyz(),
            // e23, e31, e12
            from_versor_odd_orthogonal_origin.group1().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_versor_odd_orthogonal_origin[e3215],
        )
    }
}
