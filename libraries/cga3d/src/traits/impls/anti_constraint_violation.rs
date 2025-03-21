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
//   Median:         5      15       0     N/A
//  Average:        15      26       0     N/A
//  Maximum:       124     185       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:        14      31       0       0
//  Average:        35      46       0       0
//  Maximum:       268     320       0       0
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
    //      f32        2        9        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       14       31        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0) + Simd32x3::from(0.0).with_w((self[e43] * self[e12]) * -2.0)
                - Simd32x4::from(2.0) * (self.group1().xyzx() * Simd32x3::from(self[e45]).with_w(self[e41]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e42] * self[e31]),
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
    //      f32       13       25        0        0
    //    simd3        0        3        0      N/A
    //    simd4       14       10        0      N/A
    // Totals...
    // yes simd       27       38        0      N/A
    //  no simd       69       74        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x4::from([anti_reverse_g2[1], anti_reverse_g2[2], anti_reverse_g2[0], self[e412] * self[e435]]) * self.group0().zxy().with_w(1.0))
                + (self.group1().xyzx() * Simd32x3::from(anti_reverse_g1[3]).with_w(self[e423]))
                + (self.group1().yzxy() * self.group3().zxy().with_w(self[e431]))
                + Simd32x3::from(0.0).with_w((self[e431] * self[e2]) * -2.0)
                + (Simd32x3::from(anti_reverse_g2[3]) * self.group2().xyz()).with_w(0.0)
                + (self.group0().yzx() * self.group2().zxy()).with_w(0.0)
                + (anti_reverse_g1.zxy() * self.group3().yzx()).with_w(0.0)
                - Simd32x4::from([anti_reverse_g1[1] * self[e3], self[e415] * self[e3], self[e425] * self[e1], 0.0])
                - Simd32x4::from([self[e435] * self[e2], anti_reverse_g1[2] * self[e1], 0.0, self[e423] * self[e1]])
                - (anti_reverse_g2 * Simd32x3::from(self[e4]).with_w(self[e321]))
                - (Simd32x4::from(self[e423]) * Simd32x4::from([self[e5], self[e125], anti_reverse_g2[1], anti_reverse_g1[0]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([anti_reverse_g2[2], self[e5], self[e235], anti_reverse_g1[1]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([self[e315], anti_reverse_g2[0], self[e5], self[e3]]))
                - Simd32x2::from(0.0).with_zw(anti_reverse_g1[0] * self[e2], anti_reverse_g1[2] * self[e412]),
            // e5
            (self[e321] * self[e5]) + (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3])
                - (anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g1[3] * self[e5])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[0] * self[e1])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[1] * self[e2])
                - (anti_reverse_g2[2] * self[e435])
                - (anti_reverse_g2[2] * self[e3]),
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
    //      f32        3       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd       15       32        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e412] * self[e435])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz()).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
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
    //      f32        3       11        0        0
    //    simd3        0        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd       15       32        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e412] * self[e435])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz()).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0),
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
    //      f32        2        9        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       14       31        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0) + Simd32x3::from(0.0).with_w((self[e43] * self[e12]) * -2.0)
                - Simd32x4::from(2.0) * (self.group1().xyzx() * Simd32x3::from(self[e45]).with_w(self[e41]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e42] * self[e31]),
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
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
    //      f32       11       31        0        0
    //    simd3        0        2        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       23       40        0      N/A
    //  no simd       59       65        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group0().zyz()).with_w(0.0)
                + (anti_reverse_g1 * Simd32x3::from(self[e45]).with_w(self[e1234]))
                + (Simd32x4::from([self[e3215], self[e35], self[e15], anti_reverse_g1[2] * self[e43]]) * self.group0().xxy().with_w(1.0))
                + (anti_reverse_g1.zxyx() * self.group3().yzx().with_w(self[e41]))
                + (anti_reverse_g1.wwwy() * self.group1().xyz().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w(-(self[e43] * self[e12]) - 2.0 * (self[e43] * self[e4125]))
                + (self.group1().yzx() * self.group3().zxy()).with_w(0.0)
                - Simd32x4::from([0.0, self[e23] * self[e4125], self[e31] * self[e4235], self[e42] * self[e4315]])
                - Simd32x4::from([anti_reverse_g1[1] * self[e4125], anti_reverse_g1[2] * self[e4235], 0.0, self[e41] * self[e23]])
                - Simd32x4::from([self[e42] * self[e35], 0.0, anti_reverse_g1[0] * self[e4315], self[e41] * self[e4235]])
                - Simd32x4::from([self[e12] * self[e4315], self[e43] * self[e15], self[e41] * self[e25], 0.0])
                - (self.group2() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - Simd32x3::from(0.0).with_w(self[e42] * self[e31]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (self[e45] * self[e3215])
                + 2.0 * (self[e15] * self[e4235])
                + 2.0 * (self[e25] * self[e4315])
                + 2.0 * (self[e35] * self[e4125])
                - (anti_reverse_g1[3] * self[e3215])
                - (self[e23] * self[e15])
                - (self[e31] * self[e25])
                - (self[e12] * self[e35]),
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
    //      f32       76      135        0        0
    //    simd3        0       15        0      N/A
    //    simd4       48       35        0      N/A
    // Totals...
    // yes simd      124      185        0      N/A
    //  no simd      268      320        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g3 = self.group3() * Simd32x4::from(-1.0);
        let anti_reverse_g6 = self.group6() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    + 2.0 * (self[e41] * self[e235])
                    + 2.0 * (self[e42] * self[e315])
                    + 2.0 * (self[e43] * self[e125])
                    + (self[scalar] * self[e12345])
                    + (self[e2] * self[e4315])
                    + (self[e15] * self[e423])
                    + (self[e25] * self[e431])
                    + (self[e35] * self[e412])
                    + (self[e23] * self[e415])
                    + (self[e31] * self[e425])
                    + (self[e12] * self[e435])
                    - (anti_reverse_g3[0] * self[e423])
                    - (anti_reverse_g3[1] * self[e431])
                    - (anti_reverse_g3[2] * self[e412])
                    - (anti_reverse_g3[3] * self[e321])
                    - (anti_reverse_g6[0] * self[e23])
                    - (anti_reverse_g6[1] * self[e31])
                    - (anti_reverse_g6[2] * self[e12])
                    - (anti_reverse_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * self.group1())
                + Simd32x4::from(2.0) * (self.group5().yzx() * self.group9().zxy()).with_w(0.0)
                + Simd32x4::from([
                    (self[e35] * self[e42]) * -1.0,
                    (self[e423] * self[e125]) * -1.0,
                    (self[e431] * self[e235]) * -1.0,
                    -(self[e2] * self[e431])
                        - (self[e4] * self[e321])
                        - (self[e42] * self[e4315])
                        - 2.0 * (self[e3] * self[e412])
                        - 2.0 * (self[e41] * self[e23])
                        - 2.0 * (self[e43] * self[e12])
                        - 2.0 * (self[e43] * self[e4125]),
                ])
                + (anti_reverse_g3 * Simd32x4::from(self[e1234]))
                + (anti_reverse_g6 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x4::from([anti_reverse_g3[2], anti_reverse_g3[0], anti_reverse_g3[1], self[e435] * self[e412]]) * self.group4().yzx().with_w(1.0))
                + (self.group6().xyzx() * Simd32x3::from(anti_reverse_g6[3]).with_w(self[e423]))
                + (self.group6().yzxy() * self.group1().zxy().with_w(self[e431]))
                + (Simd32x3::from(anti_reverse_g3[3]) * self.group5()).with_w(0.0)
                + (Simd32x3::from(self[scalar]) * self.group9().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e4]) * self.group8()).with_w(0.0)
                + (Simd32x3::from(self[e3215]) * self.group4()).with_w(0.0)
                + (self.group4().zxy() * self.group3().yzx()).with_w(0.0)
                + (self.group7().yzx() * self.group8().zxy()).with_w(0.0)
                + (anti_reverse_g6.zxy() * self.group1().yzx()).with_w(0.0)
                - Simd32x4::from([self[e412] * self[e315], anti_reverse_g3[2] * self[e41], anti_reverse_g3[0] * self[e42], 0.0])
                - (Simd32x4::from(self[e1234]) * self.group3())
                - (Simd32x4::from([anti_reverse_g3[1], self[e15], self[e25], anti_reverse_g6[1] * self[e431]]) * self.group4().zzx().with_w(1.0))
                - (Simd32x4::from([self[e5], self[e5], self[e5], anti_reverse_g6[2] * self[e412]]) * self.group7().with_w(1.0))
                - (anti_reverse_g6.yzxx() * self.group1().zxy().with_w(self[e423]))
                - (self.group1().yzxx() * self.group6().zxy().with_w(self[e423]))
                - (self.group9().yzxx() * self.group5().zxy().with_w(self[e41]))
                - Simd32x3::from(0.0).with_w(self[e42] * self[e31])
                - (Simd32x3::from(self[e45]) * self.group5()).with_w(self[scalar] * self[e1234]),
            // e5
            2.0 * (self[e12345] * self[e5])
                + 2.0 * (self[e1] * self[e235])
                + 2.0 * (self[e2] * self[e315])
                + 2.0 * (self[e3] * self[e125])
                + (anti_reverse_g3[0] * self[e23])
                + (anti_reverse_g3[1] * self[e31])
                + (anti_reverse_g3[2] * self[e12])
                + (self[e5] * self[e321])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                + (self[e45] * self[e3215])
                + (self[e415] * self[e235])
                + (self[e425] * self[e315])
                + (self[e435] * self[e125])
                - (anti_reverse_g3[0] * self[e4235])
                - (anti_reverse_g3[1] * self[e4315])
                - (anti_reverse_g3[2] * self[e4125])
                - (anti_reverse_g3[3] * self[e3215])
                - (anti_reverse_g6[0] * self[e235])
                - (anti_reverse_g6[1] * self[e315])
                - (anti_reverse_g6[2] * self[e125])
                - (anti_reverse_g6[3] * self[e5])
                - (self[e15] * self[e23])
                - (self[e25] * self[e31])
                - (self[e35] * self[e12])
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
            Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * self.group9())
                + Simd32x4::from(2.0) * (Simd32x3::from(self[e3215]) * self.group7()).with_w(self[e31] * self[e315])
                + Simd32x4::from(2.0) * (self.group4().zxy() * self.group8().yzx()).with_w(self[e23] * self[e235])
                + Simd32x4::from([anti_reverse_g6[2] * self[e4315], anti_reverse_g6[0] * self[e4125], anti_reverse_g6[1] * self[e4235], 0.0])
                + (anti_reverse_g3 * Simd32x4::from([self[e4], self[e4], self[e4], self[e5]]))
                + (Simd32x4::from(self[e5]) * self.group4().with_w(self[scalar]))
                + (Simd32x4::from([anti_reverse_g3[2], anti_reverse_g3[0], anti_reverse_g3[1], self[e12] * self[e125]]) * self.group7().yzx().with_w(1.0))
                + (Simd32x4::from([anti_reverse_g6[0], anti_reverse_g6[1], anti_reverse_g6[2], self[e1]]) * self.group3().wwwx())
                + (self.group6() * Simd32x3::from(anti_reverse_g3[3]).with_w(self[e3215]))
                + (self.group1().yzxy() * self.group5().zxy().with_w(self[e25]))
                + Simd32x3::from(0.0).with_w(self[e3] * self[e35])
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e235] * self[e4235]) - 2.0 * (self[e315] * self[e4315]) - 2.0 * (self[e125] * self[e4125]))
                + (Simd32x3::from(self[e321]) * self.group5()).with_w(0.0)
                + (self.group7().zxy() * self.group3().yzx()).with_w(0.0)
                + (self.group6().yzx() * self.group9().zxy()).with_w(0.0)
                - (Simd32x4::from(anti_reverse_g6[3]) * self.group5().with_w(self[e3215]))
                - (Simd32x4::from([self[e4], self[e4], self[e4], self[e5]]) * self.group3())
                - (Simd32x4::from([self[e1234], self[e1234], self[e1234], anti_reverse_g6[2] * self[e35]]) * self.group8().with_w(1.0))
                - (anti_reverse_g3.yzxx() * self.group7().zxy().with_w(self[e1]))
                - (anti_reverse_g6.yzxx() * self.group9().zxy().with_w(self[e15]))
                - (self.group1().xyzy() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g3[1]))
                - (self.group1().zxyz() * self.group5().yzx().with_w(anti_reverse_g3[2]))
                - (self.group3().zxyy() * self.group7().yzx().with_w(anti_reverse_g6[1]))
                - (self.group6().zxyx() * self.group9().yzx().with_w(anti_reverse_g3[0]))
                - Simd32x3::from(0.0).with_w(anti_reverse_g3[2] * self[e435])
                - (self.group4().yzx() * self.group8().zxy()).with_w(anti_reverse_g3[1] * self[e425]),
            // e1234
            2.0 * (self[scalar] * self[e4])
                + 2.0 * (self[e12345] * self[e1234])
                + 2.0 * (self[e423] * self[e4235])
                + 2.0 * (self[e431] * self[e4315])
                + 2.0 * (self[e412] * self[e4125])
                + (anti_reverse_g6[3] * self[e1234])
                + (self[e4] * self[e45])
                + (self[e41] * self[e415])
                + (self[e42] * self[e425])
                + (self[e43] * self[e435])
                + 2.0 * (self[e23] * self[e423])
                + 2.0 * (self[e31] * self[e431])
                + 2.0 * (self[e12] * self[e412])
                - (anti_reverse_g3[3] * self[e4])
                - (anti_reverse_g6[0] * self[e41])
                - (anti_reverse_g6[1] * self[e42])
                - (anti_reverse_g6[2] * self[e43])
                - (self[e321] * self[e1234])
                - 2.0 * (self[e1] * self[e41])
                - 2.0 * (self[e2] * self[e42])
                - 2.0 * (self[e3] * self[e43]),
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
    //      f32        9       20        0        0
    //    simd3        0        1        0      N/A
    //    simd4       14       14        0      N/A
    // Totals...
    // yes simd       23       35        0      N/A
    //  no simd       65       79        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz()).with_w(0.0)
                + (anti_reverse_g0 * Simd32x3::from(self[e5]).with_w(self[e4]))
                + (self.group3() * Simd32x3::from(anti_reverse_g0[3]).with_w(self[e12345]))
                + (anti_reverse_g0.zxyx() * self.group2().yzx().with_w(self[e1]))
                + (self.group0().yzxx() * self.group2().zxy().with_w(self[e415]))
                + (self.group3().xyzy() * Simd32x3::from(self[e12345]).with_w(anti_reverse_g0[1]))
                + (self.group3().zxyz() * self.group1().yzx().with_w(anti_reverse_g0[2]))
                + Simd32x3::from(0.0).with_w(self[e431] * self[e425])
                + Simd32x3::from(0.0).with_w((self[e412] * self[e435]) - (self[e412] * self[e3]))
                - (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xxyx() * self.group2().wzx().with_w(self[e1]))
                - (self.group0().zyzy() * self.group2().yww().with_w(self[e2]))
                - (self.group1().zxyy() * self.group3().yzx().with_w(anti_reverse_g0[1]))
                - Simd32x3::from(0.0).with_w(anti_reverse_g0[2] * self[e435])
                - Simd32x4::from(2.0) * (self.group1() * Simd32x3::from(self[e321]).with_w(self[e4])),
            // e5
            (anti_reverse_g0[3] * self[e5])
                + (self[e12345] * self[e5])
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
    //      f32       12       18        0        0
    //    simd3        0        1        0      N/A
    //    simd4       16       16        0      N/A
    // Totals...
    // yes simd       28       35        0      N/A
    //  no simd       76       85        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x3::from(self[e45]).with_w(self[e1234]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([anti_reverse_g0[3], self[e12], anti_reverse_g1[1], anti_reverse_g0[0]]))
                + (Simd32x4::from(self[e4315]) * Simd32x4::from([anti_reverse_g1[2], anti_reverse_g0[3], self[e23], anti_reverse_g0[1]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e31], anti_reverse_g1[0], anti_reverse_g0[3], anti_reverse_g0[2]]))
                + (Simd32x4::from([self[e25], self[e3215], self[e3215], anti_reverse_g1[1]]) * self.group0().zyzy())
                + (Simd32x4::from([self[e3215], self[e35], self[e15], anti_reverse_g1[0]]) * self.group0().xxyx())
                + (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e23]))
                + (self.group0().wwwz() * self.group3().xyz().with_w(anti_reverse_g1[2]))
                + (self.group1().xyzy() * Simd32x3::from(anti_reverse_g1[3]).with_w(anti_reverse_g0[1]))
                + Simd32x3::from(0.0).with_w(anti_reverse_g0[2] * self[e12])
                - (anti_reverse_g0 * Simd32x3::from(self[e3215]).with_w(self[e1234]))
                - (Simd32x4::from([anti_reverse_g1[1], anti_reverse_g1[2], anti_reverse_g1[0], self[e41]]) * self.group3().zxyx())
                - (self.group2() * Simd32x3::from(self[e1234]).with_w(self[scalar]))
                - (self.group0().yzxy() * self.group2().zxy().with_w(self[e4315]))
                - (self.group3().yzxz() * self.group1().zxy().with_w(self[e43]))
                - Simd32x3::from(0.0).with_w(self[e45] * self[e1234])
                - (anti_reverse_g0.zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (self[e45] * self[e3215])
                + 2.0 * (self[e15] * self[e4235])
                + 2.0 * (self[e25] * self[e4315])
                + 2.0 * (self[e35] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[3] * self[e3215])
                - (self[scalar] * self[e3215])
                - (self[e23] * self[e15])
                - (self[e31] * self[e25])
                - (self[e12] * self[e35]),
        )
    }
}
