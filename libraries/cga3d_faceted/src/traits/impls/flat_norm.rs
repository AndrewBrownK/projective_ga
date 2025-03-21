// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 30
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         2       3       0     N/A
//   Median:         5       8       0     N/A
//  Average:         4       6       0     N/A
//  Maximum:        14      16       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         2       3       0       0
//   Median:         5       8       0       0
//  Average:         4       7       0       0
//  Maximum:        14      16       0       0
impl FlatNorm for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], self[e45]]),
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
impl FlatNorm for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], self[e45]]),
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
impl FlatNorm for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
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
impl FlatNorm for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
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
impl FlatNorm for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e5] * self[e5] + self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
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
impl FlatNorm for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
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
impl FlatNorm for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
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
impl FlatNorm for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
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
impl FlatNorm for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        let sub_type_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2],
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
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
impl FlatNorm for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
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
impl FlatNorm for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
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
impl FlatNorm for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group1().xyz() * Simd32x3::from(-1.0);
        let sub_type_g0_xyz = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2],
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
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
impl FlatNorm for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35], self[e45]]),
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
impl FlatNorm for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35], self[e45]]),
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
impl FlatNorm for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35], self[e45]]),
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
impl FlatNorm for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
impl FlatNorm for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
impl FlatNorm for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
impl FlatNorm for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], self[e45]]),
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
impl FlatNorm for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
impl FlatNorm for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
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
impl FlatNorm for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       12        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group1().wxyz() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
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
impl FlatNorm for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       14       16        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g6_xyz = self.group6().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e5] * self[e5] + self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125]
                    - self[e15] * self[e15]
                    - self[e25] * self[e25]
                    - self[e35] * self[e35]
                    - self[e3215] * self[e3215],
                sub_type_g6_xyz[0] * sub_type_g6_xyz[0]
                    + sub_type_g6_xyz[1] * sub_type_g6_xyz[1]
                    + sub_type_g6_xyz[2] * sub_type_g6_xyz[2]
                    + self[e12345] * self[e12345]
                    + self[e45] * self[e45]
                    + self[e4235] * self[e4235]
                    + self[e4315] * self[e4315]
                    + self[e4125] * self[e4125],
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
impl FlatNorm for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        4        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e3215] * -1.0, sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]]),
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
impl FlatNorm for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        4        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[e3215] * -1.0, sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]]),
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
impl FlatNorm for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       12        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
                self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
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
impl FlatNorm for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       12        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
                self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435],
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
impl FlatNorm for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       12        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().wxyz() * Simd32x4::from(-1.0);
        let sub_type_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3],
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
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
impl FlatNorm for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
impl FlatNorm for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        8        0        0
    fn flat_norm(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().yzw();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2] - self[e3215] * self[e3215],
                self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125],
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
