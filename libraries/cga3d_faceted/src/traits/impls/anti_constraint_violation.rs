// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 59
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         3       8       0     N/A
//  Average:         7      15       0     N/A
//  Maximum:       123     207       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         9      18       0       0
//  Average:        17      27       0       0
//  Maximum:       279     311       0       0
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
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
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e41] * self[e23]),
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]))
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            self[e23] * self[e45] * -2.0,
            self[e31] * self[e45] * -2.0,
            self[e12] * self[e45] * -2.0,
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        ]))
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
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
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       17       34        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(2.0) * (self.group0().yzxy() * self.group2().zxy().with_w(self[e315]))
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e415] * self[e235])
                        + 2.0 * (self[e435] * self[e125])
                        + 2.0 * (self[e321] * self[e5])
                        + 2.0 * (self[e235] * self[e1])
                        + 2.0 * (self[e315] * self[e2])
                        + 2.0 * (self[e125] * self[e3]),
                )
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e321], self[e3], self[e1]]) * self.group0().xxy()).with_w(0.0)
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e2], self[e321], self[e321]]) * self.group0().zyz()).with_w(0.0),
        )
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -2.0 * (self[e423] * self[e1]) - 2.0 * (self[e431] * self[e2]) - 2.0 * (self[e412] * self[e3]) - 2.0 * (self[e321] * self[e4]),
        )
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd       19       39        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x4::from([self[e125], self[e235], self[e315], self[e435]]) * self.group0().yzxz())
                + Simd32x4::from(2.0) * (Simd32x3::from(self[e4]) * self.group2().xyz()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]))
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e5], self[e125], self[e235]]) * self.group0().xxy()).with_w(0.0)
                - Simd32x4::from(2.0) * (Simd32x3::from([self[e315], self[e5], self[e5]]) * self.group0().zyz()).with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[scalar] * self[e1234] * -2.0)
    }
}
impl AntiConstraintViolation for AntiFlector {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) + 2.0 * (self[e125] * self[e3]) + 2.0 * (self[e321] * self[e5]),
        )
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]))
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
        )
    }
}
impl AntiConstraintViolation for AntiMysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e23] * self[e45] * -2.0, self[e31] * self[e45] * -2.0, self[e12] * self[e45] * -2.0]),
        )
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        6        0      N/A
    // no simd        6       18        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (self.group1().zxy() * self.group0().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group1().yzx() * self.group0().zxy()),
        )
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]) - 2.0 * (self[scalar] * self[e1234]),
        )
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
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e412] * self[e435])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]))
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        2       10        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321] * -2.0) * self.group0().xyz()).with_w(2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125])),
        )
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        6        0      N/A
    // no simd        9       18        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.zxy() * self.group1().yzx()) + (anti_reverse_g1.yzx() * self.group0().zxy())
                - (anti_reverse_g0.yzx() * self.group1().zxy())
                - (anti_reverse_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ 2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]))
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       19        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g1.yzx() * self.group0().zxy()) + (self.group1().yzx() * anti_reverse_g0.zxy())
                - (anti_reverse_g1.zxy() * self.group0().yzx())
                - (self.group1().zxy() * anti_reverse_g0.yzx()),
        )
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
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e412] * self[e435])
                + Simd32x3::from(0.0).with_w(2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]))
                - Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0),
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]))
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        2       10        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            (Simd32x3::from(self[e321] * -2.0) * self.group0().xyz()).with_w(2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125])),
        )
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ 2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]))
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
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       19        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g1.zxy() * self.group0().yzx()) + (self.group1().zxy() * anti_reverse_g0.yzx())
                - (anti_reverse_g1.yzx() * self.group0().zxy())
                - (self.group1().yzx() * anti_reverse_g0.zxy()),
        )
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2       12        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            self[e23] * self[e45] * -2.0,
            self[e31] * self[e45] * -2.0,
            self[e12] * self[e45] * -2.0,
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        ]))
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        6        0      N/A
    // no simd        9       18        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.yzx() * self.group1().zxy()) + (anti_reverse_g1.zxy() * self.group0().yzx())
                - (anti_reverse_g0.zxy() * self.group1().yzx())
                - (anti_reverse_g1.yzx() * self.group0().zxy()),
        )
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
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd        8       20        0      N/A
    //  no simd       20       42        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group0().zyz()).with_w(0.0)
                + Simd32x4::from(2.0) * (Simd32x3::from([self[e3215], self[e35], self[e15]]) * self.group0().xxy()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e42] * self[e4315]) - 2.0 * (self[e43] * self[e4125]))
                - Simd32x4::from(2.0) * (self.group1() * Simd32x3::from(self[e1234]).with_w(self[e45]))
                - Simd32x4::from(2.0) * (self.group0().yzxx() * self.group1().zxy().with_w(self[e4235])),
            // e5
            2.0 * (self[e45] * self[e3215]) + 2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        3        6        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       15       32        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(2.0) * (self.group2().zxyz() * self.group0().yzx().with_w(self[e35]))
                + Simd32x3::from(0.0).with_w(2.0 * (self[e45] * self[e3215]) + 2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) - 2.0 * (self[e12] * self[e35]))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e15]]) * self.group0().xxyx())
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e4315], self[e45], self[e45], self[e25]]) * self.group0().zyzy()),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * self.group0().xyz())
                + Simd32x3::from(2.0) * (self.group0().zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((self[e41] * self[e25]) * -2.0)
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().yz() * self.group1().zx()).with_z(0.0),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -2.0 * (self[e41] * self[e4235]) - 2.0 * (self[e42] * self[e4315]) - 2.0 * (self[e43] * self[e4125]) - 2.0 * (self[e45] * self[e1234]),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd       19       39        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (Simd32x3::from([self[e3215], self[e35], self[e15]]) * self.group0().xxy()).with_w(0.0)
                + Simd32x4::from(2.0) * (Simd32x3::from([self[e25], self[e3215], self[e3215]]) * self.group0().zyz()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
                - Simd32x4::from(2.0) * (self.group0().yzxx() * self.group2().zxy().with_w(self[e23]))
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e1234]) * self.group2().xyz()).with_w(0.0),
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       11       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().zxy() * self.group2().yzx()).with_w(0.0)
                + Simd32x3::from(0.0).with_w(-2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]))
                - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e41] * self[e23]),
            // e5
            -2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e12345] * self[e4] * 2.0)
    }
}
impl AntiConstraintViolation for Flector {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            2.0 * (self[e15] * self[e4235]) + 2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]) + 2.0 * (self[e45] * self[e3215]),
        )
    }
}
impl AntiConstraintViolation for Line {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]))
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]) + 2.0 * (self[e12345] * self[e5]),
        )
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       71      170        0        0
    //    simd3        0        7        0      N/A
    //    simd4       52       30        0      N/A
    // Totals...
    // yes simd      123      207        0      N/A
    //  no simd      279      311        0        0
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
                    + (self[scalar] * self[e12345])
                    + (self[e2] * self[e4315])
                    + (self[e41] * self[e235])
                    + (self[e42] * self[e315])
                    + (self[e43] * self[e125])
                    + 2.0 * (self[e15] * self[e423])
                    + 2.0 * (self[e25] * self[e431])
                    + 2.0 * (self[e35] * self[e412])
                    + (self[e23] * self[e415])
                    + (self[e31] * self[e425])
                    + (self[e12] * self[e435])
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
            Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * self.group1())
                + Simd32x4::from(2.0) * (self.group7().yzx() * self.group8().zxy()).with_w(0.0)
                + Simd32x4::from([
                    -(anti_reverse_g3[0] * self[e3215]) - (anti_reverse_g3[2] * self[e25]),
                    (self[e23] * self[e4125]) * -2.0,
                    (self[e31] * self[e4235]) * -2.0,
                    -(self[e41] * self[e23])
                        - (self[e41] * self[e4235])
                        - (self[e42] * self[e31])
                        - (self[e42] * self[e4315])
                        - (self[e43] * self[e12])
                        - (self[e43] * self[e4125]),
                ])
                + (anti_reverse_g6 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x4::from(anti_reverse_g3[3]) * self.group5().with_w(self[e1234]))
                + (Simd32x4::from([self[e35], self[e15], self[e25], self[e23]]) * anti_reverse_g3.yzxx())
                + (self.group6().xyzx() * Simd32x3::from(anti_reverse_g6[3]).with_w(self[e423]))
                + (self.group6().yzxy() * self.group1().zxy().with_w(self[e431]))
                + (self.group9().yzwy() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g3[0]))
                + (self.group9().wyzz() * self.group5().yzx().with_w(anti_reverse_g3[1]))
                + Simd32x3::from(0.0).with_w(anti_reverse_g3[2] * self[e12])
                + Simd32x3::from(0.0).with_w(anti_reverse_g3[2] * self[e4125])
                + (Simd32x3::from(self[e4]) * self.group8()).with_w(anti_reverse_g3[1] * self[e31])
                + (Simd32x3::from(self[e3215]) * self.group3().xyz()).with_w(0.0)
                + (self.group4().yzx() * self.group3().zxy()).with_w(self[e435] * self[e412])
                + (anti_reverse_g6.zxy() * self.group1().yzx()).with_w(0.0)
                - Simd32x4::from([0.0, anti_reverse_g3[1] * self[e3215], anti_reverse_g3[2] * self[e3215], anti_reverse_g6[0] * self[e423]])
                - Simd32x4::from([self[e42] * self[e35], self[e43] * self[e15], self[e41] * self[e25], 0.0])
                - Simd32x4::from([self[e12] * self[e4315], anti_reverse_g3[0] * self[e35], anti_reverse_g3[1] * self[e15], 0.0])
                - (Simd32x4::from(self[e45]) * self.group5().with_w(self[e1234]))
                - (Simd32x4::from(self[e1234]) * self.group4().with_w(self[scalar]))
                - (anti_reverse_g6.yzxy() * self.group1().zxy().with_w(self[e431]))
                - (self.group1().yzxy() * self.group6().zxy().with_w(self[e431]))
                - Simd32x3::from(0.0).with_w(self[e1] * self[e423])
                - Simd32x3::from(0.0).with_w(self[e3] * self[e412])
                - Simd32x3::from(0.0).with_w(self[e4] * self[e321])
                - (Simd32x3::from(self[e5]) * self.group7()).with_w(0.0)
                - (self.group7().zxy() * self.group8().yzx()).with_w(anti_reverse_g6[2] * self[e412]),
            // e5
            2.0 * (self[e12345] * self[e5])
                + 2.0 * (self[e1] * self[e235])
                + 2.0 * (self[e2] * self[e315])
                + 2.0 * (self[e3] * self[e125])
                + (self[e5] * self[e321])
                + (self[e45] * self[e3215])
                + 2.0 * (self[e15] * self[e4235])
                + 2.0 * (self[e25] * self[e4315])
                + 2.0 * (self[e35] * self[e4125])
                + (self[e415] * self[e235])
                + (self[e425] * self[e315])
                + (self[e435] * self[e125])
                - (anti_reverse_g3[3] * self[e3215])
                - (anti_reverse_g6[0] * self[e235])
                - (anti_reverse_g6[1] * self[e315])
                - (anti_reverse_g6[2] * self[e125])
                - (anti_reverse_g6[3] * self[e5])
                - 2.0 * (self[e15] * self[e23])
                - 2.0 * (self[e25] * self[e31])
                - 2.0 * (self[e35] * self[e12])
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
            Simd32x4::from(2.0) * (Simd32x4::from([self[e23], self[e25], self[e35], self[e15] * self[e431]]) * self.group7().xzx().with_w(1.0))
                + Simd32x4::from([0.0, self[e5] * self[e41], self[e431] * self[e3215], self[e412] * self[e3215]])
                + Simd32x4::from([0.0, self[e423] * self[e3215], self[e5] * self[e42], self[e5] * self[e43]])
                + Simd32x4::from([self[scalar] * self[e4], anti_reverse_g6[2] * self[e4315], anti_reverse_g6[0] * self[e4125], 0.0])
                + Simd32x4::from([self[e12345] * self[e1234], 0.0, 0.0, anti_reverse_g6[1] * self[e4235]])
                + Simd32x4::from([self[e431] * self[e4315], 0.0, self[e41] * self[e125], self[e42] * self[e235]])
                + Simd32x4::from([self[e412] * self[e4125], self[e43] * self[e315], 0.0, 0.0])
                + Simd32x4::from([
                    (self[e12] * self[e412]) * 2.0,
                    -(self[e42] * self[e125]) - 2.0 * (self[e35] * self[e431]),
                    -(self[e43] * self[e235]) - 2.0 * (self[e15] * self[e412]),
                    -(self[e41] * self[e315]) - 2.0 * (self[e25] * self[e423]),
                ])
                + (anti_reverse_g3 * Simd32x4::from([self[e1], self[e125], self[e235], self[e435]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([self[e4], anti_reverse_g6[0], anti_reverse_g6[1], anti_reverse_g6[2]]))
                + (Simd32x4::from([anti_reverse_g3[2], self[e12], self[e23], self[e31]]) * self.group1().zyzx())
                + (Simd32x4::from([anti_reverse_g6[3], self[e12345], self[e12345], self[e12345]]) * self.group9())
                + (Simd32x4::from([self[e2], self[e415], self[e425], self[e315]]) * anti_reverse_g3.ywwx())
                + (Simd32x4::from([self[e423], self[e425], self[e435], self[e415]]) * self.group9().ywyz())
                + (Simd32x4::from([self[e431], self[e321], self[e321], self[e12] * self[e321]]) * self.group5().yxy().with_w(1.0))
                - Simd32x4::from([anti_reverse_g3[0] * self[e415], anti_reverse_g6[3] * self[e23], anti_reverse_g6[3] * self[e31], 0.0])
                - Simd32x4::from([anti_reverse_g3[1] * self[e425], self[scalar] * self[e1], 0.0, anti_reverse_g6[0] * self[e4315]])
                - Simd32x4::from([anti_reverse_g6[1] * self[e42], anti_reverse_g3[0] * self[e5], anti_reverse_g3[0] * self[e125], 0.0])
                - Simd32x4::from([anti_reverse_g6[2] * self[e43], anti_reverse_g3[2] * self[e315], 0.0, anti_reverse_g3[1] * self[e235]])
                - Simd32x4::from([self[e1] * self[e41], 0.0, anti_reverse_g3[1] * self[e5], anti_reverse_g3[2] * self[e5]])
                - Simd32x4::from([self[e3] * self[e43], 0.0, self[e1] * self[e12], self[e2] * self[e23]])
                - (anti_reverse_g6 * Simd32x4::from([self[e41], self[e4125], self[e4235], self[e12]]))
                - (Simd32x4::from(self[e4]) * Simd32x4::from([anti_reverse_g3[3], self[e15], self[e25], self[e35]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e321], self[e235], self[e315], self[e125]]))
                - (Simd32x4::from([anti_reverse_g3[2], self[e4315], self[e4125], self[e4235]]) * self.group6().zzxy())
                - (Simd32x4::from([self[e42], self[e31], self[scalar], self[scalar]]) * self.group1().yzyz()),
            // e3215
            2.0 * (self[scalar] * self[e5])
                + 2.0 * (self[e12345] * self[e3215])
                + (anti_reverse_g3[3] * self[e5])
                + 2.0 * (self[e1] * self[e15])
                + 2.0 * (self[e2] * self[e25])
                + 2.0 * (self[e3] * self[e35])
                + (self[e15] * self[e415])
                + (self[e25] * self[e425])
                + (self[e35] * self[e435])
                + 2.0 * (self[e23] * self[e235])
                + 2.0 * (self[e31] * self[e315])
                + 2.0 * (self[e12] * self[e125])
                + (self[e321] * self[e3215])
                - (anti_reverse_g6[0] * self[e15])
                - (anti_reverse_g6[1] * self[e25])
                - (anti_reverse_g6[2] * self[e35])
                - (anti_reverse_g6[3] * self[e3215])
                - (self[e5] * self[e45])
                - 2.0 * (self[e235] * self[e4235])
                - 2.0 * (self[e315] * self[e4315])
                - 2.0 * (self[e125] * self[e4125]),
        )
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e415] * self[e321] * -2.0, self[e425] * self[e321] * -2.0, self[e435] * self[e321] * -2.0]),
        )
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e415] * self[e321] * -2.0, self[e425] * self[e321] * -2.0, self[e435] * self[e321] * -2.0]),
        )
    }
}
impl AntiConstraintViolation for MysteryDipole {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e23] * self[e45] * -2.0, self[e31] * self[e45] * -2.0, self[e12] * self[e45] * -2.0]),
        )
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        6        0      N/A
    // no simd        6       18        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (self.group1().zxy() * self.group0().yzx())
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group1().yzx() * self.group0().zxy()),
        )
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        6        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       13       26        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group0().yzw())
                + Simd32x3::from(2.0) * (self.group0().wy() * self.group1().yz()).with_z(0.0)
                + Simd32x2::from(0.0).with_z(2.0 * (self[e2] * self[e415]) - 2.0 * (self[e1] * self[e425]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().zw() * self.group1().zx()).with_z(0.0),
        )
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        6        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       13       26        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group0().yzw())
                + Simd32x3::from(2.0) * (self.group0().wy() * self.group1().yz()).with_z(0.0)
                + Simd32x2::from(0.0).with_z(2.0 * (self[e4315] * self[e23]) - 2.0 * (self[e4235] * self[e31]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * self.group1().xyz())
                - Simd32x3::from(2.0) * (self.group0().zw() * self.group1().zx()).with_z(0.0),
        )
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
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       15        0        0
    //    simd3        0        2        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       13       22        0      N/A
    //  no simd       37       41        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([anti_reverse_g0[2] * self[e315], anti_reverse_g0[0] * self[e125], 0.0, self[e423] * self[e415]])
                + (anti_reverse_g0 * Simd32x3::from(self[e5]).with_w(self[e4]))
                + (Simd32x4::from(self[e4]) * self.group2().xyz().with_w(self[e12345]))
                + (self.group0().yzxy() * self.group2().zxy().with_w(self[e425]))
                + Simd32x2::from(0.0).with_zw(anti_reverse_g0[1] * self[e235], self[e412] * self[e435])
                + Simd32x3::from(0.0).with_w(-(anti_reverse_g0[1] * self[e425]) - (anti_reverse_g0[2] * self[e435]))
                - (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e415]))
                - (self.group0().xxy() * self.group2().wzx()).with_w(0.0)
                - (self.group0().zyz() * self.group2().yww()).with_w(0.0),
            // e5
            (anti_reverse_g0[3] * self[e5]) + (self[e12345] * self[e5]) + 2.0 * (self[e415] * self[e235]) + 2.0 * (self[e425] * self[e315]) + 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       16        0        0
    //    simd3        0        2        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd       10       21        0      N/A
    //  no simd       25       34        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([self[e3] * self[e425], self[e12345] * self[e2], self[e12345] * self[e3], 0.0])
                + (Simd32x4::from([self[e1], self[e435], self[e415], self[e125]]) * self.group0())
                + Simd32x3::from(0.0).with_w(self[e2] * self[e315])
                + Simd32x3::from(0.0).with_w(
                    2.0 * (self[e12345] * self[e5])
                        + 2.0 * (self[e1] * self[e235])
                        + 2.0 * (self[e415] * self[e235])
                        + 2.0 * (self[e425] * self[e315])
                        + 2.0 * (self[e435] * self[e125])
                        + 2.0 * (self[e321] * self[e5]),
                )
                - Simd32x4::from(2.0) * (Simd32x3::from(self[e321]) * self.group1().xyz()).with_w(0.0)
                - Simd32x4::from(2.0) * (self.group1().zxy() * self.group0().zwy()).with_w(0.0),
        )
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        7        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       25        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * self.group1().xyz())
                + Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((self[e431] * self[e235]) * -2.0)
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * self.group0().xyz())
                - Simd32x3::from(2.0) * (self.group0().zx() * self.group1().yz()).with_z(0.0),
        )
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            2.0 * (self[e423] * self[e415]) + 2.0 * (self[e431] * self[e425]) + 2.0 * (self[e412] * self[e435]) + 2.0 * (self[e12345] * self[e4]),
        )
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       37       41        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x3::from(self[e5]).with_w(self[e4]))
                + (anti_reverse_g0.zxyx() * self.group1().yzx().with_w(self[e1]))
                + (self.group2().wwwy() * self.group1().xyz().with_w(anti_reverse_g0[1]))
                + Simd32x3::from(0.0).with_w(anti_reverse_g0[2] * self[e3])
                + Simd32x3::from(0.0).with_w(-(self[e431] * self[e2]) - (self[e412] * self[e3]))
                + (self.group0().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group0() * Simd32x3::from(self[e5]).with_w(self[e4]))
                - (self.group0().zxyx() * self.group1().yzx().with_w(self[e1]))
                - (anti_reverse_g0.yzx() * self.group1().zxy()).with_w(0.0),
            // e5
            (self[e321] * self[e5]) + 2.0 * (self[e235] * self[e1]) + 2.0 * (self[e315] * self[e2]) + 2.0 * (self[e125] * self[e3]) - (anti_reverse_g0[3] * self[e5]),
        )
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
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        4        8        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       19       40        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(2.0) * (self.group2() * Simd32x3::from(self[scalar]).with_w(self[e45]))
                + Simd32x4::from(2.0) * (self.group2().zxyx() * self.group1().yzx().with_w(self[e15]))
                + Simd32x3::from(0.0)
                    .with_w(2.0 * (self[e25] * self[e4315]) + 2.0 * (self[e35] * self[e4125]) - 2.0 * (self[scalar] * self[e3215]) - 2.0 * (self[e35] * self[e12]))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e45], self[e4125], self[e4235], self[e15]]) * self.group1().xxyx())
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e4315], self[e45], self[e45], self[e25]]) * self.group1().zyzy()),
        )
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       18        0        0
    //    simd3        0        1        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       36       41        0        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.yzxx() * self.group2().zxy().with_w(self[e23]))
                + (self.group1().wwwy() * self.group0().xyz().with_w(anti_reverse_g0[1]))
                + Simd32x3::from(0.0).with_w(anti_reverse_g0[2] * self[e12])
                + (self.group0().zxy() * self.group2().yzx()).with_w(0.0)
                - Simd32x4::from([0.0, self[e43] * self[e15], self[e41] * self[e25], self[e43] * self[e12]])
                - Simd32x4::from([anti_reverse_g0[2] * self[e25], anti_reverse_g0[0] * self[e35], 0.0, self[e41] * self[e23]])
                - Simd32x4::from([self[e42] * self[e35], 0.0, anti_reverse_g0[1] * self[e15], self[e42] * self[e31]])
                - (anti_reverse_g0 * Simd32x3::from(self[e3215]).with_w(self[e1234]))
                - (self.group2() * Simd32x3::from(self[e1234]).with_w(self[scalar])),
            // e5
            -(anti_reverse_g0[3] * self[e3215]) - (self[scalar] * self[e3215]) - 2.0 * (self[e23] * self[e15]) - 2.0 * (self[e31] * self[e25]) - 2.0 * (self[e12] * self[e35]),
        )
    }
}
