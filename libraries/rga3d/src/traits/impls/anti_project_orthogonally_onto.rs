// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 84
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       6       0
//  Average:         6      12       0
//  Maximum:        63      84       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2      14       0
//  Average:        12      24       0
//  Maximum:       113     151       0
impl std::ops::Div<AntiProjectOrthogonallyOntoInfix> for AntiScalar {
    type Output = AntiProjectOrthogonallyOntoInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiProjectOrthogonallyOntoInfix) -> Self::Output {
        AntiProjectOrthogonallyOntoInfixPartial(self)
    }
}
impl AntiProjectOrthogonallyOnto<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[scalar] * other[scalar] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       10       16        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = self[e1234] * other[e321] * -1.0;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * Simd32x3::from(0.0).with_w(anti_wedge_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w((anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]))
                - (Simd32x3::from(0.0).with_w(anti_wedge_g0_w).wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234]) * other.group1() * Simd32x3::from(-1.0);
        return AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]));
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        6       15        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd2        0        1        0
    //    simd3        3       14        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       45       64        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from([1.0, self[e1234] * other[scalar]]) * Simd32x2::from([0.0, 1.0]);
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_wedge_g2 = Simd32x3::from(self[e1234]) * other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g4_xyz = Simd32x3::from(self[e1234]) * other.group1().xyz();
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0[0] * other[e1234])
                    + (anti_wedge_g0[1] * other[scalar])
                    + (anti_wedge_g4_xyz[0] * other[e1])
                    + (anti_wedge_g4_xyz[1] * other[e2])
                    + (anti_wedge_g4_xyz[2] * other[e3])
                    - (anti_wedge_g2[0] * other[e23])
                    - (anti_wedge_g2[1] * other[e31])
                    - (anti_wedge_g2[2] * other[e12])
                    - (anti_wedge_g1[0] * other[e423])
                    - (anti_wedge_g1[1] * other[e431])
                    - (anti_wedge_g1[2] * other[e412])
                    - (anti_wedge_g1[3] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((anti_wedge_g0[0] * other[e4]) + (anti_wedge_g1[3] * other[scalar])),
            // e41, e42, e43
            (anti_wedge_g2 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0[0]) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz())
                - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g4_xyz * Simd32x3::from(other[scalar])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0[0]) * other.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g1[3]) * other.group3()).with_w(0.0)
                + (anti_wedge_g2.yzx() * other.group1().zxy()).with_w(0.0)
                + (other.group2().yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * other.group1().yzx()).with_w(0.0)
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
        return AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0_xyz[0] * other[e1]) + (anti_wedge_g0_xyz[1] * other[e2]) + (anti_wedge_g0_xyz[2] * other[e3]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[scalar] * other[scalar] * self[e1234]);
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            anti_wedge_g0[0] * other[scalar],
            (anti_wedge_g0[0] * other[e1234]) + (anti_wedge_g0[1] * other[scalar]),
        ]));
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       10       16        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_w = self[e1234] * other[e321] * -1.0;
        let anti_wedge_g1_xyz = other.group0().xyz() * self.group0().yy().with_z(self[e1234]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * Simd32x3::from(0.0).with_w(anti_wedge_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w((anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]))
                - (Simd32x3::from(0.0).with_w(anti_wedge_g0_w).wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x3::from(self[e1234]) * other.group1() * Simd32x3::from(-1.0);
        return AntiScalar::from_groups(/* e1234 */ -(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]));
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       10       24        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = right_anti_dual_g0 * Simd32x4::from(self[e1234]);
        let anti_wedge_g1_w = right_anti_dual_g0[3] * self[scalar];
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g1_w) * other.group0())
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
            // e23, e31, e12, scalar
            (Simd32x3::from(anti_wedge_g1_w) * other.group1().xyz()).with_w(anti_wedge_g1_w * other[scalar]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       27        0
    //    simd2        0        1        0
    //    simd3        5       11        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       55       86        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_wedge_g2 = Simd32x3::from(self[e1234]) * other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g4 = (other.group1().xyz() * self.group0().yy().with_z(self[e1234])).with_w(0.0);
        return MultiVector::from_groups(
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
            (Simd32x3::from(anti_wedge_g0[0]) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx()) - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0[0]) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(0.0))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = other.group0().xyz() * self.group0().yy().with_z(self[e1234]);
        return AntiScalar::from_groups(
            // e1234
            (anti_wedge_g0_xyz[0] * other[e1]) + (anti_wedge_g0_xyz[1] * other[e2]) + (anti_wedge_g0_xyz[2] * other[e3]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::powi(Simd32x2::from(other[scalar]), 2) * self.group0());
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
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       27       43        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_wedge_g0_xyz = (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx());
        let anti_wedge_g1 = (other.group0().xyz().with_w(0.0).wwwx() * self.group1().xyz().with_w(self[e1]))
            + Simd32x3::from(0.0).with_w(
                (other[e2] * self[e2]) + (other[e3] * self[e3])
                    - (right_anti_dual_g0[1] * self[e431])
                    - (right_anti_dual_g0[2] * self[e412])
                    - (right_anti_dual_g0[3] * self[e321]),
            )
            - (self.group1().wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0]));
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g1[0] * other[e4]) + (anti_wedge_g1[3] * other[e423]),
                (anti_wedge_g1[1] * other[e4]) + (anti_wedge_g1[3] * other[e431]),
                (anti_wedge_g1[2] * other[e4]) + (anti_wedge_g1[3] * other[e412]),
                -(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
            ]) + (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g1[3] * other[e321])
                - (other.group0().yzxx() * anti_wedge_g0_xyz.zxy().with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       13       25        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0 = (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
            * right_anti_dual_g0.with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])))
            - (self.group1().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[2] * other[e42]) + (anti_wedge_g0[3] * other[e23]),
                (anti_wedge_g0[0] * other[e43]) + (anti_wedge_g0[3] * other[e31]),
                (anti_wedge_g0[1] * other[e41]) + (anti_wedge_g0[3] * other[e12]),
                -(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
            ]) - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        0        1        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       21       42        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = (Simd32x4::from(right_anti_dual_g0[3]) * self.group0())
            + (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * right_anti_dual_g0.xyz().with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])))
            - (self.group1().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        let anti_wedge_g1 = Simd32x4::from(right_anti_dual_g0[3]) * self.group1();
        return Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[3] * other[e23]) + (anti_wedge_g1[0] * other[scalar]),
                (anti_wedge_g0[3] * other[e31]) + (anti_wedge_g1[1] * other[scalar]),
                (anti_wedge_g0[3] * other[e12]) + (anti_wedge_g1[2] * other[scalar]),
                -(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
            ]) + (anti_wedge_g0.zxy() * other.group0().yzx()).with_w(anti_wedge_g1[3] * other[scalar])
                - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       42        0
    //    simd3        7       14        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       47       66        0
    //  no simd       82      124        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
            - (right_anti_dual_g1[0] * self[e423])
            - (right_anti_dual_g1[1] * self[e431])
            - (right_anti_dual_g1[2] * self[e412])
            - (right_anti_dual_g1[3] * self[e321]);
        let anti_wedge_g1 = (Simd32x4::from(other[scalar]) * self.group0())
            + (Simd32x4::from([self[e321], self[e321], self[e321], 1.0]) * right_anti_dual_g2.with_w(-(right_anti_dual_g2[1] * self[e431]) - (right_anti_dual_g2[2] * self[e412])))
            - (self.group1().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]));
        let anti_wedge_g2 = (self.group1().zxy() * other.group1().yzx()) - (self.group1().yzx() * other.group1().zxy());
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0);
        let anti_wedge_g4 = Simd32x4::from(other[scalar]) * self.group1();
        return MultiVector::from_groups(
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
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       16       35        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = (self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy());
        let anti_wedge_g1 = ((Simd32x3::from(self[e321]) * other.group0().xyz()).with_w((self[e2] * other[e2]) + (self[e3] * other[e3])) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().xyz().with_w(0.0).wwwx() * self.group1().xyz().with_w(self[e1]));
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0_xyz[1] * other[e3]) + (anti_wedge_g1[0] * other[e4]),
                (anti_wedge_g0_xyz[2] * other[e1]) + (anti_wedge_g1[1] * other[e4]),
                (anti_wedge_g0_xyz[0] * other[e2]) + (anti_wedge_g1[2] * other[e4]),
                -(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge_g0_xyz.zxy().with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
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
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321]);
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       26        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321] * -1.0) * Simd32x4::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g1[0] * other[e4]) + (anti_wedge_g1[3] * other[e423]),
                (anti_wedge_g1[1] * other[e4]) + (anti_wedge_g1[3] * other[e431]),
                (anti_wedge_g1[2] * other[e4]) + (anti_wedge_g1[3] * other[e412]),
                -(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
            ]) + Simd32x3::from(0.0).with_w(anti_wedge_g1[3] * other[e321])
                - (other.group0().yzxx() * Simd32x3::from(0.0).with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       14        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321]) * other.group1() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * other.group0().yzx()) - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()))
                .with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        0        4        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        9       20        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321]) * right_anti_dual_g0.xyz();
        return Flector::from_groups(
            // e1, e2, e3, e4
            (anti_wedge_g0_xyz * other.group1().www()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]))
                + (anti_wedge_g0_xyz.zxy() * other.group0().yzx()).with_w(right_anti_dual_g0[3] * self[e321] * other[scalar])
                - (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       30        0
    //    simd2        0        1        0
    //    simd3        5       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       31       48        0
    //  no simd       56       88        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from([self[e321] * other[e321], 1.0]) * Simd32x2::from([1.0, 0.0]);
        let anti_wedge_g1 = (Simd32x3::from(self[e321]) * other.group3() * Simd32x3::from(-1.0)).with_w(0.0);
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0);
        let anti_wedge_g4 = Simd32x3::from(0.0).with_w(self[e321] * other[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0[0] * other[scalar],
                (anti_wedge_g0[0] * other[e1234])
                    + (anti_wedge_g0[1] * other[scalar])
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
            (anti_wedge_g1 * Simd32x4::from(other[scalar])) + (Simd32x4::from(anti_wedge_g0[0]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge_g0[0]) * other.group2()) + (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()),
            // e23, e31, e12
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0[0]) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0[0]) * other.group4())
                - (other.group1().yzxx() * Simd32x3::from(0.0).with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5       16        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from([other[e4], other[e4], other[e4], 1.0]) * anti_wedge_g1.with_w(-(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3])))
                - (other.group0().yzxx() * Simd32x3::from(0.0).with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321]);
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
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group1(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       14       25        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([other[e3] * self[e31], other[e1] * self[e12], other[e2] * self[e23], -(other[e2] * self[e42]) - (other[e3] * self[e43])])
            - (self.group1().zxy() * other.group0().yzx()).with_w(0.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([other[e4], other[e4], other[e4], 1.0])
                * anti_wedge_g0
                    .xyz()
                    .with_w(-(anti_wedge_g0[1] * other[e431]) - (anti_wedge_g0[2] * other[e412]) - (anti_wedge_g0[3] * other[e321])))
                - (anti_wedge_g0.wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       12        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0 = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge_g0) * other.group1(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        1        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       18       34        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = (self.group0() * right_anti_dual_g0.www()).with_w(0.0);
        let anti_wedge_g1_xyz = Simd32x3::from(right_anti_dual_g0[3]) * self.group1();
        let anti_wedge_g1_w = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        return Motor::from_groups(
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
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        6       13        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       36       54        0
    //  no simd       63       92        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = -(right_anti_dual_g2[0] * self[e23]) - (right_anti_dual_g2[1] * self[e31]) - (right_anti_dual_g2[2] * self[e12]);
        let anti_wedge_g1 = Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], -(self[e42] * other[e2]) - (self[e43] * other[e3])])
            - (self.group1().zxy() * other.group1().yzx()).with_w(0.0);
        let anti_wedge_g2 = Simd32x3::from(other[scalar]) * self.group0();
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group1();
        return MultiVector::from_groups(
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
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        2        5        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       11       20        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], -(self[e42] * other[e2]) - (self[e43] * other[e3])])
            - (self.group1().zxy() * other.group0().yzx()).with_w(0.0);
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge_g0.xyz()) - (Simd32x3::from(anti_wedge_g0[3]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy()),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group1();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            other.group0().xx().with_zw(other[scalar], (anti_wedge_g0[3] * other[scalar]) + (anti_wedge_g1[3] * other[e1234])) * anti_wedge_g0.xyz().with_w(1.0),
            // e23, e31, e12, scalar
            anti_wedge_g1 * Simd32x4::from(other[scalar]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       24       38        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([other[e3] * self[e31], other[e1] * self[e12], other[e2] * self[e23], -(other[e2] * self[e42]) - (other[e3] * self[e43])])
            + (Simd32x4::from(self[e1234]) * Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]))
            - (other.group0().yzx() * self.group1().zxy()).with_w(0.0);
        let anti_wedge_g1_xyz = other.group0().xyz() * self.group0().www();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1_xyz[0]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3])
                        - (anti_wedge_g0[1] * other[e431])
                        - (anti_wedge_g0[2] * other[e412])
                        - (anti_wedge_g0[3] * other[e321]),
                )
                - (anti_wedge_g0.wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       18        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_xyz = right_anti_dual_g0 * self.group0().www();
        let anti_wedge_g1_w = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(anti_wedge_g1_w) * other.group0())
                .with_w(-(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
            // e23, e31, e12, scalar
            (Simd32x3::from(anti_wedge_g1_w) * other.group1()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        2        4        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       12       19        0
    //  no simd       25       39        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 =
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * self[e1234]);
        let anti_wedge_g1 = (Simd32x4::from(right_anti_dual_g0[3]) * self.group1())
            + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]));
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g1[3]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge_g0[0] * other[e23])
                        - (anti_wedge_g0[1] * other[e31])
                        - (anti_wedge_g0[2] * other[e12])
                        - (anti_wedge_g1[0] * other[e41])
                        - (anti_wedge_g1[1] * other[e42])
                        - (anti_wedge_g1[2] * other[e43]),
                ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(anti_wedge_g1[3]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * anti_wedge_g1.xyz())).with_w(anti_wedge_g1[3] * other[scalar]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       45        0
    //    simd3        7       15        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       48       67        0
    //  no simd       83      118        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = (self[scalar] * other[scalar]) - (right_anti_dual_g2[0] * self[e23]) - (right_anti_dual_g2[1] * self[e31]) - (right_anti_dual_g2[2] * self[e12]);
        let anti_wedge_g1 = Simd32x4::from([
            (right_anti_dual_g1[0] * self[e1234]) + (self[e31] * other[e3]),
            (right_anti_dual_g1[1] * self[e1234]) + (self[e12] * other[e1]),
            (right_anti_dual_g1[2] * self[e1234]) + (self[e23] * other[e2]),
            -(self[e42] * other[e2]) - (self[e43] * other[e3]),
        ]) + (self.group0() * Simd32x3::from(0.0).with_w(right_anti_dual_g1[3]))
            - (self.group1().zxy() * other.group1().yzx()).with_w(0.0);
        let anti_wedge_g2 = (right_anti_dual_g2 * Simd32x3::from(self[e1234])) + (Simd32x3::from(other[scalar]) * self.group0().xyz());
        let anti_wedge_g3 = Simd32x3::from(other[scalar]) * self.group1().xyz();
        let anti_wedge_g4 = (other.group1().xyz() * self.group0().www()).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234])
                    + (anti_wedge_g4[0] * other[e1])
                    + (anti_wedge_g4[1] * other[e2])
                    + (anti_wedge_g4[2] * other[e3])
                    + (anti_wedge_g4[3] * other[e4])
                    + (self[e1234] * f32::powi(other[scalar], 2))
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
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * other[e321] * self[e1234]);
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        6        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       13       29        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], -(self[e42] * other[e2]) - (self[e43] * other[e3])])
            - (self.group1().zxy() * other.group0().yzx()).with_w(0.0);
        let anti_wedge_g1_xyz = other.group0().xyz() * self.group0().www();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1_xyz[0]))
                + (other.group0().xyz() * anti_wedge_g0.www() * Simd32x3::from(-1.0)).with_w((anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3])),
            // e23, e31, e12, scalar
            ((anti_wedge_g0.zxy() * other.group0().yzx()) - (anti_wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       19        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(other[scalar]) * self.group0();
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([anti_wedge_g0[0] * other[scalar], (anti_wedge_g0[0] * other[e1234]) + (anti_wedge_g0[1] * other[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group2(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group4(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       27        0
    //    simd3        3       10        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       42       77        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_wedge_g0_x = (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
            - (right_anti_dual_g0[0] * self[e423])
            - (right_anti_dual_g0[1] * self[e431])
            - (right_anti_dual_g0[2] * self[e412])
            - (right_anti_dual_g0[3] * self[e321]);
        let anti_wedge_g1 = Simd32x4::from([other[e3] * self[e31], other[e1] * self[e12], other[e2] * self[e23], -(other[e2] * self[e42]) - (other[e3] * self[e43])])
            + (right_anti_dual_g0 * Simd32x4::from(self[e1234]))
            - (self.group3().zxy() * other.group0().yzx()).with_w(0.0);
        let anti_wedge_g2 = (other.group0().yzx() * self.group4().zxy()) - (other.group0().zxy() * self.group4().yzx());
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0);
        let anti_wedge_g4_xyz = other.group0().xyz() * self.group0().yy().with_z(self[e1234]);
        return MultiVector::from_groups(
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
            (anti_wedge_g1.zxy() * other.group0().yzx()) - (anti_wedge_g1.yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]),
                -(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge_g0_x) * other.group1())
                - (other.group0().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        8        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, right_anti_dual_g0 * other[e321] * self[e1234]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * other[e321] * self[e321] * -1.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        0        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       17       40        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = -(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        let anti_wedge_g1 = (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
            * right_anti_dual_g0.with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])))
            - (self.group4().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        let anti_wedge_g2 = right_anti_dual_g0 * Simd32x3::from(self[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(anti_wedge_g2[0] * other[e23]) - (anti_wedge_g2[1] * other[e31]) - (anti_wedge_g2[2] * other[e12])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0_x) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge_g0_x) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) - (anti_wedge_g1.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       25        0
    //    simd3        3        9        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       22       40        0
    //  no simd       40       76        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0_x =
            (right_anti_dual_g0[3] * self[scalar]) - (right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12]);
        let anti_wedge_g1 = (right_anti_dual_g0 * self.group4().www().with_w(self[e4]))
            + (self.group1().xyz() * right_anti_dual_g0.www()).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]))
            - (self.group4().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        let anti_wedge_g2 = (Simd32x3::from(right_anti_dual_g0[3]) * self.group2()) + (Simd32x3::from(self[e1234]) * right_anti_dual_g0.xyz());
        let anti_wedge_g3 = Simd32x3::from(right_anti_dual_g0[3]) * self.group3();
        let anti_wedge_g4 = Simd32x4::from(right_anti_dual_g0[3]) * self.group4();
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge_g0_x * other[scalar],
                (anti_wedge_g0_x * other[e1234]) + (right_anti_dual_g0[3] * other[scalar] * self[e1234])
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
            Simd32x4::from([
                (anti_wedge_g1[3] * other[e23]) + (anti_wedge_g4[0] * other[scalar]),
                (anti_wedge_g1[3] * other[e31]) + (anti_wedge_g4[1] * other[scalar]),
                (anti_wedge_g1[3] * other[e12]) + (anti_wedge_g4[2] * other[scalar]),
                -(anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g1.zxy() * other.group0().yzx()).with_w(anti_wedge_g4[3] * other[scalar])
                - (anti_wedge_g1.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       56        0
    //    simd3       10       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       63       84        0
    //  no simd      113      151        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x = (other[scalar] * self[scalar]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
            - (right_anti_dual_g2[0] * self[e23])
            - (right_anti_dual_g2[1] * self[e31])
            - (right_anti_dual_g2[2] * self[e12])
            - (right_anti_dual_g1[0] * self[e423])
            - (right_anti_dual_g1[1] * self[e431])
            - (right_anti_dual_g1[2] * self[e412])
            - (right_anti_dual_g1[3] * self[e321]);
        let anti_wedge_g1 = Simd32x4::from([
            (right_anti_dual_g2[0] * self[e321]) + (other[e3] * self[e31]),
            (right_anti_dual_g2[1] * self[e321]) + (other[e1] * self[e12]),
            (right_anti_dual_g2[2] * self[e321]) + (other[e2] * self[e23]),
            -(right_anti_dual_g2[2] * self[e412]) - (other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]),
        ]) + (right_anti_dual_g1 * Simd32x4::from(self[e1234]))
            + (Simd32x4::from(other[scalar]) * self.group1())
            - (self.group4().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
            - (self.group3().zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * self[e431]);
        let anti_wedge_g2 = (right_anti_dual_g2 * Simd32x3::from(self[e1234])) + (Simd32x3::from(other[scalar]) * self.group2()) + (other.group1().yzx() * self.group4().zxy())
            - (other.group1().zxy() * self.group4().yzx());
        let anti_wedge_g3 = (Simd32x3::from(other[scalar]) * self.group3()) - (Simd32x3::from(self[e321]) * other.group1().xyz());
        let anti_wedge_g4 = (Simd32x4::from(other[scalar]) * self.group4()) + (Simd32x4::from(self[e1234]) * other.group1().xyz().with_w(0.0));
        return MultiVector::from_groups(
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
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, right_anti_dual_g0 * self[e1234] * other[e321]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(right_anti_dual_g0 * self[e321] * -1.0) * other.group0(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        3       10        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       26       57        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], -(self[e42] * other[e2]) - (self[e43] * other[e3])])
            - (self.group3().zxy() * other.group0().yzx()).with_w(0.0);
        let anti_wedge_g2 = (self.group4().zxy() * other.group0().yzx()) - (self.group4().yzx() * other.group0().zxy());
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0);
        let anti_wedge_g4_xyz = other.group0().xyz() * self.group0().yy().with_z(self[e1234]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (anti_wedge_g4_xyz[0] * other[e1]) + (anti_wedge_g4_xyz[1] * other[e2]) + (anti_wedge_g4_xyz[2] * other[e3])]),
            // e1, e2, e3, e4
            Simd32x4::from((self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge_g1.xyz()) - (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge_g1.zxy() * other.group0().yzx()) - (anti_wedge_g1.yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]),
                -(anti_wedge_g3[1] * other[e2]) - (anti_wedge_g3[2] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::powi(Simd32x2::from(other[scalar]), 2) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group2(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group4(),
        );
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
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4]);
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[e4];
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[scalar]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group1().xyz()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       14        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[e4];
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, anti_wedge_g0 * other[e321]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[scalar]),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group3()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4]);
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
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       21       37        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_wedge_g0_xyz = (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx());
        let anti_wedge_g1 = Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412]) - (right_anti_dual_g0[3] * self[e321]))
            - (self.group0().wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0]));
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g1[3]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g1[0] * other[e4]) + (anti_wedge_g1[3] * other[e423]),
                (anti_wedge_g1[1] * other[e4]) + (anti_wedge_g1[3] * other[e431]),
                (anti_wedge_g1[2] * other[e4]) + (anti_wedge_g1[3] * other[e412]),
                -(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
            ]) + (anti_wedge_g0_xyz.yzx() * other.group0().zxy()).with_w(anti_wedge_g1[3] * other[e321])
                - (other.group0().yzxx() * anti_wedge_g0_xyz.zxy().with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321]);
    }
}
impl AntiProjectOrthogonallyOnto<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       13       25        0
    fn anti_project_orthogonally_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let anti_wedge_g0 = (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
            * right_anti_dual_g0.with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])))
            - (self.group0().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[2] * other[e42]) + (anti_wedge_g0[3] * other[e23]),
                (anti_wedge_g0[0] * other[e43]) + (anti_wedge_g0[3] * other[e31]),
                (anti_wedge_g0[1] * other[e41]) + (anti_wedge_g0[3] * other[e12]),
                -(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
            ]) - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        0        1        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        8       18        0
    //  no simd       17       38        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_wedge_g0 = (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
            * right_anti_dual_g0.xyz().with_w(-(right_anti_dual_g0[1] * self[e431]) - (right_anti_dual_g0[2] * self[e412])))
            - (self.group0().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0]));
        let anti_wedge_g1 = Simd32x4::from(right_anti_dual_g0[3]) * self.group0();
        return Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[3] * other[e23]) + (anti_wedge_g1[0] * other[scalar]),
                (anti_wedge_g0[3] * other[e31]) + (anti_wedge_g1[1] * other[scalar]),
                (anti_wedge_g0[3] * other[e12]) + (anti_wedge_g1[2] * other[scalar]),
                -(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
            ]) + (anti_wedge_g0.zxy() * other.group0().yzx()).with_w(anti_wedge_g1[3] * other[scalar])
                - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        7       14        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       75      117        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let anti_wedge_g0_x =
            -(right_anti_dual_g1[0] * self[e423]) - (right_anti_dual_g1[1] * self[e431]) - (right_anti_dual_g1[2] * self[e412]) - (right_anti_dual_g1[3] * self[e321]);
        let anti_wedge_g1 = (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
            * right_anti_dual_g2.with_w(-(right_anti_dual_g2[1] * self[e431]) - (right_anti_dual_g2[2] * self[e412])))
            - (self.group0().yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]));
        let anti_wedge_g2 = (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx());
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0);
        let anti_wedge_g4 = Simd32x4::from(other[scalar]) * self.group0();
        return MultiVector::from_groups(
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
            (anti_wedge_g3 * Simd32x3::from(other[scalar])) + (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx())
                - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g2[1] * other[e3]) + (anti_wedge_g3[0] * other[e4]) + (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g2[2] * other[e1]) + (anti_wedge_g3[1] * other[e4]) + (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g2[0] * other[e2]) + (anti_wedge_g3[2] * other[e4]) + (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g3[2] * other[e3]) - (anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (anti_wedge_g4 * Simd32x4::from(other[scalar]))
                + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group1().yzxx() * anti_wedge_g2.zxy().with_w(anti_wedge_g3[0]))
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(anti_wedge_g3[1] * other[e2]),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_orthogonally_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       11       24        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy());
        let anti_wedge_g1 = Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[1] * other[e3]) + (anti_wedge_g1[0] * other[e4]),
                (anti_wedge_g0[2] * other[e1]) + (anti_wedge_g1[1] * other[e4]),
                (anti_wedge_g0[0] * other[e2]) + (anti_wedge_g1[2] * other[e4]),
                -(anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge_g0.zxy().with_w(anti_wedge_g1[0])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0());
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
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn anti_project_orthogonally_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge_g0) * other.group1(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        return Flector::from_groups(
            // e1, e2, e3, e4
            anti_wedge_g0 * Simd32x4::from(other[scalar]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g0[2] * other[e42]) + (anti_wedge_g0[3] * other[e23]),
                (anti_wedge_g0[0] * other[e43]) + (anti_wedge_g0[3] * other[e31]),
                (anti_wedge_g0[1] * other[e41]) + (anti_wedge_g0[3] * other[e12]),
                -(anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
            ]) - (anti_wedge_g0.yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        4        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       35       55        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]);
        let anti_wedge_g1 = Simd32x4::from(other[scalar]) * self.group0();
        return MultiVector::from_groups(
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
            (Simd32x3::from(anti_wedge_g0_x) * other.group3()) + (anti_wedge_g1.zxy() * other.group1().yzx()) - (anti_wedge_g1.yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge_g1[2] * other[e42]) + (anti_wedge_g1[3] * other[e23]),
                (anti_wedge_g1[0] * other[e43]) + (anti_wedge_g1[3] * other[e31]),
                (anti_wedge_g1[1] * other[e41]) + (anti_wedge_g1[3] * other[e12]),
                -(anti_wedge_g1[0] * other[e23]) - (anti_wedge_g1[1] * other[e31]) - (anti_wedge_g1[2] * other[e12]),
            ]) + (Simd32x4::from(anti_wedge_g0_x) * other.group4())
                - (other.group2().zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_project_orthogonally_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])) * other.group0(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0());
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_project_orthogonally_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl AntiProjectOrthogonallyOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_project_orthogonally_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[scalar];
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_wedge_g0) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_wedge_g0) * other.group1(),
        );
    }
}
impl AntiProjectOrthogonallyOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn anti_project_orthogonally_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[scalar] * self[scalar];
        return MultiVector::from_groups(
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
        );
    }
}
impl AntiProjectOrthogonallyOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_orthogonally_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
    }
}
