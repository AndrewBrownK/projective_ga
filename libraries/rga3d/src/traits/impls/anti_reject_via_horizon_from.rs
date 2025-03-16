// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 86
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       6       0
//  Average:         6      12       0
//  Maximum:        63      87       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2      11       0
//  Average:        10      22       0
//  Maximum:       109     144       0
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for AntiScalar {
    type Output = AntiRejectViaHorizonFromInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl AntiRejectViaHorizonFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234])
                * Simd32x4::from([other[e23] * other[scalar], other[e31] * other[scalar], other[e12] * other[scalar], other[scalar] * other[scalar]])
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e1234] * other[scalar];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, wedge_g0 * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(wedge_g0 * -1.0) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for DualNum {
    type Output = AntiRejectViaHorizonFromInfixPartial<DualNum>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            other[scalar] * other[scalar] * self[scalar],
            (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]),
        ]))
    }
}
impl AntiRejectViaHorizonFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       24        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz() * wedge_g1.www() * Simd32x3::from(-1.0))
                .with_w((wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g1[3] * other[e321])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for DualNum {
    type Output = Scalar;
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectViaHorizonFrom<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Scalar::from_groups(/* scalar */ -(wedge_g1[0] * other[e23]) - (wedge_g1[1] * other[e31]) - (wedge_g1[2] * other[e12]))
    }
}
impl AntiRejectViaHorizonFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        7       25        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])) * other.group0().xyz().with_w(1.0);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[scalar],
                other[scalar],
                other[scalar],
                (wedge_g1[3] * other[scalar]) - (wedge_g1[0] * other[e23]) - (wedge_g1[1] * other[e31]) - (wedge_g1[2] * other[e12]),
            ]) * wedge_g1.xyz().with_w(1.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       28        0
    //    simd3        5       13        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       26       44        0
    //  no simd       45       79        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (self[scalar] * other[scalar] * other[scalar])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for DualNum {
    type Output = Scalar;
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectViaHorizonFrom<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Flector {
    type Output = AntiRejectViaHorizonFromInfixPartial<Flector>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       18        0
    //  no simd       22       33        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]))
            + Simd32x3::from(0.0).with_w(
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            )
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let wedge_g1_xyz = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g1_xyz[1] * other[e3],
                wedge_g1_xyz[2] * other[e1],
                wedge_g1_xyz[0] * other[e2],
                -(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]) - (wedge_g0[3] * other[e321]),
            ]) - (other.group0().yzxx() * wedge_g1_xyz.zxy().with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[e321] * other[e321] * -1.0)
    }
}
impl AntiRejectViaHorizonFrom<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       10       21        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (self[e3] * other[e42]) + (self[e4] * other[e23]),
            (self[e1] * other[e43]) + (self[e4] * other[e31]),
            (self[e2] * other[e41]) + (self[e4] * other[e12]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (right_dual_g0 * Simd32x3::from(wedge_g0[3])).with_w(-(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        0        1        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       18       38        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g1 = Simd32x4::from([
            (self[e4] * other[e23]) + (self[e423] * other[scalar]),
            (self[e4] * other[e31]) + (self[e431] * other[scalar]),
            (self[e4] * other[e12]) + (self[e412] * other[scalar]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) + (self.group0().zxy() * other.group0().yzx()).with_w(self[e321] * other[scalar])
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                right_dual_g0[3] * wedge_g0[0],
                right_dual_g0[3] * wedge_g0[1],
                right_dual_g0[3] * wedge_g0[2],
                -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]),
            ]) + (right_dual_g0 * wedge_g1.www().with_w(wedge_g0[3])),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(right_dual_g0[3]),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       43        0
    //    simd3        7       16        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       39       62        0
    //  no simd       68      103        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
            - (self[e423] * other[e1])
            - (self[e431] * other[e2])
            - (self[e412] * other[e3])
            - (self[e321] * other[e4]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g3 = (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx());
        let wedge_g4 = Simd32x4::from([
            (self[e4] * other[e23]) + (self[e423] * other[scalar]),
            (self[e4] * other[e31]) + (self[e431] * other[scalar]),
            (self[e4] * other[e12]) + (self[e412] * other[scalar]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) + (other.group2().yzx() * self.group0().zxy()).with_w(self[e321] * other[scalar])
            - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (self[e1] * other[e423] * other[e321]) - (self[e2] * other[e431] * other[e321]) - (self[e3] * other[e412] * other[e321]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       14       28        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            self[e4] * other[e1],
            self[e4] * other[e2],
            self[e4] * other[e3],
            -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let wedge_g1_xyz = (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g1_xyz[1] * other[e3],
                wedge_g1_xyz[2] * other[e1],
                wedge_g1_xyz[0] * other[e2],
                -(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]),
            ]) - (other.group0().yzxx() * wedge_g1_xyz.zxy().with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Horizon {
    type Output = AntiRejectViaHorizonFromInfixPartial<Horizon>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321])
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e4] * self[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e321] * other[scalar];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(wedge_g0) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(wedge_g0 * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd2        0        3        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0       21        0
    //  no simd        0       30        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = self[e321] * other[e4] * -1.0;
        let wedge_g4_w = self[e321] * other[scalar];
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g1_w * wedge_g4_w, wedge_g0_y * other[scalar]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            (Simd32x2::from(wedge_g4_w) * right_dual_g2.xy()).with_zw(wedge_g4_w * right_dual_g2[2], right_dual_g1_w * wedge_g0_y * -1.0),
            // e41, e42, e43
            right_dual_g2 * Simd32x3::from(wedge_g0_y * -1.0),
            // e23, e31, e12
            Simd32x3::from(wedge_g4_w * -1.0) * other.group1().xyz(),
            // e423, e431, e412, e321
            (Simd32x2::from(wedge_g0_y * -1.0) * other.group1().xy()).with_zw(wedge_g0_y * other[e3] * -1.0, wedge_g4_w * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e321] * other[e4] * -1.0) * other.group0().xyz()).with_w(0.0))
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Horizon {
    type Output = Horizon;
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Line {
    type Output = AntiRejectViaHorizonFromInfixPartial<Line>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       11       25        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e4] * self[e23]),
            (other[e1] * self[e43]) + (other[e4] * self[e31]),
            (other[e2] * self[e41]) + (other[e4] * self[e12]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(wedge_g0[3]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e321]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        5        8        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       15       27        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e41] * self[e23]) * other.group1())
                + (Simd32x3::from(other[e42] * self[e31]) * other.group1())
                + (Simd32x3::from(other[e43] * self[e12]) * other.group1())
                + (other.group1() * other.group1() * self.group0())
                + (other.group1() * other.group1().yxx() * self.group0().yxx())
                + (other.group1() * other.group1().zzy() * self.group0().zzy()),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       10       27        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            other[scalar],
            other[scalar],
            other[scalar],
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        ]) * self.group0().with_w(1.0);
        let wedge_g1_xyz = Simd32x3::from(other[scalar]) * self.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[scalar],
                other[scalar],
                other[scalar],
                -(wedge_g1_xyz[0] * other[e23]) - (wedge_g1_xyz[1] * other[e31]) - (wedge_g1_xyz[2] * other[e12]),
            ]) * wedge_g1_xyz.with_w(1.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       36        0
    //    simd3        5       12        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       30       50        0
    //  no simd       49       80        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let wedge_g2 = Simd32x3::from(other[scalar]) * self.group0();
        let wedge_g3 = Simd32x3::from(other[scalar]) * self.group1();
        let wedge_g4 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23]));
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd3        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       11       24        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Line::from_groups(
            // e41, e42, e43
            (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from([wedge_g0[3] * other[e1], wedge_g0[3] * other[e2], wedge_g0[3] * other[e3]]) * Simd32x3::from(-1.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Motor {
    type Output = AntiRejectViaHorizonFromInfixPartial<Motor>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[scalar] * self[e41],
                other[scalar] * self[e42],
                other[scalar] * other[scalar] * self[e43],
                (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]),
            ]) * other.group0().xx().with_zw(1.0, 1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        1        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       18       36        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from([
            (other[e4] * self[e23]) + (other[e423] * self[scalar]),
            (other[e4] * self[e31]) + (other[e431] * self[scalar]),
            (other[e4] * self[e12]) + (other[e412] * self[scalar]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) + (other.group0().zxy() * self.group0().yzx()).with_w(other[e321] * self[scalar])
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz() * wedge_g1.www() * Simd32x3::from(-1.0))
                .with_w((wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g1[3] * other[e321])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl AntiRejectViaHorizonFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        0        8        0
    //    simd4        5        0        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       22       33        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[scalar]) * other.group1();
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            -(right_dual_g0 * Simd32x3::from(other[e41] * self[e23])).with_w(0.0)
                - (right_dual_g0 * Simd32x3::from(other[e42] * self[e31])).with_w(0.0)
                - (right_dual_g0 * Simd32x3::from(other[e43] * self[e12])).with_w(0.0)
                - (right_dual_g0 * Simd32x3::from(other[e23] * self[e41])).with_w(0.0)
                - (right_dual_g0 * Simd32x3::from(other[e31] * self[e42])).with_w(0.0)
                - (right_dual_g0 * Simd32x3::from(other[e12] * self[e43])).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * wedge_g1_xyz[0]) - (right_dual_g0[1] * wedge_g1_xyz[1]) - (right_dual_g0[2] * wedge_g1_xyz[2])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       22       35        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x4::from(other[scalar]) * self.group0())
            + (Simd32x4::from(self[scalar]) * other.group0())
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            );
        let wedge_g1_xyz = (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[scalar],
                other[scalar],
                other[scalar],
                (other[scalar] * other[scalar] * self[scalar]) - (wedge_g1_xyz[0] * other[e23]) - (wedge_g1_xyz[1] * other[e31]) - (wedge_g1_xyz[2] * other[e12]),
            ]) * wedge_g1_xyz.with_w(1.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        7       16        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       40       62        0
    //  no simd       69      103        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[e1234] * other[scalar]) + (self[scalar] * other[e1234])
            - (self[e41] * other[e23])
            - (self[e42] * other[e31])
            - (self[e43] * other[e12])
            - (self[e23] * other[e41])
            - (self[e31] * other[e42])
            - (self[e12] * other[e43]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * self.group0().xyz());
        let wedge_g3 = (Simd32x3::from(self[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * self.group1().xyz());
        let wedge_g4 = Simd32x4::from([
            (self[e23] * other[e4]) + (self[scalar] * other[e423]),
            (self[e31] * other[e4]) + (self[scalar] * other[e431]),
            (self[e12] * other[e4]) + (self[scalar] * other[e412]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) + (self.group0().yzx() * other.group1().zxy()).with_w(self[scalar] * other[e321])
            - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23]));
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (self[scalar] * other[scalar] * other[scalar])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for Motor {
    type Output = Scalar;
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectViaHorizonFrom<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       19        0
    //  no simd       13       32        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g1[3] * other[e1],
                wedge_g1[3] * other[e2],
                wedge_g1[3] * other[e3],
                (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]),
            ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for MultiVector {
    type Output = AntiRejectViaHorizonFromInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       18        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[scalar] * other[scalar] * self[scalar],
                (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group4(),
        )
    }
}
impl AntiRejectViaHorizonFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       31        0
    //    simd3        3        9        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       43       74        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz());
        let wedge_g3 = (other.group0().zxy() * self.group1().yzx()) - (other.group0().yzx() * self.group1().zxy());
        let wedge_g4 = Simd32x4::from([
            (other[e4] * self[e23]) + (other[e423] * self[scalar]),
            (other[e4] * self[e31]) + (other[e431] * self[scalar]),
            (other[e4] * self[e12]) + (other[e412] * self[scalar]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) + (self.group2().yzx() * other.group0().zxy()).with_w(other[e321] * self[scalar])
            - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23]));
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
                    - (right_dual_g0[0] * wedge_g4[0])
                    - (right_dual_g0[1] * wedge_g4[1])
                    - (right_dual_g0[2] * wedge_g4[2])
                    - (right_dual_g0[3] * wedge_g4[3]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g3[1] * other[e3],
                wedge_g3[2] * other[e1],
                wedge_g3[0] * other[e2],
                -(wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g0 * Simd32x4::from(wedge_g0_y))
                - (other.group0().yzxx() * wedge_g3.zxy().with_w(wedge_g2[0])),
            // e41, e42, e43
            (wedge_g4.zxy() * other.group0().yzx()) - (wedge_g4.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3] * -1.0) * other.group0().xyz(),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0_y) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        7        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * other[e321] * self[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0 * other[e321] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       20        0
    //    simd3        0        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       17       36        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group1();
        let wedge_g4 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]), 0.0]),
            // e1, e2, e3, e4
            (right_dual_g0 * Simd32x3::from(wedge_g4[3])).with_w(-(right_dual_g0[0] * wedge_g4[0]) - (right_dual_g0[1] * wedge_g4[1]) - (right_dual_g0[2] * wedge_g4[2])),
            // e41, e42, e43
            right_dual_g0
                * Simd32x3::from(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd3        3        7        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       22       43        0
    //  no simd       37       72        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e1234] * self[scalar]) + (other[scalar] * self[e1234])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let wedge_g3 = (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g4 = Simd32x4::from([
            (other[e23] * self[e4]) + (other[scalar] * self[e423]),
            (other[e31] * self[e4]) + (other[scalar] * self[e431]),
            (other[e12] * self[e4]) + (other[scalar] * self[e412]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) + (other.group0().yzx() * self.group1().zxy()).with_w(other[scalar] * self[e321])
            - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_dual_g0[3] * other[scalar] * self[scalar]) - (wedge_g3[0] * right_dual_g0[0]) - (wedge_g3[1] * right_dual_g0[1]) - (wedge_g3[2] * right_dual_g0[2]),
                wedge_g0_y * right_dual_g0[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                right_dual_g0[3] * wedge_g1[0],
                right_dual_g0[3] * wedge_g1[1],
                right_dual_g0[3] * wedge_g1[2],
                -(right_dual_g0[0] * wedge_g4[0]) - (right_dual_g0[1] * wedge_g4[1]) - (right_dual_g0[2] * wedge_g4[2]),
            ]) + (right_dual_g0 * wedge_g4.www().with_w(wedge_g1[3])),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * right_dual_g0.xyz())
                + (Simd32x3::from(right_dual_g0[3] * other[scalar]) * self.group2())
                + (Simd32x3::from(right_dual_g0[3] * self[scalar]) * other.group0().xyz()),
            // e23, e31, e12
            wedge_g3 * Simd32x3::from(right_dual_g0[3]),
            // e423, e431, e412, e321
            wedge_g4 * Simd32x4::from(right_dual_g0[3]),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3       11       21        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       63       87        0
    //  no simd      109      144        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[scalar] * self[e1234])
            + (other[e1234] * self[scalar])
            + (other[e423] * self[e1])
            + (other[e431] * self[e2])
            + (other[e412] * self[e3])
            + (other[e321] * self[e4])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43]);
        let wedge_g1 = (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1());
        let wedge_g2 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e4]) * other.group1().xyz())
            - (Simd32x3::from(other[e4]) * self.group1().xyz());
        let wedge_g3 = (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(self[scalar]) * other.group3()) + (other.group1().zxy() * self.group1().yzx())
            - (other.group1().yzx() * self.group1().zxy());
        let wedge_g4 = Simd32x4::from([
            (other[e4] * self[e23]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e423] * self[scalar]),
            (other[e4] * self[e31]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e431] * self[scalar]),
            (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e12] * self[e4]) + (other[e412] * self[scalar]),
            -(other[e3] * self[e12]) - (other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) + (Simd32x4::from(other[scalar]) * self.group4())
            + (self.group2().yzx() * other.group1().zxy()).with_w(other[e321] * self[scalar])
            - (other.group1().yzxx() * self.group2().zxy().with_w(self[e23]))
            - (other.group2().zxy() * self.group1().yzx()).with_w(other[e2] * self[e31]);
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (other[scalar] * other[scalar] * self[scalar])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       13        0
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * self[scalar] * other[e321], 0.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (right_dual_g0 * self[e1] * other[e423])
                    + (right_dual_g0 * self[e2] * other[e431])
                    + (right_dual_g0 * self[e3] * other[e412])
                    + (right_dual_g0 * self[e4] * other[e321]),
            ),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       23        0
    //    simd3        3        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       34        0
    //  no simd       27       59        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz());
        let wedge_g3 = (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx());
        let wedge_g4 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g3[1] * other[e3],
                wedge_g3[2] * other[e1],
                wedge_g3[0] * other[e2],
                -(wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) - (other.group0().yzxx() * wedge_g3.zxy().with_w(wedge_g2[0])),
            // e41, e42, e43
            (wedge_g4.zxy() * other.group0().yzx()) - (wedge_g4.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from([wedge_g4[3] * other[e1], wedge_g4[3] * other[e2], wedge_g4[3] * other[e3]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar] * other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group4(),
        )
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Origin {
    type Output = AntiRejectViaHorizonFromInfixPartial<Origin>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4])
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       11        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e1]) - (wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]) - (wedge_g0[3] * other[e321])),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl AntiRejectViaHorizonFrom<Line> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4]) * other.group1();
        Origin::from_groups(/* e4 */ -(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12]))
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       10        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[e4]) * other.group1().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0)
                .with_w((other[scalar] * other[scalar] * self[e4]) - (wedge_g1_xyz[0] * other[e23]) - (wedge_g1_xyz[1] * other[e31]) - (wedge_g1_xyz[2] * other[e12])),
            // e423, e431, e412, e321
            (wedge_g1_xyz * Simd32x3::from(other[scalar])).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        4        9        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       19       37        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = other[e321] * self[e4];
        let wedge_g2 = Simd32x3::from(self[e4]) * other.group1().xyz();
        let wedge_g4_xyz = Simd32x3::from(self[e4]) * other.group3();
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, wedge_g0_y * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (other[scalar] * other[scalar] * self[e4])
                    - (wedge_g0_y * other[e321])
                    - (right_dual_g2[0] * wedge_g4_xyz[0])
                    - (right_dual_g2[1] * wedge_g4_xyz[1])
                    - (right_dual_g2[2] * wedge_g4_xyz[2])
                    - (wedge_g2[0] * other[e1])
                    - (wedge_g2[1] * other[e2])
                    - (wedge_g2[2] * other[e3]),
            ),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4_xyz.zxy() * other.group1().yzx())
                - (wedge_g4_xyz.yzx() * other.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((wedge_g4_xyz * Simd32x3::from(other[scalar])) + (Simd32x3::from(wedge_g0_y) * other.group1().xyz())).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[e321] * other[e321] * -1.0)
    }
}
impl AntiRejectViaHorizonFrom<Point> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0().xyz();
        Origin::from_groups(/* e4 */ -(wedge_g0[0] * other[e1]) - (wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]))
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Origin {
    type Output = Origin;
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Plane {
    type Output = AntiRejectViaHorizonFromInfixPartial<Plane>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3        9        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       14        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (other.group1().xyz() * wedge_g0.www()).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
            // e423, e431, e412, e321
            wedge_g0 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd3        3        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       21       47        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let wedge_g4 = Simd32x4::from(other[scalar]) * self.group0();
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                right_dual_g2[0] * wedge_g4[3],
                right_dual_g2[1] * wedge_g4[3],
                right_dual_g2[2] * wedge_g4[3],
                -(right_dual_g2[0] * wedge_g4[0]) - (right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y)),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g4.zxy() * other.group1().yzx()) - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3] * -1.0) * other.group1().xyz(),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        3        7        0
    //  no simd       12       19        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            -(Simd32x3::from(self[e321] * other[e4]) * other.group0().xyz()).with_w(0.0)
                - (other.group0().xyz() * other.group0().xyz() * self.group0().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e1]) * self.group0().yxx() * other.group0().yyz()).with_w(0.0)
                - (Simd32x3::from(other[e3]) * self.group0().zzy() * other.group0().xyy()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Point {
    type Output = AntiRejectViaHorizonFromInfixPartial<Point>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       20        0
    //  no simd       15       32        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            other[e4] * self[e1] * -1.0,
            other[e4] * self[e2] * -1.0,
            other[e4] * self[e3] * -1.0,
            (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]));
        let wedge_g1_xyz = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g1_xyz[1] * other[e3],
                wedge_g1_xyz[2] * other[e1],
                wedge_g1_xyz[0] * other[e2],
                -(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]) - (wedge_g0[3] * other[e321]),
            ]) - (other.group0().yzxx() * wedge_g1_xyz.zxy().with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl AntiRejectViaHorizonFrom<Line> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       10       21        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (right_dual_g0 * Simd32x3::from(wedge_g0[3])).with_w(-(right_dual_g0[0] * wedge_g0[0]) - (right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        8       19        0
    //  no simd       14       34        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g1 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                right_dual_g0[3] * wedge_g0[0],
                right_dual_g0[3] * wedge_g0[1],
                right_dual_g0[3] * wedge_g0[2],
                -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]),
            ]) + (right_dual_g0 * wedge_g1.www().with_w(wedge_g0[3])),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(right_dual_g0[3]),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       38        0
    //    simd3        7       15        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       34       56        0
    //  no simd       60       95        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g3 = (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy());
        let wedge_g4 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0_y) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        2        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       11       21        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g1 = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                wedge_g1[1] * other[e3],
                wedge_g1[2] * other[e1],
                wedge_g1[0] * other[e2],
                -(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]),
            ]) - (other.group0().yzxx() * wedge_g1.zxy().with_w(wedge_g0[0])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectViaHorizonFromInfix> for Scalar {
    type Output = AntiRejectViaHorizonFromInfixPartial<Scalar>;
    fn div(self, _rhs: AntiRejectViaHorizonFromInfix) -> Self::Output {
        AntiRejectViaHorizonFromInfixPartial(self)
    }
}
impl AntiRejectViaHorizonFrom<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_reject_via_horizon_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar] * other[scalar], other[scalar] * other[e1234]]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       24        0
    fn anti_reject_via_horizon_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz() * wedge_g1.www() * Simd32x3::from(-1.0))
                .with_w((wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g1[3] * other[e321])),
        )
    }
}
impl AntiRejectViaHorizonFrom<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl AntiRejectViaHorizonFrom<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_reject_via_horizon_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Scalar::from_groups(/* scalar */ -(wedge_g1[0] * other[e23]) - (wedge_g1[1] * other[e31]) - (wedge_g1[2] * other[e12]))
    }
}
impl AntiRejectViaHorizonFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       23        0
    fn anti_reject_via_horizon_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[scalar],
                other[scalar],
                other[scalar],
                (wedge_g1[3] * other[scalar]) - (wedge_g1[0] * other[e23]) - (wedge_g1[1] * other[e31]) - (wedge_g1[2] * other[e12]),
            ]) * wedge_g1.xyz().with_w(1.0),
        )
    }
}
impl AntiRejectViaHorizonFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       27        0
    //    simd2        0        1        0
    //    simd3        5       13        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       44       80        0
    fn anti_reject_via_horizon_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[scalar]) + (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
                    - (right_dual_g2[0] * wedge_g3[0])
                    - (right_dual_g2[1] * wedge_g3[1])
                    - (right_dual_g2[2] * wedge_g3[2])
                    - (right_dual_g1[0] * wedge_g4[0])
                    - (right_dual_g1[1] * wedge_g4[1])
                    - (right_dual_g1[2] * wedge_g4[2])
                    - (right_dual_g1[3] * wedge_g4[3]),
                wedge_g0[1] * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g3[1] * other[e3]) + (wedge_g1[0] * other[scalar]),
                (wedge_g3[2] * other[e1]) + (wedge_g1[1] * other[scalar]),
                (wedge_g3[0] * other[e2]) + (wedge_g1[2] * other[scalar]),
                -(right_dual_g2[1] * wedge_g4[1]) - (right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_dual_g1 * Simd32x4::from(wedge_g0[1]))
                + (right_dual_g2 * wedge_g4.www()).with_w(wedge_g1[3] * other[scalar])
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_dual_g2[0] * wedge_g4[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(wedge_g0[1])) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(wedge_g0[1]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * wedge_g4.xyz())).with_w(wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiRejectViaHorizonFrom<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl AntiRejectViaHorizonFrom<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_reject_via_horizon_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]))
    }
}
impl AntiRejectViaHorizonFrom<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_via_horizon_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
