// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 74
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         2       7       0     N/A
//  Average:         4      10       0     N/A
//  Maximum:        41      52       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         3      13       0       0
//  Average:        10      18       0       0
//  Maximum:        82      94       0       0
impl std::ops::Div<RejectViaOriginFromInfix> for AntiScalar {
    type Output = RejectViaOriginFromInfixPartial<AntiScalar>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl RejectViaOriginFrom<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       13        0      N/A
    //  no simd        3       16        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual_g0_w * anti_wedge_g0[0] * -1.0,
                right_dual_g0_w * anti_wedge_g0[1] * -1.0,
                right_dual_g0_w * anti_wedge_g0[2] * -1.0,
                (right_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_dual_g1_xyz[2] * anti_wedge_g0[2])
                    - (right_dual_g0_w * self[e1234] * other[e321]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl RejectViaOriginFrom<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e1234]) * other.group1();
        AntiScalar::from_groups(/* e1234 */ (anti_wedge_g1[0] * other[e23]) + (anti_wedge_g1[1] * other[e31]) + (anti_wedge_g1[2] * other[e12]))
    }
}
impl RejectViaOriginFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       12        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g1[3] * -1.0) * other.group1().xyz())
                .with_w((anti_wedge_g1[0] * other[e23]) + (anti_wedge_g1[1] * other[e31]) + (anti_wedge_g1[2] * other[e12]) + (anti_wedge_g1[3] * other[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        8        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       22       41        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(self[e1234]) * other.group0();
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group3();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0[0] * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * self[e1234] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g1_w * anti_wedge_g0[0]),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0[0])) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0[0])).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl RejectViaOriginFrom<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]),
        )
    }
}
impl RejectViaOriginFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for DualNum {
    type Output = RejectViaOriginFromInfixPartial<DualNum>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        4        0        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]))
    }
}
impl RejectViaOriginFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       13        0      N/A
    //  no simd        3       16        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual_g0_w * anti_wedge_g0[0] * -1.0,
                right_dual_g0_w * anti_wedge_g0[1] * -1.0,
                right_dual_g0_w * anti_wedge_g0[2] * -1.0,
                (right_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_dual_g1_xyz[2] * anti_wedge_g0[2])
                    - (right_dual_g0_w * self[e1234] * other[e321]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl RejectViaOriginFrom<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e1234]) * other.group1();
        AntiScalar::from_groups(/* e1234 */ (anti_wedge_g1[0] * other[e23]) + (anti_wedge_g1[1] * other[e31]) + (anti_wedge_g1[2] * other[e12]))
    }
}
impl RejectViaOriginFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd        4       14        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group1().xyz();
        let anti_wedge_g1_w = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x2::from(anti_wedge_g1_w * -1.0) * other.group1().xy()).with_zw(
                anti_wedge_g1_w * other[e12] * -1.0,
                (anti_wedge_g1_w * other[scalar]) + (anti_wedge_g1_xyz[0] * other[e23]) + (anti_wedge_g1_xyz[1] * other[e31]) + (anti_wedge_g1_xyz[2] * other[e12]),
            ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd3        1        8        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       23       41        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group3();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * self[e1234] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl RejectViaOriginFrom<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]),
        )
    }
}
impl RejectViaOriginFrom<Scalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Flector {
    type Output = RejectViaOriginFromInfixPartial<Flector>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       16       22        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
            + Simd32x3::from(0.0).with_w(
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            )
            - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]));
        let right_dual_g0_w = other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0_w * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            ((Simd32x3::from(right_dual_g0_w) * anti_wedge_g1.xyz()) + (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]);
        let right_dual_g0 = other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0 * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual_g0) * anti_wedge_g1.xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        5        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       16       21        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(self[e431] * other[e42]) - (self[e412] * other[e43]))
            + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((right_dual_g0.yzx() * anti_wedge_g0.zxy()) - (right_dual_g0.zxy() * anti_wedge_g0.yzx())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       20       26        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x4::from(other[e1234]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(self[e431] * other[e42]) - (self[e412] * other[e43]))
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            + (self.group1().zxy() * other.group1().yzx()).with_w(0.0)
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_dual_g0.zxy())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       21        0        0
    //    simd3        2       11        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       24       34        0      N/A
    //  no simd       49       62        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
            - (self[e423] * other[e1])
            - (self[e431] * other[e2])
            - (self[e412] * other[e3])
            - (self[e321] * other[e4]);
        let anti_wedge_g1 = (Simd32x4::from(other[e1234]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(self[e431] * other[e42]) - (self[e412] * other[e43]))
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            + (other.group3().yzx() * self.group1().zxy()).with_w(0.0)
            - (self.group1().yzxx() * other.group3().zxy().with_w(other[e41]));
        let anti_wedge_g3 = (Simd32x3::from(other[e321]) * self.group1().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * self[e321] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       15        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
            + Simd32x3::from(0.0).with_w((self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]))
            - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0);
        let right_dual_g0 = other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0 * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual_g0) * anti_wedge_g1.xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Point> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       22        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            -(Simd32x3::from(self[e321] * other[e4]) * other.group0().xyz()).with_w(0.0)
                - (other.group0().xyz() * other.group0().xyz() * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(other[e1]) * self.group1().yxx() * other.group0().yyz()).with_w(0.0)
                - (Simd32x3::from(other[e3]) * self.group1().zzy() * other.group0().xyy()).with_w(0.0),
        )
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Horizon {
    type Output = RejectViaOriginFromInfixPartial<Horizon>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        3       13        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e321] * -1.0) * other.group1().xyz().with_w(other[e4]);
        let right_dual_g0_w = other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0_w * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            ((Simd32x3::from(right_dual_g0_w) * anti_wedge_g1.xyz()) + (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        4        0      N/A
    // no simd        3       12        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0);
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((right_dual_g0.yzx() * anti_wedge_g0.zxy()) - (right_dual_g0.zxy() * anti_wedge_g0.yzx())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       13        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0);
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_dual_g0.zxy())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd3        1        9        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       22       41        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = self[e321] * other[e4] * -1.0;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e321]) * other.group2();
        let anti_wedge_g3 = Simd32x3::from(self[e321] * -1.0) * other.group4().xyz();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (anti_wedge_g1_xyz[0] * right_dual_g4_xyz[0])
                    + (anti_wedge_g1_xyz[1] * right_dual_g4_xyz[1])
                    + (anti_wedge_g1_xyz[2] * right_dual_g4_xyz[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * self[e321] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (anti_wedge_g1_xyz * Simd32x3::from(right_dual_g1_w)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (anti_wedge_g1_xyz.zxy() * right_dual_g2.yzx()).with_w(0.0)
                - (anti_wedge_g1_xyz.yzx() * right_dual_g2.zxy()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e321] * other[e321]) * other.group0().xyz()).with_w(0.0))
    }
}
impl RejectViaOriginFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e321] * other[e4] * -1.0) * other.group0().xyz()).with_w(0.0))
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Line {
    type Output = RejectViaOriginFromInfixPartial<Line>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd       15       22        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(other[e431] * self[e42]) - (other[e412] * self[e43]))
            + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
            + (self.group1().yzx() * other.group1().zxy()).with_w(0.0)
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]));
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual_g0_w * anti_wedge_g0[0] * -1.0,
                right_dual_g0_w * anti_wedge_g0[1] * -1.0,
                right_dual_g0_w * anti_wedge_g0[2] * -1.0,
                (right_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_dual_g1_xyz[2] * anti_wedge_g0[2]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * other[e321]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        0
    //    simd3        5        7        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd       15       29        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e41] * self[e23]) * other.group1())
                + (Simd32x3::from(other[e42] * self[e31]) * other.group1())
                + (Simd32x3::from(other[e43] * self[e12]) * other.group1())
                + (Simd32x3::from([other[e31] * self[e42], other[e23] * self[e41], other[e23] * self[e41]]) * other.group1())
                + (other.group1() * Simd32x2::from(other[e12] * self[e43]).with_z(other[e31] * self[e42]))
                + (other.group1() * other.group1() * self.group0()),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd        8       18        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_xyz = Simd32x3::from(other[e1234]) * self.group1();
        let anti_wedge_g1_w =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x2::from(anti_wedge_g1_w * -1.0) * other.group1().xy()).with_zw(
                anti_wedge_g1_w * other[e12] * -1.0,
                (anti_wedge_g1_w * other[scalar]) + (anti_wedge_g1_xyz[0] * other[e23]) + (anti_wedge_g1_xyz[1] * other[e31]) + (anti_wedge_g1_xyz[2] * other[e12]),
            ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       17        0        0
    //    simd3        1       10        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       19       28        0      N/A
    //  no simd       39       51        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
            + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
            + (self.group1().yzx() * other.group4().zxy()).with_w(0.0)
            - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]));
        let anti_wedge_g3 = Simd32x3::from(other[e1234]) * self.group1();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        6       16        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e31] * other[e412], self[e12] * other[e423], self[e23] * other[e431]]))
                + (Simd32x3::from(other[e321] * other[e321]) * self.group0())
                - (Simd32x3::from(other[e321]) * Simd32x3::from([self[e12] * other[e431], self[e23] * other[e412], self[e31] * other[e423]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Motor {
    type Output = RejectViaOriginFromInfixPartial<Motor>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        4        0        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]))
    }
}
impl RejectViaOriginFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       14        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       20       28        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(other[e431] * self[e42]) - (other[e412] * self[e43]))
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            + (other.group1().zxy() * self.group1().yzx()).with_w(0.0)
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]));
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual_g0_w * anti_wedge_g0[0] * -1.0,
                right_dual_g0_w * anti_wedge_g0[1] * -1.0,
                right_dual_g0_w * anti_wedge_g0[2] * -1.0,
                (right_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_dual_g1_xyz[2] * anti_wedge_g0[2])
                    - (right_dual_g0_w * other[e321] * self[e1234]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321]) * (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(other[e321] * self[e1234]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd        7       18        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group1();
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (right_dual_g0
                * Simd32x3::from(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ))
            .with_w(-(anti_wedge_g1_xyz[0] * right_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       16       22        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from(self[e1234]) * other.group1())
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            );
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g1[3] * -1.0) * other.group1().xyz())
                .with_w((anti_wedge_g1[0] * other[e23]) + (anti_wedge_g1[1] * other[e31]) + (anti_wedge_g1[2] * other[e12]) + (anti_wedge_g1[3] * other[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       21        0        0
    //    simd3        2       11        0      N/A
    //    simd4        7        2        0      N/A
    // Totals...
    // yes simd       24       34        0      N/A
    //  no simd       49       62        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[e1234] * other[scalar]) + (self[scalar] * other[e1234])
            - (self[e41] * other[e23])
            - (self[e42] * other[e31])
            - (self[e43] * other[e12])
            - (self[e23] * other[e41])
            - (self[e31] * other[e42])
            - (self[e12] * other[e43]);
        let anti_wedge_g1 = (self.group0() * Simd32x3::from(other[e321]).with_w(other[e4]))
            + Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
            + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
            + (self.group1().yzx() * other.group4().zxy()).with_w(0.0)
            - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]));
        let anti_wedge_g3 = (Simd32x3::from(self[e1234]) * other.group3()) + (Simd32x3::from(other[e1234]) * self.group1().xyz());
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * self[e1234] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       17        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321])
                * (Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
                    + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                    + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                    - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])))
                .xyz()
                .with_w(self[e1234] * other[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<Point> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]),
        )
    }
}
impl RejectViaOriginFrom<Scalar> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for MultiVector {
    type Output = RejectViaOriginFromInfixPartial<MultiVector>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        4        0        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]))
    }
}
impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       25        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd       17       32        0      N/A
    //  no simd       35       48        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321]);
        let anti_wedge_g1 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(other[e431] * self[e42]) - (other[e412] * self[e43]))
            + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
            + (self.group3().yzx() * other.group1().zxy()).with_w(0.0)
            - (other.group1().yzxx() * self.group3().zxy().with_w(self[e41]));
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual_g1_xyz[0] * anti_wedge_g1[0]) + (right_dual_g1_xyz[1] * anti_wedge_g1[1]) + (right_dual_g1_xyz[2] * anti_wedge_g1[2])
                    - (right_dual_g0_w * other[e321] * self[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g0_w),
            // e41, e42, e43
            Simd32x3::from([
                right_dual_g0_w * anti_wedge_g1[0] * -1.0,
                right_dual_g0_w * anti_wedge_g1[1] * -1.0,
                right_dual_g0_w * anti_wedge_g1[2] * -1.0,
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (right_dual_g1_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0) + (Simd32x3::from(right_dual_g0_w * other[e321]) * self.group4().xyz()).with_w(0.0)
                - (Simd32x3::from(right_dual_g0_w * self[e321]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       15        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_dual_g0 * other[e321] * self[e1234] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0 * other[e321] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(right_dual_g0 * other[e321] * -1.0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual_g0 * other[e321]) * self.group4().xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        6       12        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       33       51        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
            - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]));
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group1();
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(anti_wedge_g3[0] * right_dual_g0[0]) - (anti_wedge_g3[1] * right_dual_g0[1]) - (anti_wedge_g3[2] * right_dual_g0[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            -(right_dual_g0 * Simd32x3::from(other[e41] * self[e23]))
                - (right_dual_g0 * Simd32x3::from(other[e42] * self[e31]))
                - (right_dual_g0 * Simd32x3::from(other[e43] * self[e12]))
                - (right_dual_g0 * Simd32x3::from(other[e23] * self[e41]))
                - (right_dual_g0 * Simd32x3::from(other[e31] * self[e42]))
                - (right_dual_g0 * Simd32x3::from(other[e12] * self[e43])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((right_dual_g0.yzx() * anti_wedge_g1.zxy()) - (right_dual_g0.zxy() * anti_wedge_g1.yzx())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       15        0        0
    //    simd3        2        7        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       17       24        0      N/A
    //  no simd       33       44        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e1234] * self[scalar]) + (other[scalar] * self[e1234])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43]);
        let anti_wedge_g1 = (other.group0() * Simd32x3::from(self[e321]).with_w(self[e4]))
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
            - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]));
        let anti_wedge_g3 = (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(self[e1234]) * other.group1().xyz());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g3[0] * other[e23]) + (anti_wedge_g3[1] * other[e31]) + (anti_wedge_g3[2] * other[e12]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0_x * -1.0) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((anti_wedge_g1.yzx() * other.group1().zxy()) - (anti_wedge_g1.zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       26       33        0        0
    //    simd3        4       15        0      N/A
    //    simd4       11        4        0      N/A
    // Totals...
    // yes simd       41       52        0      N/A
    //  no simd       82       94        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[scalar] * self[e1234])
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
        let anti_wedge_g1 = (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from(self[e1234]) * other.group1())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]) - (other[e431] * self[e42]) - (other[e412] * self[e43]))
            + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            + (other.group3().yzx() * self.group4().zxy()).with_w(0.0)
            + (self.group3().yzx() * other.group4().zxy()).with_w(0.0)
            - (other.group4().yzxx() * self.group3().zxy().with_w(self[e41]))
            - (self.group4().yzxx() * other.group3().zxy().with_w(other[e41]));
        let anti_wedge_g3 = (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(other[e321]) * self.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group3())
            - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * other[e1234] * self[e321])
                    - (right_dual_g1_w * other[e321] * self[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       21        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        6       26        0      N/A
    //  no simd       12       36        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_dual_g0 * self[e1234] * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (right_dual_g0 * self[e1] * other[e423])
                    + (right_dual_g0 * self[e2] * other[e431])
                    + (right_dual_g0 * self[e3] * other[e412])
                    + (right_dual_g0 * self[e4] * other[e321]),
            ),
            // e41, e42, e43
            (Simd32x3::from(right_dual_g0) * Simd32x3::from([self[e12] * other[e431], self[e23] * other[e412], self[e31] * other[e423]]))
                - (Simd32x3::from(right_dual_g0) * Simd32x3::from([self[e31] * other[e412], self[e12] * other[e423], self[e23] * other[e431]]))
                - (Simd32x3::from(right_dual_g0 * other[e321]) * self.group2()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(right_dual_g0 * other[e321]) * self.group4().xyz()) - (Simd32x3::from(right_dual_g0 * self[e321]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       14       23        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group0();
        let right_dual_g0_xyz = other.group0().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual_g0_xyz[0] * anti_wedge_g1[0]) + (right_dual_g0_xyz[1] * anti_wedge_g1[1]) + (right_dual_g0_xyz[2] * anti_wedge_g1[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            -(right_dual_g0_xyz * Simd32x3::from(self[e423] * other[e1])).with_w(0.0)
                - (right_dual_g0_xyz * Simd32x3::from(self[e431] * other[e2])).with_w(0.0)
                - (right_dual_g0_xyz * Simd32x3::from(self[e412] * other[e3])).with_w(0.0)
                - (right_dual_g0_xyz * Simd32x3::from(self[e321] * other[e4])).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Scalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Origin {
    type Output = RejectViaOriginFromInfixPartial<Origin>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[e321] * self[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl RejectViaOriginFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       11        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = other[e321] * self[e4];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, anti_wedge_g0_x * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0_x * -1.0) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[e321] * other[e321] * -1.0)
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Plane {
    type Output = RejectViaOriginFromInfixPartial<Plane>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       13       18        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]))
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]));
        let right_dual_g0_w = other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual_g0_w * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            ((Simd32x3::from(right_dual_g0_w) * anti_wedge_g1.xyz()) + (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(other[e321] * other[e321] * -1.0) * self.group0().xyz()).with_w(0.0))
    }
}
impl RejectViaOriginFrom<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        5        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       16       21        0        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((right_dual_g0.yzx() * anti_wedge_g0.zxy()) - (right_dual_g0.zxy() * anti_wedge_g0.yzx())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        4        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       16       22        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_dual_g0.zxy())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd3        2       11        0      N/A
    //    simd4        6        1        0      N/A
    // Totals...
    // yes simd       19       29        0      N/A
    //  no simd       41       54        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            + (other.group3().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let anti_wedge_g3 = (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2])
                    - (anti_wedge_g3[0] * right_dual_g2[0])
                    - (anti_wedge_g3[1] * right_dual_g2[1])
                    - (anti_wedge_g3[2] * right_dual_g2[2])
                    - (right_dual_g1_w * other[e1234] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_dual_g1_w)).with_w(0.0)
                + (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0)
                + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3        8        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(other[e321] * self[e321]) * other.group0().xyz()) - (Simd32x3::from(other[e321] * other[e321]) * self.group0().xyz())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        7        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd       12       22        0        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
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
impl std::ops::Div<RejectViaOriginFromInfix> for Point {
    type Output = RejectViaOriginFromInfixPartial<Point>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl RejectViaOriginFrom<Motor> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       14        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_dual_g0.zxy())).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        1        6        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       17       32        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        let anti_wedge_g1 = Simd32x4::from(other[e1234]) * self.group0();
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar])
                    + (right_dual_g4_xyz[0] * anti_wedge_g1[0])
                    + (right_dual_g4_xyz[1] * anti_wedge_g1[1])
                    + (right_dual_g4_xyz[2] * anti_wedge_g1[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_dual_g1_w),
            // e41, e42, e43
            (right_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) - (Simd32x3::from(right_dual_g1_w) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (right_dual_g4_xyz * Simd32x3::from(anti_wedge_g0_x)).with_w(0.0) + (right_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        )
    }
}
impl RejectViaOriginFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        )
    }
}
impl std::ops::Div<RejectViaOriginFromInfix> for Scalar {
    type Output = RejectViaOriginFromInfixPartial<Scalar>;
    fn div(self, _rhs: RejectViaOriginFromInfix) -> Self::Output {
        RejectViaOriginFromInfixPartial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[scalar] * other[e1234] * self[scalar])
    }
}
impl RejectViaOriginFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234] * self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl RejectViaOriginFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       11        0        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[e1234] * self[scalar];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, anti_wedge_g0 * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0 * -1.0) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
