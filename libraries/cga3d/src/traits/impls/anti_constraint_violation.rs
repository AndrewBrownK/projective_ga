// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         4      12       0     N/A
//  Average:        11      25       0     N/A
//  Maximum:        82     176       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         8      24       0       0
//  Average:        18      40       0       0
//  Maximum:       140     295       0       0
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy()))
            .with_w(0.0),
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversion {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       20        0        0
    //    simd2        2        6        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       15       34        0      N/A
    //  no simd       28       56        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 2.0 * (self[e415] * self[e2]) - 2.0 * (self[e425] * self[e1]) - 2.0 * (self[e435] * self[e321]), 0.0])
                + (Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz())
                    + Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy())
                    + (Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx())
                        - Simd32x2::from(2.0) * (Simd32x2::from(self[e415]) * Simd32x2::from([self[e321], self[e3]]))
                        - Simd32x2::from(2.0) * (Simd32x2::from([self[e2], self[e321]]) * self.group1().zy()))
                    .with_z(0.0)
                    - Simd32x3::from(2.0) * (Simd32x3::from([self[e315], self[e5], self[e5]]) * self.group0().zyz())
                    - Simd32x3::from(2.0) * (Simd32x3::from([self[e5], self[e125], self[e235]]) * self.group0().xxy()))
                .with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235])
                + 2.0 * (self[e425] * self[e315])
                + 2.0 * (self[e435] * self[e125])
                + 2.0 * (self[e321] * self[e5])
                + 2.0 * (self[e235] * self[e1])
                + 2.0 * (self[e315] * self[e2])
                + 2.0 * (self[e125] * self[e3]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * self[scalar] * -2.0, 0.0]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiFlector {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) + 2.0 * (self[e125] * self[e3]) + 2.0 * (self[e321] * self[e5]),
            0.0,
        ]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiLine {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([-2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]), 0.0]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMotor {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
            0.0,
        ]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Circle {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       27        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((anti_reverse_g0.zxy() * self.group2().yzx()) + (self.group0().yzx() * self.group2().zxy())
                - (anti_reverse_g0.yzx() * self.group2().zxy())
                - (self.group0().zxy() * self.group2().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz()))
            .with_w(0.0),
            // e5
            2.0 * (self[e235] * self[e415]) + 2.0 * (self[e315] * self[e425]) + 2.0 * (self[e125] * self[e435]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotor {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       27        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((anti_reverse_g0.zxy() * self.group2().yzx()) + (self.group0().yzx() * self.group2().zxy())
                - (anti_reverse_g0.yzx() * self.group2().zxy())
                - (self.group0().zxy() * self.group2().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz()))
            .with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Dipole {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(2.0) * (self.group0().zxy() * self.group2().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy()))
            .with_w(0.0),
            // e5
            -2.0 * (self[e15] * self[e23]) - 2.0 * (self[e25] * self[e31]) - 2.0 * (self[e35] * self[e12]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversion {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       20        0        0
    //    simd2        2        6        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       15       34        0      N/A
    //  no simd       28       56        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 2.0 * (self[e23] * self[e4315]) - 2.0 * (self[e31] * self[e4235]) - 2.0 * (self[e12] * self[e45]), 0.0])
                + (Simd32x3::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group0().zyz())
                    + Simd32x3::from(2.0) * (Simd32x3::from([self[e3215], self[e35], self[e15]]) * self.group0().xxy())
                    + (Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx())
                        - Simd32x2::from(2.0) * (Simd32x2::from(self[e23]) * Simd32x2::from([self[e45], self[e4125]]))
                        - Simd32x2::from(2.0) * (Simd32x2::from([self[e4315], self[e45]]) * self.group1().zy()))
                    .with_z(0.0)
                    - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group2().xyz())
                    - Simd32x3::from(2.0) * (self.group0().yzx() * self.group2().zxy()))
                .with_w(0.0),
            // e5
            2.0 * (self[e45] * self[e3215]) + 2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125])
                - 2.0 * (self[e23] * self[e15])
                - 2.0 * (self[e31] * self[e25])
                - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5] * self[e12345] * 2.0, 0.0]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Flector {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]) + 2.0 * (self[e45] * self[e3215]),
            0.0,
        ]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Line {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
            0.0,
        ]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Motor {
    type Output = DualNum;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]) + 2.0 * (self[e12345] * self[e5]),
            0.0,
        ]))
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl std::ops::DivAssign<AntiConstraintViolationPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: AntiConstraintViolationPrefixOrPostfix) {
        *self = self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       52      113        0        0
    //    simd2        2        7        0      N/A
    //    simd3       28       56        0      N/A
    // Totals...
    // yes simd       82      176        0      N/A
    //  no simd      140      295        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e41] * self[e235])
                    + 2.0 * (self[e42] * self[e315])
                    + 2.0 * (self[e43] * self[e125])
                    + 2.0 * (self[e23] * self[e415])
                    + 2.0 * (self[e31] * self[e425])
                    + 2.0 * (self[e12] * self[e435])
                    + 2.0 * (self[e423] * self[e15])
                    + 2.0 * (self[e431] * self[e25])
                    + 2.0 * (self[e412] * self[e35])
                    + 2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e45] * self[e321])
                    + 2.0 * (self[e5] * self[e1234]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group9().xyz())
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group1().xyz())
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group8())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group4().zyz())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e3215], self[e35], self[e15]]) * self.group4().xxy())
                + Simd32x3::from(2.0) * (self.group5().yzx() * self.group9().zxy())
                + Simd32x3::from(2.0) * (self.group7().yzx() * self.group8().zxy())
                + (Simd32x2::from(2.0) * (self.group1().zx() * self.group6().yz()) - Simd32x2::from(2.0) * (self.group1().yz() * self.group6().zx()))
                    .with_z(2.0 * (self[e2] * self[e415]) - 2.0 * (self[e1] * self[e425]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group6().xyz())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group3().xyz())
                - Simd32x3::from(2.0) * (Simd32x3::from([self[e315], self[e5], self[e5]]) * self.group7().zyz())
                - Simd32x3::from(2.0) * (Simd32x3::from([self[e45], self[e4125], self[e4235]]) * self.group5().xxy())
                - Simd32x3::from(2.0) * (Simd32x3::from([self[e4315], self[e45], self[e45]]) * self.group5().zyz())
                - Simd32x3::from(2.0) * (Simd32x3::from([self[e5], self[e125], self[e235]]) * self.group7().xxy())
                - Simd32x3::from(2.0) * (self.group4().yzx() * self.group3().zxy()))
            .with_w(self[e12345] * self[e4] * 2.0),
            // e5
            2.0 * (self[e12345] * self[e5])
                + 2.0 * (self[e235] * self[e1])
                + 2.0 * (self[e235] * self[e415])
                + 2.0 * (self[e315] * self[e2])
                + 2.0 * (self[e315] * self[e425])
                + 2.0 * (self[e125] * self[e3])
                + 2.0 * (self[e125] * self[e435])
                + 2.0 * (self[e321] * self[e5])
                + 2.0 * (self[e15] * self[e4235])
                + 2.0 * (self[e25] * self[e4315])
                + 2.0 * (self[e35] * self[e4125])
                + 2.0 * (self[e45] * self[e3215])
                - 2.0 * (self[e23] * self[e15])
                - 2.0 * (self[e31] * self[e25])
                - 2.0 * (self[e12] * self[e35])
                - 2.0 * (self[scalar] * self[e3215]),
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
            (Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group9().xyz())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e315], self[e5], self[e5]]) * self.group4().zyz())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e2], self[e321], self[e321]]) * self.group5().zyz())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group7().zyz())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e321], self[e3], self[e1]]) * self.group5().xxy())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e3215], self[e35], self[e15]]) * self.group7().xxy())
                + Simd32x3::from(2.0) * (Simd32x3::from([self[e5], self[e125], self[e235]]) * self.group4().xxy())
                + ((self.group6().yz() * self.group9().zx()) - Simd32x2::from(2.0) * (self.group6().zx() * self.group9().yz()))
                    .with_z((self[e415] * self[e4315]) - 2.0 * (self[e425] * self[e4235]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group3().xyz())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group6().xyz())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group8())
                - Simd32x3::from(2.0) * (self.group4().yzx() * self.group8().zxy())
                - Simd32x3::from(2.0) * (self.group5().yzx() * self.group1().zxy())
                - Simd32x3::from(2.0) * (self.group7().yzx() * self.group3().zxy()))
            .with_w(
                2.0 * (self[scalar] * self[e5])
                    + 2.0 * (self[e12345] * self[e3215])
                    + (self[e1] * self[e15])
                    + (self[e2] * self[e25])
                    + (self[e3] * self[e35])
                    + (self[e321] * self[e3215]),
            ),
            // e1234
            2.0 * (self[scalar] * self[e4])
                + 2.0 * (self[e12345] * self[e1234])
                + 2.0 * (self[e41] * self[e415])
                + 2.0 * (self[e42] * self[e425])
                + 2.0 * (self[e43] * self[e435])
                + 2.0 * (self[e23] * self[e423])
                + 2.0 * (self[e31] * self[e431])
                + 2.0 * (self[e12] * self[e412])
                + 2.0 * (self[e423] * self[e4235])
                + 2.0 * (self[e431] * self[e4315])
                + 2.0 * (self[e412] * self[e4125])
                + 2.0 * (self[e4] * self[e45])
                - 2.0 * (self[e41] * self[e1])
                - 2.0 * (self[e42] * self[e2])
                - 2.0 * (self[e43] * self[e3])
                - 2.0 * (self[e321] * self[e1234]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEven {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       26        0        0
    //    simd2        4       10        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       19       42        0      N/A
    //  no simd       33       66        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzxw() * self.group2().zxy().with_w(self[e4]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    2.0 * (self[e415] * self[e2])
                        - 2.0 * (self[e431] * self[e235])
                        - 2.0 * (self[e412] * self[e5])
                        - 2.0 * (self[e425] * self[e1])
                        - 2.0 * (self[e435] * self[e321]),
                    0.0,
                ])
                + (Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group3().xyz())
                    + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz())
                    + (Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx())
                        - Simd32x2::from(2.0) * (Simd32x2::from(self[e423]) * self.group2().wz())
                        - Simd32x2::from(2.0) * (Simd32x2::from(self[e415]) * Simd32x2::from([self[e321], self[e3]]))
                        - Simd32x2::from(2.0) * (Simd32x2::from([self[e2], self[e321]]) * self.group1().zy())
                        - Simd32x2::from(2.0) * (self.group0().zy() * self.group2().yw()))
                    .with_z(0.0))
                .with_w(0.0),
            // e5
            2.0 * (self[e12345] * self[e5])
                + 2.0 * (self[e415] * self[e235])
                + 2.0 * (self[e425] * self[e315])
                + 2.0 * (self[e435] * self[e125])
                + 2.0 * (self[e321] * self[e5])
                + 2.0 * (self[e235] * self[e1])
                + 2.0 * (self[e315] * self[e2])
                + 2.0 * (self[e125] * self[e3]),
        )
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOdd {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       28        0        0
    //    simd2        5       12        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd       20       44        0      N/A
    //  no simd       32       64        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                2.0 * (self[e42] * self[e15]) + 2.0 * (self[e43] * self[e3215]) + 2.0 * (self[e23] * self[e4315])
                    - 2.0 * (self[e41] * self[e25])
                    - 2.0 * (self[e31] * self[e4235])
                    - 2.0 * (self[e12] * self[e45]),
                0.0,
            ]) + (Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group3().xyz())
                + (Simd32x2::from(2.0) * (Simd32x2::from(self[e41]) * Simd32x2::from([self[e3215], self[e35]]))
                    + Simd32x2::from(2.0) * (Simd32x2::from([self[e25], self[e3215]]) * self.group0().zy())
                    + Simd32x2::from(2.0) * (self.group1().yz() * self.group3().zx())
                    - Simd32x2::from(2.0) * (Simd32x2::from(self[e23]) * Simd32x2::from([self[e45], self[e4125]]))
                    - Simd32x2::from(2.0) * (Simd32x2::from([self[e4315], self[e45]]) * self.group1().zy())
                    - Simd32x2::from(2.0) * (self.group0().yz() * self.group2().zx()))
                .with_z(0.0)
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group2().xyz()))
            .with_w(0.0),
            // e5
            2.0 * (self[e45] * self[e3215]) + 2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125])
                - 2.0 * (self[scalar] * self[e3215])
                - 2.0 * (self[e23] * self[e15])
                - 2.0 * (self[e31] * self[e25])
                - 2.0 * (self[e12] * self[e35]),
        )
    }
}
