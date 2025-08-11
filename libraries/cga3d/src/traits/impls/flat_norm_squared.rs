// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 16
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         5       7       0     N/A
//  Average:         4       6       0     N/A
//  Maximum:        14      16       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         5       7       0       0
//  Average:         4       7       0       0
//  Maximum:        14      16       0       0
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiCircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        4        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group2().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], self[e45] * self[e45]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for AntiDipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        let sub_type_g0 = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
                sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Circle {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for CircleRotor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       10        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        let sub_type_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Dipole {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        4        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-self[e15] * self[e15] - self[e25] * self[e25] - self[e35] * self[e35], self[e45] * self[e45]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DipoleInversion {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        let sub_type_g1_xyz = self.group3().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for DualNum {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn flat_norm_squared(self) -> MultiVector {
        MultiVector::from_groups(
            // scalar, e12345
            self.group0() * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for FlatPoint {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        4        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0 = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], self[e45] * self[e45]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Flector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group0().xyz();
        let sub_type_g1_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Line {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        6        0        0
    fn flat_norm_squared(self) -> MultiVector {
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Motor {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       11        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g1_xyz = self.group1().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
                self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl std::ops::DivAssign<FlatNormSquaredPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: FlatNormSquaredPrefixOrPostfix) {
        *self = self.flat_norm_squared()
    }
}
impl FlatNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       14       16        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g6_xyz = self.group3().xyz();
        let sub_type_g6_xyz = self.group6().xyz();
        let sub_type_g9_xyz = self.group9().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e235] * self[e235] + self[e315] * self[e315] + self[e125] * self[e125] + self[e5] * self[e5]
                    - wedge_g6_xyz[0] * wedge_g6_xyz[0]
                    - wedge_g6_xyz[1] * wedge_g6_xyz[1]
                    - wedge_g6_xyz[2] * wedge_g6_xyz[2]
                    - self[e3215] * self[e3215],
                self[e12345] * self[e12345]
                    + sub_type_g6_xyz[0] * sub_type_g6_xyz[0]
                    + sub_type_g6_xyz[1] * sub_type_g6_xyz[1]
                    + sub_type_g6_xyz[2] * sub_type_g6_xyz[2]
                    + sub_type_g9_xyz[0] * sub_type_g9_xyz[0]
                    + sub_type_g9_xyz[1] * sub_type_g9_xyz[1]
                    + sub_type_g9_xyz[2] * sub_type_g9_xyz[2]
                    + self[e45] * self[e45],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Plane {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        5        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e3215] * self[e3215] * -1.0,
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for Sphere {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        5        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e3215] * self[e3215] * -1.0,
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorEven {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       11        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g1_xyz = self.group2().xyz() * Simd32x3::from(-1.0);
        let sub_type_g0_xyz = self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[e5] * self[e5] - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2],
                sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[e12345] * self[e12345],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl std::ops::Div<FlatNormSquaredPrefixOrPostfix> for VersorOdd {
    type Output = MultiVector;
    fn div(self, _rhs: FlatNormSquaredPrefixOrPostfix) -> Self::Output {
        self.flat_norm_squared()
    }
}
impl FlatNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        7        0        0
    fn flat_norm_squared(self) -> MultiVector {
        use crate::elements::*;
        let wedge_g0_xyz = self.group2().xyz();
        let sub_type_g1_xyz = self.group3().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2],
                sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + self[e45] * self[e45],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
