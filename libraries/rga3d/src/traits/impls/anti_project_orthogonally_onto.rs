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
//   Median:         1       5       0     N/A
//  Average:         4      10       0     N/A
//  Maximum:        55      73       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         1      10       0       0
//  Average:         9      20       0       0
//  Maximum:       107     141       0       0
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for AntiScalar {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[scalar] * other[scalar] * self[e1234])
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1        9        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = other[e321] * self[e1234] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g0_w * -1.0) * other.group0().xyz()).with_w((other[e1] * other[e1] * self[e1234]) - (anti_wedge_g0_w * other[e321])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234] * -1.0) * other.group1();
        AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       15        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.xyz() * Simd32x4::from(other[scalar]).xyz())
                .with_w((anti_wedge_g0[3] * other[scalar]) - (anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       19       37        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
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
            ((anti_wedge_g4_xyz * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g1_w) * other.group3()) + (anti_wedge_g2.yzx() * other.group1().zxy())
                - (anti_wedge_g2.zxy() * other.group1().yzx()))
            .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234])
    }
}
impl AntiProjectOrthogonallyOnto<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0_xyz[0] * other[e1]) + (anti_wedge_g0_xyz[1] * other[e2]) + (anti_wedge_g0_xyz[2] * other[e3]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for DualNum {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<DualNum>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        5        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            anti_wedge_g0[0] * other[scalar],
            (anti_wedge_g0[0] * other[e1234]) + (anti_wedge_g0[1] * other[scalar]),
        ]))
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1        9        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = self[e1234] * other[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g0_w * -1.0) * other.group0().xyz()).with_w((other[e1] * other[e1] * self[e1234]) - (anti_wedge_g0_w * other[e321])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234] * -1.0) * other.group1();
        AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]))
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd4        2        5        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       10       24        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = right_anti_dual_g0 * Simd32x4::from(self[e1234]);
        let anti_wedge_g1_w = self[scalar] * right_anti_dual_g0[3];
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
impl AntiProjectOrthogonallyOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       23        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       12        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       26       41        0      N/A
    //  no simd       51       81        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0);
        let anti_wedge_g2 = Simd32x3::from(self[e1234] * -1.0) * other.group3();
        let anti_wedge_g4 = Simd32x4::from([1.0, 1.0, self[e1234], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[e1234]).with_z(1.0)).with_w(0.0);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (Simd32x3::from(anti_wedge_g0[0]) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0[0]) * other.group4())
                + ((Simd32x3::from(anti_wedge_g1[3]) * other.group3()) + (anti_wedge_g2.yzx() * other.group1().zxy()) + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (anti_wedge_g2.zxy() * other.group1().yzx())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3       11        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([1.0, 1.0, self[e1234], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[e1234]).with_z(1.0)).with_w(0.0);
        AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0[0] * other[e1]) + (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3]) + (anti_wedge_g0[3] * other[e4]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Flector {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Flector>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd       13       26        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group1().yzx());
        let anti_wedge_g1_w = (right_anti_dual_g1_xyz[0] * self[e1]) + (other[e321] * self[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1_w) * other.group0(),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz()) + (anti_wedge_g0_xyz.yzx() * other.group0().zxy())
                - (right_anti_dual_g1_xyz * Simd32x3::from(other[e4] * self[e321]))
                - (anti_wedge_g0_xyz.zxy() * other.group0().yzx()))
            .with_w(anti_wedge_g1_w * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       10        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1();
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       12       28        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = ((Simd32x3::from(other[scalar]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group1().xyz())).with_w(self[e4] * other[scalar]);
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (anti_wedge_g0[2] * other[e42]) - (anti_wedge_g0[1] * other[e43]),
                (anti_wedge_g0[0] * other[e43]) - (anti_wedge_g0[2] * other[e41]),
                (anti_wedge_g0[1] * other[e41]) - (anti_wedge_g0[0] * other[e42]),
            ]) + (Simd32x3::from(anti_wedge_g0[3]) * other.group1().xyz())
                + (Simd32x3::from(other[scalar]) * anti_wedge_g1.xyz()))
            .with_w(anti_wedge_g1[3] * other[scalar]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       28        0        0
    //    simd3       12       17        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       35       50        0      N/A
    //  no simd       68       99        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (right_anti_dual_g4_xyz[0] * self[e1]) + (right_anti_dual_g4_xyz[1] * self[e2]) + (right_anti_dual_g4_xyz[2] * self[e3]) + (self[e321] * other[e321]);
        let anti_wedge_g1 = ((Simd32x3::from(other[scalar]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group3())).with_w(other[scalar] * self[e4]);
        let anti_wedge_g2 = (right_anti_dual_g4_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g4_xyz.zxy() * self.group1().yzx());
        let anti_wedge_g3 = right_anti_dual_g4_xyz * Simd32x3::from(self[e321] * -1.0);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + ((anti_wedge_g3 * Simd32x3::from(other[e4]))
                    + (Simd32x3::from(anti_wedge_g1[3]) * other.group3())
                    + (anti_wedge_g2.yzx() * other.group1().zxy())
                    + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (anti_wedge_g2.zxy() * other.group1().yzx())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(self[e321]).xyz() * Simd32x4::from(other[e321]).xyz() * other.group0().xyz()).with_w(other[e321] * other[e321] * self[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        9       21        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_xyz[0] * self[e1]) * other.group0(),
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.yzx() * other.group0().zxy())
                - (right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * other[e4]))
                - (anti_wedge_g0_xyz.zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Horizon {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Horizon>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        3       18        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e321]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())).with_w(anti_wedge_g1[3] * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       10        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1();
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        3       15        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            (anti_wedge_g0_xyz * Simd32x4::from(other[scalar]).xyz()).with_w(0.0),
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(other[scalar] * other[scalar] * self[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       23        0        0
    //    simd3        7       11        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       24       38        0      N/A
    //  no simd       47       72        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + ((anti_wedge_g3 * Simd32x3::from(other[e4])) + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()) + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        7        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321] * -1.0) * Simd32x4::from(other[e4]).xyz() * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Line {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Line>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        5        0      N/A
    // no simd        6       15        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0_xyz * Simd32x4::from(other[e4]).xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        1        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       18       30        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (self.group0() * Simd32x4::from(other[scalar]).xyz()).with_w(0.0);
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
impl AntiProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       22        0        0
    //    simd3       12       17        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       28       41        0      N/A
    //  no simd       55       81        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        let anti_wedge_g1 = ((right_anti_dual_g4_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g4_xyz.yzx() * self.group1().zxy())).with_w(0.0);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            ((anti_wedge_g3 * Simd32x3::from(other[e4]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group4().xyz())
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3())
                + (anti_wedge_g2.yzx() * other.group1().zxy())
                + (other.group2().yzx() * anti_wedge_g1.zxy())
                - (anti_wedge_g2.zxy() * other.group1().yzx())
                - (other.group2().zxy() * anti_wedge_g1.yzx()))
            .with_w(anti_wedge_g0_x * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        5        0      N/A
    // no simd        6       15        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy());
        Line::from_groups(
            // e41, e42, e43
            anti_wedge_g0_xyz * Simd32x3::from(other[e4]),
            // e23, e31, e12
            (anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Motor {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Motor>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        1       17        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.xyz() * Simd32x2::from(other[scalar]).with_z(other[scalar])).with_w((other[scalar] * anti_wedge_g0[3]) + (other[e1234] * anti_wedge_g1[3])),
            // e23, e31, e12, scalar
            anti_wedge_g1 * Simd32x4::from(other[scalar]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        5        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       13       26        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().xyzx() * Simd32x3::from(other[e321] * self[e1234]).with_w(right_anti_dual_g1_xyz[0] * self[e1234]))
                + (anti_wedge_g0_xyz * Simd32x3::from(other[e4])).with_w(
                    (other[e321] * other[e321] * self[e1234]) - (anti_wedge_g0_xyz[0] * other[e423]) - (anti_wedge_g0_xyz[1] * other[e431]) - (anti_wedge_g0_xyz[2] * other[e412]),
                ),
            // e23, e31, e12, scalar
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        4       18        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_xyz = right_anti_dual_g0 * Simd32x3::from(self[e1234]);
        let anti_wedge_g1_w = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g1_w) * other.group0())
                .with_w(-(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
            // e23, e31, e12, scalar
            (Simd32x3::from(anti_wedge_g1_w) * other.group1()).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       22       35        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       21       30        0        0
    //    simd3       12       19        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       36       53        0      N/A
    //  no simd       69      103        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (other[scalar] * self[scalar]) - (right_anti_dual_g2[0] * self[e23]) - (right_anti_dual_g2[1] * self[e31]) - (right_anti_dual_g2[2] * self[e12]);
        let anti_wedge_g1 = ((right_anti_dual_g4_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g4_xyz.yzx() * self.group1().zxy())).with_w(self[e1234] * other[e321] * -1.0);
        let anti_wedge_g2 = (right_anti_dual_g2 * Simd32x3::from(self[e1234])) + (Simd32x3::from(other[scalar]) * self.group0().xyz());
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group1().xyz();
        let anti_wedge_g4 = (right_anti_dual_g4_xyz * Simd32x4::from(self[e1234]).xyz()).with_w(0.0);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + ((anti_wedge_g3 * Simd32x3::from(other[e4]))
                    + (Simd32x3::from(anti_wedge_g1[3]) * other.group3())
                    + (anti_wedge_g2.yzx() * other.group1().zxy())
                    + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (anti_wedge_g2.zxy() * other.group1().yzx())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       17        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0_xyz * Simd32x4::from(other[e4]).xyz()).with_w(right_anti_dual_g0_xyz[0] * self[e1234] * other[e1]),
            // e23, e31, e12, scalar
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar] * other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar] * other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for MultiVector {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for MultiVector {
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
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd3        7       13        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       17       31        0      N/A
    //  no simd       31       63        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_x =
            (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]) - (right_anti_dual_g0_w * self[e321]);
        let anti_wedge_g1_xyz = (right_anti_dual_g1_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy());
        let anti_wedge_g1_w = right_anti_dual_g0_w * self[e1234];
        let anti_wedge_g2 = (right_anti_dual_g1_xyz.yzx() * self.group4().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group4().yzx());
        let anti_wedge_g4 = Simd32x4::from([1.0, 1.0, self[e1234], 0.0]) * (right_anti_dual_g1_xyz * Simd32x2::from(self[e1234]).with_z(1.0)).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g4[0] * other[e1]) + (anti_wedge_g4[1] * other[e2]) + (anti_wedge_g4[2] * other[e3]) + (anti_wedge_g4[3] * other[e4])
                    - (anti_wedge_g1_w * other[e321])
                    - (anti_wedge_g1_xyz[0] * other[e423])
                    - (anti_wedge_g1_xyz[1] * other[e431])
                    - (anti_wedge_g1_xyz[2] * other[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0_x) * other.group0(),
            // e41, e42, e43
            (anti_wedge_g1_xyz * Simd32x3::from(other[e4])) - (Simd32x3::from(anti_wedge_g1_w) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g1_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g1_xyz.yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()) + (anti_wedge_g2.yzx() * other.group0().zxy())
                - (right_anti_dual_g1_xyz * Simd32x3::from(other[e4] * self[e321]))
                - (anti_wedge_g2.zxy() * other.group0().yzx()))
            .with_w(anti_wedge_g0_x * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        7        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_anti_dual_g0 * self[e1234] * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * self[e321] * other[e321] * -1.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        1        7        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        7       27        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        let anti_wedge_g1_xyz = right_anti_dual_g0 * Simd32x3::from(self[e321]);
        let anti_wedge_g2 = right_anti_dual_g0 * Simd32x3::from(self[e1234]);
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
            ((anti_wedge_g1_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g1_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       22        0        0
    //    simd3        6       11        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       19       35        0      N/A
    //  no simd       31       63        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[scalar] * other[scalar]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]);
        let anti_wedge_g1 = ((Simd32x3::from(other[scalar]) * self.group1().xyz()) - (Simd32x3::from(self[e321]) * other.group1().xyz())).with_w(other[scalar] * self[e4]);
        let anti_wedge_g2 = (Simd32x3::from(other[scalar]) * self.group2()) - (Simd32x3::from(self[e1234]) * other.group1().xyz());
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group3();
        let anti_wedge_g4 = Simd32x4::from(other[scalar]) * self.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234]) + (self[e1234] * other[scalar] * other[scalar])
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
            (Simd32x3::from([
                (anti_wedge_g1[2] * other[e42]) - (anti_wedge_g1[1] * other[e43]),
                (anti_wedge_g1[0] * other[e43]) - (anti_wedge_g1[2] * other[e41]),
                (anti_wedge_g1[1] * other[e41]) - (anti_wedge_g1[0] * other[e42]),
            ]) + (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz())
                + (Simd32x3::from(other[scalar]) * anti_wedge_g4.xyz()))
            .with_w(anti_wedge_g4[3] * other[scalar]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       32       43        0        0
    //    simd3       17       22        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd       55       73        0      N/A
    //  no simd      107      141        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let right_anti_dual_g4 = other.group1().xyz().with_w(0.0);
        let anti_wedge_g0_x = (other[scalar] * self[scalar])
            + (right_anti_dual_g4[0] * self[e1])
            + (right_anti_dual_g4[1] * self[e2])
            + (right_anti_dual_g4[2] * self[e3])
            + (right_anti_dual_g4[3] * self[e4])
            - (right_anti_dual_g2[0] * self[e23])
            - (right_anti_dual_g2[1] * self[e31])
            - (right_anti_dual_g2[2] * self[e12])
            - (right_anti_dual_g1[0] * self[e423])
            - (right_anti_dual_g1[1] * self[e431])
            - (right_anti_dual_g1[2] * self[e412])
            - (right_anti_dual_g1[3] * self[e321]);
        let anti_wedge_g1 = (right_anti_dual_g1 * Simd32x4::from(self[e1234]))
            + (Simd32x4::from(other[scalar]) * self.group1())
            + ((right_anti_dual_g2 * Simd32x3::from(self[e321])) + (Simd32x3::from(right_anti_dual_g4[3]) * self.group2()) + (self.group3().yzx() * right_anti_dual_g4.zxy())
                - (self.group3().zxy() * right_anti_dual_g4.yzx()))
            .with_w(0.0);
        let anti_wedge_g2 = Simd32x3::from([
            (right_anti_dual_g4[1] * self[e412]) - (right_anti_dual_g4[2] * self[e431]),
            (right_anti_dual_g4[2] * self[e423]) - (right_anti_dual_g4[0] * self[e412]),
            (right_anti_dual_g4[0] * self[e431]) - (right_anti_dual_g4[1] * self[e423]),
        ]) + (right_anti_dual_g2 * Simd32x3::from(self[e1234]))
            + (Simd32x3::from(other[scalar]) * self.group2());
        let anti_wedge_g3 = (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(right_anti_dual_g4[3]) * self.group4().xyz())
            - (Simd32x3::from(self[e321]) * right_anti_dual_g4.xyz());
        let anti_wedge_g4 = (right_anti_dual_g4 * Simd32x4::from(self[e1234])) + (Simd32x4::from(other[scalar]) * self.group4());
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + ((anti_wedge_g3 * Simd32x3::from(other[e4]))
                    + (Simd32x3::from(anti_wedge_g1[3]) * other.group3())
                    + (anti_wedge_g2.yzx() * other.group1().zxy())
                    + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (anti_wedge_g2.zxy() * other.group1().yzx())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       10        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, right_anti_dual_g0 * self[e1234] * other[e321] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(right_anti_dual_g0 * self[e321] * -1.0) * other.group0(),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        5       11        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       10       23        0      N/A
    //  no simd       26       57        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g1_xyz = (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy());
        let anti_wedge_g2 = (right_anti_dual_g0_xyz.yzx() * self.group4().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group4().yzx());
        let anti_wedge_g4 = Simd32x4::from([1.0, 1.0, self[e1234], 0.0]) * (right_anti_dual_g0_xyz * Simd32x2::from(self[e1234]).with_z(1.0)).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g4[0] * other[e1]) + (anti_wedge_g4[1] * other[e2]) + (anti_wedge_g4[2] * other[e3]) + (anti_wedge_g4[3] * other[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_anti_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_anti_dual_g0_xyz[2] * self[e3]) * other.group0()),
            // e41, e42, e43
            anti_wedge_g1_xyz * Simd32x3::from(other[e4]),
            // e23, e31, e12
            (anti_wedge_g1_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g1_xyz.yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            ((anti_wedge_g2.yzx() * other.group0().zxy()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * other[e4])) - (anti_wedge_g2.zxy() * other.group0().yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for MultiVector {
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
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
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
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Origin {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Origin>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4])
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       11        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[scalar] * other[scalar])
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Plane {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Plane>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       12       25        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0_xyz = (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx());
        let anti_wedge_g1_w = other[e321] * self[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1_w) * other.group0(),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz()) + (anti_wedge_g0_xyz.yzx() * other.group0().zxy())
                - (right_anti_dual_g1_xyz * Simd32x3::from(other[e4] * self[e321]))
                - (anti_wedge_g0_xyz.zxy() * other.group0().yzx()))
            .with_w(anti_wedge_g1_w * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[e321] * other[e321])
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3       10        0        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321] * -1.0) * other.group1();
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        9       27        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (Simd32x4::from(self[e321]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0);
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (anti_wedge_g0[2] * other[e42]) - (anti_wedge_g0[1] * other[e43]),
                (anti_wedge_g0[0] * other[e43]) - (anti_wedge_g0[2] * other[e41]),
                (anti_wedge_g0[1] * other[e41]) - (anti_wedge_g0[0] * other[e42]),
            ]) + (Simd32x3::from(anti_wedge_g0[3]) * other.group1().xyz())
                + (Simd32x3::from(other[scalar]) * anti_wedge_g1.xyz()))
            .with_w(anti_wedge_g1[3] * other[scalar]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       24        0        0
    //    simd3       11       17        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       31       46        0      N/A
    //  no simd       62       95        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = other[e321] * self[e321];
        let anti_wedge_g1 = (other.group3() * Simd32x4::from(self[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0);
        let anti_wedge_g2 = (right_anti_dual_g4_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g4_xyz.zxy() * self.group0().yzx());
        let anti_wedge_g3 = right_anti_dual_g4_xyz * Simd32x3::from(self[e321] * -1.0);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (anti_wedge_g3 * Simd32x3::from(other[scalar]))
                + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                + ((anti_wedge_g3 * Simd32x3::from(other[e4]))
                    + (Simd32x3::from(anti_wedge_g1[3]) * other.group3())
                    + (anti_wedge_g2.yzx() * other.group1().zxy())
                    + (other.group2().yzx() * anti_wedge_g1.zxy())
                    - (anti_wedge_g2.zxy() * other.group1().yzx())
                    - (other.group2().zxy() * anti_wedge_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0())
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       16        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let anti_wedge_g0 = (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx());
        Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.yzx() * other.group0().zxy()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e321] * other[e4])) - (anti_wedge_g0.zxy() * other.group0().yzx()))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Point {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Point>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2       11        0        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group0().xyz();
        let anti_wedge_g0 = (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge_g0) * other.group1(),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       17        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, (anti_wedge_g0[1] * other[e41]) - (anti_wedge_g0[0] * other[e42]), 0.0])
                + ((Simd32x3::from(anti_wedge_g0[3]) * other.group1().xyz())
                    + ((anti_wedge_g0.zx() * other.group0().yz()) - (anti_wedge_g0.yz() * other.group0().zx())).with_z(0.0))
                .with_w(0.0),
        )
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       16        0        0
    //    simd3        6        8        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       16       27        0      N/A
    //  no simd       31       52        0        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g4_xyz = other.group1().xyz();
        let anti_wedge_g0_x = (right_anti_dual_g4_xyz[0] * self[e1]) + (right_anti_dual_g4_xyz[1] * self[e2]) + (right_anti_dual_g4_xyz[2] * self[e3]);
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
            Simd32x3::from([
                (anti_wedge_g1[2] * other[e2]) - (anti_wedge_g1[1] * other[e3]),
                (anti_wedge_g1[0] * other[e3]) - (anti_wedge_g1[2] * other[e1]),
                (anti_wedge_g1[1] * other[e1]) - (anti_wedge_g1[0] * other[e2]),
            ]) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g0_x) * other.group4().xyz()) + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()) + (other.group2().yzx() * anti_wedge_g1.zxy())
                - (other.group2().zxy() * anti_wedge_g1.yzx()))
            .with_w(anti_wedge_g0_x * other[e321]),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       15        0        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0_xyz[0] * self[e1]) * other.group0())
                + (Simd32x4::from(right_anti_dual_g0_xyz[1] * self[e2]) * other.group0())
                + (Simd32x4::from(right_anti_dual_g0_xyz[2] * self[e3]) * other.group0()),
        )
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar] * other[scalar]) * self.group0())
    }
}
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for Scalar {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<Scalar>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0())
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<MultiVector> for Scalar {
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
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
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
impl AntiProjectOrthogonallyOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar])
    }
}
