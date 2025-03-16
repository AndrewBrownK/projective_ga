// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 15
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         6      22       0
//  Average:        17      35       0
//  Maximum:       106     195       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       3       0
//   Median:        11      30       0
//  Average:        27      46       0
//  Maximum:       160     265       0
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotor {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       18        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       15       34        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group1().xyzx() * self.group1().www().with_w(self[e41]))
                + Simd32x4::from([
                    self[e43] * self[e25] * -2.0,
                    self[e41] * self[e35] * -2.0,
                    self[e42] * self[e15] * -2.0,
                    (self[e42] * self[e31]) + (self[e43] * self[e12]),
                ])
                + (Simd32x4::from([self.group2().zxy()[0], self.group2().zxy()[1], self.group2().zxy()[2] * self[e41], self.group1().xyzz()[3] * self[e43]])
                    * self.group0().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().zxy()[0], self.group2().zxy()[1], self.group2().zxy()[2] * self[e41], self.group1().wwwy()[3] * self[e42]])
                    * self.group0().yz().with_zw(1.0, 1.0)),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
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
    //           add/sub      mul      div
    //      f32       30       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       32       50        0
    //  no simd       38       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + (self.group1().zyzw()[0] * self[e2]) + (self[e415] * self[e321]) + (self[e425] * self[e3]) + (self[e235] * self[e4])
                    - (self.group1().yzx()[0] * self.group3().zxy()[0])
                    - (self.group1().zxy()[0] * self.group3().yzx()[0])
                    - (self.group2().zxy()[0] * self[e431])
                    - (self.group2().zyz()[0] * self[e431]),
                2.0 * (self[e423] * self[e125]) + (self.group1().xxy()[1] * self[e3]) + (self[e425] * self[e321]) + (self[e435] * self[e1]) + (self[e315] * self[e4])
                    - (self.group1().yzx()[1] * self.group3().zxy()[1])
                    - (self.group1().zxy()[1] * self.group3().yzx()[1])
                    - (self.group2().xxy()[1] * self[e412])
                    - (self.group2().zxy()[1] * self[e412]),
                2.0 * (self[e431] * self[e235])
                    + (self.group1().zyzw()[2] * self[e321])
                    + (self[e412] * self[e5])
                    + (self[e415] * self[e2])
                    + (self[e435] * self[e321])
                    + (self[e125] * self[e4])
                    - (self.group1().zxy()[2] * self.group3().yzx()[2])
                    - (self.group2().xxy()[2] * self[e423])
                    - (self.group2().zxy()[2] * self[e423])
                    - (self.group2().zyz()[2] * self[e4])
                    - (self.group3().wwww()[2] * self[e412]),
                -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]) + (Simd32x4::from([self.group1().xxy()[0], self.group1().zyzw()[1], self.group1().xxy()[2] * self[e1], self.group1().zyzw()[3] * self[e4]])
                * self.group1().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group2().xxy()[0], self.group2().zyz()[1], self.group1().yzx()[2] * self.group3().zxy()[2], self[e321] * self[e4]])
                    * self.group2().ww().with_zw(1.0, 1.0)),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e3215] * self[scalar], 0.0]) * Simd32x2::from([-2.0, 0.0]))
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
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // e5, e12345
            Simd32x2::from([2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]), 0.0]),
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
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
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
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       25        0
    //  no simd       10       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321]),
                2.0 * (self[e423] * self[e125]) + 2.0 * (self[e425] * self[e321]),
                2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321]),
                -2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]) - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e423] * self[e415]),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //      add/sub      mul      div
    // f32       13       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321]) - (self.group2().zxy()[0] * self[e431]) - (self.group2().zxy()[0] * self[e431]),
                2.0 * (self[e423] * self[e125]) + 2.0 * (self[e425] * self[e321]) - (self.group2().zxy()[1] * self[e412]) - (self.group2().zxy()[1] * self[e412]),
                2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321]) - (self.group2().zxy()[2] * self[e423]) - (self.group2().zxy()[2] * self[e423]),
                -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       19        0
    //  no simd       11       30        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group1().xyzz() * self.group1().www().with_w(self[e43]))
                + Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e42] * self[e31])
                + Simd32x4::from([
                    self[e43] * self[e25] * -2.0,
                    self[e41] * self[e35] * -2.0,
                    self[e42] * self[e15] * -2.0,
                    (self.group1().wwwx()[3] * self[e41]) + (self[e41] * self[e23]),
                ]),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
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
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       35       50        0
    //  no simd       41       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1().yzx()[0] * self.group3().zxy()[0])
                    + (self.group1().zyz()[0] * self[e4315])
                    + (self.group2().xxy()[0] * self[e1234])
                    + (self.group2().zxy()[0] * self[e42])
                    + (self.group2().zyz()[0] * self[e42])
                    + (self[e23] * self[e45])
                    - (self.group2().yzx()[0] * self[e43])
                    - (self[e43] * self[e25])
                    - (self[e12] * self[e4315])
                    - (self[e15] * self[e1234]),
                (self.group1().xxy()[1] * self[e4125])
                    + (self.group1().yzx()[1] * self.group3().zxy()[1])
                    + (self.group2().xxy()[1] * self[e43])
                    + (self.group2().zxy()[1] * self[e43])
                    + (self.group2().zyz()[1] * self[e1234])
                    + (self[e31] * self[e45])
                    - (self.group2().yzx()[1] * self[e41])
                    - (self[e41] * self[e35])
                    - (self[e23] * self[e4125])
                    - (self[e25] * self[e1234]),
                (self.group1().yzx()[2] * self.group3().zxy()[2])
                    + (self.group1().zyz()[2] * self[e45])
                    + (self.group2().xxy()[2] * self[e41])
                    + (self.group2().zxy()[2] * self[e41])
                    + (self.group2().zyz()[2] * self[e1234])
                    + (self.group1().wwww()[2] * self[e12])
                    + (self.group3().wwww()[2] * self[e43])
                    - (self.group2().yzx()[2] * self[e42])
                    - (self[e42] * self[e15])
                    - (self[e43] * self[e3215])
                    - (self[e31] * self[e4235])
                    - (self[e35] * self[e1234]),
                2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
            ]) + (Simd32x4::from([self.group1().xxy()[0], self.group1().zyz()[1], self.group1().xxy()[2] * self[e4235], self[e42] * self[e4315]])
                * self.group1().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group1().yzx()[0],
                    self.group1().yzx()[1],
                    self.group1().yzx()[2] * self.group3().zxyy()[2],
                    self.group3().zxyy()[3] * self[e42],
                ]) * self.group3().zx().with_zw(1.0, 1.0)),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([self[e5] * self[e12345], 0.0]) * Simd32x2::from([2.0, 0.0]))
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
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            2.0 * (self[e12345] * self[e5]) - 2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //           add/sub      mul      div
    //      f32       88      168        0
    //    simd2        0        4        0
    //    simd3        0        3        0
    //    simd4       18       20        0
    // Totals...
    // yes simd      106      195        0
    //  no simd      160      265        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self.group1().xwzw()[0] * self[e4235])
                    + 2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    - (self.group3().xwzw()[0] * self[e423])
                    - (self[e15] * self[e423])
                    - 2.0 * (self[e25] * self[e431])
                    - 2.0 * (self[e35] * self[e412])
                    - 2.0 * (self[e45] * self[e321])
                    - 2.0 * (self[e41] * self[e235])
                    - 2.0 * (self[e42] * self[e315])
                    - 2.0 * (self[e43] * self[e125])
                    - 2.0 * (self[e23] * self[e415])
                    - 2.0 * (self[e31] * self[e425])
                    - 2.0 * (self[e12] * self[e435]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().xx().with_zw(self[scalar], self[e12345] * self[e4]) * self.group9().xyz().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[e41] * self[e23]) * self.group1().xyz().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group5().xy() * self.group3().ww()).with_zw(self[e431] * self[e235], self[e42] * self[e31])
                + Simd32x4::from([
                    2.0 * (self[e412] * self[e315])
                        + (self.group1().zxy()[0] * self.group6().yzx()[0])
                        + (self.group3().xxy()[0] * self[e1234])
                        + (self.group6().zyz()[0] * self[e2])
                        + (self[e31] * self[e4125])
                        + (self[e415] * self[e321])
                        - (self.group9().zxy()[0] * self[e31])
                        - (self[e15] * self[e1234])
                        - (self[e25] * self[e43]),
                    2.0 * (self[e423] * self[e125])
                        + (self.group1().zxy()[1] * self.group6().yzx()[1])
                        + (self.group3().zyz()[1] * self[e1234])
                        + (self.group6().xxy()[1] * self[e3])
                        + (self[e12] * self[e4235])
                        + (self[e425] * self[e321])
                        - (self.group9().zxy()[1] * self[e12])
                        - (self[e25] * self[e1234])
                        - (self[e35] * self[e41]),
                    (self.group3().zyz()[2] * self[e1234])
                        + (self.group6().xxy()[2] * self[e1])
                        + (self.group6().zyz()[2] * self[e321])
                        + (self.group3().wwww()[2] * self[e12])
                        + (self.group6().wwww()[2] * self[e435])
                        + (self[e45] * self[e12])
                        + (self[e23] * self[e4315])
                        - (self.group9().zxy()[2] * self[e23])
                        - (self[e15] * self[e42])
                        - (self[e35] * self[e1234]),
                    2.0 * (self[e43] * self[e12]) - 2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e425] * self[e431]) - 2.0 * (self[e435] * self[e412]),
                ])
                + (Simd32x4::from([self.group3().zxy()[0], self.group3().xxy()[1], self.group1().zxy()[2] * self.group6().yzx()[2], self[e2] * self[e431]])
                    * self.group4().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group3().zyz()[0], self.group3().zxy()[1], self.group3().xxy()[2] * self[e41], self[e3] * self[e412]])
                    * self.group4().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group6().xxy()[0], self.group6().zyz()[1], self.group3().zxy()[2] * self[e41], self[e41] * self[e4235]])
                    * self.group6().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group3().yzx()[0], self.group3().yzx()[1], self.group3().yzx()[2] * self[e42], self.group1().yzxz()[3] * self[e412]])
                    * self.group4().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group6().yzx()[0],
                    self.group6().yzx()[1],
                    self.group6().yzx()[2] * self.group1().zxyy()[2],
                    self.group1().zxyy()[3] * self[e431],
                ]) * self.group1().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group6().zxy()[0],
                    self.group6().zxy()[1],
                    self.group6().zxy()[2] * self.group1().yzxz()[2],
                    self.group9().yzxx()[3] * self[e41],
                ]) * self.group1().yz().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (self.group7().yzx() * self.group8().zxy()).with_w(self[e415] * self[e423]),
            // e5
            2.0 * (self[e12345] * self[e5]) + 2.0 * (self[e15] * self[e23]) + 2.0 * (self[e25] * self[e31]) + 2.0 * (self[e35] * self[e12])
                - 2.0 * (self[e415] * self[e235])
                - 2.0 * (self[e425] * self[e315])
                - 2.0 * (self[e435] * self[e125])
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
            Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[scalar] * self[e5]) * self.group9().xyz().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group4().yzx() * self.group8().zxy()).with_w(self[e12345] * self[e3215])
                + Simd32x4::from([
                    (self.group3().xxy()[0] * self[e4])
                        + (self.group3().zxy()[0] * self[e431])
                        + (self.group3().zyz()[0] * self[e431])
                        + (self.group3().wwww()[0] * self[e415])
                        + (self[e45] * self[e415])
                        + (self[e425] * self[e4125])
                        + (self[e435] * self[e4315])
                        - (self.group6().yzx()[0] * self.group9().zxy()[0])
                        - (self.group6().zxy()[0] * self.group9().yzx()[0])
                        - 2.0 * (self[scalar] * self[e1]),
                    (self.group3().xxy()[1] * self[e412])
                        + (self.group3().zxy()[1] * self[e412])
                        + (self.group3().zyz()[1] * self[e4])
                        + (self.group3().wwww()[1] * self[e425])
                        + (self[e45] * self[e425])
                        + (self[e415] * self[e4125])
                        + (self[e435] * self[e4235])
                        - (self.group6().yzx()[1] * self.group9().zxy()[1])
                        - (self.group6().zxy()[1] * self.group9().yzx()[1])
                        - 2.0 * (self[scalar] * self[e2]),
                    (self.group3().xxy()[2] * self[e423])
                        + (self.group3().zxy()[2] * self[e423])
                        + (self.group3().zyz()[2] * self[e4])
                        + (self.group3().wwww()[2] * self[e435])
                        + (self[e45] * self[e435])
                        + (self[e415] * self[e4315])
                        + (self[e425] * self[e4235])
                        - (self.group1().wwww()[2] * self[e35])
                        - (self.group6().wwww()[2] * self[e12])
                        - (self[e2] * self[e23])
                        - (self[e12] * self[e321]),
                    -(self[e15] * self[e415]) - 2.0 * (self[e31] * self[e315]) - 2.0 * (self[e12] * self[e125]),
                ])
                + (Simd32x4::from([self.group1().zxy()[0], self.group1().zxy()[1], self.group1().zxy()[2] * self[e23], self[e3] * self[e35]])
                    * self.group5().yz().with_zw(1.0, 1.0))
                - (self.group5().yz() * self.group1().zx()).with_zw(self.group6().yzx()[2] * self.group9().zxy()[2], self.group1().zxyz()[3] * self[e35])
                - (self.group3().xy() * self.group1().ww()).with_zw(self.group6().zxy()[2] * self.group9().yzx()[2], self.group3().yzxx()[3] * self[e415])
                - Simd32x4::from(2.0) * (self.group3().yzxy() * self.group7().zxy().with_w(self[e425]))
                - Simd32x4::from(2.0) * (self.group5().xy() * self.group6().ww()).with_zw(self[scalar] * self[e3], self[e23] * self[e235])
                - Simd32x4::from(2.0) * (self.group4().zxy() * self.group8().yzx()).with_w(self[e35] * self[e435]),
            // e1234
            2.0 * (self[scalar] * self[e4]) + 2.0 * (self[e12345] * self[e1234])
                - 2.0 * (self[e41] * self[e415])
                - 2.0 * (self[e42] * self[e425])
                - 2.0 * (self[e43] * self[e435])
                - 2.0 * (self[e23] * self[e423])
                - 2.0 * (self[e31] * self[e431])
                - 2.0 * (self[e12] * self[e412]),
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
    //           add/sub      mul      div
    //      f32       19       43        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       27       52        0
    //  no simd       51       76        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().zxyw() * self.group2().yzx().with_w(self[e4]))
                + Simd32x4::from([
                    2.0 * (self[e12345] * self[e1]) + (self[e235] * self[e4])
                        - (self.group1().yzx()[0] * self.group3().zxy()[0])
                        - (self.group1().zxy()[0] * self.group3().yzx()[0]),
                    2.0 * (self[e12345] * self[e2]) + (self[e315] * self[e4])
                        - (self.group1().yzx()[1] * self.group3().zxy()[1])
                        - (self.group1().zxy()[1] * self.group3().yzx()[1]),
                    2.0 * (self[e12345] * self[e3]) + (self[e415] * self[e2]) + (self[e125] * self[e4])
                        - (self.group2().zxy()[2] * self.group0().yzxx()[2])
                        - (self.group2().zyz()[2] * self[e4])
                        - (self.group0().xyzy()[2] * self.group2().wwww()[2]),
                    -(self[e423] * self[e1])
                        - (self[e431] * self[e425])
                        - (self[e431] * self[e2])
                        - (self[e412] * self[e3])
                        - (self[e321] * self[e4])
                        - 2.0 * (self[e412] * self[e435]),
                ])
                + (Simd32x4::from([self.group1().xxy()[0], self.group1().zyzw()[1], self.group1().xxy()[2] * self[e1], self.group0().xxyx()[3] * self[e1]])
                    * self.group1().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group1().zyzw()[0],
                    self.group1().xxy()[1],
                    self.group1().zyzw()[2] * self[e321],
                    self.group0().zyzy()[3] * self[e2],
                ]) * self.group3().yz().with_zw(1.0, 1.0))
                + (self.group1().yz() * self.group3().zx()).with_zw(self[e412] * self[e5], self.group1().zyzw()[3] * self[e4])
                + (self.group1().xyz() * self.group1().www()).with_w(self.group0().wwwz()[3] * self.group3().xyzz()[3])
                - (Simd32x4::from([
                    self.group2().xxy()[0],
                    self.group2().zyz()[1],
                    self.group1().yzx()[2] * self.group3().zxy()[2],
                    self.group0().xyzy()[3] * self[e425],
                ]) * self.group3().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group2().zxy()[0],
                    self.group2().xxy()[1],
                    self.group1().zxy()[2] * self.group3().yzx()[2],
                    self.group0().yzxx()[3] * self[e415],
                ]) * self.group0().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group2().zyz()[0], self.group2().zxy()[1], self.group2().xxy()[2], self[e415]]) * self.group0().yzxx()),
            // e5
            2.0 * (self[e12345] * self[e5]) - 2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
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
    //           add/sub      mul      div
    //      f32       13       46        0
    //    simd2        0        2        0
    //    simd4       11        9        0
    // Totals...
    // yes simd       24       57        0
    //  no simd       57       86        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[scalar] * self[e4235]) + (self.group1().yzx()[0] * self.group3().zxy()[0])
                    - (self.group1().yzx()[0] * self.group3().zxy()[0])
                    - (self[e15] * self[e1234]),
                2.0 * (self[scalar] * self[e4315]) + (self.group1().yzx()[1] * self.group3().zxy()[1])
                    - (self.group1().yzx()[1] * self.group3().zxy()[1])
                    - (self[e25] * self[e1234]),
                (self.group0().wwwx()[2] * self.group3().xyzx()[2]) + (self.group0().wwwz()[2] * self[e4125]) + (self.group1().xyzz()[2] * self.group1().wwww()[2])
                    - (self.group0().xxyw()[2] * self[e15])
                    - (self.group2().wwww()[2] * self[e35]),
                self[e43] * self[e4125],
            ]) + (Simd32x4::from([
                self.group1().xxy()[0],
                self.group1().zyz()[1],
                self.group1().xxy()[2] * self[e4235],
                self.group0().xyzy()[3] * self[e31],
            ]) * self.group1().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group1().zyz()[0],
                    self.group1().xxy()[1],
                    self.group1().yzx()[2] * self.group3().zxy()[2],
                    self.group0().yzxx()[3] * self[e23],
                ]) * self.group3().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group2().xxy()[0],
                    self.group2().zyz()[1],
                    self.group1().zyz()[2] * self[e45],
                    self.group0().wwwx()[3] * self.group3().xyzx()[3],
                ]) * self.group2().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().zxy()[0], self.group2().xxy()[1], self.group2().xxy()[2] * self[e41], self.group0().wwwz()[3] * self[e12]])
                    * self.group0().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group2().zyz()[0],
                    self.group2().zxy()[1],
                    self.group2().zxy()[2] * self.group0().yzxx()[2],
                    self.group1().xyzz()[3] * self[e43],
                ]) * self.group0().yz().with_zw(1.0, 1.0))
                + (self.group0().xy() * self.group3().ww()).with_zw(self.group2().zyz()[2] * self[e1234], self[e41] * self[e23])
                + (self.group1().xy() * self.group1().ww()).with_zw(self.group0().xyzy()[2] * self.group3().wwww()[2], self[e42] * self[e31])
                - (Simd32x4::from([
                    self.group0().zyz()[0],
                    self.group0().xxyw()[1],
                    self.group0().zyz()[2] * self[e3215],
                    self.group0().xxyw()[3] * self[e1234],
                ]) * self.group2().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group1().zxy()[0],
                    self.group0().zyz()[1],
                    self.group1().yzx()[2] * self.group3().zxy()[2],
                    self.group0().zxyx()[3] * self[e4235],
                ]) * self.group3().yw().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group2().yzx()[0],
                    self.group2().yzx()[1],
                    self.group1().zxy()[2] * self.group3().yzxz()[2],
                    self.group3().yzxz()[3] * self[e43],
                ]) * self.group0().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().xxyw()[0],
                    self.group1().zxy()[1],
                    self.group2().yzx()[2] * self.group0().zxyx()[2],
                    self[scalar] * self[e1234],
                ]) * self.group3().wz().with_zw(1.0, 1.0)),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
        )
    }
}
