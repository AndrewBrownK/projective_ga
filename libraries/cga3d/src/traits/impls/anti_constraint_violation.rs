// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 17
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        13      24       0
//  Average:        23      32       0
//  Maximum:       166     213       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        25      40       0
//  Average:        44      56       0
//  Maximum:       334     372       0
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
    //      f32        9       17        0
    //    simd3        0        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       25       41        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g0[2] * self[e25]) - (anti_reverse_g2[1] * self[e43]),
                -(anti_reverse_g0[0] * self[e35]) - (anti_reverse_g2[2] * self[e41]),
                -(anti_reverse_g0[1] * self[e15]) - (anti_reverse_g2[0] * self[e42]),
                (anti_reverse_g1[1] * self[e42]) + (anti_reverse_g1[2] * self[e43]),
            ]) + (self.group1().wwwz() * anti_reverse_g1.xyz().with_w(anti_reverse_g0[2]))
                + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e23])
                + (self.group0().yzx() * anti_reverse_g2.zxy()).with_w(anti_reverse_g0[1] * self[e31])
                + (self.group1().xyz() * anti_reverse_g1.www()).with_w(anti_reverse_g1[0] * self[e41]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12]),
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
    //      f32       25       35        0
    //    simd3        0        6        0
    //    simd4       10        7        0
    // Totals...
    // yes simd       35       48        0
    //  no simd       65       81        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g1[2] * self[e2]) + (anti_reverse_g1[3] * self[e415]) + (anti_reverse_g2[3] * self[e235]) + (self[e425] * self[e3]),
                (anti_reverse_g1[1] * self[e321]) + (anti_reverse_g1[3] * self[e425]) + (anti_reverse_g2[3] * self[e315]) + (self[e435] * self[e1]),
                (anti_reverse_g1[2] * self[e321]) + (anti_reverse_g1[3] * self[e435]) + (anti_reverse_g2[3] * self[e125]) + (self[e415] * self[e2]),
                -(anti_reverse_g1[2] * self[e412]) - (anti_reverse_g2[3] * self[e321]) - (self[e431] * self[e2]) - (self[e412] * self[e3]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * anti_reverse_g1.xxyw())
                + (Simd32x4::from([self[e315], self[e5], self[e5], self[e2]]) * anti_reverse_g0.zyz().with_w(anti_reverse_g0[1]))
                + (Simd32x4::from([self[e5], self[e125], self[e235], self[e1]]) * anti_reverse_g0.xxy().with_w(anti_reverse_g0[0]))
                + (self.group0().zxy() * anti_reverse_g2.yzx()).with_w(anti_reverse_g0[2] * self[e3])
                - (anti_reverse_g2.zx().with_zw(self[e5], self[e435]) * self.group0().yzz().with_w(anti_reverse_g0[2]))
                - (self.group3().ww().with_zw(anti_reverse_g2[1], self[e425]) * self.group0().xyx().with_w(anti_reverse_g0[1]))
                - (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g1.yzx() * self.group3().zxy()).with_w(anti_reverse_g1[0] * self[e423])
                - (self.group1().zxy() * self.group3().yzx()).with_w(anti_reverse_g1[1] * self[e431])
                - (anti_reverse_g2.xyz() * self.group2().www()).with_w(self[e423] * self[e1]),
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
    //      add/sub      mul      div
    // f32        0        2        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3]) + (self[e321] * self[e5])
                - (anti_reverse_g0[0] * self[e1])
                - (anti_reverse_g0[1] * self[e2])
                - (anti_reverse_g0[2] * self[e3])
                - (anti_reverse_g0[3] * self[e5]),
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
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12]),
            0.0,
        ]))
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[3] * self[scalar]),
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
    //      f32       17       24        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       40        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[2] * self[e315]) + (anti_reverse_g2[1] * self[e412]) + (anti_reverse_g1[0] * self[e321]) + (anti_reverse_g1[3] * self[e415]),
                (anti_reverse_g0[0] * self[e125]) + (anti_reverse_g2[2] * self[e423]) + (anti_reverse_g1[1] * self[e321]) + (anti_reverse_g1[3] * self[e425]),
                (anti_reverse_g0[1] * self[e235]) + (anti_reverse_g2[0] * self[e431]) + (anti_reverse_g1[2] * self[e321]) + (anti_reverse_g1[3] * self[e435]),
                -(anti_reverse_g0[2] * self[e435]) - (anti_reverse_g1[0] * self[e423]) - (anti_reverse_g1[1] * self[e431]) - (anti_reverse_g1[2] * self[e412]),
            ]) - (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g2.zxy() * self.group0().yzx()).with_w(anti_reverse_g0[1] * self[e425]),
            // e5
            -(anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e435])
                - (anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125]),
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
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       25       41        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[2] * self[e315]) + (anti_reverse_g1[0] * self[e321]) + (anti_reverse_g1[3] * self[e415]) + (anti_reverse_g2[1] * self[e412]),
                (anti_reverse_g0[0] * self[e125]) + (anti_reverse_g1[1] * self[e321]) + (anti_reverse_g1[3] * self[e425]) + (anti_reverse_g2[2] * self[e423]),
                (anti_reverse_g0[1] * self[e235]) + (anti_reverse_g1[2] * self[e321]) + (anti_reverse_g1[3] * self[e435]) + (anti_reverse_g2[0] * self[e431]),
                -(anti_reverse_g0[2] * self[e435]) - (anti_reverse_g1[0] * self[e423]) - (anti_reverse_g1[1] * self[e431]) - (anti_reverse_g1[2] * self[e412]),
            ]) - (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e415])
                - (self.group0().yzx() * anti_reverse_g2.zxy()).with_w(anti_reverse_g0[1] * self[e425]),
            // e5
            -(anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e435]),
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
    //      f32        9       17        0
    //    simd3        0        5        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       25       40        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g0[2] * self[e25]) - (anti_reverse_g2[1] * self[e43]),
                -(anti_reverse_g0[0] * self[e35]) - (anti_reverse_g2[2] * self[e41]),
                -(anti_reverse_g0[1] * self[e15]) - (anti_reverse_g2[0] * self[e42]),
                (anti_reverse_g1[1] * self[e42]) + (anti_reverse_g1[2] * self[e43]),
            ]) + (self.group1().wwwz() * anti_reverse_g1.xyz().with_w(anti_reverse_g0[2]))
                + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e23])
                + (anti_reverse_g2.zxy() * self.group0().yzx()).with_w(anti_reverse_g0[1] * self[e31])
                + (self.group1().xyz() * anti_reverse_g1.www()).with_w(anti_reverse_g1[0] * self[e41]),
            // e5
            (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12])
                + (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35]),
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
    //      f32       17       28        0
    //    simd3        0        7        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       65       81        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g2[3] * self[e15]) - (self[e12] * self[e4315]),
                -(anti_reverse_g2[3] * self[e25]) - (self[e23] * self[e4125]),
                -(anti_reverse_g2[3] * self[e35]) - (self[e31] * self[e4235]),
                (anti_reverse_g1[2] * self[e43]) + (anti_reverse_g1[3] * self[e1234]),
            ]) + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e12]]) * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                + (anti_reverse_g2.zx().with_zw(self[e3215], self[e31]) * self.group0().yzz().with_w(anti_reverse_g0[1]))
                + (self.group3().ww().with_zw(anti_reverse_g2[1], self[e4235]) * self.group0().xyx().with_w(anti_reverse_g0[0]))
                + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e23])
                + (self.group1().yzx() * self.group3().zxy()).with_w(anti_reverse_g1[1] * self[e42])
                + (anti_reverse_g2.xyz() * self.group2().www()).with_w(anti_reverse_g1[0] * self[e41])
                + (self.group1().xyz() * anti_reverse_g1.www()).with_w(anti_reverse_g0[2] * self[e4125])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e4315]]) * anti_reverse_g0.zyz().with_w(self[e42]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e4235]]) * anti_reverse_g0.xxy().with_w(self[e41]))
                - (self.group0().zxy() * anti_reverse_g2.yzx()).with_w(self[e43] * self[e4125])
                - (anti_reverse_g1.yzx() * self.group3().zxy()).with_w(anti_reverse_g2[3] * self[e45]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12])
                + (self[e45] * self[e3215])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                - (anti_reverse_g1[3] * self[e3215])
                - (anti_reverse_g2[0] * self[e4235])
                - (anti_reverse_g2[1] * self[e4315])
                - (anti_reverse_g2[2] * self[e4125]),
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
    //      add/sub      mul      div
    // f32        0        2        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        9        0
    //  no simd        7       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (self[e15] * self[e4235]) + (self[e25] * self[e4315]) + (self[e35] * self[e4125]) + (self[e45] * self[e3215])
                - (anti_reverse_g0[0] * self[e4235])
                - (anti_reverse_g0[1] * self[e4315])
                - (anti_reverse_g0[2] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215]),
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
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (anti_reverse_g0[3] * self[e5]) + (anti_reverse_g1[3] * self[e12345])
                - (anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
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
    //      f32      110      151        0
    //    simd3        0       27        0
    //    simd4       56       35        0
    // Totals...
    // yes simd      166      213        0
    //  no simd      334      372        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g3 = self.group3() * Simd32x4::from(-1.0);
        let anti_reverse_g4 = self.group4() * Simd32x3::from(-1.0);
        let anti_reverse_g5 = self.group5() * Simd32x3::from(-1.0);
        let anti_reverse_g6 = self.group6() * Simd32x4::from(-1.0);
        let anti_reverse_g7 = self.group7() * Simd32x3::from(-1.0);
        let anti_reverse_g8 = self.group8() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[e5] * self[e1234])
                    + (self.group9().yx()[0] * self[e2])
                    + 2.0 * (self.group1().xwzw()[0] * self[e4235])
                    + (self.group9().zyzw()[0] * self[e3])
                    + (self.group9().wzzw()[0] * self[e4])
                    + 2.0 * (self[scalar] * self[e12345])
                    + (self[e2] * self[e4315])
                    + (self[e3] * self[e4125])
                    + (self[e4] * self[e3215])
                    - (anti_reverse_g4[0] * self[e235])
                    - (anti_reverse_g4[1] * self[e315])
                    - (anti_reverse_g4[2] * self[e125])
                    - (anti_reverse_g5[0] * self[e415])
                    - (anti_reverse_g5[1] * self[e425])
                    - (anti_reverse_g5[2] * self[e435])
                    - (anti_reverse_g7[0] * self.group3().xwzw()[0])
                    - (anti_reverse_g7[1] * self[e25])
                    - (anti_reverse_g7[2] * self[e35])
                    - (anti_reverse_g8[0] * self[e41])
                    - (anti_reverse_g8[1] * self[e42])
                    - (anti_reverse_g8[2] * self[e43])
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
            Simd32x4::from([
                (anti_reverse_g3[0] * self[e1234]) + (anti_reverse_g6[0] * self[e321]) + (anti_reverse_g6[2] * self[e2]) + (anti_reverse_g6[3] * self[e415]),
                (anti_reverse_g3[1] * self[e1234]) + (anti_reverse_g6[0] * self[e3]) + (anti_reverse_g6[1] * self[e321]) + (anti_reverse_g6[3] * self[e425]),
                (anti_reverse_g3[2] * self[e1234]) + (anti_reverse_g6[1] * self[e1]) + (anti_reverse_g6[2] * self[e321]) + (anti_reverse_g6[3] * self[e435]),
                -(anti_reverse_g6[2] * self[e412]) - (self[e3] * self[e412]) - (self[e4] * self[e321]) - (self[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e5], self[e125], self[e235], self[e4125]]) * anti_reverse_g7.xxy().with_w(anti_reverse_g4[2]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse_g5.xxy().with_w(anti_reverse_g4[1]))
                + (Simd32x4::from([self[e315], self[e5], self[e5], self[e41]]) * anti_reverse_g7.zyz().with_w(anti_reverse_g5[0]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e12]]) * anti_reverse_g5.zyz().with_w(anti_reverse_g4[2]))
                + (self.group0().xx().with_zw(self[scalar], anti_reverse_g4[0]) * self.group9().xyz().with_w(self[e23]))
                + (self.group0().xx().with_zw(self[scalar], self[e12345]) * self.group9().xyz().with_w(self[e4]))
                + (self.group0().yy().with_zw(self[e12345], anti_reverse_g4[0]) * self.group1().xyz().with_w(self[e4235]))
                + (self.group0().yy().with_zw(self[e12345], self[e12345]) * self.group1().xyz().with_w(self[e4]))
                + (anti_reverse_g3.zx().with_zw(self[e3215], self[e1]) * self.group4().yzz().with_w(anti_reverse_g7[0]))
                + (anti_reverse_g3.ww().with_zw(self[e4315], self[e2]) * self.group5().xyx().with_w(anti_reverse_g7[1]))
                + (self.group9().zx().with_zw(anti_reverse_g3[3], self[e3]) * self.group5().yzz().with_w(anti_reverse_g7[2]))
                + (self.group9().ww().with_zw(anti_reverse_g3[1], self[e43]) * self.group4().xyx().with_w(anti_reverse_g5[2]))
                + (self.group8() * self.group1().www()).with_w(anti_reverse_g3[3] * self[e1234])
                + (anti_reverse_g4.yzx() * self.group3().zxy()).with_w(anti_reverse_g4[1] * self[e31])
                + (anti_reverse_g8.yzx() * self.group7().zxy()).with_w(anti_reverse_g5[1] * self[e42])
                + (self.group1().zxy() * self.group6().yzx()).with_w(anti_reverse_g6[3] * self[e4])
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e435]]) * anti_reverse_g8.xxy().with_w(anti_reverse_g7[2]))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e1234]]) * anti_reverse_g4.zyz().with_w(self[scalar]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e4235]]) * anti_reverse_g8.zyz().with_w(self[e41]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse_g4.xxy().with_w(self[scalar]))
                - (self.group9().yzxz() * self.group5().zxy().with_w(self[e43]))
                - (Simd32x3::from(self[e5]) * self.group7()).with_w(self[e1] * self[e423])
                - (Simd32x3::from(self[e1234]) * self.group3().xyz()).with_w(anti_reverse_g6[1] * self[e431])
                - (anti_reverse_g5.yzx() * self.group9().zxy()).with_w(anti_reverse_g7[0] * self[e415])
                - (anti_reverse_g7.yzx() * self.group8().zxy()).with_w(anti_reverse_g7[1] * self[e425])
                - (self.group4().zxy() * anti_reverse_g3.yzx()).with_w(self[e42] * self[e4315])
                - (anti_reverse_g6.yzx() * self.group1().zxy()).with_w(self[e2] * self[e431])
                - (self.group1().yzx() * self.group6().zxy()).with_w(anti_reverse_g6[0] * self[e423]),
            // e5
            (anti_reverse_g5[0] * self[e15])
                + (anti_reverse_g5[1] * self[e25])
                + (anti_reverse_g5[2] * self[e35])
                + (anti_reverse_g3[0] * self[e23])
                + (anti_reverse_g3[1] * self[e31])
                + (anti_reverse_g3[2] * self[e12])
                + 2.0 * (self[e12345] * self[e5])
                + (self[e1] * self[e235])
                + (self[e2] * self[e315])
                + (self[e3] * self[e125])
                + (self[e5] * self[e321])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                + (self[e45] * self[e3215])
                - (anti_reverse_g8[0] * self[e1])
                - (anti_reverse_g8[0] * self[e415])
                - (anti_reverse_g8[1] * self[e2])
                - (anti_reverse_g8[1] * self[e425])
                - (anti_reverse_g8[2] * self[e3])
                - (anti_reverse_g8[2] * self[e435])
                - (anti_reverse_g3[0] * self[e4235])
                - (anti_reverse_g3[1] * self[e4315])
                - (anti_reverse_g3[2] * self[e4125])
                - (anti_reverse_g3[3] * self[e3215])
                - (anti_reverse_g6[0] * self[e235])
                - (anti_reverse_g6[1] * self[e315])
                - (anti_reverse_g6[2] * self[e125])
                - (anti_reverse_g6[3] * self[e5])
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
            Simd32x4::from([
                (anti_reverse_g3[3] * self[e415]) + (anti_reverse_g6[0] * self[e45]) + (anti_reverse_g6[2] * self[e4315]) + (self[e425] * self[e4125]),
                (anti_reverse_g3[3] * self[e425]) + (anti_reverse_g6[0] * self[e4125]) + (anti_reverse_g6[1] * self[e45]) + (self[e435] * self[e4235]),
                (anti_reverse_g3[3] * self[e435]) + (anti_reverse_g6[1] * self[e4235]) + (anti_reverse_g6[2] * self[e45]) + (self[e415] * self[e4315]),
                -(anti_reverse_g6[1] * self[e25]) - (anti_reverse_g6[2] * self[e35]) - (anti_reverse_g6[3] * self[e3215]) - (self[e5] * self[e45]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e4125]]) * anti_reverse_g8.zyz().with_w(anti_reverse_g8[2]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4315]]) * anti_reverse_g8.xxy().with_w(anti_reverse_g8[1]))
                + (self.group0().yy().with_zw(self[e12345], self[scalar]) * self.group9().xyz().with_w(self[e5]))
                + (self.group0().yy().with_zw(self[e12345], self[e12345]) * self.group9().xyz().with_w(self[e3215]))
                + (anti_reverse_g3.zx().with_zw(self[e3215], self[e5]) * self.group7().yzz().with_w(anti_reverse_g3[3]))
                + (self.group9().ww().with_zw(anti_reverse_g3[1], self[e35]) * self.group7().xyx().with_w(self[e3]))
                + (Simd32x3::from(self[e5]) * self.group4()).with_w(self[e1] * self[e15])
                + (anti_reverse_g4.yzx() * self.group8().zxy()).with_w(self[scalar] * self[e5])
                + (anti_reverse_g5.yzx() * self.group1().zxy()).with_w(self[e12345] * self[e3215])
                + (anti_reverse_g7.yzx() * self.group3().zxy()).with_w(anti_reverse_g8[0] * self[e4235])
                + (self.group5().zxy() * self.group1().yzx()).with_w(self[e2] * self[e25])
                + (anti_reverse_g3.xyz() * self.group1().www()).with_w(self[e321] * self[e3215])
                - (Simd32x4::from([self[e2], self[e321], self[e321], self[e12]]) * anti_reverse_g5.zyz().with_w(anti_reverse_g8[2]))
                - (Simd32x4::from([self[e5], self[e125], self[e235], self[e125]]) * anti_reverse_g4.xxy().with_w(anti_reverse_g5[2]))
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e4315]]) * anti_reverse_g7.zyz().with_w(self[e315]))
                - (Simd32x4::from([self[e321], self[e3], self[e1], self[e31]]) * anti_reverse_g5.xxy().with_w(anti_reverse_g8[1]))
                - (Simd32x4::from([self[e315], self[e5], self[e5], self[e23]]) * anti_reverse_g4.zyz().with_w(anti_reverse_g8[0]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e4235]]) * anti_reverse_g7.xxy().with_w(self[e235]))
                - (self.group0().xx().with_zw(self[scalar], anti_reverse_g5[0]) * self.group1().xyz().with_w(self[e235]))
                - (self.group0().xx().with_zw(self[scalar], anti_reverse_g5[1]) * self.group1().xyz().with_w(self[e315]))
                - (anti_reverse_g6.ww().with_zw(self[e2], self[e1]) * self.group5().xyx().with_w(anti_reverse_g3[0]))
                - (self.group1().zx().with_zw(anti_reverse_g6[3], self[e415]) * self.group5().yzz().with_w(anti_reverse_g3[0]))
                - (Simd32x3::from(self[e1234]) * self.group8()).with_w(anti_reverse_g3[1] * self[e425])
                - (anti_reverse_g8.yzx() * self.group4().zxy()).with_w(self[e125] * self[e4125])
                - (self.group7().zxy() * anti_reverse_g3.yzx()).with_w(anti_reverse_g3[1] * self[e2])
                - (anti_reverse_g6.yzx() * self.group9().zxy()).with_w(anti_reverse_g3[2] * self[e435])
                - (self.group6().zxy() * self.group9().yzx()).with_w(anti_reverse_g6[0] * self[e15])
                - (self.group3().xyz() * self.group1().www()).with_w(anti_reverse_g3[2] * self[e3]),
            // e1234
            (anti_reverse_g4[0] * self[e1])
                + (anti_reverse_g4[1] * self[e2])
                + (anti_reverse_g4[2] * self[e3])
                + (anti_reverse_g6[3] * self[e1234])
                + 2.0 * (self[scalar] * self[e4])
                + 2.0 * (self[e12345] * self[e1234])
                + (self[e4] * self[e45])
                + (self[e423] * self[e4235])
                + (self[e431] * self[e4315])
                + (self[e412] * self[e4125])
                - (anti_reverse_g4[0] * self[e415])
                - (anti_reverse_g4[1] * self[e425])
                - (anti_reverse_g4[2] * self[e435])
                - (anti_reverse_g5[0] * self[e423])
                - (anti_reverse_g5[1] * self[e431])
                - (anti_reverse_g5[2] * self[e412])
                - (anti_reverse_g7[0] * self[e23])
                - (anti_reverse_g7[0] * self[e4235])
                - (anti_reverse_g7[1] * self[e31])
                - (anti_reverse_g7[1] * self[e4315])
                - (anti_reverse_g7[2] * self[e12])
                - (anti_reverse_g7[2] * self[e4125])
                - (anti_reverse_g3[3] * self[e4])
                - (anti_reverse_g6[0] * self[e41])
                - (anti_reverse_g6[1] * self[e42])
                - (anti_reverse_g6[2] * self[e43])
                - (self[e1] * self[e41])
                - (self[e2] * self[e42])
                - (self[e3] * self[e43])
                - (self[e321] * self[e1234]),
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
    //      f32       27       35        0
    //    simd3        0        3        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       39       50        0
    //  no simd       75       92        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[1] * self[e412]) + (self[e12345] * self[e1]) + (self[e425] * self[e3]) + (self[e235] * self[e4]),
                (anti_reverse_g2[2] * self[e423]) + (self[e12345] * self[e2]) + (self[e435] * self[e1]) + (self[e315] * self[e4]),
                (anti_reverse_g2[0] * self[e431]) + (self[e12345] * self[e3]) + (self[e415] * self[e2]) + (self[e125] * self[e4]),
                -(self[e423] * self[e1]) - (self[e431] * self[e2]) - (self[e412] * self[e3]) - (self[e321] * self[e4]),
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * anti_reverse_g1.xxy().with_w(anti_reverse_g0[3]))
                + (Simd32x4::from([self[e2], self[e321], self[e321], self[e4]]) * anti_reverse_g1.zyzw())
                + (anti_reverse_g0.xxyx() * self.group2().wzx().with_w(self[e1]))
                + (anti_reverse_g0.zyzy() * self.group2().yww().with_w(self[e2]))
                + (anti_reverse_g0.wwwz() * self.group3().xyzz())
                + (self.group1().xyz() * anti_reverse_g1.www()).with_w(self[e12345] * self[e4])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * anti_reverse_g2.zyz().with_w(anti_reverse_g1[0]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e435]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g0[2]))
                - (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xyzy() * anti_reverse_g2.www().with_w(anti_reverse_g1[1]))
                - (anti_reverse_g1.yzx() * self.group3().zxy()).with_w(anti_reverse_g0[1] * self[e425])
                - (self.group1().zxy() * self.group3().yzx()).with_w(anti_reverse_g1[2] * self[e412]),
            // e5
            (anti_reverse_g0[3] * self[e5])
                + (anti_reverse_g2[3] * self[e12345])
                + (anti_reverse_g2[3] * self[e321])
                + (self[e235] * self[e1])
                + (self[e315] * self[e2])
                + (self[e125] * self[e3])
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
    //      f32       15       19        0
    //    simd3        0        3        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       30       38        0
    //  no simd       75       92        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * anti_reverse_g2.zyz().with_w(anti_reverse_g1[0]))
                + (Simd32x4::from([self[scalar], self[e12], self[e23], self[e42]]) * self.group3().xxy().with_w(anti_reverse_g1[1]))
                + (Simd32x4::from([self[e31], self[scalar], self[scalar], self[e43]]) * self.group3().zyz().with_w(anti_reverse_g1[2]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e31]]) * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4125]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g0[2]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e4315]]) * anti_reverse_g1.zyz().with_w(anti_reverse_g0[1]))
                + (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e23]))
                + (anti_reverse_g0.wwwx() * self.group3().xyzx())
                + (self.group1().xyzz() * anti_reverse_g1.www().with_w(anti_reverse_g0[2]))
                + (self.group0().xyz() * self.group3().www()).with_w(anti_reverse_g1[3] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * anti_reverse_g0.zyz().with_w(anti_reverse_g2[3]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse_g0.xxyw())
                - (self.group0().zxyx() * anti_reverse_g2.yzx().with_w(self[e4235]))
                - (self.group3().yzxz() * self.group1().zxy().with_w(self[e43]))
                - (anti_reverse_g1.yzx() * self.group3().zxy()).with_w(anti_reverse_g2[3] * self[e45])
                - (self.group2().xyz() * anti_reverse_g2.www()).with_w(self[e42] * self[e4315]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12])
                + (self[e45] * self[e3215])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[3] * self[e3215])
                - (anti_reverse_g2[0] * self[e4235])
                - (anti_reverse_g2[1] * self[e4315])
                - (anti_reverse_g2[2] * self[e4125])
                - (self[scalar] * self[e3215]),
        )
    }
}
