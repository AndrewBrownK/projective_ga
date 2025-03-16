// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 23
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       0       0
//  Average:         4       0       0
//  Maximum:        13       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5       0       0
//  Average:         4       0       0
//  Maximum:        14       0       0
impl RoundNormSquared for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
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
impl RoundNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
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
impl RoundNormSquared for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12] + self[scalar] * self[scalar],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
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
impl RoundNormSquared for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12], self[e42] * self[e42] + self[e43] * self[e43]])
                + Simd32x2::powi(self.group0().wx(), 2),
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
impl RoundNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3],
                self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412] + self[e4] * self[e4],
            ]),
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
impl RoundNormSquared for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e2] * self[e2] + self[e3] * self[e3], self[e431] * self[e431] + self[e412] * self[e412]])
                + Simd32x2::powi(self.group1().yx(), 2)
                + Simd32x2::powi(self.group0().wx(), 2),
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
impl RoundNormSquared for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321] * self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
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
impl RoundNormSquared for AntiDualNum {
    fn round_norm_squared(self) -> MultiVector {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::powi(self.group0().yx(), 2),
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
impl RoundNormSquared for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3], self[e4] * self[e4]]),
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
impl RoundNormSquared for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e31] * self[e31] + self[e12] * self[e12], self[e42] * self[e42] + self[e43] * self[e43]])
                + Simd32x2::powi(self.group0().wx(), 2)
                + Simd32x2::powi(self.group1().xw(), 2),
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
impl RoundNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321] * self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
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
impl RoundNormSquared for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321] * self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
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
impl RoundNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e321] * self[e321], self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]),
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
impl RoundNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
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
impl RoundNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234],
            ]),
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
impl RoundNormSquared for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234],
            ]),
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
impl RoundNormSquared for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43],
            ]),
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
impl RoundNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd       13        0        0
    //  no simd       14        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * self[scalar]
                    + self[e2] * self[e2]
                    + self[e3] * self[e3]
                    + self[e23] * self[e23]
                    + self[e31] * self[e31]
                    + self[e12] * self[e12]
                    + self[e321] * self[e321],
                self[e41] * self[e41]
                    + self[e42] * self[e42]
                    + self[e43] * self[e43]
                    + self[e423] * self[e423]
                    + self[e431] * self[e431]
                    + self[e412] * self[e412]
                    + self[e1234] * self[e1234],
            ]) + Simd32x2::powi(self.group1().xw(), 2),
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
impl RoundNormSquared for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3], self[e4] * self[e4]]),
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
impl RoundNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e321] * self[e321] + self[e2] * self[e2] + self[e3] * self[e3],
                self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412],
            ]) + Simd32x2::powi(self.group3().xw(), 2),
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
impl RoundNormSquared for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e2] * self[e2] + self[e3] * self[e3], self[e431] * self[e431] + self[e412] * self[e412]])
                + Simd32x2::powi(self.group0().wx(), 2)
                + Simd32x2::powi(self.group2().xw(), 2),
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
impl RoundNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234],
            ]) + Simd32x2::powi(self.group0().wx(), 2),
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
impl RoundNormSquared for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd2        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        6        0        0
    fn round_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12],
                self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234],
            ]) + Simd32x2::powi(self.group0().wx(), 2),
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
