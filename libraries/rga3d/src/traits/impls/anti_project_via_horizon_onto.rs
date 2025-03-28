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
//  Average:         6      12       0     N/A
//  Maximum:        64      78       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         2      11       0       0
//  Average:        14      23       0       0
//  Maximum:       138     153       0       0
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for AntiScalar {
    type Output = AntiProjectViaHorizonOntoInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[scalar] * other[scalar] * self[e1234])
    }
}
impl AntiProjectViaHorizonOnto<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3       13        0      N/A
    //  no simd        3       15        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = other[e321] * self[e1234] * -1.0;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                anti_wedge_g0_w * other[e1] * -1.0,
                anti_wedge_g0_w * other[e2] * -1.0,
                anti_wedge_g0_w * other[e3] * -1.0,
                (anti_wedge_g1_xyz[0] * other[e1]) + (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234] * -1.0) * other.group1();
        AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
    }
}
impl AntiProjectViaHorizonOnto<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[scalar]) * anti_wedge_g0.xyz())
                .with_w((anti_wedge_g0[3] * other[scalar]) - (anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        1        8        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       22       37        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_w = other[e321] * self[e1234] * -1.0;
        let anti_wedge_g2 = Simd32x3::from(self[e1234] * -1.0) * other.group3();
        let anti_wedge_g4_xyz = Simd32x3::from(self[e1234]) * other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g4_xyz[0] * other[e1]) + (anti_wedge_g4_xyz[1] * other[e2]) + (anti_wedge_g4_xyz[2] * other[e3]) + (other[scalar] * other[scalar] * self[e1234])
                    - (anti_wedge_g1_w * other[e321])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g1_w * other[scalar]),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) - (Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4_xyz * Simd32x3::from(other[scalar])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1_w) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * other.group1().yzx()).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234])
    }
}
impl AntiProjectViaHorizonOnto<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0_xyz[0] * other[e1]) + (anti_wedge_g0_xyz[1] * other[e2]) + (anti_wedge_g0_xyz[2] * other[e3]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for DualNum {
    type Output = AntiProjectViaHorizonOntoInfixPartial<DualNum>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        5        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            anti_wedge_g0[0] * other[scalar],
            (anti_wedge_g0[0] * other[e1234]) + (anti_wedge_g0[1] * other[scalar]),
        ]))
    }
}
impl AntiProjectViaHorizonOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3       13        0      N/A
    //  no simd        3       15        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = self[e1234] * other[e321] * -1.0;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                anti_wedge_g0_w * other[e1] * -1.0,
                anti_wedge_g0_w * other[e2] * -1.0,
                anti_wedge_g0_w * other[e3] * -1.0,
                (anti_wedge_g1_xyz[0] * other[e1]) + (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234] * -1.0) * other.group1();
        AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
    }
}
impl AntiProjectViaHorizonOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd4        2        5        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       10       24        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = right_dual_g0 * Simd32x4::from(self[e1234]);
        let anti_wedge_g1_w = self[scalar] * right_dual_g0[3];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g1_w) * other.group0())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_wedge_g1_w) * other.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       22        0        0
    //    simd2        0        3        0      N/A
    //    simd3        6       11        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       27       41        0      N/A
    //  no simd       63       81        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0);
        let anti_wedge_g2 = Simd32x3::from(self[e1234] * -1.0) * other.group3();
        let anti_wedge_g4 = (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0[0] * other[scalar],
                (anti_wedge_g0[0] * other[e1234])
                    + (anti_wedge_g0[1] * other[scalar])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0[0]) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0[0]) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge_g0[0]) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0[0]) * other.group4())
                + Simd32x3::from(0.0).with_w((other[e12] * anti_wedge_g1[2]) * -1.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (anti_wedge_g2.zxy() * other.group1().yzx()).with_w(other[e31] * anti_wedge_g1[1]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0_xyz[0] * other[e1]) + (anti_wedge_g0_xyz[1] * other[e2]) + (anti_wedge_g0_xyz[2] * other[e3]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Flector {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Flector>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd3        1        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        9       19        0      N/A
    //  no simd       23       36        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = ((right_dual_g1_xyz.yzx() * self.group1().zxy()) - (right_dual_g1_xyz.zxy() * self.group1().yzx())).with_w(0.0);
        let anti_wedge_g1 = Simd32x4::from([
            right_dual_g1_xyz[0] * self[e321] * -1.0,
            right_dual_g1_xyz[1] * self[e321] * -1.0,
            right_dual_g1_xyz[2] * self[e321] * -1.0,
            (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]) + (other[e321] * self[e321]),
        ]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            (anti_wedge_g1 * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]))
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g0.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g0.zxy().with_w(anti_wedge_g1[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       15       21        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[e321]);
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]))
                + (Simd32x3::from(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])) * other.group1()).with_w(0.0)
                + (anti_wedge_g0_xyz.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g0_xyz[0] * other[e23]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        3        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       27       38        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = (right_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
            + Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]))
            + (Simd32x3::from(right_dual_g0[3]) * self.group0().xyz()).with_w(0.0);
        let anti_wedge_g1 = Simd32x4::from(right_dual_g0[3]) * self.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            (other.group1() * Simd32x3::from(anti_wedge_g0[3]).with_w(anti_wedge_g1[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
                + (Simd32x3::from(other[scalar]) * anti_wedge_g1.xyz()).with_w(0.0)
                + (anti_wedge_g0.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       35        0        0
    //    simd2        0        2        0      N/A
    //    simd3        8       14        0      N/A
    //    simd4       11        8        0      N/A
    // Totals...
    // yes simd       42       59        0      N/A
    //  no simd       91      113        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (right_dual_g4_xyz[0] * self[e1]) + (right_dual_g4_xyz[1] * self[e2]) + (right_dual_g4_xyz[2] * self[e3]) + (self[e321] * other[e321]);
        let anti_wedge_g1 = (Simd32x4::from(other[scalar]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(right_dual_g2[0] * self[e423]) - (right_dual_g2[1] * self[e431]) - (right_dual_g2[2] * self[e412]))
            + (right_dual_g2 * Simd32x3::from(self[e321])).with_w(0.0);
        let anti_wedge_g2 = (right_dual_g4_xyz.yzx() * self.group1().zxy()) - (right_dual_g4_xyz.zxy() * self.group1().yzx());
        let anti_wedge_g3 = Simd32x3::from([
            right_dual_g4_xyz[0] * self[e321] * -1.0,
            right_dual_g4_xyz[1] * self[e321] * -1.0,
            right_dual_g4_xyz[2] * self[e321] * -1.0,
        ]);
        let anti_wedge_g4 = Simd32x4::from(other[scalar]) * self.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]) - (other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0())
    }
}
impl AntiProjectViaHorizonOnto<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       11        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd        7       19        0      N/A
    //  no simd       24       39        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0 = ((right_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_dual_g0_xyz.zxy() * self.group1().yzx())).with_w(0.0);
        let anti_wedge_g1_x = right_dual_g0_xyz[0] * self[e321] * -1.0;
        let anti_wedge_g1_y = right_dual_g0_xyz[1] * self[e321] * -1.0;
        let anti_wedge_g1_z = right_dual_g0_xyz[2] * self[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g1_y * other[e2]) - (anti_wedge_g1_z * other[e3]))
                + (Simd32x3::from(other[e4]) * Simd32x3::from([anti_wedge_g1_x, anti_wedge_g1_y, anti_wedge_g1_z])).with_w(0.0)
                + (anti_wedge_g0.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g0.zxy().with_w(anti_wedge_g1_x)),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Horizon {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Horizon>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321])
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       10       20        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e321] * -1.0) * other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            (anti_wedge_g1 * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[0] * other[e1]) - (anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]))
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       13        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1();
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])) + (anti_wedge_g0_xyz.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g0_xyz[0] * other[e23]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        9       18        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            (anti_wedge_g0_xyz * Simd32x3::from(other[scalar])).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]))
                + (anti_wedge_g0_xyz.zxy() * other.group0().yzx()).with_w(other[scalar] * other[scalar] * self[e321])
                - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g0_xyz[0] * other[e23]),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
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
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = other[e321] * self[e321];
        let anti_wedge_g1 = (Simd32x3::from(self[e321] * -1.0) * other.group3()).with_w(0.0);
        let anti_wedge_g3 = Simd32x3::from(self[e321] * -1.0) * other.group1().xyz();
        let anti_wedge_g4 = Simd32x3::from(0.0).with_w(other[scalar] * self[e321]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge_g3[0] * other[e1])
                        - (anti_wedge_g3[1] * other[e2])
                        - (anti_wedge_g3[2] * other[e3])
                        - (other[e31] * anti_wedge_g1[1])
                        - (other[e12] * anti_wedge_g1[2]),
                )
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl AntiProjectViaHorizonOnto<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       10        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e321] * -1.0) * other.group0().xyz();
        Plane::from_groups(
            // e423, e431, e412, e321
            (anti_wedge_g1 * Simd32x3::from(other[e4])).with_w(-(anti_wedge_g1[0] * other[e1]) - (anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Line {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Line>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        5        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       22       25        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(right_dual_g1_xyz[1] * self[e42]) - (right_dual_g1_xyz[2] * self[e43]))
            + (right_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g1_xyz[0] * self[e41]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0[0] * other[e423]) - (anti_wedge_g0[1] * other[e431]) - (anti_wedge_g0[2] * other[e412]))
                + (Simd32x3::from(other[e4]) * anti_wedge_g0.xyz()).with_w(0.0)
                - (Simd32x4::from(anti_wedge_g0[3]) * other.group0().xyz().with_w(other[e321])),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]);
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge_g0) * other.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        1        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       18       30        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x3::from(other[scalar]) * self.group0()).with_w(0.0);
        let anti_wedge_g1_xyz = Simd32x3::from(other[scalar]) * self.group1();
        let anti_wedge_g1_w = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g1_w) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge_g1_xyz[0] * other[e41])
                        - (anti_wedge_g1_xyz[1] * other[e42])
                        - (anti_wedge_g1_xyz[2] * other[e43])
                        - (anti_wedge_g0[0] * other[e23])
                        - (anti_wedge_g0[1] * other[e31])
                        - (anti_wedge_g0[2] * other[e12]),
                ),
            // e23, e31, e12, scalar
            ((anti_wedge_g1_xyz * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz())).with_w(anti_wedge_g1_w * other[scalar]),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       24        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       14        0      N/A
    //    simd4       10        5        0      N/A
    // Totals...
    // yes simd       34       45        0      N/A
    //  no simd       78       90        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(right_dual_g4_xyz[1] * self[e42]) - (right_dual_g4_xyz[2] * self[e43]))
            + (right_dual_g4_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g4_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g4_xyz[0] * self[e41]);
        let anti_wedge_g2 = Simd32x3::from(other[scalar]) * self.group0();
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]) - (other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       18       22        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(right_dual_g0_xyz[1] * self[e42]) - (right_dual_g0_xyz[2] * self[e43]))
            + (right_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g0_xyz[0] * self[e41]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge_g0.xyz()) - (Simd32x3::from(anti_wedge_g0[3]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g0.zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((anti_wedge_g0[0] * other[e2]) * -1.0)
                - (anti_wedge_g0.yz() * other.group0().zx()).with_z(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Motor {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Motor>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[scalar]) * anti_wedge_g0.xyz()).with_w((other[scalar] * anti_wedge_g0[3]) + (other[e1234] * anti_wedge_g1[3])),
            // e23, e31, e12, scalar
            anti_wedge_g1 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        1        5        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd       11       16        0      N/A
    //  no simd       25       32        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(right_dual_g1_xyz[1] * self[e42]) - (right_dual_g1_xyz[2] * self[e43]) - (other[e321] * self[e1234]))
            + (right_dual_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g1_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g1_xyz[0] * self[e41]);
        let anti_wedge_g1_xyz = right_dual_g1_xyz * Simd32x3::from(self[e1234]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3])
                        - (anti_wedge_g0[0] * other[e423])
                        - (anti_wedge_g0[1] * other[e431])
                        - (anti_wedge_g0[2] * other[e412]),
                )
                - (Simd32x4::from(anti_wedge_g0[3]) * other.group0().xyz().with_w(other[e321])),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       18        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[e1234]);
        let anti_wedge_g1_w = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g1_w) * other.group0())
                .with_w(-(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
            // e23, e31, e12, scalar
            (Simd32x3::from(anti_wedge_g1_w) * other.group1()).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       22       35        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = ((Simd32x3::from(other[scalar]) * self.group0().xyz()) - (Simd32x3::from(self[e1234]) * other.group1().xyz())).with_w(other[scalar] * self[e1234]);
        let anti_wedge_g1_xyz = Simd32x3::from(other[scalar]) * self.group1().xyz();
        let anti_wedge_g1_w = (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g1_w) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge_g1_xyz[0] * other[e41])
                        - (anti_wedge_g1_xyz[1] * other[e42])
                        - (anti_wedge_g1_xyz[2] * other[e43])
                        - (anti_wedge_g0[0] * other[e23])
                        - (anti_wedge_g0[1] * other[e31])
                        - (anti_wedge_g0[2] * other[e12]),
                ),
            // e23, e31, e12, scalar
            ((anti_wedge_g1_xyz * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz())).with_w(anti_wedge_g1_w * other[scalar]),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       32        0        0
    //    simd2        0        2        0      N/A
    //    simd3        8       17        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       43       57        0      N/A
    //  no simd       92      111        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (other[scalar] * self[scalar]) - (right_dual_g2[0] * self[e23]) - (right_dual_g2[1] * self[e31]) - (right_dual_g2[2] * self[e12]);
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(right_dual_g4_xyz[1] * self[e42]) - (right_dual_g4_xyz[2] * self[e43]) - (self[e1234] * other[e321]))
            + (right_dual_g4_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g4_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g4_xyz[0] * self[e41]);
        let anti_wedge_g2 = (right_dual_g2 * Simd32x3::from(self[e1234])) + (Simd32x3::from(other[scalar]) * self.group0().xyz());
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group1().xyz();
        let anti_wedge_g4 = (right_dual_g4_xyz * Simd32x3::from(self[e1234])).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    + (other[scalar] * other[scalar] * self[e1234])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]) - (other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        1        6        0      N/A
    //    simd4        4        1        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       21       27        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0 = Simd32x3::from(0.0).with_w(-(right_dual_g0_xyz[1] * self[e42]) - (right_dual_g0_xyz[2] * self[e43]))
            + (right_dual_g0_xyz.zxy() * self.group1().yzx()).with_w(0.0)
            - (right_dual_g0_xyz.yzx() * self.group1().zxy()).with_w(right_dual_g0_xyz[0] * self[e41]);
        let anti_wedge_g1_xyz = right_dual_g0_xyz * Simd32x3::from(self[e1234]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w((anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]))
                - (Simd32x3::from(anti_wedge_g0[3]) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for MultiVector {
    type Output = AntiProjectViaHorizonOntoInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1       12        0      N/A
    //  no simd        1       23        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([anti_wedge_g0[0] * other[scalar], (anti_wedge_g0[0] * other[e1234]) + (anti_wedge_g0[1] * other[scalar])]),
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
impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       28        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       10        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       25       43        0      N/A
    //  no simd       54       76        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_x = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3])
            - (right_dual_g0[0] * self[e423])
            - (right_dual_g0[1] * self[e431])
            - (right_dual_g0[2] * self[e412])
            - (right_dual_g0[3] * self[e321]);
        let anti_wedge_g1 = (right_dual_g0 * Simd32x4::from(self[e1234]))
            + Simd32x3::from(0.0).with_w(-(right_dual_g1_xyz[1] * self[e42]) - (right_dual_g1_xyz[2] * self[e43]))
            + (right_dual_g1_xyz.zxy() * self.group3().yzx()).with_w(0.0)
            - (right_dual_g1_xyz.yzx() * self.group3().zxy()).with_w(right_dual_g1_xyz[0] * self[e41]);
        let anti_wedge_g2 = (right_dual_g1_xyz.yzx() * self.group4().zxy()) - (right_dual_g1_xyz.zxy() * self.group4().yzx());
        let anti_wedge_g3 = Simd32x3::from([
            right_dual_g1_xyz[0] * self[e321] * -1.0,
            right_dual_g1_xyz[1] * self[e321] * -1.0,
            right_dual_g1_xyz[2] * self[e321] * -1.0,
        ]);
        let anti_wedge_g4_xyz = right_dual_g1_xyz * Simd32x3::from(self[e1234]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g4_xyz[0] * other[e1]) + (anti_wedge_g4_xyz[1] * other[e2]) + (anti_wedge_g4_xyz[2] * other[e3])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0_x) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g1.zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((anti_wedge_g1[0] * other[e2]) * -1.0)
                - (anti_wedge_g1.yz() * other.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(anti_wedge_g0_x) * other.group1())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        7        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_dual_g0 * self[e1234] * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_dual_g0 * self[e321] * other[e321] * -1.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        0        8        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       19       36        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        let anti_wedge_g1_xyz = right_dual_g0 * Simd32x3::from(self[e321]);
        let anti_wedge_g2 = right_dual_g0 * Simd32x3::from(self[e1234]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(anti_wedge_g2[0] * other[e23]) - (anti_wedge_g2[1] * other[e31]) - (anti_wedge_g2[2] * other[e12])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0_x) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge_g0_x) * other.group1(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g1_xyz[1] * other[e31]) - (anti_wedge_g1_xyz[2] * other[e12]))
                + (Simd32x3::from(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])) * other.group1()).with_w(0.0)
                + (anti_wedge_g1_xyz.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g1_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g1_xyz[0] * other[e23]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       19        0        0
    //    simd3        3       10        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       22       35        0      N/A
    //  no simd       46       73        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0_x = (self[scalar] * right_dual_g0[3]) - (self[e23] * right_dual_g0[0]) - (self[e31] * right_dual_g0[1]) - (self[e12] * right_dual_g0[2]);
        let anti_wedge_g1 = (right_dual_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
            + Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]))
            + (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()).with_w(0.0);
        let anti_wedge_g2 = (Simd32x3::from(self[e1234]) * right_dual_g0.xyz()) + (Simd32x3::from(right_dual_g0[3]) * self.group2());
        let anti_wedge_g3 = Simd32x3::from(right_dual_g0[3]) * self.group3();
        let anti_wedge_g4 = Simd32x4::from(right_dual_g0[3]) * self.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234]) + (self[e1234] * right_dual_g0[3] * other[scalar])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43]),
            ]),
            // e1, e2, e3, e4
            anti_wedge_g1 * Simd32x4::from(other[scalar]),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()),
            // e423, e431, e412, e321
            (other.group1() * Simd32x3::from(anti_wedge_g1[3]).with_w(anti_wedge_g4[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]))
                + (Simd32x3::from(other[scalar]) * anti_wedge_g4.xyz()).with_w(0.0)
                + (anti_wedge_g1.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       35       44        0        0
    //    simd2        0        4        0      N/A
    //    simd3       13       19        0      N/A
    //    simd4       16       11        0      N/A
    // Totals...
    // yes simd       64       78        0      N/A
    //  no simd      138      153        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4 = other.group1().xyz().with_w(0.0);
        let anti_wedge_g0_x =
            (other[scalar] * self[scalar]) + (right_dual_g4[0] * self[e1]) + (right_dual_g4[1] * self[e2]) + (right_dual_g4[2] * self[e3]) + (right_dual_g4[3] * self[e4])
                - (right_dual_g2[0] * self[e23])
                - (right_dual_g2[1] * self[e31])
                - (right_dual_g2[2] * self[e12])
                - (right_dual_g1[0] * self[e423])
                - (right_dual_g1[1] * self[e431])
                - (right_dual_g1[2] * self[e412])
                - (right_dual_g1[3] * self[e321]);
        let anti_wedge_g1 = (right_dual_g1 * Simd32x4::from(self[e1234]))
            + (Simd32x4::from(other[scalar]) * self.group1())
            + Simd32x3::from(0.0).with_w(
                -(right_dual_g2[0] * self[e423])
                    - (right_dual_g2[1] * self[e431])
                    - (right_dual_g2[2] * self[e412])
                    - (self[e42] * right_dual_g4[1])
                    - (self[e43] * right_dual_g4[2]),
            )
            + (right_dual_g2 * Simd32x3::from(self[e321])).with_w(0.0)
            + (Simd32x3::from(right_dual_g4[3]) * self.group2()).with_w(0.0)
            + (self.group3().yzx() * right_dual_g4.zxy()).with_w(0.0)
            - (right_dual_g4.yzxx() * self.group3().zxy().with_w(self[e41]));
        let anti_wedge_g2 = (right_dual_g2 * Simd32x3::from(self[e1234]))
            + (Simd32x3::from(other[scalar]) * self.group2())
            + Simd32x2::from(0.0).with_z((right_dual_g4[0] * self[e431]) - (right_dual_g4[1] * self[e423]))
            + (right_dual_g4.yz() * self.group4().zx()).with_z(0.0)
            - (right_dual_g4.zx() * self.group4().yz()).with_z(0.0);
        let anti_wedge_g3 =
            (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(right_dual_g4[3]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * right_dual_g4.xyz());
        let anti_wedge_g4 = (right_dual_g4 * Simd32x4::from(self[e1234])) + (Simd32x4::from(other[scalar]) * self.group4());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    + (other[scalar] * other[scalar] * self[e1234])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]) - (other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_dual_g0 * self[e1234] * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual_g0 * self[e321] * -1.0) * other.group0(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       19        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       10        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       15       34        0      N/A
    //  no simd       44       67        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(right_dual_g0_xyz[1] * self[e42]) - (right_dual_g0_xyz[2] * self[e43]))
            + (right_dual_g0_xyz.zxy() * self.group3().yzx()).with_w(0.0)
            - (right_dual_g0_xyz.yzx() * self.group3().zxy()).with_w(right_dual_g0_xyz[0] * self[e41]);
        let anti_wedge_g2 = (right_dual_g0_xyz.yzx() * self.group4().zxy()) - (right_dual_g0_xyz.zxy() * self.group4().yzx());
        let anti_wedge_g3 = Simd32x3::from([
            right_dual_g0_xyz[0] * self[e321] * -1.0,
            right_dual_g0_xyz[1] * self[e321] * -1.0,
            right_dual_g0_xyz[2] * self[e321] * -1.0,
        ]);
        let anti_wedge_g4_xyz = right_dual_g0_xyz * Simd32x3::from(self[e1234]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (anti_wedge_g4_xyz[0] * other[e1]) + (anti_wedge_g4_xyz[1] * other[e2]) + (anti_wedge_g4_xyz[2] * other[e3])]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_dual_g0_xyz[2] * self[e3]) * other.group0()),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g1.zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((anti_wedge_g1[0] * other[e2]) * -1.0)
                - (anti_wedge_g1.yz() * other.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for MultiVector {
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
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
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
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Origin {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Origin>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4])
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[scalar]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       11        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[e4];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, anti_wedge_g0 * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[scalar]),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0 * -1.0) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group3()).with_w(0.0),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Plane {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Plane>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        9        0        0
    //    simd3        1        4        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       20       33        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = ((right_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_dual_g1_xyz.zxy() * self.group0().yzx())).with_w(0.0);
        let anti_wedge_g1 = Simd32x4::from([
            right_dual_g1_xyz[0] * self[e321] * -1.0,
            right_dual_g1_xyz[1] * self[e321] * -1.0,
            right_dual_g1_xyz[2] * self[e321] * -1.0,
            other[e321] * self[e321],
        ]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            (anti_wedge_g1 * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]))
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g0.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g0.zxy().with_w(anti_wedge_g1[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[e321] * other[e321])
    }
}
impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        5        0      N/A
    //    simd4        3        0        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       15       21        0        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[e321]);
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]))
                + (Simd32x3::from(-(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412])) * other.group1()).with_w(0.0)
                + (anti_wedge_g0_xyz.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g0_xyz[0] * other[e23]),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd       19       31        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x3::from(self[e321] * -1.0) * other.group1().xyz()).with_w((other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412]));
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            (other.group1() * Simd32x3::from(anti_wedge_g0[3]).with_w(anti_wedge_g1[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
                + (Simd32x3::from(other[scalar]) * anti_wedge_g1.xyz()).with_w(0.0)
                + (anti_wedge_g0.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       32        0        0
    //    simd2        0        2        0      N/A
    //    simd3        8       14        0      N/A
    //    simd4        9        7        0      N/A
    // Totals...
    // yes simd       37       55        0      N/A
    //  no simd       80      106        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = other[e321] * self[e321];
        let anti_wedge_g1 =
            (right_dual_g2 * Simd32x3::from(self[e321])).with_w(-(right_dual_g2[0] * self[e423]) - (right_dual_g2[1] * self[e431]) - (right_dual_g2[2] * self[e412]));
        let anti_wedge_g2 = (right_dual_g4_xyz.yzx() * self.group0().zxy()) - (right_dual_g4_xyz.zxy() * self.group0().yzx());
        let anti_wedge_g3 = Simd32x3::from([
            right_dual_g4_xyz[0] * self[e321] * -1.0,
            right_dual_g4_xyz[1] * self[e321] * -1.0,
            right_dual_g4_xyz[2] * self[e321] * -1.0,
        ]);
        let anti_wedge_g4 = Simd32x4::from(other[scalar]) * self.group0();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g3[0] * other[e41])
                    - (anti_wedge_g3[1] * other[e42])
                    - (anti_wedge_g3[2] * other[e43])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]) - (other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (anti_wedge_g3 * Simd32x3::from(other[e4])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl AntiProjectViaHorizonOnto<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        8        0        0
    //    simd3        1        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd       16       24        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0 = (right_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_dual_g0_xyz.zxy() * self.group0().yzx());
        let anti_wedge_g1 = Simd32x3::from([
            right_dual_g0_xyz[0] * self[e321] * -1.0,
            right_dual_g0_xyz[1] * self[e321] * -1.0,
            right_dual_g0_xyz[2] * self[e321] * -1.0,
        ]);
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]))
                + (anti_wedge_g1 * Simd32x3::from(other[e4])).with_w(0.0)
                + (anti_wedge_g0.yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * anti_wedge_g0.zxy().with_w(anti_wedge_g1[0])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Point {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Point>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = (right_dual_g1_xyz[0] * self[e1]) + (right_dual_g1_xyz[1] * self[e2]) + (right_dual_g1_xyz[2] * self[e3]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge_g0) * other.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       13       20        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
                + (Simd32x3::from(anti_wedge_g0[3]) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g0.zxy() * other.group0().yzx()).with_w(0.0)
                - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd2        0        2        0      N/A
    //    simd3        5        6        0      N/A
    //    simd4        5        5        0      N/A
    // Totals...
    // yes simd       18       26        0      N/A
    //  no simd       43       55        0        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (right_dual_g4_xyz[0] * self[e1]) + (right_dual_g4_xyz[1] * self[e2]) + (right_dual_g4_xyz[2] * self[e3]);
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0_x) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge_g0_x) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge_g0_x) * other.group3())
                + Simd32x2::from(0.0).with_z((anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]))
                + (anti_wedge_g1.zx() * other.group1().yz()).with_z(0.0)
                - (anti_wedge_g1.yz() * other.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + Simd32x3::from(0.0).with_w(-(other[e31] * anti_wedge_g1[1]) - (other[e12] * anti_wedge_g1[2]))
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g1.yzxx() * other.group2().zxy().with_w(other[e23])),
        )
    }
}
impl AntiProjectViaHorizonOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
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
impl AntiProjectViaHorizonOnto<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectViaHorizonOntoInfix> for Scalar {
    type Output = AntiProjectViaHorizonOntoInfixPartial<Scalar>;
    fn div(self, _rhs: AntiProjectViaHorizonOntoInfix) -> Self::Output {
        AntiProjectViaHorizonOntoInfixPartial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[scalar];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_wedge_g0) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_wedge_g0) * other.group1(),
        )
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Scalar {
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
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[scalar];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_wedge_g0) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge_g0) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge_g0) * other.group4(),
        )
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
