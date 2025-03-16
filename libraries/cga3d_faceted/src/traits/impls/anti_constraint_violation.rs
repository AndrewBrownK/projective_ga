// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 55
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         3      10       0
//  Average:         8      18       0
//  Maximum:        99     203       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:         7      15       0
//  Average:        13      25       0
//  Maximum:       150     267       0
impl AntiConstraintViolation for AntiCircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]))
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
impl AntiConstraintViolation for AntiCircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2().zxy()[0] * self[e42]) + (self.group2().zxy()[0] * self[e42]) - 2.0 * (self[e43] * self[e25]),
                (self.group2().zxy()[1] * self[e43]) + (self.group2().zxy()[1] * self[e43]) - 2.0 * (self[e41] * self[e35]),
                (self.group2().zxy()[2] * self[e41]) + (self.group2().zxy()[2] * self[e41]) - 2.0 * (self[e42] * self[e15]),
                2.0 * (self[e43] * self[e12]) + 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]),
            ]),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for AntiCircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ 2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]))
    }
}
impl AntiConstraintViolation for AntiCircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e23] * self[e45],
                self[e31] * self[e45],
                self[e12] * self[e45],
                2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for AntiCircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]))
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
impl AntiConstraintViolation for AntiDipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0().wwzw()[1] * self[e415]) + (self[e415] * self[e321]),
                self[e425] * self[e321],
                self[e435] * self[e321],
                -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
            ]) * Simd32x4::from([1.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for AntiDipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       19       34        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) + (self[e235] * self[e4]) - (self.group0().yzx()[0] * self.group2().zxy()[0]),
                2.0 * (self[e423] * self[e125]) + (self[e315] * self[e4]) - (self.group0().yzx()[1] * self.group2().zxy()[1]),
                2.0 * (self[e431] * self[e235]) + (self[e412] * self[e5]) + (self[e125] * self[e4])
                    - (self.group2().zyz()[2] * self[e4])
                    - (self.group0().xyzy()[2] * self.group0().wwww()[2]),
                -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e412] * self[e435]),
            ]) - (Simd32x4::from([
                self.group2().xxy()[0],
                self.group2().zyz()[1],
                self.group0().yzx()[2] * self.group2().zxy()[2],
                self.group0().xyzy()[3] * self[e425],
            ]) * self.group2().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group2().zyz()[0], self.group2().xxy()[1], self.group2().xxy()[2], self[e425]]) * self.group0().yzxy()),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for AntiDualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ (self[e1234] * self[scalar]) * -2.0)
    }
}
impl AntiConstraintViolation for AntiLine {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ 2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]))
    }
}
impl AntiConstraintViolation for AntiMotor {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
        )
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.xxyz().yzw() * self.group0().xwww().yzw()) + (anti_reverse_g0.ywww().yzw() * self.group0().yxyz().yzw()),
        )
    }
}
impl AntiConstraintViolation for AntiMysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1        7        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([(self.group0().wwzw()[1] * self[e415]) + (self[e415] * self[e321]), self[e425] * self[e321], self[e435] * self[e321]])
                * Simd32x3::from([1.0, 2.0, 2.0]),
        )
    }
}
impl AntiConstraintViolation for AntiVersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]) - 2.0 * (self[scalar] * self[e1234]),
        )
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
impl AntiConstraintViolation for CircleAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        7       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e412] * self[e315] * 2.0,
                self[e423] * self[e125] * 2.0,
                self[e431] * self[e235] * 2.0,
                -2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]) - Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e423] * self[e415]),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for CircleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        2       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e415] * self[e321] * 2.0,
                self[e425] * self[e321] * 2.0,
                self[e435] * self[e321],
                -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
            ]) * Simd32x4::from([2.0, 2.0, 4.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for CircleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        5        0
    // no simd        9       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.zxy() * self.group1().yzx()) + (self.group0().zxy() * self.group1().yzx())
                - (anti_reverse_g0.yzx() * self.group1().zxy())
                - (self.group0().yzx() * self.group1().zxy()),
        )
    }
}
impl AntiConstraintViolation for CircleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]))
    }
}
impl AntiConstraintViolation for CircleOrthogonalOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (self.group1().yzx() * anti_reverse_g0.zxy()) + (self.group1().yzx() * self.group0().zxy())
                - (self.group1().zxy() * self.group0().yzx())
                - (self.group1().zxy() * anti_reverse_g0.xyzx().yzw()),
        )
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
impl AntiConstraintViolation for CircleRotorAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                2.0 * (self[e412] * self[e315]) - (self.group2().zxy()[0] * self[e431]) - (self.group2().zxy()[0] * self[e431]),
                2.0 * (self[e423] * self[e125]) - (self.group2().zxy()[1] * self[e412]) - (self.group2().zxy()[1] * self[e412]),
                2.0 * (self[e431] * self[e235]) - (self.group2().zxy()[2] * self[e423]) - (self.group2().zxy()[2] * self[e423]),
                -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]),
            // e5
            -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for CircleRotorAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
    }
}
impl AntiConstraintViolation for CircleRotorAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        2       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e415] * self[e321] * 2.0,
                self[e425] * self[e321] * 2.0,
                self[e435] * self[e321],
                -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
            ]) * Simd32x4::from([2.0, 2.0, 4.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for CircleRotorOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ -2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]))
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
impl AntiConstraintViolation for DipoleAligningOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       16        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (self.group1().zxy() * self.group0().yzx()) + (self.group1().zxy() * anti_reverse_g0.xyzx().yzw())
                - (self.group1().yzx() * anti_reverse_g0.zxy())
                - (self.group1().yzx() * self.group0().zxy()),
        )
    }
}
impl AntiConstraintViolation for DipoleAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       13        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                self[e23] * self[e45],
                self[e31] * self[e45],
                self[e12] * self[e45],
                2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
            ]) * Simd32x4::from([2.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for DipoleAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        5        0
    // no simd        9       15        0
    fn anti_constraint_violation(self) -> Self::Output {
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.yzx() * self.group1().zxy()) + (self.group0().yzx() * self.group1().zxy())
                - (anti_reverse_g0.zxy() * self.group1().yzx())
                - (self.group0().zxy() * self.group1().yzx()),
        )
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
impl AntiConstraintViolation for DipoleInversionAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       25       36        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1().zyz()[0] * self[e42]) + (self[e41] * self[e3215]) - (self.group1().yzx()[0] * self[e43]) - (self[e15] * self[e1234]),
                (self.group1().zxy()[1] * self[e43]) + (self[e42] * self[e3215]) - (self.group1().yzx()[1] * self[e41]) - (self[e25] * self[e1234]),
                (self.group1().zyz()[2] * self[e1234]) + (self[e43] * self[e3215]) - (self.group1().yzx()[2] * self.group0().zxyy()[2]) - (self[e35] * self[e1234]),
                0.0,
            ]) + (Simd32x4::from([
                self.group1().xxy()[0],
                self.group1().zyz()[1],
                self.group1().xxy()[2] * self[e41],
                self.group0().yzxx()[3] * self[e4235],
            ]) * self.group1().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([
                    self.group1().zxy()[0],
                    self.group1().xxy()[1],
                    self.group1().zxy()[2] * self.group0().yzxx()[2],
                    self[e42] * self[e4315],
                ]) * self.group0().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().xxy()[0],
                    self.group0().zyz()[1],
                    self.group0().xxy()[2] * self[e15],
                    self.group0().zxyy()[3] * self[e4315],
                ]) * self.group2().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group0().zyz()[0], self.group0().xxy()[1], self.group0().zyz()[2] * self[e3215], self[e41] * self[e4235]])
                    * self.group1().yz().with_zw(1.0, 1.0)),
            // e5
            0.0,
        )
    }
}
impl AntiConstraintViolation for DipoleInversionAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       14        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0().xwzw()[1] * self[e23]) + (self[e23] * self[e45]),
                self[e31] * self[e45],
                self[e12] * self[e45],
                2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
            ]) * Simd32x4::from([1.0, 2.0, 2.0, 1.0]),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        2        3        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       10       18        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy())
                + Simd32x3::from([
                    (self.group1().zwzw()[1] * self[e15]) - (self[e43] * self[e25]) - (self[e15] * self[e1234]),
                    (self[e42] * self[e3215]) - 2.0 * (self[e41] * self[e35]),
                    (self[e43] * self[e3215]) - 2.0 * (self[e42] * self[e15]),
                ])
                - (Simd32x3::from([self.group1().wyzw()[1], self[e42], self[e43]]) * self.group0().wzww().yzw()),
        )
    }
}
impl AntiConstraintViolation for DipoleInversionOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       19       34        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0().yzx()[0] * self.group2().zxy()[0]) - (self[e15] * self[e1234]) - 2.0 * (self[e43] * self[e25]),
                (self.group0().yzx()[1] * self.group2().zxy()[1]) - (self[e25] * self[e1234]) - 2.0 * (self[e41] * self[e35]),
                (self.group2().zyz()[2] * self[e1234]) + (self.group0().xyzy()[2] * self.group0().wwww()[2])
                    - (self[e43] * self[e3215])
                    - (self[e35] * self[e1234])
                    - 2.0 * (self[e42] * self[e15]),
                2.0 * (self[e41] * self[e23]) + 2.0 * (self[e43] * self[e12]),
            ]) + (Simd32x4::from([
                self.group2().xxy()[0],
                self.group2().zyz()[1],
                self.group0().yzx()[2] * self.group2().zxy()[2],
                self.group0().xyzy()[3] * self[e31],
            ]) * self.group2().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().zyz()[0], self.group2().xxy()[1], self.group2().xxy()[2], self[e31]]) * self.group0().yzxy()),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for DipoleOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        7       24        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group0().yzx() * self.group2().zxy()).with_w(self[e43] * self[e12])
                + Simd32x4::from([
                    self[e43] * self[e25] * -2.0,
                    self[e41] * self[e35] * -2.0,
                    self[e42] * self[e15] * -2.0,
                    2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]),
                ]),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]),
        )
    }
}
impl AntiConstraintViolation for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ (self[e4] * self[e12345]) * 2.0)
    }
}
impl AntiConstraintViolation for Line {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]))
    }
}
impl AntiConstraintViolation for Motor {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(
            // e5
            2.0 * (self[e12345] * self[e5]) - 2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82      180        0
    //    simd2        0        2        0
    //    simd3        0        1        0
    //    simd4       17       20        0
    // Totals...
    // yes simd       99      203        0
    //  no simd      150      267        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e4] * self[e3215])
                    + 2.0 * (self[e5] * self[e1234])
                    - (self.group3().xwzw()[0] * self[e235])
                    - (self[e41] * self[e235])
                    - 2.0 * (self[e42] * self[e315])
                    - 2.0 * (self[e43] * self[e125])
                    - 2.0 * (self[e45] * self[e321])
                    - 2.0 * (self[e15] * self[e423])
                    - 2.0 * (self[e25] * self[e431])
                    - 2.0 * (self[e35] * self[e412])
                    - 2.0 * (self[e23] * self[e415])
                    - 2.0 * (self[e31] * self[e425])
                    - 2.0 * (self[e12] * self[e435]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(2.0) * (self.group3().yzxz() * self.group4().zxy().with_w(self[e12]))
                + Simd32x4::from(2.0) * (self.group0().yy().with_zw(self[e12345], self[e41] * self[e23]) * self.group1().xyz().with_w(1.0))
                + Simd32x4::from(2.0) * (self.group0().xx() * self.group9().yz()).with_zw(self.group9().yzww()[2] * self[scalar], self[e12345] * self[e4])
                + Simd32x4::from([
                    2.0 * (self[e45] * self[e23]) + 2.0 * (self[e412] * self[e315]) + (self.group1().zxy()[0] * self.group6().yzx()[0]) - (self.group3().xxy()[0] * self[e3215]),
                    2.0 * (self[e45] * self[e31]) + 2.0 * (self[e423] * self[e125]) + (self.group1().zxy()[1] * self.group6().yzx()[1]) - (self.group3().zyz()[1] * self[e3215]),
                    2.0 * (self[e431] * self[e235]) + (self.group6().wwww()[2] * self[e435]) + (self[e43] * self[e3215]) + (self[e45] * self[e12]) + (self[e35] * self[e1234])
                        - (self.group6().zxy()[2] * self.group1().yzxz()[2])
                        - (self.group9().xxxy()[2] * self[e35]),
                    (self[e42] * self[e31]) + (self[e42] * self[e4315]) - 2.0 * (self[scalar] * self[e1234]) - 2.0 * (self[e425] * self[e431]) - 2.0 * (self[e435] * self[e412]),
                ])
                + (Simd32x4::from([
                    self.group6().xxy()[0],
                    self.group6().zyz()[1],
                    self.group1().zxy()[2] * self.group6().yzx()[2],
                    self.group3().yzxx()[3] * self[e4235],
                ]) * self.group6().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group6().zyz()[0], self.group6().xxy()[1], self.group6().xxy()[2] * self[e1], self.group3().wwwy()[3] * self[e31]])
                    * self.group1().yz().with_zw(1.0, 1.0))
                + (Simd32x4::from([self[e3215], self[e3215], self.group6().zyz()[2] * self[e321], self[e2] * self[e431]]) * self.group3().xy().with_zw(1.0, 1.0))
                + (self.group6().xy() * self.group6().ww()).with_zw(self.group3().wwwy()[2] * self[e12], self[e3] * self[e412])
                - (Simd32x4::from([self.group3().zxy()[0], self.group3().xxy()[1], self.group3().xxy()[2] * self[e15], self.group1().yzxz()[3] * self[e412]])
                    * self.group4().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group3().zyz()[0], self.group3().zxy()[1], self.group3().zxy()[2] * self[e15], self.group1().zxyy()[3] * self[e431]])
                    * self.group4().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group6().yzx()[0],
                    self.group6().yzx()[1],
                    self.group3().zyz()[2] * self[e3215],
                    self.group9().xxxy()[3] * self[e41],
                ]) * self.group1().zx().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group6().zxy()[0],
                    self.group6().zxy()[1],
                    self.group6().yzx()[2] * self.group1().zxyy()[2],
                    self.group9().zwyz()[3] * self[e42],
                ]) * self.group1().yz().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (self.group7().yzx() * self.group8().zxy()).with_w(self[e415] * self[e423]),
            // e5
            2.0 * (self[e12345] * self[e5]) + 2.0 * (self[e15] * self[e23]) + 2.0 * (self[e25] * self[e31]) + 2.0 * (self[e35] * self[e12])
                - 2.0 * (self[e415] * self[e235])
                - 2.0 * (self[e425] * self[e315])
                - 2.0 * (self[e435] * self[e125])
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
            Simd32x4::from(2.0) * (Simd32x4::from([self[e4], self[e4235], self[e12345] * self[e4315], self[e12345] * self[e4125]]) * self.group0().with_zw(1.0, 1.0))
                + Simd32x4::from([
                    2.0 * (self[e12345] * self[e1234]) + (self.group9().zzzw()[0] * self[e431])
                        - (self.group5().yx()[0] * self[e431])
                        - (self.group3().xzzw()[0] * self[e415])
                        - (self[e321] * self[e1234])
                        - (self[e431] * self[e4315])
                        - 2.0 * (self[e42] * self[e425])
                        - 2.0 * (self[e43] * self[e435])
                        - 2.0 * (self[e12] * self[e412]),
                    2.0 * (self[e42] * self[e125])
                        + 2.0 * (self[e45] * self[e415])
                        + 2.0 * (self[e35] * self[e431])
                        + (self.group9().xx()[1] * self[e235])
                        + (self[e5] * self[e41])
                        - (self.group3().xx()[1] * self[e5])
                        - (self.group3().xzzw()[1] * self[e315])
                        - (self[e43] * self[e315])
                        - (self[e435] * self[e4315])
                        - (self[e235] * self[e1234])
                        - 2.0 * (self[scalar] * self[e1]),
                    2.0 * (self[e43] * self[e235])
                        + 2.0 * (self[e45] * self[e425])
                        + 2.0 * (self[e15] * self[e412])
                        + (self[e3] * self[e23])
                        + (self[e435] * self[e4235])
                        + (self[e431] * self[e3215])
                        - (self.group7().xxx()[2] * self[e35])
                        - (self.group7().xzy()[2] * self[e3215])
                        - (self.group9().wzyz()[2] * self[e435])
                        - (self[e4] * self[e25])
                        - (self[e35] * self[e423])
                        - (self[e31] * self[e321])
                        - 2.0 * (self[scalar] * self[e2]),
                    2.0 * (self[e41] * self[e315]) + 2.0 * (self[e45] * self[e435]) + 2.0 * (self[e25] * self[e423]) + (self[e415] * self[e4315])
                        - 2.0 * (self[scalar] * self[e3])
                        - 2.0 * (self[e15] * self[e431])
                        - 2.0 * (self[e12] * self[e321]),
                ])
                + (Simd32x4::from([self.group9().xx()[0], self.group9().zzzw()[1], self.group1().yzww()[2] * self[e25], self.group1().yzww()[3] * self[e35]])
                    * self.group6().wz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group3().xx()[0], self.group5().yx()[1], self.group5().yxx()[2] * self[e3], self.group9().wzyz()[3] * self[e415]])
                    * self.group6().xw().with_zw(1.0, 1.0))
                - (Simd32x4::from([self[e431], self[e321], self.group5().zzy()[2] * self[e321], self[e4] * self[e35]]) * self.group5().yx().with_zw(1.0, 1.0))
                - Simd32x4::from(2.0) * (Simd32x4::from([self[e23], self[e25], self[e41] * self[e125], self[e42] * self[e235]]) * self.group7().xz().with_zw(1.0, 1.0)),
            // e3215
            2.0 * (self[scalar] * self[e5]) + 2.0 * (self[e12345] * self[e3215])
                - 2.0 * (self[e15] * self[e415])
                - 2.0 * (self[e25] * self[e425])
                - 2.0 * (self[e35] * self[e435])
                - 2.0 * (self[e23] * self[e235])
                - 2.0 * (self[e31] * self[e315])
                - 2.0 * (self[e12] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for MysteryCircle {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e415] * self[e321], self[e425] * self[e321], self[e435] * self[e321]]) * Simd32x3::from(2.0),
        )
    }
}
impl AntiConstraintViolation for MysteryCircleRotor {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([self[e415] * self[e321], self[e425] * self[e321], self[e435] * self[e321]]) * Simd32x3::from(2.0),
        )
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (anti_reverse_g0.xxyz().yzw() * self.group0().xwww().yzw()) + (anti_reverse_g0.ywww().yzw() * self.group0().yxyz().yzw()),
        )
    }
}
impl AntiConstraintViolation for MysteryDipoleInversion {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       15       22        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            (Simd32x3::from([self.group0().xwzw()[1], self[e4125], self[e4235]]) * anti_reverse_g0.xxxy().yzw())
                + (Simd32x3::from([self[e4315], self[e45], self[e45]]) * anti_reverse_g0.yzyz().yzw())
                + (self.group1().zxy() * self.group0().yzx())
                + (anti_reverse_g0.zwww().yzw() * self.group0().zxyz().yzw())
                - (self.group1().yzx() * self.group0().zxy())
                - (self.group1().zxy() * anti_reverse_g0.wyzx().yzw()),
        )
    }
}
impl AntiConstraintViolation for MysteryVersorEven {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * self.group0().yzw())
                + Simd32x3::from([
                    (self.group1().wwzw()[1] * self[e415]) + (self[e415] * self[e321]),
                    self[e425] * self[e321] * 2.0,
                    self[e435] * self[e321] * 2.0,
                ]),
        )
    }
}
impl AntiConstraintViolation for MysteryVersorOdd {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        6        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        9        0
    //  no simd       18       28        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * self.group0().yzw())
                + (Simd32x3::from([self.group1().xwzw()[1], self[e4125], self[e4235]]) * anti_reverse_g1.xxxy().yzw())
                + (Simd32x3::from([self[e4315], self[e45], self[e45]]) * anti_reverse_g1.yzyz().yzw())
                + (self.group1().yzx() * self.group0().wyz())
                + (anti_reverse_g1.zwww().yzw() * self.group1().zxyz().yzw())
                - (self.group1().zxy() * self.group0().zwy())
                - (self.group0().wyz() * anti_reverse_g1.wyzx().yzw()),
        )
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
impl AntiConstraintViolation for VersorEvenAligningOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       17       35        0
    //  no simd       29       46        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * self[e315]) + (self[e4] * self[e235]) - (self.group2().zyz()[0] * self[e431]) - (self[e423] * self[e5]),
                (self[e423] * self[e125]) + (self[e4] * self[e315]) - (self.group2().zxy()[1] * self[e412]) - (self[e431] * self[e5]),
                (self[e431] * self[e235]) + (self[e4] * self[e125]) - (self.group2().zyz()[2] * self[e4]) - (self[e412] * self[e5]),
                -2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
            ]) + (Simd32x4::from([
                self.group0().zyz()[0],
                self.group0().zyz()[1],
                self.group0().zyz()[2] * self.group2().ywww()[2],
                self.group0().xxyw()[3] * self[e4],
            ]) * self.group2().yw().with_zw(1.0, 1.0))
                + (self.group0().xxy() * self.group2().wzx()).with_w(self[e12345] * self[e4])
                - (Simd32x4::from([
                    self.group2().xxy()[0],
                    self.group2().zyz()[1],
                    self.group2().xxy()[2] * self[e423],
                    self.group0().xyzx()[3] * self[e415],
                ]) * self.group1().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group2().zxy()[0],
                    self.group2().xxy()[1],
                    self.group2().zxy()[2] * self.group0().yzxx()[2],
                    self.group0().yzxx()[3] * self[e415],
                ]) * self.group0().yz().with_zw(1.0, 1.0)),
            // e5
            2.0 * (self[e12345] * self[e5]) - 2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
        )
    }
}
impl AntiConstraintViolation for VersorEvenAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        7       20        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * self.group0().yzw().with_w(self[e5]))
                + Simd32x4::from([
                    (self.group1().wwzw()[1] * self[e415]) + (self[e415] * self[e321]),
                    self[e425] * self[e321] * 2.0,
                    self[e435] * self[e321] * 2.0,
                    -2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125]),
                ]),
        )
    }
}
impl AntiConstraintViolation for VersorEvenAtOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       12        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([
                (self.group0().wzzw()[1] * self.group1().wyzw()[1]) + (self[e412] * self[e315]),
                self[e423] * self[e125] * 2.0,
                self[e431] * self[e235] * 2.0,
            ]) - Simd32x3::from(2.0) * (self.group0().yzx() * self.group1().zxy()),
        )
    }
}
impl AntiConstraintViolation for VersorEvenOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            2.0 * (self[e12345] * self[e4]) - 2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435]),
        )
    }
}
impl AntiConstraintViolation for VersorEvenOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       14        0
    //    simd2        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       21       26        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0().zxy()[0] * self.group1().yzx()[0]) - (self.group0().yzx()[0] * self.group1().zxy()[0]),
                (self.group0().zxy()[1] * self.group1().yzx()[1]) - (self.group0().yzx()[1] * self.group1().zxy()[1]),
                (self.group0().zyzy()[2] * self.group1().ywww()[2]) + (self[e125] * self[e4]) - (self.group1().zyz()[2] * self[e4]) - (self.group1().wwww()[2] * self[e412]),
                0.0,
            ]) + (self.group1().xy() * self.group2().ww()).with_zw(self.group0().xxyx()[2] * self.group1().wzxw()[2], self.group0().zyzy()[3] * self[e2])
                + (self.group0().zx() * self.group1().yz()).with_zw(self.group0().zxy()[2] * self.group1().yzx()[2], self.group0().xxyx()[3] * self[e1])
                - (Simd32x4::from([self.group1().xxy()[0], self.group1().zyz()[1], self.group0().yzx()[2] * self.group1().zxy()[2], self[e423] * self[e1]])
                    * self.group2().ww().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group1().zyz()[0], self.group1().xxy()[1], self.group1().xxy()[2], self[e2]]) * self.group0().yzxy()),
            // e5
            0.0,
        )
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
impl AntiConstraintViolation for VersorOddAtInfinity {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       10       22        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from(2.0) * (self.group0().xxxy() * self.group2().xyz().with_w(self[e23]))
                + Simd32x4::from([
                    (self.group1().xx()[1] * self.group1().xwzw()[1]) + (self[e23] * self[e45]),
                    self[e31] * self[e45] * 2.0,
                    (self.group1().yzxy()[3] * self[e4235]) + (self.group1().zxww()[3] * self.group1().zwyz()[3]) + (self[e12] * self[e45]) - (self[e31] * self[e4235]),
                    2.0 * (self[e25] * self[e31]) + 2.0 * (self[e35] * self[e12]) - 2.0 * (self[scalar] * self[e3215]),
                ]),
        )
    }
}
impl AntiConstraintViolation for VersorOddOrthogonalOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       17       36        0
    //  no simd       29       48        0
    fn anti_constraint_violation(self) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2().zyz()[0] * self[e42]) + (self[e41] * self[e3215]) - (self[e43] * self[e25]) - (self[e15] * self[e1234]),
                (self.group2().zxy()[1] * self[e43]) + (self[e42] * self[e3215]) - (self[e41] * self[e35]) - (self[e25] * self[e1234]),
                (self.group2().zyz()[2] * self[e1234]) + (self[e43] * self[e3215]) - (self[e42] * self[e15]) - (self[e35] * self[e1234]),
                2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]),
            ]) + (Simd32x4::from([self.group2().xxy()[0], self.group2().zyz()[1], self.group2().xxy()[2] * self[e41], self.group0().yzxx()[3] * self[e23]])
                * self.group2().ww().with_zw(1.0, 1.0))
                + (Simd32x4::from([self.group2().zxy()[0], self.group2().xxy()[1], self.group2().zxy()[2] * self.group0().yzxx()[2], self[e41] * self[e23]])
                    * self.group0().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([
                    self.group0().zyz()[0],
                    self.group0().xxyw()[1],
                    self.group0().zyz()[2] * self[e3215],
                    self.group0().xxyw()[3] * self[e1234],
                ]) * self.group2().yz().with_zw(1.0, 1.0))
                - (Simd32x4::from([self.group0().xxyw()[0], self.group0().zyz()[1], self.group0().xxyw()[2] * self[e15], self[scalar] * self[e1234]])
                    * self.group1().ww().with_zw(1.0, 1.0)),
            // e5
            2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35]) - 2.0 * (self[scalar] * self[e3215]),
        )
    }
}
