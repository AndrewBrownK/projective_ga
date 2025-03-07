// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 59
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7      11       0
//  Average:        13      19       0
//  Maximum:       169     205       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        15      23       0
//  Average:        24      35       0
//  Maximum:       334     372       0
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            (anti_reverse_g0[0] * self[e23])
                + (anti_reverse_g0[1] * self[e31])
                + (anti_reverse_g0[2] * self[e12])
                + (anti_reverse_g1[0] * self[e41])
                + (anti_reverse_g1[1] * self[e42])
                + (anti_reverse_g1[2] * self[e43]),
        );
    }
}
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       19       34        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g0[2] * self[e25]) - (anti_reverse_g2[1] * self[e43]),
                -(anti_reverse_g0[0] * self[e35]) - (anti_reverse_g2[2] * self[e41]),
                -(anti_reverse_g0[1] * self[e15]) - (anti_reverse_g2[0] * self[e42]),
                (anti_reverse_g0[2] * self[e12]) + (anti_reverse_g1[0] * self[e41]) + (anti_reverse_g1[1] * self[e42]) + (anti_reverse_g1[2] * self[e43]),
            ]) + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e23])
                + (self.group0().yzx() * anti_reverse_g2.zxy()).with_w(anti_reverse_g0[1] * self[e31]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Infinity::from_groups(
            // e5
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       15       25        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g0 = Simd32x4::from([(anti_reverse_g0[2] * self[e12]) - (anti_reverse_g0[3] * self[e45]) - (anti_reverse_g1[3] * self[scalar]), 0.0, 0.0, 0.0])
            + (anti_reverse_g0.xxyz() * self.group0().xwww())
            + (anti_reverse_g0.ywww() * self.group0().yxyz());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiCircleRotorOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Origin::from_groups(
            // e4
            (anti_reverse_g1[0] * self[e41])
                + (anti_reverse_g1[1] * self[e42])
                + (anti_reverse_g1[2] * self[e43])
                + (anti_reverse_g0[0] * self[e23])
                + (anti_reverse_g0[1] * self[e31])
                + (anti_reverse_g0[2] * self[e12]),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       24        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       25       29        0
    //  no simd       34       43        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let geometric_anti_product_g0 = Simd32x4::from([
            -self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3] - (anti_reverse_g0[2] * self[e435]),
            (anti_reverse_g0[2] * self[e2]) + (anti_reverse_g0[3] * self[e415]) + (self[e425] * self[e3]),
            (anti_reverse_g0[1] * self[e321]) + (anti_reverse_g0[3] * self[e425]) + (self[e435] * self[e1]),
            (anti_reverse_g0[2] * self[e321]) + (anti_reverse_g0[3] * self[e435]) + (self[e415] * self[e2]),
        ]) + (anti_reverse_g0.wxxy() * self.group0().ww().with_zw(self[e3], self[e1]))
            - (Simd32x4::from([anti_reverse_g0[1], self[e2], self[e415], self[e425]]) * self.group0().yz().with_zw(self[e3], self[e1]))
            - (Simd32x4::from([self[e415], self[e3], self[e1], self[e2]]) * anti_reverse_g0.xyzx());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (self[e321] * self[e5]) + (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[0] * self[e1])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[1] * self[e2])
                - (anti_reverse_g1[2] * self[e435])
                - (anti_reverse_g1[2] * self[e3])
                - (anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g0[3] * self[e5]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            (anti_reverse_g0[0] * self[e1]) + (anti_reverse_g0[1] * self[e2]) + (anti_reverse_g0[2] * self[e3]) + (anti_reverse_g0[3] * self[e4])
                - (self[e423] * self[e1])
                - (self[e431] * self[e2])
                - (self[e412] * self[e3])
                - (self[e321] * self[e4]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        0        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       31       47        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[0] * self[e5]) + (anti_reverse_g0[2] * self[e315]) + (anti_reverse_g2[1] * self[e412]) + (anti_reverse_g2[3] * self[e235]),
                (anti_reverse_g0[0] * self[e125]) + (anti_reverse_g0[1] * self[e5]) + (anti_reverse_g2[2] * self[e423]) + (anti_reverse_g2[3] * self[e315]),
                (anti_reverse_g0[1] * self[e235]) + (anti_reverse_g0[2] * self[e5]) + (anti_reverse_g2[0] * self[e431]) + (anti_reverse_g2[3] * self[e125]),
                -(anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435]),
            ]) - (Simd32x4::from([self[e431], self[e4], self[e4], anti_reverse_g0[0]]) * anti_reverse_g2.zyz().with_w(self[e415]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g1[2]))
                - (self.group0().xyzy() * anti_reverse_g0.www().with_w(anti_reverse_g1[1]))
                - (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g1[0] * self[e423]),
            // e5
            -(anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e435]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiDualNum {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * self[scalar] * -2.0);
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiFlector {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3]) + (self[e321] * self[e5])
                - (anti_reverse_g0[0] * self[e1])
                - (anti_reverse_g0[1] * self[e2])
                - (anti_reverse_g0[2] * self[e3])
                - (anti_reverse_g0[3] * self[e5]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiLine {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMotor {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[3] * self[scalar]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * anti_reverse_g0.xyz()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       11        0
    //  no simd       15       22        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([
                (self[e425] * self[e3]) - (self[e435] * self[e2]),
                (self[e435] * self[e1]) - (self[e415] * self[e3]),
                (self[e415] * self[e2]) - (self[e425] * self[e1]),
            ]) + (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * anti_reverse_g0.xxy())
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * anti_reverse_g0.zyz())
                - (self.group1().zxy() * anti_reverse_g0.yzx()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for AntiVersorEvenOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            (anti_reverse_g0[0] * self[e23])
                + (anti_reverse_g0[1] * self[e31])
                + (anti_reverse_g0[2] * self[e12])
                + (anti_reverse_g1[0] * self[e41])
                + (anti_reverse_g1[1] * self[e42])
                + (anti_reverse_g1[2] * self[e43])
                - (anti_reverse_g0[3] * self[e1234])
                - (anti_reverse_g1[3] * self[scalar]),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAligningOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       19       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[2] * self[e315]) + (anti_reverse_g2[1] * self[e412]),
                (anti_reverse_g0[0] * self[e125]) + (anti_reverse_g2[2] * self[e423]),
                (anti_reverse_g0[1] * self[e235]) + (anti_reverse_g2[0] * self[e431]),
                -(anti_reverse_g0[2] * self[e435]) - (anti_reverse_g1[0] * self[e423]) - (anti_reverse_g1[1] * self[e431]) - (anti_reverse_g1[2] * self[e412]),
            ]) - (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g2.zxy() * self.group0().yzx()).with_w(anti_reverse_g0[1] * self[e425]),
            // e5
            -(anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e435]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       23        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let geometric_anti_product_g0 = Simd32x4::from([
            -(anti_reverse_g0[0] * self[e415]) - (anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435]),
            anti_reverse_g0[3] * self[e415],
            anti_reverse_g0[3] * self[e425],
            anti_reverse_g0[3] * self[e435],
        ]) + (Simd32x4::from(self[e321]) * anti_reverse_g0.wxyz());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            -(anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435])
                - (anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        6        0
    // no simd        9       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.zxy() * self.group1().yzx()) + (anti_reverse_g1.yzx() * self.group0().zxy())
                - (anti_reverse_g0.yzx() * self.group1().zxy())
                - (anti_reverse_g1.zxy() * self.group0().yzx()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            -(anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g0[1] * self[e425])
                - (anti_reverse_g0[2] * self[e435])
                - (anti_reverse_g1[0] * self[e423])
                - (anti_reverse_g1[1] * self[e431])
                - (anti_reverse_g1[2] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       19        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g1.yzx() * self.group0().zxy()) + (self.group1().yzx() * anti_reverse_g0.zxy())
                - (anti_reverse_g1.zxy() * self.group0().yzx())
                - (self.group1().zxy() * anti_reverse_g0.yzx()),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       19       34        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[2] * self[e315]) + (anti_reverse_g2[1] * self[e412]),
                (anti_reverse_g0[0] * self[e125]) + (anti_reverse_g2[2] * self[e423]),
                (anti_reverse_g0[1] * self[e235]) + (anti_reverse_g2[0] * self[e431]),
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Infinity::from_groups(
            // e5
            -(anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       12       25        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g0 = Simd32x4::from([
            (anti_reverse_g1[3] * self[e12345]) - (anti_reverse_g0[0] * self[e415]) - (anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435]),
            anti_reverse_g0[3] * self[e415],
            anti_reverse_g0[3] * self[e425],
            anti_reverse_g0[3] * self[e435],
        ]) + (Simd32x4::from(self[e321]) * anti_reverse_g0.wxyz());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            -(anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Origin::from_groups(
            // e4
            -(anti_reverse_g1[0] * self[e423])
                - (anti_reverse_g1[1] * self[e431])
                - (anti_reverse_g1[2] * self[e412])
                - (anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g0[1] * self[e425])
                - (anti_reverse_g0[2] * self[e435]),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       19        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g1.zxy() * self.group0().yzx()) + (self.group1().zxy() * anti_reverse_g0.yzx())
                - (anti_reverse_g1.yzx() * self.group0().zxy())
                - (self.group1().yzx() * anti_reverse_g0.zxy()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       14       23        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let geometric_anti_product_g0 = Simd32x4::from([(anti_reverse_g0[2] * self[e12]) - (anti_reverse_g0[3] * self[e45]), 0.0, 0.0, 0.0])
            + (anti_reverse_g0.xxyz() * self.group0().xwww())
            + (anti_reverse_g0.ywww() * self.group0().yxyz());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12])
                + (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        6        0
    // no simd        9       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.yzx() * self.group1().zxy()) + (anti_reverse_g1.zxy() * self.group0().yzx())
                - (anti_reverse_g0.zxy() * self.group1().yzx())
                - (anti_reverse_g1.yzx() * self.group0().zxy()),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        0        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       35       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e4125]]) * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e4315]]) * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                + (anti_reverse_g0.yzxx() * self.group1().zxy().with_w(self[e4235]))
                + (self.group0().xyz() * self.group2().www()).with_w(anti_reverse_g0[3] * self[e1234])
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[e41]]) * anti_reverse_g0.zyz().with_w(self[e4235]))
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e45]]) * anti_reverse_g0.xxy().with_w(anti_reverse_g1[3]))
                - (self.group0().zxyy() * anti_reverse_g1.yzx().with_w(self[e4315]))
                - (self.group1().xyz() * anti_reverse_g1.www()).with_w(self[e43] * self[e4125]),
            // e5
            (self[e45] * self[e3215]) + (self[e15] * self[e4235]) + (self[e25] * self[e4315]) + (self[e35] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[0] * self[e4235])
                - (anti_reverse_g1[1] * self[e4315])
                - (anti_reverse_g1[2] * self[e4125]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       17        0
    //    simd3        0        1        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       19       25        0
    //  no simd       34       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let geometric_anti_product_g0 = (Simd32x4::from([
            self[e4315] * self[e4315] + self[e4125] * self[e4125],
            self[e12] * self[e4315],
            self[e23] * self[e4125],
            self[e31] * self[e4235],
        ]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]))
            + (Simd32x4::from([self[e31], self[e4315], self[e45], self[e45]]) * anti_reverse_g0.yzyz())
            + (Simd32x4::from([self[e4235], self[e31], self[e12], self[e23]]) * self.group2().xzxy())
            + (anti_reverse_g0.xxxy() * self.group0().xw().with_zw(self[e4125], self[e4235]))
            + (anti_reverse_g0.zwww() * self.group0().zxyz())
            - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse_g0.wyzx());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (anti_reverse_g1[0] * self[e23])
                + (anti_reverse_g1[1] * self[e31])
                + (anti_reverse_g1[2] * self[e12])
                + (anti_reverse_g0[0] * self[e15])
                + (anti_reverse_g0[1] * self[e25])
                + (anti_reverse_g0[2] * self[e35])
                + (self[e45] * self[e3215])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                - (anti_reverse_g1[0] * self[e4235])
                - (anti_reverse_g1[1] * self[e4315])
                - (anti_reverse_g1[2] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd       21       32        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz())
                + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * anti_reverse_g1.zyz())
                + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * anti_reverse_g1.xxy())
                + (anti_reverse_g0.yzx() * self.group1().zxy())
                - (Simd32x3::from(anti_reverse_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e3215], self[e35], self[e15]]) * anti_reverse_g0.xxy())
                - (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * anti_reverse_g0.zyz())
                - (anti_reverse_g1.yzx() * self.group0().zxy()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            (anti_reverse_g0[0] * self[e4235]) + (anti_reverse_g0[1] * self[e4315]) + (anti_reverse_g0[2] * self[e4125]) + (anti_reverse_g0[3] * self[e1234])
                - (self[e41] * self[e4235])
                - (self[e42] * self[e4315])
                - (self[e43] * self[e4125])
                - (self[e45] * self[e1234]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        0        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       31       47        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g0[0] * self[e3215]) - (anti_reverse_g0[2] * self[e25]) - (anti_reverse_g2[1] * self[e43]) - (anti_reverse_g2[3] * self[e15]),
                -(anti_reverse_g0[0] * self[e35]) - (anti_reverse_g0[1] * self[e3215]) - (anti_reverse_g2[2] * self[e41]) - (anti_reverse_g2[3] * self[e25]),
                -(anti_reverse_g0[1] * self[e15]) - (anti_reverse_g0[2] * self[e3215]) - (anti_reverse_g2[0] * self[e42]) - (anti_reverse_g2[3] * self[e35]),
                (anti_reverse_g0[1] * self[e31]) + (anti_reverse_g0[2] * self[e12]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], anti_reverse_g0[0]]) * anti_reverse_g2.zyz().with_w(self[e23]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g1[2]))
                + (self.group0().xyzy() * anti_reverse_g0.www().with_w(anti_reverse_g1[1]))
                + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g1[0] * self[e41]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       19       33        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g0[2] * self[e25]) - (anti_reverse_g2[1] * self[e43]),
                -(anti_reverse_g0[0] * self[e35]) - (anti_reverse_g2[2] * self[e41]),
                -(anti_reverse_g0[1] * self[e15]) - (anti_reverse_g2[0] * self[e42]),
                (anti_reverse_g0[2] * self[e12]) + (anti_reverse_g1[0] * self[e41]) + (anti_reverse_g1[1] * self[e42]) + (anti_reverse_g1[2] * self[e43]),
            ]) + (anti_reverse_g0.yzx() * self.group2().zxy()).with_w(anti_reverse_g0[0] * self[e23])
                + (anti_reverse_g2.zxy() * self.group0().yzx()).with_w(anti_reverse_g0[1] * self[e31]),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for DualNum {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * self[e12345] * 2.0);
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Flector {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            (self[e15] * self[e4235]) + (self[e25] * self[e4315]) + (self[e35] * self[e4125]) + (self[e45] * self[e3215])
                - (anti_reverse_g0[0] * self[e4235])
                - (anti_reverse_g0[1] * self[e4315])
                - (anti_reverse_g0[2] * self[e4125])
                - (anti_reverse_g0[3] * self[e3215]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Line {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Line {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            -(anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for Motor {
    type Output = Infinity;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Infinity;
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
        return Infinity::from_groups(
            // e5
            (anti_reverse_g0[3] * self[e5]) + (anti_reverse_g1[3] * self[e12345])
                - (anti_reverse_g0[0] * self[e235])
                - (anti_reverse_g0[1] * self[e315])
                - (anti_reverse_g0[2] * self[e125])
                - (anti_reverse_g1[0] * self[e415])
                - (anti_reverse_g1[1] * self[e425])
                - (anti_reverse_g1[2] * self[e435]),
        );
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
    //      f32      114      144        0
    //    simd3        0       16        0
    //    simd4       55       45        0
    // Totals...
    // yes simd      169      205        0
    //  no simd      334      372        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g3 = self.group3() * Simd32x4::from(-1.0);
        let anti_reverse_g4 = self.group4() * Simd32x3::from(-1.0);
        let anti_reverse_g5 = self.group5() * Simd32x3::from(-1.0);
        let anti_reverse_g6 = self.group6() * Simd32x4::from(-1.0);
        let anti_reverse_g7 = self.group7() * Simd32x3::from(-1.0);
        let anti_reverse_g8 = self.group8() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self.group1().xwzw()[0] * self[e4235])
                    + (self.group9().xwzw()[0] * self[e5])
                    + (self.group9().zyzw()[0] * self[e2])
                    + (self.group9().wzzw()[0] * self[e3])
                    + 2.0 * (self[scalar] * self[e12345])
                    + (self[e2] * self[e4315])
                    + (self[e3] * self[e4125])
                    + (self[e5] * self[e1234])
                    - (anti_reverse_g4[0] * self[e423])
                    - (anti_reverse_g4[1] * self[e431])
                    - (anti_reverse_g4[2] * self[e412])
                    - (anti_reverse_g5[0] * self[e415])
                    - (anti_reverse_g5[1] * self[e425])
                    - (anti_reverse_g5[2] * self[e435])
                    - (anti_reverse_g7[0] * self[e15])
                    - (anti_reverse_g7[1] * self[e25])
                    - (anti_reverse_g7[2] * self[e35])
                    - (anti_reverse_g8[0] * self.group3().xwzw()[0])
                    - (anti_reverse_g8[1] * self[e42])
                    - (anti_reverse_g8[2] * self[e43])
                    - (anti_reverse_g3[0] * self[e235])
                    - (anti_reverse_g3[1] * self[e315])
                    - (anti_reverse_g3[2] * self[e125])
                    - (anti_reverse_g3[3] * self[e321])
                    - (anti_reverse_g6[0] * self[e23])
                    - (anti_reverse_g6[1] * self[e31])
                    - (anti_reverse_g6[2] * self[e12])
                    - (anti_reverse_g6[3] * self[e45]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g6[0] * self[e321]) + (anti_reverse_g6[2] * self[e2]) + (anti_reverse_g6[3] * self[e415]) + (self[e41] * self[e3215]),
                (anti_reverse_g6[0] * self[e3]) + (anti_reverse_g6[1] * self[e321]) + (anti_reverse_g6[3] * self[e425]) + (self[e42] * self[e3215]),
                (anti_reverse_g6[1] * self[e1]) + (anti_reverse_g6[2] * self[e321]) + (anti_reverse_g6[3] * self[e435]) + (self[e43] * self[e3215]),
                -(self[e41] * self[e4235]) - (self[e42] * self[e4315]) - (self[e43] * self[e4125]) - (self[e45] * self[e1234]),
            ]) + (Simd32x4::from([self[e5], self[e125], self[e235], anti_reverse_g3[0]]) * anti_reverse_g7.xxy().with_w(self[e23]))
                + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e1]]) * anti_reverse_g4.zyz().with_w(anti_reverse_g7[0]))
                + (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e2]]) * anti_reverse_g5.xxy().with_w(anti_reverse_g7[1]))
                + (Simd32x4::from([self[e315], self[e5], self[e5], anti_reverse_g3[1]]) * anti_reverse_g7.zyz().with_w(self[e31]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e43]]) * anti_reverse_g4.xxy().with_w(anti_reverse_g5[2]))
                + (Simd32x4::from([self[e4315], self[e45], self[e45], self[e3]]) * anti_reverse_g5.zyz().with_w(anti_reverse_g7[2]))
                + (self.group0().xx().with_zw(self[scalar], anti_reverse_g5[0]) * self.group9().yzw().with_w(self[e41]))
                + (self.group0().xx().with_zw(self[scalar], self[e12345]) * self.group9().yzw().with_w(self[e4]))
                + (self.group0().yy().with_zw(self[e12345], anti_reverse_g5[1]) * self.group1().xyz().with_w(self[e42]))
                + (self.group0().yy().with_zw(self[e12345], self[e12345]) * self.group1().xyz().with_w(self[e4]))
                + (anti_reverse_g3.ww().with_zw(self[e4315], self[e4315]) * self.group5().xyx().with_w(anti_reverse_g3[1]))
                + (self.group9().wy().with_zw(anti_reverse_g3[3], self[e4125]) * self.group5().yzz().with_w(anti_reverse_g3[2]))
                + (self.group8() * self.group1().www()).with_w(anti_reverse_g3[3] * self[e1234])
                + (anti_reverse_g8.yzx() * self.group7().zxy()).with_w(anti_reverse_g3[2] * self[e12])
                + (self.group4().zxy() * anti_reverse_g3.yzx()).with_w(anti_reverse_g3[0] * self[e4235])
                + (self.group1().zxy() * self.group6().yzx()).with_w(anti_reverse_g6[3] * self[e4])
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * anti_reverse_g8.xxy().with_w(anti_reverse_g7[1]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * anti_reverse_g8.zyz().with_w(anti_reverse_g7[2]))
                - (self.group9().xx().with_zw(anti_reverse_g3[1], self[e1]) * self.group4().xyx().with_w(self[e423]))
                - (anti_reverse_g3.zx().with_zw(self[e1234], anti_reverse_g6[0]) * self.group4().yzz().with_w(self[e423]))
                - (Simd32x3::from(self[e5]) * self.group7()).with_w(anti_reverse_g6[1] * self[e431])
                - (Simd32x3::from(self[e3215]) * anti_reverse_g3.xyz()).with_w(anti_reverse_g6[2] * self[e412])
                - (anti_reverse_g4.yzx() * self.group3().zxy()).with_w(self[scalar] * self[e1234])
                - (anti_reverse_g5.yzx() * self.group9().wyz()).with_w(self[scalar] * self[e1234])
                - (anti_reverse_g7.yzx() * self.group8().zxy()).with_w(anti_reverse_g7[0] * self[e415])
                - (self.group5().zxy() * self.group9().zwy()).with_w(self[e2] * self[e431])
                - (anti_reverse_g6.yzx() * self.group1().zxy()).with_w(self[e4] * self[e321])
                - (self.group1().yzx() * self.group6().zxy()).with_w(self[e3] * self[e412]),
            // e5
            (anti_reverse_g4[0] * self[e23])
                + (anti_reverse_g4[1] * self[e31])
                + (anti_reverse_g4[2] * self[e12])
                + (anti_reverse_g5[0] * self[e15])
                + (anti_reverse_g5[1] * self[e25])
                + (anti_reverse_g5[2] * self[e35])
                + 2.0 * (self[e12345] * self[e5])
                + (self[e1] * self[e235])
                + (self[e2] * self[e315])
                + (self[e3] * self[e125])
                + (self[e5] * self[e321])
                + (self[e45] * self[e3215])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                - (anti_reverse_g4[0] * self[e4235])
                - (anti_reverse_g4[1] * self[e4315])
                - (anti_reverse_g4[2] * self[e4125])
                - (anti_reverse_g8[0] * self[e1])
                - (anti_reverse_g8[0] * self[e415])
                - (anti_reverse_g8[1] * self[e2])
                - (anti_reverse_g8[1] * self[e425])
                - (anti_reverse_g8[2] * self[e3])
                - (anti_reverse_g8[2] * self[e435])
                - (anti_reverse_g3[3] * self[e3215])
                - (anti_reverse_g6[0] * self[e235])
                - (anti_reverse_g6[1] * self[e315])
                - (anti_reverse_g6[2] * self[e125])
                - (anti_reverse_g6[3] * self[e5])
                - 2.0 * (self[scalar] * self[e3215]),
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
            Simd32x4::from([
                (self[e412] * self[e4125])
                    - (anti_reverse_g7[2] * self[e4125])
                    - (anti_reverse_g6[0] * self[e41])
                    - (anti_reverse_g6[1] * self[e42])
                    - (anti_reverse_g6[2] * self[e43]),
                (anti_reverse_g7[1] * self[e35]) + (anti_reverse_g3[3] * self[e415]) + (anti_reverse_g6[2] * self[e4315]) + (self[e2] * self[e12]) + (self[e5] * self[e41]),
                (anti_reverse_g3[3] * self[e425]) + (anti_reverse_g6[0] * self[e4125]) + (anti_reverse_g6[1] * self[e45]) + (self[e5] * self[e42]) + (self[e435] * self[e4235]),
                (anti_reverse_g3[3] * self[e435]) + (anti_reverse_g6[1] * self[e4235]) + (anti_reverse_g6[2] * self[e45]) + (self[e5] * self[e43]) + (self[e415] * self[e4315]),
            ]) + (Simd32x4::from([anti_reverse_g3[0], anti_reverse_g4[0], self[e1234], self[e1234]]) * self.group1().xw().with_zw(anti_reverse_g8[1], anti_reverse_g8[2]))
                + (Simd32x4::from([anti_reverse_g3[1], anti_reverse_g5[1], self[e3], self[e1]]) * self.group1().yz().with_zw(self[e23], self[e31]))
                + (Simd32x4::from([self[e12345], anti_reverse_g8[0], self[e4315], self[e4125]]) * self.group9().xx().with_zw(self[e12345], self[e12345]))
                + (Simd32x4::from([self[e12345], self[e425], self[e4], self[e4]]) * self.group9().xw().with_zw(anti_reverse_g4[1], anti_reverse_g4[2]))
                + (Simd32x4::from([self[e3], self[e125], self[e3215], self[e3215]]) * anti_reverse_g3.zy().with_zw(self[e431], self[e412]))
                + (Simd32x4::from([self[e4], anti_reverse_g8[2], self[e43], self[e41]]) * self.group3().wy().with_zw(anti_reverse_g8[0], anti_reverse_g8[1]))
                + (Simd32x4::from([self[e4], self[e4235], self[e412], self[e423]]) * self.group0().with_zw(anti_reverse_g4[0], anti_reverse_g4[1]))
                + (Simd32x4::from([self[e4], self[e4235], self[e4315], self[e4125]]) * self.group0().with_zw(self[e12345], self[e12345]))
                + (Simd32x4::from([self[e1234], self[e45], anti_reverse_g3[2], anti_reverse_g3[0]]) * anti_reverse_g6.wx().with_zw(self[e235], self[e315]))
                + (Simd32x4::from([self[e4235], anti_reverse_g4[2], self[e1], self[e2]]) * self.group7().xy().with_zw(anti_reverse_g5[2], anti_reverse_g5[0]))
                + (Simd32x4::from([self[e4315], self[e3215], self[e15], self[e25]]) * self.group7().yx().with_zw(anti_reverse_g7[2], anti_reverse_g7[0]))
                - (Simd32x4::from([anti_reverse_g5[0], anti_reverse_g4[1], self[e2], self[e3]]) * self.group7().xz().with_zw(self[scalar], self[scalar]))
                - (Simd32x4::from([anti_reverse_g7[1], anti_reverse_g6[1], self[e3215], self[e3215]]) * self.group9().zw().with_zw(anti_reverse_g7[1], anti_reverse_g7[2]))
                - (Simd32x4::from([anti_reverse_g7[1], anti_reverse_g6[3], self[e35], self[e15]]) * self.group5().yx().with_zw(anti_reverse_g7[0], anti_reverse_g7[1]))
                - (Simd32x4::from([anti_reverse_g7[2], self[e3], self[e41], self[e42]]) * self.group5().zy().with_zw(anti_reverse_g8[2], anti_reverse_g8[0]))
                - (Simd32x4::from([anti_reverse_g3[2], self[e4315], self[e5], self[e5]]) * self.group6().zz().with_zw(anti_reverse_g3[1], anti_reverse_g3[2]))
                - (Simd32x4::from([anti_reverse_g3[3], self[scalar], self[e4235], self[e4315]]) * self.group1().wx().with_zw(anti_reverse_g6[2], anti_reverse_g6[0]))
                - (Simd32x4::from([self[e2], anti_reverse_g8[1], anti_reverse_g6[3], self[e2]]) * self.group3().yz().with_zw(self[e31], self[e23]))
                - (Simd32x4::from([self[e41], self[scalar], self[e4], self[e4]]) * self.group1().xx().with_zw(self[e25], self[e35]))
                - (Simd32x4::from([self[e43], self[e15], self[e1], anti_reverse_g6[3]]) * self.group1().zw().with_zw(self[e12], self[e12]))
                - (Simd32x4::from([self[e23], self[e3215], self[e3], self[e1]]) * anti_reverse_g7.xx().with_zw(anti_reverse_g5[0], anti_reverse_g5[1]))
                - (Simd32x4::from([self[e415], self[e315], self[e1234], anti_reverse_g3[1]]) * anti_reverse_g3.xz().with_zw(self[e315], self[e235]))
                - (Simd32x4::from([self[e425], self[e5], anti_reverse_g3[0], self[e1234]]) * anti_reverse_g3.yx().with_zw(self[e125], self[e125]))
                - (Simd32x4::from([self[e321], self[e235], self[e415], self[e425]]) * self.group9().xxwy())
                - (Simd32x4::from([self[e431], self[e321], self[e2], self[e3]]) * anti_reverse_g5.yx().with_zw(self[scalar], self[scalar]))
                - (Simd32x4::from([self[e412], self[e2], self[e423], self[e431]]) * anti_reverse_g5.zz().with_zw(anti_reverse_g4[2], anti_reverse_g4[0]))
                - (Simd32x4::from([self[e4235], self[e25], self[e321], self[e321]]) * anti_reverse_g7.xz().with_zw(anti_reverse_g5[1], anti_reverse_g5[2])),
            // e3215
            (anti_reverse_g8[0] * self[e4235])
                + (anti_reverse_g8[1] * self[e4315])
                + (anti_reverse_g8[2] * self[e4125])
                + (anti_reverse_g3[3] * self[e5])
                + 2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + (self[e1] * self[e15])
                + (self[e2] * self[e25])
                + (self[e3] * self[e35])
                + (self[e321] * self[e3215])
                - (anti_reverse_g4[0] * self[e1])
                - (anti_reverse_g4[0] * self[e415])
                - (anti_reverse_g4[1] * self[e2])
                - (anti_reverse_g4[1] * self[e425])
                - (anti_reverse_g4[2] * self[e3])
                - (anti_reverse_g4[2] * self[e435])
                - (anti_reverse_g5[0] * self[e235])
                - (anti_reverse_g5[1] * self[e315])
                - (anti_reverse_g5[2] * self[e125])
                - (anti_reverse_g8[0] * self[e23])
                - (anti_reverse_g8[1] * self[e31])
                - (anti_reverse_g8[2] * self[e12])
                - (anti_reverse_g6[0] * self[e15])
                - (anti_reverse_g6[1] * self[e25])
                - (anti_reverse_g6[2] * self[e35])
                - (anti_reverse_g6[3] * self[e3215])
                - (self[e5] * self[e45])
                - (self[e235] * self[e4235])
                - (self[e315] * self[e4315])
                - (self[e125] * self[e4125]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * anti_reverse_g0.xyz()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * anti_reverse_g0.xyz()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryDipole {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       10        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * anti_reverse_g0.xyz()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       11        0
    //  no simd       15       22        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([
                (self[e31] * self[e4125]) - (self[e12] * self[e4315]),
                (self[e12] * self[e4235]) - (self[e23] * self[e4125]),
                (self[e23] * self[e4315]) - (self[e31] * self[e4235]),
            ]) + (Simd32x3::from(anti_reverse_g0[3]) * self.group0().xyz())
                + (Simd32x3::from([self[e45], self[e4125], self[e4235]]) * anti_reverse_g0.xxy())
                + (Simd32x3::from([self[e4315], self[e45], self[e45]]) * anti_reverse_g0.zyz())
                - (self.group1().zxy() * anti_reverse_g0.yzx()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       18       28        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                + Simd32x3::from([
                    (self[e3] * self[e425]) - (self[e2] * self[e435]),
                    (self[e1] * self[e435]) - (self[e3] * self[e415]),
                    (self[e2] * self[e415]) - (self[e1] * self[e425]),
                ])
                + (Simd32x3::from(anti_reverse_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * anti_reverse_g1.zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * anti_reverse_g1.xxy())
                - (Simd32x3::from([self[e3], self[e1], self[e2]]) * anti_reverse_g1.yzx()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       18       28        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]))
                + Simd32x3::from([
                    (self[e4125] * self[e31]) - (self[e4315] * self[e12]),
                    (self[e4235] * self[e12]) - (self[e4125] * self[e23]),
                    (self[e4315] * self[e23]) - (self[e4235] * self[e31]),
                ])
                + (Simd32x3::from(anti_reverse_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([self[e4315], self[e45], self[e45]]) * anti_reverse_g1.zyz())
                + (Simd32x3::from([self[e45], self[e4125], self[e4235]]) * anti_reverse_g1.xxy())
                - (Simd32x3::from([self[e4125], self[e4235], self[e4315]]) * anti_reverse_g1.yzx()),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        0        1        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       35       52        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g1[3] * self[e235]) + (anti_reverse_g2[1] * self[e412]),
                (anti_reverse_g1[3] * self[e315]) + (anti_reverse_g2[2] * self[e423]),
                (anti_reverse_g1[3] * self[e125]) + (anti_reverse_g2[0] * self[e431]),
                -(anti_reverse_g1[1] * self[e431]) - (anti_reverse_g1[2] * self[e412]),
            ]) + (anti_reverse_g0.xxyw() * self.group2().wzx().with_w(self[e4]))
                + (anti_reverse_g0.zyz() * self.group2().yww()).with_w(anti_reverse_g1[3] * self[e12345])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e435]]) * anti_reverse_g2.zyz().with_w(anti_reverse_g0[2]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e425]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g0[1]))
                - (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xyzx() * anti_reverse_g2.www().with_w(anti_reverse_g1[0])),
            // e5
            (anti_reverse_g0[3] * self[e5]) + (anti_reverse_g2[3] * self[e12345])
                - (anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e435]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       31        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       31       37        0
    //  no simd       43       55        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g0 = Simd32x4::from([
            -self[e3] * self[e3] - (anti_reverse_g1[0] * self[e415]) - (anti_reverse_g1[1] * self[e425]) - (anti_reverse_g1[2] * self[e435]),
            (anti_reverse_g1[0] * self[e321]) + (anti_reverse_g1[2] * self[e2]) + (anti_reverse_g1[3] * self[e415]) + (self[e12345] * self[e1]),
            (anti_reverse_g1[0] * self[e3]) + (anti_reverse_g1[1] * self[e321]) + (anti_reverse_g1[3] * self[e425]) + (self[e12345] * self[e2]),
            (anti_reverse_g1[1] * self[e1]) + (anti_reverse_g1[2] * self[e321]) + (anti_reverse_g1[3] * self[e435]) + (self[e12345] * self[e3]),
        ]) + (Simd32x4::from(self[e12345]) * self.group0())
            + (Simd32x4::from([anti_reverse_g1[3], self[e3], self[e435], self[e415]]) * self.group1().wy().with_zw(self[e1], self[e2]))
            - (Simd32x4::from([self[e1], self[e435], self[e415], self[e425]]) * self.group0().yzwy())
            - (Simd32x4::from([self[e2], anti_reverse_g1[1], self[e1], self[e2]]) * self.group0().zw().with_zw(anti_reverse_g1[2], anti_reverse_g1[0]));
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (anti_reverse_g2[3] * self[e12345])
                + (anti_reverse_g2[3] * self[e321])
                + (self[e12345] * self[e5])
                + (self[e1] * self[e235])
                + (self[e2] * self[e315])
                + (self[e3] * self[e125])
                - (anti_reverse_g1[0] * self[e235])
                - (anti_reverse_g1[1] * self[e315])
                - (anti_reverse_g1[2] * self[e125])
                - (anti_reverse_g1[3] * self[e5])
                - (anti_reverse_g2[0] * self[e1])
                - (anti_reverse_g2[0] * self[e415])
                - (anti_reverse_g2[1] * self[e2])
                - (anti_reverse_g2[1] * self[e425])
                - (anti_reverse_g2[2] * self[e3])
                - (anti_reverse_g2[2] * self[e435]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        7        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd       21       32        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from(anti_reverse_g0[3]) * self.group1().xyz())
                + (Simd32x3::from([self[e315], self[e5], self[e5]]) * anti_reverse_g0.zyz())
                + (Simd32x3::from([self[e5], self[e125], self[e235]]) * anti_reverse_g0.xxy())
                + (anti_reverse_g1.yzx() * self.group0().zxy())
                - (Simd32x3::from(anti_reverse_g1[3]) * self.group0().xyz())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * anti_reverse_g1.zyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * anti_reverse_g1.xxy())
                - (anti_reverse_g0.yzx() * self.group1().zxy()),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = Origin;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = Origin;
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
        return Origin::from_groups(
            // e4
            (anti_reverse_g0[3] * self[e4]) + (anti_reverse_g1[3] * self[e12345])
                - (anti_reverse_g0[0] * self[e415])
                - (anti_reverse_g0[1] * self[e425])
                - (anti_reverse_g0[2] * self[e435])
                - (anti_reverse_g1[0] * self[e423])
                - (anti_reverse_g1[1] * self[e431])
                - (anti_reverse_g1[2] * self[e412]),
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        3        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       35       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.xxyx() * self.group1().wzx().with_w(self[e1]))
                + (anti_reverse_g0.zyzy() * self.group1().yww().with_w(self[e2]))
                + (anti_reverse_g1.yzx() * self.group0().zxy()).with_w(anti_reverse_g0[2] * self[e3])
                + (self.group1().xyz() * self.group2().www()).with_w(anti_reverse_g0[3] * self[e4])
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * anti_reverse_g1.zyz().with_w(self[e3]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * anti_reverse_g1.xxy().with_w(self[e2]))
                - (self.group0() * anti_reverse_g1.www().with_w(self[e4]))
                - (anti_reverse_g0.yzx() * self.group1().zxy()).with_w(self[e423] * self[e1]),
            // e5
            (anti_reverse_g1[3] * self[e321]) + (self[e235] * self[e1]) + (self[e315] * self[e2]) + (self[e125] * self[e3])
                - (anti_reverse_g0[3] * self[e5])
                - (anti_reverse_g1[0] * self[e1])
                - (anti_reverse_g1[1] * self[e2])
                - (anti_reverse_g1[2] * self[e3]),
        );
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
        return RoundPoint::from_groups(
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
        );
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = AntiPlane;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       22        0
    //    simd2        0        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       43       56        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g0 = (Simd32x4::from([self[e31], self[e4315], self[e4125], self[e4235]]) * anti_reverse_g1.yzxy())
            + (Simd32x4::from([self[e4235], anti_reverse_g0[0], self[e31], self[e12]]) * self.group2().xx().with_zw(anti_reverse_g1[3], anti_reverse_g1[3]))
            + (Simd32x4::from([self[e4315], self[scalar], self[e12], self[e23]]) * self.group2().yxxy())
            + (Simd32x4::from([self[e4125], self[e31], self[scalar], self[scalar]]) * self.group2().zzyz())
            + (anti_reverse_g1.zwyz() * self.group1().zxww())
            + (anti_reverse_g1.xx() * self.group1().xw()).with_zw(anti_reverse_g0[0] * self[e4315], anti_reverse_g0[0] * self[e4125])
            - Simd32x4::from([anti_reverse_g0[0] * self[scalar], self[e12] * self[e4315], self[e23] * self[e4125], self[e31] * self[e4235]])
            - (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e4315]]) * anti_reverse_g1.wyzx());
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            geometric_anti_product_g0[1],
            geometric_anti_product_g0[2],
            geometric_anti_product_g0[3],
            (anti_reverse_g0[1] * self[e23])
                + (anti_reverse_g0[2] * self[e31])
                + (anti_reverse_g0[3] * self[e12])
                + (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (self[e15] * self[e4235])
                + (self[e25] * self[e4315])
                + (self[e35] * self[e4125])
                + (self[e45] * self[e3215])
                - (anti_reverse_g0[0] * self[e3215])
                - (anti_reverse_g0[1] * self[e4235])
                - (anti_reverse_g0[2] * self[e4315])
                - (anti_reverse_g0[3] * self[e4125])
                - (anti_reverse_g1[3] * self[e3215])
                - (self[scalar] * self[e3215]),
        ]));
    }
}
impl std::ops::Div<AntiConstraintViolationPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = RoundPoint;
    fn div(self, _rhs: AntiConstraintViolationPrefixOrPostfix) -> Self::Output {
        self.anti_constraint_violation()
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        0        1        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       35       52        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g2 = self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(anti_reverse_g2[1] * self[e43]) - (anti_reverse_g2[3] * self[e15]),
                -(anti_reverse_g2[2] * self[e41]) - (anti_reverse_g2[3] * self[e25]),
                -(anti_reverse_g2[0] * self[e42]) - (anti_reverse_g2[3] * self[e35]),
                (anti_reverse_g1[1] * self[e42]) + (anti_reverse_g1[2] * self[e43]),
            ]) + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e41]]) * anti_reverse_g2.zyz().with_w(anti_reverse_g1[0]))
                + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e12]]) * anti_reverse_g2.xxy().with_w(anti_reverse_g0[2]))
                + (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e23]))
                + (self.group0().xyz() * anti_reverse_g1.www()).with_w(anti_reverse_g0[1] * self[e31])
                - (Simd32x4::from([self[e3215], self[e35], self[e15], self[e1234]]) * anti_reverse_g0.xxyw())
                - (Simd32x4::from([self[e25], self[e3215], self[e3215], self[scalar]]) * anti_reverse_g0.zyz().with_w(anti_reverse_g2[3])),
            // e5
            (anti_reverse_g1[0] * self[e15])
                + (anti_reverse_g1[1] * self[e25])
                + (anti_reverse_g1[2] * self[e35])
                + (anti_reverse_g2[0] * self[e23])
                + (anti_reverse_g2[1] * self[e31])
                + (anti_reverse_g2[2] * self[e12])
                - (anti_reverse_g0[3] * self[e3215])
                - (anti_reverse_g1[3] * self[scalar]),
        );
    }
}
