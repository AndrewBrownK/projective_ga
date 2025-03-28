// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 64
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         2       6       0     N/A
//  Average:         5      11       0     N/A
//  Maximum:        40      52       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         2      15       0       0
//  Average:        10      20       0       0
//  Maximum:        84      99       0       0
impl std::ops::Div<ProjectViaOriginOntoInfix> for DualNum {
    type Output = ProjectViaOriginOntoInfixPartial<DualNum>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4       14        0      N/A
    //  no simd        6       20        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group1().zxy()) - (wedge_g1_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g1_xyz[0] * other[e321] * -1.0,
                wedge_g1_xyz[1] * other[e321] * -1.0,
                wedge_g1_xyz[2] * other[e321] * -1.0,
                (wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) + (other[e321] * other[e321] * self[scalar]),
            ]),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl ProjectViaOriginOnto<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group1();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]))
    }
}
impl ProjectViaOriginOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       22        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())
                .with_w((wedge_g0[3] * other[scalar]) - (wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       24        0        0
    //    simd2        0        2        0      N/A
    //    simd3        6       10        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       29       41        0      N/A
    //  no simd       62       78        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = self[scalar] * other[scalar];
        let wedge_g1 = Simd32x3::from(0.0).with_w(self[scalar] * other[e321] * -1.0);
        let wedge_g2 = Simd32x3::from(self[scalar] * -1.0) * other.group3();
        let wedge_g4 = (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[e321] * other[e321])
    }
}
impl ProjectViaOriginOnto<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Flector {
    type Output = ProjectViaOriginOntoInfixPartial<Flector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       14        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       13       25        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        let wedge_g0 = Simd32x4::from([
            right_dual_g0_w * self[e1] * -1.0,
            right_dual_g0_w * self[e2] * -1.0,
            right_dual_g0_w * self[e3] * -1.0,
            (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]) - (right_dual_g0_w * self[e321]),
        ]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (wedge_g0 * Simd32x3::from(other[e321]).with_w(other[e4]))
                + Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e423]) - (wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412]))
                + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * wedge_g0.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(wedge_g0[3] * other[e321]),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       12       18        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])) + (wedge_g0_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g0_xyz.yzx() * other.group1().zxy()).with_w(wedge_g0_xyz[0] * other[e41]),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       15       19        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz =
            (self.group0().yzx() * other.group1().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e23]) * -1.0) - (self.group0().zx() * other.group1().yz()).with_z(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])) + (wedge_g0_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g0_xyz.yzx() * other.group1().zxy()).with_w(wedge_g0_xyz[0] * other[e41]),
            // e423, e431, e412, e321
            (wedge_g0_xyz * Simd32x3::from(other[e1234])).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       27        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       28       44        0      N/A
    //  no simd       60       80        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g4_xyz[0] * self[e1]) + (right_dual_g4_xyz[1] * self[e2]) + (right_dual_g4_xyz[2] * self[e3]) - (right_dual_g1_w * self[e321]);
        let wedge_g2 = Simd32x3::from([right_dual_g1_w * self[e1] * -1.0, right_dual_g1_w * self[e2] * -1.0, right_dual_g1_w * self[e3] * -1.0]);
        let wedge_g4 = ((right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx())).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       14        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * wedge_g0.xyz()).with_w(-(wedge_g0[0] * other[e423]) - (wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Horizon {
    type Output = ProjectViaOriginOntoInfixPartial<Horizon>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e321] * self[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321])
    }
}
impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       17        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e321] * self[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group4(),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Line {
    type Output = ProjectViaOriginOntoInfixPartial<Line>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       10        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3       13        0      N/A
    //  no simd        5       19        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e321] * -1.0) * self.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g0_xyz.yzx() * other.group1().zxy()) - (wedge_g0_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g0_xyz[0] * other[e321] * -1.0,
                wedge_g0_xyz[1] * other[e321] * -1.0,
                wedge_g0_xyz[2] * other[e321] * -1.0,
                (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]),
            ]),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321] * other[e321]) * self.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]);
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(wedge_g0) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd2        0        2        0      N/A
    //    simd3        5        7        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       18       27        0      N/A
    //  no simd       43       55        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        let wedge_g4 = (Simd32x3::from(other[e321] * -1.0) * self.group1()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1       10        0      N/A
    //  no simd        3       16        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e321] * -1.0) * self.group1();
        Line::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from([wedge_g0_xyz[0] * other[e321] * -1.0, wedge_g0_xyz[1] * other[e321] * -1.0, wedge_g0_xyz[2] * other[e321] * -1.0]),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Motor {
    type Output = ProjectViaOriginOntoInfixPartial<Motor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        5       16        0      N/A
    //  no simd        9       24        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let wedge_g1_xyz = (Simd32x3::from(right_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group0().xyz());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group1().zxy()) - (wedge_g1_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g1_xyz[0] * other[e321] * -1.0,
                wedge_g1_xyz[1] * other[e321] * -1.0,
                wedge_g1_xyz[2] * other[e321] * -1.0,
                (wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) - (right_dual_g0_w * other[e321] * self[scalar]),
            ]),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       10        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321] * -1.0) * (Simd32x3::from(right_dual_g0) * self.group1().xyz()).with_w(right_dual_g0 * self[scalar]),
        )
    }
}
impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       18        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g0_w = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(-(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd        9       22        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar] * -1.0) * other.group1().xyz();
        let wedge_g0_w = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g0_xyz * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_w) * other.group0().xyz())).with_w(wedge_g0_w * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x2::from(wedge_g0_w) * other.group1().xy()).with_zw(
                wedge_g0_w * other[e12],
                (wedge_g0_w * other[scalar]) - (wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12]),
            ),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       26        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       12        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       33       45        0      N/A
    //  no simd       68       86        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = (other[scalar] * self[scalar]) - (right_dual_g2[0] * self[e23]) - (right_dual_g2[1] * self[e31]) - (right_dual_g2[2] * self[e12]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g1_w * self[scalar]);
        let wedge_g2 = right_dual_g2 * Simd32x3::from(self[scalar]);
        let wedge_g4 = ((Simd32x3::from(right_dual_g1_w) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       10        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1       13        0      N/A
    //  no simd        3       19        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        let wedge_g1_xyz = Simd32x3::from(right_dual_g0) * self.group1().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g1_xyz[0] * other[e321] * -1.0,
                wedge_g1_xyz[1] * other[e321] * -1.0,
                wedge_g1_xyz[2] * other[e321] * -1.0,
                right_dual_g0 * self[scalar] * other[e321] * -1.0,
            ]),
        )
    }
}
impl ProjectViaOriginOnto<Point> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for MultiVector {
    type Output = ProjectViaOriginOntoInfixPartial<MultiVector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for MultiVector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       20        0        0
    //    simd3        2        6        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       28        0      N/A
    //  no simd       22       46        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        let wedge_g0_y = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]) - (right_dual_g0_w * self[e321]);
        let wedge_g2 = Simd32x3::from(right_dual_g0_w * -1.0) * self.group1().xyz();
        let wedge_g4_xyz = (right_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_dual_g0_w) * self.group3());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g4_xyz[0] * other[e1]) + (wedge_g4_xyz[1] * other[e2]) + (wedge_g4_xyz[2] * other[e3]) - (right_dual_g0_w * self[scalar] * other[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group0())
                + Simd32x3::from(0.0).with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0),
            // e41, e42, e43
            (wedge_g4_xyz.yzx() * other.group1().zxy()) - (wedge_g4_xyz.zxy() * other.group1().yzx()),
            // e23, e31, e12
            Simd32x3::from([wedge_g4_xyz[0] * other[e321] * -1.0, wedge_g4_xyz[1] * other[e321] * -1.0, wedge_g4_xyz[2] * other[e321] * -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0_y) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0       13        0      N/A
    //  no simd        0       17        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * self[scalar] * other[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g0 * other[e321] * -1.0) * self.group1().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0 * other[e321] * -1.0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_dual_g0 * self[e321] * other[e321] * -1.0),
        )
    }
}
impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        1        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        8       17        0      N/A
    //  no simd       16       33        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_y = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        let wedge_g2 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g4_xyz = (right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(wedge_g2[0] * other[e23]) - (wedge_g2[1] * other[e31]) - (wedge_g2[2] * other[e12]), 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g4_xyz[1] * other[e42]) - (wedge_g4_xyz[2] * other[e43])) + (wedge_g4_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g4_xyz.yzx() * other.group1().zxy()).with_w(wedge_g4_xyz[0] * other[e41]),
            // e41, e42, e43
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       15        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        8        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd       12       24        0      N/A
    //  no simd       24       41        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[scalar]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        let wedge_g2 = Simd32x3::from(self[scalar] * -1.0) * other.group1().xyz();
        let wedge_g4_xyz =
            (other.group1().zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((other[e23] * self[e2]) * -1.0) - (other.group1().yz() * self.group1().zx()).with_z(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) - (wedge_g2[0] * other[e23]) - (wedge_g2[1] * other[e31]) - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g4_xyz[1] * other[e42]) - (wedge_g4_xyz[2] * other[e43])) + (wedge_g4_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g4_xyz.yzx() * other.group1().zxy()).with_w(wedge_g4_xyz[0] * other[e41]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1().xyz(),
            // e423, e431, e412, e321
            (wedge_g4_xyz * Simd32x3::from(other[e1234])).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       30        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       15        0      N/A
    //    simd4       10        5        0      N/A
    // Totals...
    // yes simd       40       52        0      N/A
    //  no simd       84       99        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let wedge_g0_y = (other[scalar] * self[scalar]) + (right_dual_g4_xyz[0] * self[e1]) + (right_dual_g4_xyz[1] * self[e2]) + (right_dual_g4_xyz[2] * self[e3])
            - (right_dual_g1_w * self[e321])
            - (right_dual_g2[0] * self[e23])
            - (right_dual_g2[1] * self[e31])
            - (right_dual_g2[2] * self[e12]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g1_w * self[scalar]);
        let wedge_g2 = (right_dual_g2 * Simd32x3::from(self[scalar])) - (Simd32x3::from(right_dual_g1_w) * self.group1().xyz());
        let wedge_g4 = (right_dual_g4_xyz * Simd32x3::from(self[scalar])).with_w(0.0)
            + (Simd32x3::from(right_dual_g1_w) * self.group3()).with_w(0.0)
            + (right_dual_g2.yzx() * self.group1().zxy()).with_w(0.0)
            - (right_dual_g2.zxy() * self.group1().yzx()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       16        0        0
    //    simd3        1        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       22        0      N/A
    //  no simd        5       35        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        let wedge_g2 = Simd32x3::from(right_dual_g0 * -1.0) * self.group1().xyz();
        let wedge_g4_xyz = Simd32x3::from(right_dual_g0) * self.group3();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * self[scalar] * other[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            (wedge_g2 * Simd32x3::from(other[e321])).with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412])),
            // e41, e42, e43
            (wedge_g4_xyz.yzx() * other.group0().zxy()) - (wedge_g4_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from([wedge_g4_xyz[0] * other[e321] * -1.0, wedge_g4_xyz[1] * other[e321] * -1.0, wedge_g4_xyz[2] * other[e321] * -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual_g0 * self[e321] * -1.0) * other.group0(),
        )
    }
}
impl ProjectViaOriginOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       10       21        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let wedge_g4_xyz = right_dual_g0_xyz * Simd32x3::from(self[scalar]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(wedge_g4_xyz[0] * other[e1]) + (wedge_g4_xyz[1] * other[e2]) + (wedge_g4_xyz[2] * other[e3]), 0.0]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl ProjectViaOriginOnto<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Plane {
    type Output = ProjectViaOriginOntoInfixPartial<Plane>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e321] * self[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[e321] * other[e321])
    }
}
impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       17        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e321] * self[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(wedge_g0) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group4(),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Point {
    type Output = ProjectViaOriginOntoInfixPartial<Point>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       12       24        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_w = other[e321] * -1.0;
        let right_dual_g1_xyz = other.group0().xyz();
        let wedge_g0 = Simd32x4::from([
            right_dual_g0_w * self[e1] * -1.0,
            right_dual_g0_w * self[e2] * -1.0,
            right_dual_g0_w * self[e3] * -1.0,
            (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]),
        ]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (wedge_g0 * Simd32x3::from(other[e321]).with_w(other[e4]))
                + Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e423]) - (wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412]))
                + (Simd32x3::from(wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(other[e321] * other[e321]) * self.group0().xyz()).with_w(0.0))
    }
}
impl ProjectViaOriginOnto<Line> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       12       18        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])) + (wedge_g0_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g0_xyz.yzx() * other.group1().zxy()).with_w(wedge_g0_xyz[0] * other[e41]),
        )
    }
}
impl ProjectViaOriginOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd       15       19        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz =
            (other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e23] * self[e2]) * -1.0) - (other.group1().yz() * self.group0().zx()).with_z(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])) + (wedge_g0_xyz.zxy() * other.group1().yzx()).with_w(0.0)
                - (wedge_g0_xyz.yzx() * other.group1().zxy()).with_w(wedge_g0_xyz[0] * other[e41]),
            // e423, e431, e412, e321
            (wedge_g0_xyz * Simd32x3::from(other[e1234])).with_w(0.0),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       26        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        6        4        0      N/A
    // Totals...
    // yes simd       27       43        0      N/A
    //  no simd       59       79        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_w = other[e321] * -1.0;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let wedge_g0_y = (right_dual_g4_xyz[0] * self[e1]) + (right_dual_g4_xyz[1] * self[e2]) + (right_dual_g4_xyz[2] * self[e3]);
        let wedge_g2 = Simd32x3::from([right_dual_g1_w * self[e1] * -1.0, right_dual_g1_w * self[e2] * -1.0, right_dual_g1_w * self[e3] * -1.0]);
        let wedge_g4 = ((right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx())).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(other[e321]) * self.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            (wedge_g0 * Simd32x3::from(other[e321])).with_w(-(wedge_g0[0] * other[e423]) - (wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])),
        )
    }
}
impl ProjectViaOriginOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
        )
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Scalar {
    type Output = ProjectViaOriginOntoInfixPartial<Scalar>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl ProjectViaOriginOnto<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       11        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4       14        0      N/A
    //  no simd        6       20        0        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group1().zxy()) - (wedge_g1_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge_g1_xyz[0] * other[e321] * -1.0,
                wedge_g1_xyz[1] * other[e321] * -1.0,
                wedge_g1_xyz[2] * other[e321] * -1.0,
                (wedge_g1_xyz[0] * other[e1]) + (wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) + (other[e321] * other[e321] * self[scalar]),
            ]),
        )
    }
}
impl ProjectViaOriginOnto<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl ProjectViaOriginOnto<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar] * -1.0) * other.group1();
        Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]))
    }
}
impl ProjectViaOriginOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        6       22        0        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0[3]) * other.group1().xyz())
                .with_w((wedge_g0[3] * other[scalar]) - (wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
        )
    }
}
impl ProjectViaOriginOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       24        0        0
    //    simd2        0        2        0      N/A
    //    simd3        6       10        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       29       41        0      N/A
    //  no simd       62       78        0        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = other[scalar] * self[scalar];
        let wedge_g1 = Simd32x3::from(0.0).with_w(other[e321] * self[scalar] * -1.0);
        let wedge_g2 = Simd32x3::from(self[scalar] * -1.0) * other.group3();
        let wedge_g4 = (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                + Simd32x3::from(0.0)
                    .with_w(-(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (other[e42] * wedge_g4[1]) - (other[e43] * wedge_g4[2]))
                + (wedge_g2 * Simd32x3::from(other[e321])).with_w(0.0)
                + (Simd32x3::from(wedge_g4[3]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * wedge_g4.zxy()).with_w(0.0)
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234]))
                + (Simd32x3::from(wedge_g0_y) * other.group2())
                + Simd32x2::from(0.0).with_z((wedge_g4[0] * other[e431]) - (wedge_g4[1] * other[e423]))
                + (wedge_g4.yz() * other.group4().zx()).with_z(0.0)
                - (wedge_g4.zx() * other.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        )
    }
}
impl ProjectViaOriginOnto<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar])
    }
}
impl ProjectViaOriginOnto<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]))
    }
}
impl ProjectViaOriginOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
