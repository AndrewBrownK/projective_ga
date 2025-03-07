// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 74
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       8       0
//  Average:         6      11       0
//  Maximum:        55      70       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      14       0
//  Average:        11      21       0
//  Maximum:       101     126       0
impl std::ops::Div<RejectOrthogonallyFromInfix> for AntiScalar {
    type Output = RejectOrthogonallyFromInfixPartial<AntiScalar>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl RejectOrthogonallyFrom<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       25        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.wwwx() * right_anti_dual_g0.xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3])
                        - (anti_wedge_g1[1] * right_anti_dual_g0[1])
                        - (anti_wedge_g1[2] * right_anti_dual_g0[2])
                        - (anti_wedge_g1[3] * right_anti_dual_g0[3]),
                )
                - (right_anti_dual_g0.wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1[0])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for AntiScalar {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectOrthogonallyFrom<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        6       15        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (right_anti_dual_g0 * Simd32x4::from(anti_wedge_g1[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       12        0
    //    simd2        0        1        0
    //    simd3        2       12        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       36       62        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from(self[e1234]) * other.group0();
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let anti_wedge_g2 = Simd32x3::from(self[e1234]) * other.group2();
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group3();
        let anti_wedge_g4 = Simd32x4::from(self[e1234]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0[0] * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0[0] * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0[0])) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0[0]) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for AntiScalar {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectOrthogonallyFrom<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        return AntiScalar::from_groups(/* e1234 */ (anti_wedge_g0[0] * other[e1]) + (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3]));
    }
}
impl RejectOrthogonallyFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for DualNum {
    type Output = RejectOrthogonallyFromInfixPartial<DualNum>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<DualNum> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectOrthogonallyFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       25        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.wwwx() * right_anti_dual_g0.xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3])
                        - (anti_wedge_g1[1] * right_anti_dual_g0[1])
                        - (anti_wedge_g1[2] * right_anti_dual_g0[2])
                        - (anti_wedge_g1[3] * right_anti_dual_g0[3]),
                )
                - (right_anti_dual_g0.wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1[0])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for DualNum {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectOrthogonallyFrom<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x3::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        7       17        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])) * other.group1().xyz().with_w(1.0);
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (right_anti_dual_g0 * Simd32x4::from(anti_wedge_g1[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       14        0
    //    simd3        2       12        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       37       62        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let anti_wedge_g2 = Simd32x3::from(self[e1234]) * other.group2();
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group3();
        let anti_wedge_g4 = Simd32x4::from(self[e1234]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for DualNum {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectOrthogonallyFrom<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        return AntiScalar::from_groups(/* e1234 */ (anti_wedge_g0[0] * other[e1]) + (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3]));
    }
}
impl RejectOrthogonallyFrom<Scalar> for DualNum {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Flector {
    type Output = RejectOrthogonallyFromInfixPartial<Flector>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        6        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       28       37        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = (other.group1().yzx() * self.group1().zxy()) - (other.group1().zxy() * self.group1().yzx());
        let anti_wedge_g1 = (other.group1().wwwx() * self.group1().xyz().with_w(self[e1]))
            + Simd32x3::from(0.0).with_w(
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            )
            - (self.group1().wwwx() * other.group1().xyz().with_w(other[e1]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g1[3] * right_anti_dual_g0[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[3]) * anti_wedge_g1.xyz()).with_w(0.0)
                + (anti_wedge_g0_xyz.yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (anti_wedge_g0_xyz.zxy() * right_anti_dual_g0.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]);
        let right_anti_dual_g0 = other[e321] * -1.0;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_anti_dual_g0) * anti_wedge_g1.xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       11       21        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (self[e412] * other[e31]) + (self[e321] * other[e41]),
            (self[e423] * other[e12]) + (self[e321] * other[e42]),
            (self[e431] * other[e23]) + (self[e321] * other[e43]),
            -(self[e431] * other[e42]) - (self[e412] * other[e43]),
        ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((right_anti_dual_g0.yzx() * anti_wedge_g0.zxy()) - (right_anti_dual_g0.zxy() * anti_wedge_g0.yzx())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       15       26        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (self[e412] * other[e31]) + (self[e321] * other[e41]),
            (self[e423] * other[e12]) + (self[e321] * other[e42]),
            (self[e431] * other[e23]) + (self[e321] * other[e43]),
            -(self[e431] * other[e42]) - (self[e412] * other[e43]),
        ]) + (Simd32x4::from(other[e1234]) * self.group0())
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       28        0
    //    simd3        4       14        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       32       46        0
    //  no simd       61       86        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
            - (self[e423] * other[e1])
            - (self[e431] * other[e2])
            - (self[e412] * other[e3])
            - (self[e321] * other[e4]);
        let anti_wedge_g1 = Simd32x4::from([
            (self[e412] * other[e31]) + (self[e321] * other[e41]),
            (self[e423] * other[e12]) + (self[e321] * other[e42]),
            (self[e431] * other[e23]) + (self[e321] * other[e43]),
            -(self[e431] * other[e42]) - (self[e412] * other[e43]),
        ]) + (Simd32x4::from(other[e1234]) * self.group0())
            - (self.group1().yzxx() * other.group3().zxy().with_w(other[e41]));
        let anti_wedge_g2 = (self.group1().zxy() * other.group4().yzx()) - (self.group1().yzx() * other.group4().zxy());
        let anti_wedge_g3 = (Simd32x3::from(other[e321]) * self.group1().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let anti_wedge_g4 = Simd32x4::from(other[e1234]) * self.group1();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        6       19        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = (Simd32x4::from([
            self[e321] * other[e423],
            self[e321] * other[e431],
            self[e321] * other[e412],
            (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().wwwx() * self.group1().xyz().with_w(self[e1]));
        let right_anti_dual_g0 = other[e321] * -1.0;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * anti_wedge_g1[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_anti_dual_g0) * anti_wedge_g1.xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Point> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Horizon {
    type Output = RejectOrthogonallyFromInfixPartial<Horizon>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0);
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g1[3] * right_anti_dual_g0[3]),
            // e423, e431, e412, e321
            ((Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()) + (Simd32x3::from(right_anti_dual_g0[3]) * anti_wedge_g1.xyz())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321]) * other.group0();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g0_xyz.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       13        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = Simd32x3::from(self[e321]) * other.group0().xyz();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0_xyz.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g0_xyz.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd2        0        1        0
    //    simd3        1       10        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       22       47        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]);
        let anti_wedge_g1_xyz = Simd32x3::from(self[e321]) * other.group2();
        let anti_wedge_g3 = Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0[0] * other[scalar]) + (anti_wedge_g1_xyz[0] * other[e1]) + (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (right_anti_dual_g1[3] * self[e321] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0[0] * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0[0])) - (anti_wedge_g1_xyz * Simd32x3::from(right_anti_dual_g1[3])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0[0]) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g1_xyz.zxy() * right_anti_dual_g2.yzx()).with_w(0.0)
                - (anti_wedge_g1_xyz.yzx() * right_anti_dual_g2.zxy()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       10        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(other[e321] * -1.0) * Simd32x3::from([self[e321] * other[e423] * -1.0, self[e321] * other[e431] * -1.0, self[e321] * other[e412] * -1.0])).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e321] * other[e4] * -1.0) * other.group0().xyz()).with_w(0.0));
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Line {
    type Output = RejectOrthogonallyFromInfixPartial<Line>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       13       28        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e423] * self[e12]) + (other[e321] * self[e42]),
            (other[e431] * self[e23]) + (other[e321] * self[e43]),
            -(other[e431] * self[e42]) - (other[e412] * self[e43]),
        ]) - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.wwwx() * right_anti_dual_g0.xyz().with_w(other[e1]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * anti_wedge_g0.xyz() * Simd32x3::from(-1.0)).with_w((anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * -1.0) * Simd32x3::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ) * Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       12        0
    //  no simd       11       20        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_xyz = Simd32x3::from(other[e1234]) * self.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (right_anti_dual_g0
                * Simd32x4::from(
                    -(self[e41] * other[e23])
                        - (self[e42] * other[e31])
                        - (self[e43] * other[e12])
                        - (self[e23] * other[e41])
                        - (self[e31] * other[e42])
                        - (self[e12] * other[e43]),
                ))
                + Simd32x3::from(0.0)
                    .with_w(-(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       22        0
    //    simd3        2       12        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       45       66        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let anti_wedge_g1 = Simd32x4::from([
            (self[e41] * other[e321]) + (self[e31] * other[e412]),
            (self[e42] * other[e321]) + (self[e12] * other[e423]),
            (self[e43] * other[e321]) + (self[e23] * other[e431]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]));
        let anti_wedge_g2 = Simd32x3::from(other[e1234]) * self.group0();
        let anti_wedge_g3 = Simd32x3::from(other[e1234]) * self.group1();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * -1.0)
                * Simd32x3::from([
                    (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                    (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                    (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                ])
                * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Motor {
    type Output = RejectOrthogonallyFromInfixPartial<Motor>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<DualNum> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       12       19        0
    //  no simd       24       37        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e423] * self[e12]) + (other[e321] * self[e42]),
            (other[e431] * self[e23]) + (other[e321] * self[e43]),
            -(other[e431] * self[e42]) - (other[e412] * self[e43]),
        ]) + (Simd32x4::from(self[e1234]) * other.group0())
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]));
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge_g0.wwwx() * right_anti_dual_g0.xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3])
                        - (anti_wedge_g1[1] * right_anti_dual_g0[1])
                        - (anti_wedge_g1[2] * right_anti_dual_g0[2])
                        - (anti_wedge_g1[3] * right_anti_dual_g0[3]),
                )
                - (right_anti_dual_g0.wwwx() * anti_wedge_g0.xyz().with_w(anti_wedge_g1[0])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0) * (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(other[e321] * self[e1234]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       18        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (right_anti_dual_g0
                * Simd32x3::from(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ))
            .with_w(-(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       13        0
    //  no simd       19       25        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from(self[e1234]) * other.group1())
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            );
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (right_anti_dual_g0 * Simd32x4::from(anti_wedge_g1[3]))
                + Simd32x3::from(0.0).with_w(-(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       28        0
    //    simd3        4       14        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       32       46        0
    //  no simd       61       86        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (self[e1234] * other[scalar]) + (self[scalar] * other[e1234])
            - (self[e41] * other[e23])
            - (self[e42] * other[e31])
            - (self[e43] * other[e12])
            - (self[e23] * other[e41])
            - (self[e31] * other[e42])
            - (self[e12] * other[e43]);
        let anti_wedge_g1 = Simd32x4::from([
            (self[e1234] * other[e1]) + (self[e31] * other[e412]),
            (self[e1234] * other[e2]) + (self[e12] * other[e423]),
            (self[e1234] * other[e3]) + (self[e23] * other[e431]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) + (self.group0() * other.group4().www().with_w(other[e4]))
            - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]));
        let anti_wedge_g2 = (Simd32x3::from(self[e1234]) * other.group2()) + (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let anti_wedge_g3 = (Simd32x3::from(self[e1234]) * other.group3()) + (Simd32x3::from(other[e1234]) * self.group1().xyz());
        let anti_wedge_g4 = Simd32x4::from(self[e1234]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       22        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0)
                * (Simd32x4::from([
                    (self[e41] * other[e321]) + (self[e31] * other[e412]),
                    (self[e42] * other[e321]) + (self[e12] * other[e423]),
                    (self[e43] * other[e321]) + (self[e23] * other[e431]),
                    -(self[e42] * other[e431]) - (self[e43] * other[e412]),
                ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])))
                .xyz()
                .with_w(self[e1234] * other[e321])
                * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Point> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(self[e1234]) * other.group0();
        return AntiScalar::from_groups(/* e1234 */ (anti_wedge_g0[0] * other[e1]) + (anti_wedge_g0[1] * other[e2]) + (anti_wedge_g0[2] * other[e3]));
    }
}
impl RejectOrthogonallyFrom<Scalar> for Motor {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for MultiVector {
    type Output = RejectOrthogonallyFromInfixPartial<MultiVector>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<DualNum> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other[scalar] * other[scalar] * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        3       10        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       46       70        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
            - (other[e1] * self[e423])
            - (other[e2] * self[e431])
            - (other[e3] * self[e412])
            - (other[e4] * self[e321]);
        let anti_wedge_g1 = Simd32x4::from([
            (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e423] * self[e12]) + (other[e321] * self[e42]),
            (other[e431] * self[e23]) + (other[e321] * self[e43]),
            -(other[e431] * self[e42]) - (other[e412] * self[e43]),
        ]) + (Simd32x4::from(self[e1234]) * other.group0())
            - (other.group1().yzxx() * self.group3().zxy().with_w(self[e41]));
        let anti_wedge_g2 = (other.group1().yzx() * self.group4().zxy()) - (other.group1().zxy() * self.group4().yzx());
        let anti_wedge_g4 = Simd32x4::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g4[0] * right_anti_dual_g0[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g0[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g0[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g0[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g0[3]),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g0.xyz()) - (Simd32x3::from(right_anti_dual_g0[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0_x) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[3]) * ((Simd32x3::from(other[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * other.group1().xyz()))).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g0.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       22        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, right_anti_dual_g0 * other[e321] * self[e1234]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * other[e321] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0) * Simd32x3::from(other[e321]) * self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_anti_dual_g0) * Simd32x3::from(other[e321]) * self.group4().xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       18       36        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]));
        let anti_wedge_g3 = Simd32x3::from(self[e1234]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            right_anti_dual_g0
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
            ((right_anti_dual_g0.yzx() * anti_wedge_g1.zxy()) - (right_anti_dual_g0.zxy() * anti_wedge_g1.yzx())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       20        0
    //    simd3        2        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       28       47        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e1234] * self[scalar]) + (other[scalar] * self[e1234])
            - (other[e41] * self[e23])
            - (other[e42] * self[e31])
            - (other[e43] * self[e12])
            - (other[e23] * self[e41])
            - (other[e31] * self[e42])
            - (other[e12] * self[e43]);
        let anti_wedge_g1 = Simd32x4::from([
            (other[e1234] * self[e1]) + (other[e31] * self[e412]),
            (other[e1234] * self[e2]) + (other[e12] * self[e423]),
            (other[e1234] * self[e3]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) + (other.group0() * self.group4().www().with_w(self[e4]))
            - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]));
        let anti_wedge_g3 = (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(self[e1234]) * other.group1().xyz());
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * right_anti_dual_g0[3])
                    - (anti_wedge_g3[0] * right_anti_dual_g0[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g0[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g0[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0_x) * right_anti_dual_g0.xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((anti_wedge_g1.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g1.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       45        0
    //    simd3        8       19        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       55       70        0
    //  no simd      101      126        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
        let anti_wedge_g1 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]) + (other[e423] * self[e12]) + (other[e321] * self[e42]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e431] * self[e23]) + (other[e321] * self[e43]),
            -(other[e43] * self[e412]) - (other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
        ]) + (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from(self[e1234]) * other.group1())
            - (self.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
            - (self.group3().zxy() * other.group4().yzx()).with_w(other[e42] * self[e431]);
        let anti_wedge_g2 = (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * other.group2()) + (other.group4().yzx() * self.group4().zxy())
            - (other.group4().zxy() * self.group4().yzx());
        let anti_wedge_g3 = (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(other[e321]) * self.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group3())
            - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let anti_wedge_g4 = (Simd32x4::from(other[e1234]) * self.group4()) + (Simd32x4::from(self[e1234]) * other.group4());
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       19        0
    //    simd2        0        1        0
    //    simd3        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       14       40        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, right_anti_dual_g0 * self[e1234] * other[e321]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (right_anti_dual_g0 * self[e1] * other[e423])
                    + (right_anti_dual_g0 * self[e2] * other[e431])
                    + (right_anti_dual_g0 * self[e3] * other[e412])
                    + (right_anti_dual_g0 * self[e4] * other[e321]),
            ),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0)
                * (Simd32x4::from([
                    (self[e41] * other[e321]) + (self[e31] * other[e412]),
                    (self[e42] * other[e321]) + (self[e12] * other[e423]),
                    (self[e43] * other[e321]) + (self[e23] * other[e431]),
                    -(self[e42] * other[e431]) - (self[e43] * other[e412]),
                ]) - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])))
                .xyz()
                * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_anti_dual_g0) * ((Simd32x3::from(other[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz()))).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       14        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group0();
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = AntiScalar;
    fn reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Origin {
    type Output = RejectOrthogonallyFromInfixPartial<Origin>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[e321] * self[e4];
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0);
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        3       21        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, anti_wedge_g0[0] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0[0] * right_anti_dual_g1[3]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234] * self[e4]) * right_anti_dual_g1.xyz()) - (Simd32x3::from(anti_wedge_g0[0]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0[0]) * other.group1().xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * f32::powi(other[e321], 2) * -1.0);
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Plane {
    type Output = RejectOrthogonallyFromInfixPartial<Plane>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        7        0
    //    simd4        4        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       21       33        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_xyz = (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx());
        let anti_wedge_g1 = (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]))
            - (self.group0().wwwx() * other.group1().xyz().with_w(other[e1]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g1[3] * right_anti_dual_g0[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g1[3]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(right_anti_dual_g0[3]) * anti_wedge_g1.xyz()).with_w(0.0)
                + (anti_wedge_g0_xyz.yzx() * right_anti_dual_g0.zxy()).with_w(0.0)
                - (anti_wedge_g0_xyz.zxy() * right_anti_dual_g0.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(other[e321] * -1.0) * Simd32x3::from([other[e321] * self[e423], other[e321] * self[e431], other[e321] * self[e412]])).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       11       21        0
    fn reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((right_anti_dual_g0.yzx() * anti_wedge_g0.zxy()) - (right_anti_dual_g0.zxy() * anti_wedge_g0.yzx())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       11       22        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        4       14        0
    //    simd4        6        3        0
    // Totals...
    // yes simd       27       41        0
    //  no simd       53       78        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let anti_wedge_g1 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let anti_wedge_g2 = (other.group4().yzx() * self.group0().zxy()) - (other.group4().zxy() * self.group0().yzx());
        let anti_wedge_g3 = (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz());
        let anti_wedge_g4 = Simd32x4::from(other[e1234]) * self.group0();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3])
                    - (anti_wedge_g3[0] * right_anti_dual_g2[0])
                    - (anti_wedge_g3[1] * right_anti_dual_g2[1])
                    - (anti_wedge_g3[2] * right_anti_dual_g2[2])
                    - (anti_wedge_g4[0] * right_anti_dual_g1[0])
                    - (anti_wedge_g4[1] * right_anti_dual_g1[1])
                    - (anti_wedge_g4[2] * right_anti_dual_g1[2])
                    - (anti_wedge_g4[3] * right_anti_dual_g1[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (anti_wedge_g3 * Simd32x3::from(right_anti_dual_g1[3])).with_w(0.0)
                + (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0)
                + (anti_wedge_g2.yzx() * right_anti_dual_g1.zxy()).with_w(0.0)
                + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (anti_wedge_g2.zxy() * right_anti_dual_g1.yzx()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(other[e321] * -1.0)
                * Simd32x3::from([
                    (other[e321] * self[e423]) - (other[e423] * self[e321]),
                    (other[e321] * self[e431]) - (other[e431] * self[e321]),
                    (other[e321] * self[e412]) - (other[e412] * self[e321]),
                ]))
            .with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Point {
    type Output = RejectOrthogonallyFromInfixPartial<Point>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3        9        0
    fn reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0);
    }
}
impl RejectOrthogonallyFrom<Motor> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       14        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            ((anti_wedge_g0.zxy() * right_anti_dual_g0.yzx()) - (anti_wedge_g0.yzx() * right_anti_dual_g0.zxy())).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        2        7        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       20       38        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0_x = (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]);
        let anti_wedge_g1 = Simd32x4::from(other[e1234]) * self.group0();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge_g0_x * other[scalar]) + (anti_wedge_g1[0] * other[e1]) + (anti_wedge_g1[1] * other[e2]) + (anti_wedge_g1[2] * other[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0_x * right_anti_dual_g1[3]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(anti_wedge_g0_x)) + (Simd32x3::from(anti_wedge_g1[3]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * anti_wedge_g1.xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0_x) * other.group1().xyz()).with_w(0.0) + (right_anti_dual_g2.yzx() * anti_wedge_g1.zxy()).with_w(0.0)
                - (right_anti_dual_g2.zxy() * anti_wedge_g1.yzx()).with_w(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        );
    }
}
impl std::ops::Div<RejectOrthogonallyFromInfix> for Scalar {
    type Output = RejectOrthogonallyFromInfixPartial<Scalar>;
    fn div(self, _rhs: RejectOrthogonallyFromInfix) -> Self::Output {
        RejectOrthogonallyFromInfixPartial(self)
    }
}
impl RejectOrthogonallyFrom<DualNum> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[scalar] * other[e1234] * self[scalar]);
    }
}
impl RejectOrthogonallyFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234] * self[scalar]) * Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, other[scalar]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectOrthogonallyFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       15        0
    fn reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge_g0 = other[e1234] * self[scalar];
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, anti_wedge_g0 * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge_g0 * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge_g0) * other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge_g0) * other.group1().xyz()).with_w(0.0),
        );
    }
}
