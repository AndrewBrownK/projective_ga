// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 84
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         2       6       0     N/A
//  Average:         5      11       0     N/A
//  Maximum:        55      73       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         2      10       0       0
//  Average:         9      20       0       0
//  Maximum:       107     141       0       0
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for AntiScalar {
    type Output = AntiRejectOrthogonallyFromInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[scalar] * other[scalar] * self[e1234])
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar] * self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       11        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[scalar] * self[e1234];
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
impl AntiRejectOrthogonallyFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for DualNum {
    type Output = AntiRejectOrthogonallyFromInfixPartial<DualNum>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        6        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            other[scalar] * other[scalar] * self[scalar],
            (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]),
        ]))
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        4       19        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_anti_dual_g1_xyz.yzx() * wedge_g1.zxy()) - (right_anti_dual_g1_xyz.zxy() * wedge_g1.yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (right_anti_dual_g1_xyz * Simd32x4::from(wedge_g1[3]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[3] * other[e321]) + (self[scalar] * right_anti_dual_g1_xyz[0] * other[e1])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectOrthogonallyFrom<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Scalar::from_groups(/* scalar */ (wedge_g1[0] * other[e23]) + (wedge_g1[1] * other[e31]) + (wedge_g1[2] * other[e12]))
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd        7       21        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[scalar] * other[scalar]) * other.group0().xyz()) - (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[scalar]),
            // e23, e31, e12, scalar
            (wedge_g1.xyz() * Simd32x4::from(other[scalar]).xyz())
                .with_w((wedge_g1[0] * other[e23]) + (wedge_g1[1] * other[e31]) + (wedge_g1[2] * other[e12]) + (wedge_g1[3] * other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       23        0        0
    //    simd3        7       12        0      N/A
    //    simd4        3        6        0      N/A
    // Totals...
    // yes simd       25       41        0      N/A
    //  no simd       48       83        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[scalar] * other[scalar] * other[scalar])
                    + (right_anti_dual_g4[0] * wedge_g1[0])
                    + (right_anti_dual_g4[1] * wedge_g1[1])
                    + (right_anti_dual_g4[2] * wedge_g1[2])
                    + (right_anti_dual_g4[3] * wedge_g1[3])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectOrthogonallyFrom<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * wedge_g0[2]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Flector {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Flector>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        8       14        0      N/A
    //  no simd       20       28        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]))
            + Simd32x3::from(0.0).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]))
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e2] * self[e1]) - (other[e1] * self[e2]), 0.0])
            + ((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((right_anti_dual_g1_xyz.zxy() * wedge_g1.yzx()) - (right_anti_dual_g1_xyz.yzx() * wedge_g1.zxy())).with_w(wedge_g0[3] * other[e321] * -1.0),
            // e423, e431, e412, e321
            (right_anti_dual_g1_xyz * Simd32x4::from(wedge_g0[3]).xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[e321] * other[e321] * -1.0)
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       12       28        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g1 = (Simd32x3::from([
            (self[e3] * other[e42]) - (self[e2] * other[e43]),
            (self[e1] * other[e43]) - (self[e3] * other[e41]),
            (self[e2] * other[e41]) - (self[e1] * other[e42]),
        ]) + (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (Simd32x3::from(other[scalar]) * self.group1().xyz()))
        .with_w(self[e321] * other[scalar]);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[scalar]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       34        0        0
    //    simd3       11       16        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       37       55        0      N/A
    //  no simd       68      102        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
            - (self[e423] * other[e1])
            - (self[e431] * other[e2])
            - (self[e412] * other[e3])
            - (self[e321] * other[e4]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g3 = Simd32x3::from([
            (self[e2] * other[e3]) - (self[e3] * other[e2]),
            (self[e3] * other[e1]) - (self[e1] * other[e3]),
            (self[e1] * other[e2]) - (self[e2] * other[e1]),
        ]);
        let wedge_g4 = ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[e4]) * other.group3()) + (other.group2().yzx() * self.group0().zxy())
            - (other.group2().zxy() * self.group0().yzx()))
        .with_w(other[scalar] * self[e321]);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (self[e1] * other[e423] * other[e321]) - (self[e2] * other[e431] * other[e321]) - (self[e3] * other[e412] * other[e321]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        5        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       10       21        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (self[e1] * other[e2]) - (self[e2] * other[e1]), 0.0])
            + ((self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz())).with_zw(0.0, 0.0);
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((right_anti_dual_g0_xyz.zxy() * wedge_g1.yzx()) - (right_anti_dual_g0_xyz.yzx() * wedge_g1.zxy())).with_w(0.0),
            // e423, e431, e412, e321
            (right_anti_dual_g0_xyz * Simd32x4::from(self[e321]).xyz() * Simd32x4::from(other[e4]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Horizon {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Horizon>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321])
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        7        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
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
impl AntiRejectOrthogonallyFrom<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[scalar] * self[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(wedge_g0 * -1.0) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(wedge_g0 * other[scalar]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       14        0        0
    //    simd3        2        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        6       21        0      N/A
    //  no simd       13       37        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = other[e4] * self[e321] * -1.0;
        let wedge_g4 = Simd32x3::from(0.0).with_w(other[scalar] * self[e321]);
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g1_w * wedge_g4[3] * -1.0, wedge_g0_y * other[scalar]]),
            // e1, e2, e3, e4
            (right_anti_dual_g2 * Simd32x4::from(wedge_g4[3]).xyz()).with_w(right_anti_dual_g1_w * wedge_g0_y),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        7        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321] * -1.0) * Simd32x4::from(other[e4]).xyz() * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Line {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Line>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        5        0      N/A
    // no simd        9       15        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e4]) * self.group1()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx());
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_anti_dual_g1_xyz.yzx() * wedge_g0_xyz.zxy()) - (right_anti_dual_g1_xyz.zxy() * wedge_g0_xyz.yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        0
    //    simd3        5        7        0      N/A
    // Totals...
    // yes simd        5       16        0      N/A
    //  no simd       15       30        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e23]) * Simd32x3::from([other[e31] * self[e42], other[e31] * self[e41], other[e12] * self[e41]]))
                + (Simd32x3::from(other[e12]) * Simd32x3::from([other[e23] * self[e43], other[e31] * self[e43], other[e31] * self[e42]]))
                + (Simd32x3::from(other[e41] * self[e23]) * other.group1())
                + (Simd32x3::from(other[e42] * self[e31]) * other.group1())
                + (Simd32x3::from(other[e43] * self[e12]) * other.group1())
                + (other.group1() * other.group1() * self.group0()),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       10       23        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let wedge_g1_xyz = Simd32x3::from(other[scalar]) * self.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[scalar] * other[scalar]) * self.group0()) - (Simd32x3::from(wedge_g0_w) * other.group1().xyz())).with_w(wedge_g0_w * other[scalar]),
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[scalar]).xyz()).with_w((wedge_g1_xyz[0] * other[e23]) + (wedge_g1_xyz[1] * other[e31]) + (wedge_g1_xyz[2] * other[e12])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd3        9       15        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       21       36        0      N/A
    //  no simd       42       72        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let wedge_g2 = Simd32x3::from(other[scalar]) * self.group0();
        let wedge_g3 = Simd32x3::from(other[scalar]) * self.group1();
        let wedge_g4 = ((Simd32x3::from(other[e4]) * self.group1()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0);
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_anti_dual_g1_w * wedge_g4[3]) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
            .with_w(right_anti_dual_g1_w * wedge_g0_y),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        5        0      N/A
    // no simd        9       15        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = (Simd32x3::from(other[e4]) * self.group1()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx());
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Line::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz.yzx() * wedge_g0_xyz.zxy()) - (right_anti_dual_g0_xyz.zxy() * wedge_g0_xyz.yzx()),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Motor {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Motor>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        9        0      N/A
    //  no simd        1       14        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().xyz() * Simd32x2::from(other[scalar] * other[scalar]).with_z(other[scalar] * other[scalar]))
                .with_w((other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       13       26        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from([
            (other[e3] * self[e42]) - (other[e2] * self[e43]),
            (other[e1] * self[e43]) - (other[e3] * self[e41]),
            (other[e2] * self[e41]) - (other[e1] * self[e42]),
        ]) + (Simd32x3::from(other[e4]) * self.group1().xyz())
            + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        let wedge_g1_w = other[e321] * self[scalar];
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_anti_dual_g1_xyz.yzx() * wedge_g1_xyz.zxy()) - (right_anti_dual_g1_xyz.zxy() * wedge_g1_xyz.yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (right_anti_dual_g1_xyz * Simd32x3::from(wedge_g1_w * -1.0)).with_w((wedge_g1_w * other[e321]) + (right_anti_dual_g1_xyz[0] * other[e1] * self[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd3        5        8        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       17       33        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (-(right_anti_dual_g0 * Simd32x3::from(other[e41] * self[e23]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e42] * self[e31]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e43] * self[e12]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e23] * self[e41]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e31] * self[e42]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e12] * self[e43])))
            .with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       22       35        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x4::from(other[scalar]) * self.group0())
            + (Simd32x4::from(self[scalar]) * other.group0())
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            );
        let wedge_g1_xyz = (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[scalar]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            (wedge_g1_xyz * Simd32x4::from(other[scalar]).xyz())
                .with_w((wedge_g1_xyz[0] * other[e23]) + (wedge_g1_xyz[1] * other[e31]) + (wedge_g1_xyz[2] * other[e12]) + (other[scalar] * other[scalar] * self[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       36        0        0
    //    simd3       11       16        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       38       57        0      N/A
    //  no simd       69      104        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = (Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group2());
        let wedge_g3 = (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group3());
        let wedge_g4 = (Simd32x3::from([
            (self[e42] * other[e3]) - (self[e43] * other[e2]),
            (self[e43] * other[e1]) - (self[e41] * other[e3]),
            (self[e41] * other[e2]) - (self[e42] * other[e1]),
        ]) + (Simd32x3::from(self[scalar]) * other.group4().xyz())
            + (Simd32x3::from(other[e4]) * self.group1().xyz()))
        .with_w(self[scalar] * other[e321]);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g4[0] * wedge_g1[0])
                    + (right_anti_dual_g4[1] * wedge_g1[1])
                    + (right_anti_dual_g4[2] * wedge_g1[2])
                    + (right_anti_dual_g4[3] * wedge_g1[3])
                    + (other[scalar] * other[scalar] * self[scalar])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd       13       23        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (self[e41] * other[e2]) - (self[e42] * other[e1]), 0.0])
            + ((Simd32x3::from(other[e4]) * self.group1().xyz()) + ((self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz())).with_z(0.0))
                .with_w(0.0);
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_anti_dual_g0_xyz.yzx() * wedge_g1.zxy()) - (right_anti_dual_g0_xyz.zxy() * wedge_g1.yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (right_anti_dual_g0_xyz * Simd32x4::from(wedge_g1[3]).xyz() * Simd32x3::from(-1.0)).with_w(right_anti_dual_g0_xyz[0] * self[scalar] * other[e1]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for MultiVector {
    type Output = AntiRejectOrthogonallyFromInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       10        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1       14        0      N/A
    //  no simd        1       24        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
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
impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       22        0        0
    //    simd3        5       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       18       33        0      N/A
    //  no simd       28       56        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g3 = Simd32x3::from([
            (other[e3] * self[e2]) - (other[e2] * self[e3]),
            (other[e1] * self[e3]) - (other[e3] * self[e1]),
            (other[e2] * self[e1]) - (other[e1] * self[e2]),
        ]);
        let wedge_g4_xyz = (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
            - (self.group2().zxy() * other.group0().yzx());
        let wedge_g4_w = self[scalar] * other[e321];
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g1_xyz[0] * wedge_g1[0]) + (right_anti_dual_g1_xyz[1] * wedge_g1[1]) + (right_anti_dual_g1_xyz[2] * wedge_g1[2])
                    - (right_anti_dual_g0_w * wedge_g4_w),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((right_anti_dual_g1_xyz.zxy() * wedge_g3.yzx()) - (right_anti_dual_g1_xyz.yzx() * wedge_g3.zxy())).with_w(right_anti_dual_g0_w * wedge_g0_y),
            // e41, e42, e43
            (right_anti_dual_g1_xyz.yzx() * wedge_g4_xyz.zxy()) - (right_anti_dual_g1_xyz.zxy() * wedge_g4_xyz.yzx()),
            // e23, e31, e12
            right_anti_dual_g1_xyz * Simd32x3::from(wedge_g4_w * -1.0),
            // e423, e431, e412, e321
            (right_anti_dual_g1_xyz * Simd32x3::from(wedge_g0_y)).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        6        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g0 * self[scalar] * other[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * self[e4] * other[e321]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd3        5        8        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       17       33        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            -(right_anti_dual_g0 * Simd32x3::from(other[e41] * self[e23]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e42] * self[e31]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e43] * self[e12]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e23] * self[e41]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e31] * self[e42]))
                - (right_anti_dual_g0 * Simd32x3::from(other[e12] * self[e43])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       24        0        0
    //    simd3        6       10        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       19       36        0      N/A
    //  no simd       31       62        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])
            - (self[e41] * other[e23])
            - (self[e42] * other[e31])
            - (self[e43] * other[e12])
            - (self[e23] * other[e41])
            - (self[e31] * other[e42])
            - (self[e12] * other[e43]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let wedge_g3 = (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * self.group3());
        let wedge_g4 = (Simd32x3::from([
            (other[e42] * self[e3]) - (other[e43] * self[e2]),
            (other[e43] * self[e1]) - (other[e41] * self[e3]),
            (other[e41] * self[e2]) - (other[e42] * self[e1]),
        ]) + (Simd32x3::from(other[scalar]) * self.group4().xyz())
            + (Simd32x3::from(self[e4]) * other.group1().xyz()))
        .with_w(other[scalar] * self[e321]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[scalar] * other[scalar] * other[scalar]) + (wedge_g3[0] * other[e23]) + (wedge_g3[1] * other[e31]) + (wedge_g3[2] * other[e12]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(other[scalar]) * wedge_g1.xyz()) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz())).with_w(wedge_g1[3] * other[scalar]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar] * other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar] * other[scalar]) * other.group0().xyz())
                - (Simd32x3::from(wedge_g0_y) * other.group1().xyz()),
            // e23, e31, e12
            wedge_g3 * Simd32x3::from(other[scalar]),
            // e423, e431, e412, e321
            wedge_g4 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       32       43        0        0
    //    simd3       17       22        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       55       73        0      N/A
    //  no simd      107      141        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[scalar] * self[e1234])
            + (other[e1234] * self[scalar])
            + (other[e423] * self[e1])
            + (other[e431] * self[e2])
            + (other[e412] * self[e3])
            + (other[e321] * self[e4])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321]);
        let wedge_g1 = (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1());
        let wedge_g2 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e4]) * other.group1().xyz())
            - (Simd32x3::from(other[e4]) * self.group1().xyz());
        let wedge_g3 = Simd32x3::from([
            (other[e3] * self[e2]) - (other[e2] * self[e3]),
            (other[e1] * self[e3]) - (other[e3] * self[e1]),
            (other[e2] * self[e1]) - (other[e1] * self[e2]),
        ]) + (Simd32x3::from(other[scalar]) * self.group3())
            + (Simd32x3::from(self[scalar]) * other.group3());
        let wedge_g4 = (Simd32x4::from(other[scalar]) * self.group4())
            + (Simd32x4::from(self[scalar]) * other.group4())
            + ((Simd32x3::from(other[e4]) * self.group3())
                + (Simd32x3::from(self[e4]) * other.group3())
                + (other.group2().yzx() * self.group1().zxy())
                + (self.group2().yzx() * other.group1().zxy())
                - (other.group2().zxy() * self.group1().yzx())
                - (self.group2().zxy() * other.group1().yzx()))
            .with_w(0.0);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g4[0] * wedge_g1[0])
                    + (right_anti_dual_g4[1] * wedge_g1[1])
                    + (right_anti_dual_g4[2] * wedge_g1[2])
                    + (right_anti_dual_g4[3] * wedge_g1[3])
                    + (other[scalar] * other[scalar] * self[scalar])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3       12        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g0 * self[scalar] * other[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (right_anti_dual_g0 * self[e1] * other[e423])
                    + (right_anti_dual_g0 * self[e2] * other[e431])
                    + (right_anti_dual_g0 * self[e3] * other[e412])
                    + (right_anti_dual_g0 * self[e4] * other[e321]),
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
impl AntiRejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd3        7       11        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       25        0      N/A
    //  no simd       26       50        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g3 = Simd32x3::from([
            (self[e2] * other[e3]) - (self[e3] * other[e2]),
            (self[e3] * other[e1]) - (self[e1] * other[e3]),
            (self[e1] * other[e2]) - (self[e2] * other[e1]),
        ]);
        let wedge_g4_xyz = (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx());
        let right_anti_dual_g0_xyz = other.group0().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g0_xyz[0] * wedge_g1[0]) + (right_anti_dual_g0_xyz[1] * wedge_g1[1]) + (right_anti_dual_g0_xyz[2] * wedge_g1[2]),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((right_anti_dual_g0_xyz.zxy() * wedge_g3.yzx()) - (right_anti_dual_g0_xyz.yzx() * wedge_g3.zxy())).with_w(0.0),
            // e41, e42, e43
            (right_anti_dual_g0_xyz.yzx() * wedge_g4_xyz.zxy()) - (right_anti_dual_g0_xyz.zxy() * wedge_g4_xyz.yzx()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (-(right_anti_dual_g0_xyz * Simd32x3::from(self[e423] * other[e1]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e431] * other[e2]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e412] * other[e3]))
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * other[e4])))
            .with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       21        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
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
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Origin {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Origin>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4])
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       11        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]);
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                -(right_anti_dual_g1_xyz[0] * wedge_g0[0]) - (right_anti_dual_g1_xyz[1] * wedge_g0[1]) - (right_anti_dual_g1_xyz[2] * wedge_g0[2]) - (wedge_g0[3] * other[e321]),
            ),
            // e423, e431, e412, e321
            (right_anti_dual_g1_xyz * Simd32x4::from(wedge_g0[3]).xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4]) * other.group1();
        Origin::from_groups(/* e4 */ (wedge_g0_xyz[0] * other[e23]) + (wedge_g0_xyz[1] * other[e31]) + (wedge_g0_xyz[2] * other[e12]))
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[e4]) * other.group1().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0)
                .with_w((wedge_g1_xyz[0] * other[e23]) + (wedge_g1_xyz[1] * other[e31]) + (wedge_g1_xyz[2] * other[e12]) + (other[scalar] * other[scalar] * self[e4])),
            // e423, e431, e412, e321
            (wedge_g1_xyz * Simd32x4::from(other[scalar]).xyz()).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        4        9        0      N/A
    // Totals...
    // yes simd       11       20        0      N/A
    //  no simd       19       38        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = other[e321] * self[e4];
        let wedge_g2 = Simd32x3::from(self[e4]) * other.group1().xyz();
        let wedge_g4_xyz = Simd32x3::from(self[e4]) * other.group3();
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, wedge_g0_y * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (other[scalar] * other[scalar] * self[e4])
                    - (wedge_g0_y * other[e321])
                    - (right_anti_dual_g2[0] * wedge_g4_xyz[0])
                    - (right_anti_dual_g2[1] * wedge_g4_xyz[1])
                    - (right_anti_dual_g2[2] * wedge_g4_xyz[2])
                    - (right_anti_dual_g4_xyz[0] * wedge_g2[0])
                    - (right_anti_dual_g4_xyz[1] * wedge_g2[1])
                    - (right_anti_dual_g4_xyz[2] * wedge_g2[2]),
            ),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (right_anti_dual_g4_xyz.yzx() * wedge_g4_xyz.zxy())
                - (right_anti_dual_g4_xyz.zxy() * wedge_g4_xyz.yzx()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((right_anti_dual_g4_xyz * Simd32x3::from(wedge_g0_y)) + (wedge_g4_xyz * Simd32x3::from(other[scalar]))).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0)
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0().xyz();
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Origin::from_groups(
            // e4
            -(right_anti_dual_g0_xyz[0] * wedge_g0[0]) - (right_anti_dual_g0_xyz[1] * wedge_g0[1]) - (right_anti_dual_g0_xyz[2] * wedge_g0[2]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Plane {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Plane>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
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
impl AntiRejectOrthogonallyFrom<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0[3]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            wedge_g0 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        2        5        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        9       23        0      N/A
    //  no simd       16       42        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let wedge_g4 = Simd32x4::from(other[scalar]) * self.group0();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g1_w * wedge_g4[3] * -1.0, wedge_g0_y * other[scalar]]),
            // e1, e2, e3, e4
            (right_anti_dual_g2 * Simd32x4::from(wedge_g4[3]).xyz()).with_w(right_anti_dual_g1_w * wedge_g0_y),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       10        0        0
    //    simd2        0        2        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4       15        0      N/A
    //  no simd       13       23        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, other[e3] * other[e3] * self[e412] * -1.0, 0.0])
                + (-(other.group0().xy() * other.group0().xy() * self.group0().xy()).with_z(0.0)
                    - (Simd32x3::from(other[e1]) * Simd32x3::from([self[e431] * other[e2], self[e423] * other[e2], self[e423] * other[e3]]))
                    - (Simd32x3::from(other[e3]) * Simd32x3::from([self[e412] * other[e1], self[e412] * other[e2], self[e431] * other[e2]]))
                    - (Simd32x3::from(self[e321] * other[e4]) * other.group0().xyz()))
                .with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Point {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Point>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       10       18        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w = other[e423] * self[e1];
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e2] * self[e1]) - (other[e1] * self[e2]), 0.0])
            + ((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_zw(0.0, 0.0);
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((right_anti_dual_g1_xyz.zxy() * wedge_g1.yzx()) - (right_anti_dual_g1_xyz.yzx() * wedge_g1.zxy())).with_w(wedge_g0_w * other[e321] * -1.0),
            // e423, e431, e412, e321
            (right_anti_dual_g1_xyz * Simd32x3::from(wedge_g0_w)).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[e321] * other[e321] * -1.0)
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       13       24        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g1 = Simd32x4::from([0.0, 0.0, (other[e41] * self[e2]) - (other[e42] * self[e1]), 0.0])
            + ((Simd32x3::from(self[e4]) * other.group1().xyz()) + ((other.group0().yz() * self.group0().zx()) - (other.group0().zx() * self.group0().yz())).with_z(0.0))
                .with_w(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[scalar]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g1[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       29        0        0
    //    simd3       10       15        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       32       49        0      N/A
    //  no simd       61       94        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        let wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        let wedge_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g3 = Simd32x3::from([
            (other[e3] * self[e2]) - (other[e2] * self[e3]),
            (other[e1] * self[e3]) - (other[e3] * self[e1]),
            (other[e2] * self[e1]) - (other[e1] * self[e2]),
        ]);
        let wedge_g4 = ((Simd32x3::from(self[e4]) * other.group3()) + (other.group2().yzx() * self.group0().zxy()) - (other.group2().zxy() * self.group0().yzx())).with_w(0.0);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0_y)) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        8        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       12        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from([
            (other[e3] * self[e2]) - (other[e2] * self[e3]),
            (other[e1] * self[e3]) - (other[e3] * self[e1]),
            (other[e2] * self[e1]) - (other[e1] * self[e2]),
        ]);
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            ((right_anti_dual_g0_xyz.zxy() * wedge_g1.yzx()) - (right_anti_dual_g0_xyz.yzx() * wedge_g1.zxy())).with_w(0.0),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Scalar {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Scalar>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        4       19        0        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g1_xyz = other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_anti_dual_g1_xyz.yzx() * wedge_g1.zxy()) - (right_anti_dual_g1_xyz.zxy() * wedge_g1.yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (right_anti_dual_g1_xyz * Simd32x4::from(wedge_g1[3]).xyz() * Simd32x3::from(-1.0))
                .with_w((wedge_g1[3] * other[e321]) + (right_anti_dual_g1_xyz[0] * other[e1] * self[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Scalar::from_groups(/* scalar */ (wedge_g1[0] * other[e23]) + (wedge_g1[1] * other[e31]) + (wedge_g1[2] * other[e12]))
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       22        0        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[scalar]) * wedge_g0.xyz()) - (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())).with_w(wedge_g0[3] * other[scalar]),
            // e23, e31, e12, scalar
            (wedge_g1.xyz() * Simd32x4::from(other[scalar]).xyz())
                .with_w((wedge_g1[0] * other[e23]) + (wedge_g1[1] * other[e31]) + (wedge_g1[2] * other[e12]) + (wedge_g1[3] * other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       20        0        0
    //    simd2        0        1        0      N/A
    //    simd3        7       12        0      N/A
    //    simd4        3        6        0      N/A
    // Totals...
    // yes simd       24       39        0      N/A
    //  no simd       47       82        0        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[scalar])
                    + (right_anti_dual_g4[0] * wedge_g1[0])
                    + (right_anti_dual_g4[1] * wedge_g1[1])
                    + (right_anti_dual_g4[2] * wedge_g1[2])
                    + (right_anti_dual_g4[3] * wedge_g1[3])
                    - (right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0[1] * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0[1]))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                + ((right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])) + (wedge_g2 * Simd32x3::from(right_anti_dual_g4[3])) + (wedge_g3.yzx() * right_anti_dual_g4.zxy())
                    - (wedge_g3.zxy() * right_anti_dual_g4.yzx()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (right_anti_dual_g4[1] * wedge_g4[2]) - (right_anti_dual_g4[2] * wedge_g4[1]),
                (right_anti_dual_g4[2] * wedge_g4[0]) - (right_anti_dual_g4[0] * wedge_g4[2]),
                (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0]),
            ]) + (right_anti_dual_g2 * Simd32x3::from(wedge_g0[1]))
                + (wedge_g2 * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(right_anti_dual_g4[3]) * wedge_g4.xyz()) - (Simd32x3::from(wedge_g4[3]) * right_anti_dual_g4.xyz()),
            // e423, e431, e412, e321
            (right_anti_dual_g4 * Simd32x4::from(wedge_g0[1])) + (wedge_g4 * Simd32x4::from(other[scalar])),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Scalar::from_groups(
            // scalar
            (right_anti_dual_g0_xyz[0] * wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * wedge_g0[2]),
        )
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
