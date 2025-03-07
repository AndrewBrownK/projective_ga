// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 86
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       6       0
//  Average:         6      12       0
//  Maximum:        63      84       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       9       0
//  Average:        11      23       0
//  Maximum:       113     151       0
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for AntiScalar {
    type Output = AntiRejectOrthogonallyFromInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234] * other[scalar]) * Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, other[scalar]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       15        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e1234] * other[scalar];
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, wedge_g0 * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group1().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       15       31        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz().with_w(0.0).wwwx() * wedge_g1.xyz().with_w(wedge_g0[0]))
                + Simd32x3::from(0.0).with_w(
                    (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3])
                        - (right_anti_dual_g0[1] * wedge_g1[1])
                        - (right_anti_dual_g0[2] * wedge_g1[2])
                        - (right_anti_dual_g0[3] * wedge_g1[3]),
                )
                - (wedge_g1.wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for DualNum {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * f32::powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       10       28        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])) * other.group0().xyz().with_w(1.0);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * wedge_g0.xyz()) + (Simd32x3::from(wedge_g0[3]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * wedge_g0[3]),
            // e23, e31, e12, scalar
            (wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd3        4       10        0
    //    simd4        5        8        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       49       86        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]);
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (self[scalar] * f32::powi(other[scalar], 2))
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for DualNum {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * f32::powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        return Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * Simd32x2::from([self[scalar] * other[scalar], self[e1234] * other[scalar]]),
        );
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
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        1        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       17        0
    //  no simd       25       39        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]))
            + Simd32x3::from(0.0).with_w(
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            )
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let wedge_g1_xyz = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0[3]) * Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]))
                + (wedge_g1_xyz.yzx() * other.group0().zxy()).with_w(-(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]))
                - (wedge_g1_xyz.zxy() * other.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (other.group0().xyz() * wedge_g0.www()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * f32::powi(other[e321], 2) * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       13       24        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (self[e3] * other[e42]) + (self[e4] * other[e23]),
            (self[e1] * other[e43]) + (self[e4] * other[e31]),
            (self[e2] * other[e41]) + (self[e4] * other[e12]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x3::from(wedge_g0[3])).with_w(-(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
                - (wedge_g0.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        0        2        0
    //    simd4        4        6        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       21       41        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([
            (self[e4] * other[e23]) + (self[e423] * other[scalar]),
            (self[e4] * other[e31]) + (self[e431] * other[scalar]),
            (self[e4] * other[e12]) + (self[e412] * other[scalar]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) + (self.group0().zxy() * other.group0().yzx()).with_w(self[e321] * other[scalar])
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0[3]) * Simd32x4::from(other[scalar]) * self.group0())
                + (Simd32x3::from(wedge_g1[3]) * right_anti_dual_g0.xyz()).with_w(-(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))
                - (wedge_g1.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd3        6       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       39       59        0
    //  no simd       72      110        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
            (self[e3] * other[e42]) + (self[e4] * other[e23]),
            (self[e1] * other[e43]) + (self[e4] * other[e31]),
            (self[e2] * other[e41]) + (self[e4] * other[e12]),
            -(self[e2] * other[e31]) - (self[e3] * other[e12]),
        ]) + (Simd32x4::from(other[scalar]) * self.group1())
            - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (self[e1] * other[e423] * other[e321]) - (self[e2] * other[e431] * other[e321]) - (self[e3] * other[e412] * other[e321]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        2        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       13       27        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]))
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let wedge_g1_xyz = (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx());
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(-(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3])),
            // e423, e431, e412, e321
            (other.group0().xyz() * wedge_g0.www()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
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
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[scalar] * other[scalar] * self[e321]);
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e4] * self[e321] * -1.0;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e321] * other[scalar];
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(wedge_g0) * right_anti_dual_g0.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(wedge_g0 * right_anti_dual_g0[3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd2        0        1        0
    //    simd3        2        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       26       54        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]);
        let wedge_g4 = Simd32x3::from(0.0).with_w(self[e321] * other[scalar]);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[scalar])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0[1] * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0[1])) + (right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])).with_w(right_anti_dual_g2[2] * wedge_g4[2] * -1.0)
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - Simd32x3::from(0.0).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0[1])) + (wedge_g4.zxy() * other.group1().yzx()) - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0[1]) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e321] * other[e4] * -1.0) * other.group0().xyz()).with_w(0.0));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Horizon {
    type Output = Horizon;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e321] * f32::powi(other[scalar], 2));
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
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group1(),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       17       29        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e4] * self[e23]),
            (other[e1] * self[e43]) + (other[e4] * self[e31]),
            (other[e2] * self[e41]) + (other[e4] * self[e12]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]) - (right_anti_dual_g0[3] * wedge_g0[3]))
                - (wedge_g0.wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
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
impl AntiRejectOrthogonallyFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        1        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       14       30        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_w =
            -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]);
        let wedge_g1 = (self.group1() * other.group1().www()).with_w(0.0);
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0_w) * right_anti_dual_g0.xyz()) + (Simd32x3::from(right_anti_dual_g0[3]) * Simd32x3::from(other[scalar]) * self.group0()))
                .with_w(wedge_g0_w * right_anti_dual_g0[3]),
            // e23, e31, e12, scalar
            (wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       33        0
    //    simd3        4       10        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       53       87        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_anti_dual_g2[0] * wedge_g3[0])
                    - (right_anti_dual_g2[1] * wedge_g3[1])
                    - (right_anti_dual_g2[2] * wedge_g3[2])
                    - (right_anti_dual_g1[0] * wedge_g4[0])
                    - (right_anti_dual_g1[1] * wedge_g4[1])
                    - (right_anti_dual_g1[2] * wedge_g4[2])
                    - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        1        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd       11       24        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        return Line::from_groups(
            // e41, e42, e43
            (wedge_g0.zxy() * other.group0().yzx()) - (wedge_g0.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g0[3]) * other.group0().xyz() * Simd32x3::from(-1.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(other[scalar]), 2) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1       14        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar])
                * other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
                * self.group0().xyz().with_w(1.0),
            // e23, e31, e12, scalar
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd3        1        3        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       27       43        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from([
            (other[e4] * self[e23]) + (other[e423] * self[scalar]),
            (other[e4] * self[e31]) + (other[e431] * self[scalar]),
            (other[e4] * self[e12]) + (other[e412] * self[scalar]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) + (other.group0().zxy() * self.group0().yzx()).with_w(other[e321] * self[scalar])
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz().with_w(0.0).wwwx() * wedge_g1.xyz().with_w(wedge_g0[0]))
                + Simd32x3::from(0.0).with_w(
                    (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3])
                        - (right_anti_dual_g0[1] * wedge_g1[1])
                        - (right_anti_dual_g0[2] * wedge_g1[2])
                        - (right_anti_dual_g0[3] * wedge_g1[3]),
                )
                - (wedge_g1.wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       18        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = other.group1() * self.group1().www();
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
            .with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        2        4        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       12       19        0
    //  no simd       25       39        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x4::from(other[scalar]) * self.group0())
            + (Simd32x4::from(self[scalar]) * other.group0())
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            );
        let wedge_g1 = ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[scalar] * self[scalar]);
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * wedge_g0.xyz()) + (Simd32x3::from(wedge_g0[3]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * wedge_g0[3]),
            // e23, e31, e12, scalar
            (wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       39        0
    //    simd3        6       13        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       40       60        0
    //  no simd       73      110        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (self[scalar] * f32::powi(other[scalar], 2))
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Motor {
    type Output = Scalar;
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * f32::powi(other[e321], 2));
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       16       34        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz().with_w(0.0).wwwx() * wedge_g1.xyz().with_w(wedge_g0[0]))
                + (other.group0().xyz() * wedge_g1.www() * Simd32x3::from(-1.0)).with_w((wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::powi(Simd32x4::from(other[scalar]), 2) * self.group1(),
        );
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       19        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
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
impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        3       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       43       78        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
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
            (other[e3] * self[e42]) + (other[e4] * self[e23]),
            (other[e1] * self[e43]) + (other[e4] * self[e31]),
            (other[e2] * self[e41]) + (other[e4] * self[e12]),
            -(other[e2] * self[e31]) - (other[e3] * self[e12]),
        ]) + (Simd32x4::from(self[scalar]) * other.group1())
            - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23]));
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
                    - (right_anti_dual_g0[0] * wedge_g4[0])
                    - (right_anti_dual_g0[1] * wedge_g4[1])
                    - (right_anti_dual_g0[2] * wedge_g4[2])
                    - (right_anti_dual_g0[3] * wedge_g4[3]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x4::from(wedge_g0_y)) + (wedge_g3.yzx() * other.group0().zxy()).with_w(-(wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]))
                - (wedge_g3.zxy() * other.group0().yzx()).with_w(0.0),
            // e41, e42, e43
            (wedge_g4.zxy() * other.group0().yzx()) - (wedge_g4.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3]) * other.group0().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0_y) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        7        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g0 * other[e321] * self[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual_g0 * other[e321] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       19        0
    //    simd3        0        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       20       39        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group1();
        let wedge_g4 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x3::from(wedge_g4[3])).with_w(-(right_anti_dual_g0[1] * wedge_g4[1]) - (right_anti_dual_g0[2] * wedge_g4[2]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
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
            Simd32x4::from(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       25        0
    //    simd3        3        8        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       22       40        0
    //  no simd       40       77        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
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
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_anti_dual_g0[3] * other[scalar] * self[scalar])
                    - (wedge_g3[0] * right_anti_dual_g0[0])
                    - (wedge_g3[1] * right_anti_dual_g0[1])
                    - (wedge_g3[2] * right_anti_dual_g0[2]),
                wedge_g0_y * right_anti_dual_g0[3],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g0 * wedge_g4.www().with_w(wedge_g1[3]))
                + (Simd32x4::from([wedge_g1[0], wedge_g1[1], wedge_g1[2], 1.0])
                    * right_anti_dual_g0.www().with_w(-(right_anti_dual_g0[1] * wedge_g4[1]) - (right_anti_dual_g0[2] * wedge_g4[2])))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * right_anti_dual_g0.xyz())
                + (Simd32x3::from(right_anti_dual_g0[3]) * ((Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * other.group0().xyz()))),
            // e23, e31, e12
            wedge_g3 * Simd32x3::from(right_anti_dual_g0[3]),
            // e423, e431, e412, e321
            wedge_g4 * Simd32x4::from(right_anti_dual_g0[3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       56        0
    //    simd3       10       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       63       84        0
    //  no simd      113      151        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
            (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e12] * self[e3]),
        ]) + (Simd32x4::from(other[scalar]) * self.group4())
            + (Simd32x4::from(self[scalar]) * other.group4())
            - (self.group1().yzxx() * other.group2().zxy().with_w(other[e23]))
            - (self.group2().zxy() * other.group1().yzx()).with_w(other[e31] * self[e2]);
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (other[scalar] * other[scalar] * self[scalar])
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       13        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_anti_dual_g0 * self[scalar] * other[e321], 1.0]) * Simd32x2::from([-1.0, 0.0]),
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
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        4       11        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       26       58        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
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
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]), 0.0]),
            // e1, e2, e3, e4
            ((wedge_g3.yzx() * other.group0().zxy()) - (wedge_g3.zxy() * other.group0().yzx())).with_w(-(wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3])),
            // e41, e42, e43
            (wedge_g4.zxy() * other.group0().yzx()) - (wedge_g4.yzx() * other.group0().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3]) * other.group0().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
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
impl std::ops::Div<AntiRejectOrthogonallyFromInfix> for Origin {
    type Output = AntiRejectOrthogonallyFromInfixPartial<Origin>;
    fn div(self, _rhs: AntiRejectOrthogonallyFromInfix) -> Self::Output {
        AntiRejectOrthogonallyFromInfixPartial(self)
    }
}
impl AntiRejectOrthogonallyFrom<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[scalar] * other[scalar] * self[e4]);
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       11        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e1]) - (wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]) - (wedge_g0[3] * other[e321])),
            // e423, e431, e412, e321
            (other.group0().xyz() * wedge_g0.www()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[e4]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Origin::from_groups(
            // e4
            -(right_anti_dual_g0[0] * wedge_g0_xyz[0]) - (right_anti_dual_g0[1] * wedge_g0_xyz[1]) - (right_anti_dual_g0[2] * wedge_g0_xyz[2]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       15        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[e4]) * other.group1().xyz();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (right_anti_dual_g0[3] * other[scalar] * self[e4])
                    - (wedge_g1_xyz[0] * right_anti_dual_g0[0])
                    - (wedge_g1_xyz[1] * right_anti_dual_g0[1])
                    - (wedge_g1_xyz[2] * right_anti_dual_g0[2]),
            ),
            // e423, e431, e412, e321
            (wedge_g1_xyz * right_anti_dual_g0.www()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd2        0        2        0
    //    simd3        4        9        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       19       41        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, other[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g2 = Simd32x3::from(self[e4]) * other.group1().xyz();
        let wedge_g4_xyz = Simd32x3::from(self[e4]) * other.group3();
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, wedge_g0[1] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (other[scalar] * other[scalar] * self[e4])
                    - (wedge_g0[1] * other[e321])
                    - (right_anti_dual_g2[0] * wedge_g4_xyz[0])
                    - (right_anti_dual_g2[1] * wedge_g4_xyz[1])
                    - (right_anti_dual_g2[2] * wedge_g4_xyz[2])
                    - (wedge_g2[0] * other[e1])
                    - (wedge_g2[1] * other[e2])
                    - (wedge_g2[2] * other[e3]),
            ),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0[1])) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4_xyz.zxy() * other.group1().yzx())
                - (wedge_g4_xyz.yzx() * other.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            ((wedge_g4_xyz * Simd32x3::from(other[scalar])) + (Simd32x3::from(wedge_g0[1]) * other.group1().xyz())).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * f32::powi(other[e321], 2) * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[e4]) * other.group0().xyz();
        return Origin::from_groups(/* e4 */ -(wedge_g0[0] * other[e1]) - (wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Origin {
    type Output = Origin;
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * f32::powi(other[scalar], 2));
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
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * Simd32x4::from([other[scalar] * self[e423], other[scalar] * self[e431], other[scalar] * self[e412], other[scalar] * self[e321]]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3        9        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(wedge_g0 * other[e321] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(wedge_g0) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5       21        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(wedge_g0[3]) * right_anti_dual_g0.xyz()).with_w(-(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
                - (wedge_g0.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
            // e423, e431, e412, e321
            wedge_g0 * Simd32x4::from(right_anti_dual_g0[3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        2        7        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       28       57        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_y = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let wedge_g4 = Simd32x4::from(other[scalar]) * self.group0();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]),
                wedge_g0_y * other[scalar],
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y)) + (right_anti_dual_g2 * Simd32x3::from(wedge_g4[3])).with_w(right_anti_dual_g2[2] * wedge_g4[2] * -1.0)
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - Simd32x3::from(0.0).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g4.zxy() * other.group1().yzx()) - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(wedge_g4[3]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) * other.group0().xyz()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], self[e321] * other[scalar]]),
        );
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
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * Simd32x4::from([other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], other[scalar] * self[e4]]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       18       39        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = ((Simd32x3::from(other[e4]) * self.group0().xyz()).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]))
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]));
        let wedge_g1_xyz = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0[3]) * Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]))
                + (wedge_g1_xyz.yzx() * other.group0().zxy()).with_w(-(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3]))
                - (wedge_g1_xyz.zxy() * other.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (other.group0().xyz() * wedge_g0.www()).with_w(0.0),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e321] * other[e321] * self[e4] * -1.0);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       14        0
    //  no simd       13       24        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (right_anti_dual_g0 * Simd32x3::from(wedge_g0[3])).with_w(-(right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
                - (wedge_g0.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        1        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       17       37        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0[3]) * Simd32x4::from(other[scalar]) * self.group0())
                + (Simd32x3::from(wedge_g1[3]) * right_anti_dual_g0.xyz()).with_w(-(right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]))
                - (wedge_g1.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g0[0])),
            // e423, e431, e412, e321
            wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       34        0
    //    simd3        6       12        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       34       54        0
    //  no simd       64      102        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
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
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0_y))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0_y)) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0_y) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(
            // e4
            -(other[e321] * other[e321] * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        3        6        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       10       20        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let wedge_g1 = (other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy());
        return Point::from_groups(
            // e1, e2, e3, e4
            ((wedge_g1.yzx() * other.group0().zxy()) - (wedge_g1.zxy() * other.group0().yzx())).with_w(-(wedge_g0[1] * other[e2]) - (wedge_g0[2] * other[e3])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * Simd32x4::from([self[e1] * other[scalar], self[e2] * other[scalar], self[e3] * other[scalar], self[e4] * other[scalar]]),
        );
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
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn anti_reject_orthogonally_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * Simd32x2::from([other[scalar] * self[scalar], other[e1234] * self[scalar]]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       15       31        0
    fn anti_reject_orthogonally_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.zxy() * other.group0().yzx()) - (wedge_g1.yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xyz().with_w(0.0).wwwx() * wedge_g1.xyz().with_w(wedge_g0[0]))
                + Simd32x3::from(0.0).with_w(
                    (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3])
                        - (right_anti_dual_g0[1] * wedge_g1[1])
                        - (right_anti_dual_g0[2] * wedge_g1[2])
                        - (right_anti_dual_g0[3] * wedge_g1[3]),
                )
                - (wedge_g1.wwwx() * other.group0().xyz().with_w(right_anti_dual_g0[0])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_reject_orthogonally_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g1 = Simd32x3::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2]),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       26        0
    fn anti_reject_orthogonally_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let right_anti_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * wedge_g0.xyz()) + (Simd32x3::from(wedge_g0[3]) * right_anti_dual_g0.xyz())).with_w(right_anti_dual_g0[3] * wedge_g0[3]),
            // e23, e31, e12, scalar
            (wedge_g1 * Simd32x4::from(right_anti_dual_g0[3]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])),
        );
    }
}
impl AntiRejectOrthogonallyFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd2        0        1        0
    //    simd3        4       10        0
    //    simd4        5        8        0
    // Totals...
    // yes simd       25       42        0
    //  no simd       48       87        0
    fn anti_reject_orthogonally_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from(self[scalar]) * other.group0();
        let wedge_g1 = Simd32x4::from(self[scalar]) * other.group1();
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group2();
        let wedge_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let wedge_g4 = Simd32x4::from(self[scalar]) * other.group4();
        let right_anti_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_anti_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[scalar]) + (wedge_g1[0] * other[e1]) + (wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3])
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
            Simd32x4::from([
                (right_anti_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other[e3]),
                (right_anti_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other[e1]),
                (right_anti_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other[e2]),
                -(right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other[e1]) - (wedge_g2[1] * other[e2]) - (wedge_g2[2] * other[e3]),
            ]) + (right_anti_dual_g1 * Simd32x4::from(wedge_g0[1]))
                + (wedge_g1 * Simd32x4::from(other[scalar]))
                - (wedge_g4.yzxx() * Simd32x3::from(0.0).with_w(right_anti_dual_g2[0]))
                - (wedge_g3.zxy() * other.group1().yzx()).with_w(right_anti_dual_g2[1] * wedge_g4[1]),
            // e41, e42, e43
            (right_anti_dual_g2 * Simd32x3::from(wedge_g0[1])) + (wedge_g2 * Simd32x3::from(other[scalar])) + (wedge_g4.zxy() * other.group1().yzx())
                - (wedge_g4.yzx() * other.group1().zxy()),
            // e23, e31, e12
            (wedge_g3 * Simd32x3::from(other[scalar])) - (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[scalar])) + (Simd32x4::from(wedge_g0[1]) * other.group1().xyz().with_w(0.0)),
        );
    }
}
impl AntiRejectOrthogonallyFrom<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl AntiRejectOrthogonallyFrom<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_reject_orthogonally_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group0();
        return Scalar::from_groups(/* scalar */ (wedge_g0[0] * other[e1]) + (wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]));
    }
}
impl AntiRejectOrthogonallyFrom<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reject_orthogonally_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
    }
}
