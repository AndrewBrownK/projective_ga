// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 64
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         2       7       0
//  Average:         5      11       0
//  Maximum:        47      62       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3      16       0
//  Average:        11      23       0
//  Maximum:        91     114       0
impl std::ops::Div<ProjectViaOriginOntoInfix> for DualNum {
    type Output = ProjectViaOriginOntoInfixPartial<DualNum>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5        7        0
    //  no simd       13       19        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = other.group0().xyz() * self.group0().xx().with_z(self[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group1().zxy()) - (wedge_g1_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge_g1_xyz.with_w(0.0).wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) + (self[scalar] * f32::powi(other[e321], 2)))
                - (other.group1().wwwx() * wedge_g1_xyz.with_w(0.0)),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]));
    }
}
impl ProjectViaOriginOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        5        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        9       23        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = self.group0().xx().with_zw(self[scalar], right_dual_g0[3] * self[scalar]) * right_dual_g0.xyz().with_w(1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]) * other.group1()) + Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       30        0
    //    simd2        0        1        0
    //    simd3        5       10        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       56       86        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, self[scalar] * other[scalar]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(self[scalar] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group3() * Simd32x3::from(-1.0);
        let wedge_g4 = (other.group1().xyz() * self.group0().xx().with_z(self[scalar])).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[e1234])
                    + (wedge_g0[1] * other[scalar])
                    + (wedge_g4[0] * other[e1])
                    + (wedge_g4[1] * other[e2])
                    + (wedge_g4[2] * other[e3])
                    + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0[1] * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0[1]) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[1]) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0[1]) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0[1]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = other.group0().xyz() * self.group0().xx().with_z(self[scalar]);
        return Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]));
    }
}
impl ProjectViaOriginOnto<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
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
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       21       35        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0 = (self.group0().wwwx() * right_dual_g0.xyz().with_w(other[e1]))
            + Simd32x3::from(0.0)
                .with_w((other[e2] * self[e2]) + (other[e3] * self[e3]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]))
            - (right_dual_g0.wwwx() * self.group0().xyz().with_w(self[e423]));
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0[3]) * other.group0())
                + (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g0.xyz().with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])))
                - (other.group1().yzxx() * Simd32x3::from(0.0).with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * wedge_g0.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(wedge_g0[3] * other[e321]),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        7       17        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        return Point::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        7       21        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])),
            // e423, e431, e412, e321
            (wedge_g0_xyz * other.group0().www()).with_w(0.0),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       30        0
    //    simd3        7       12        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       33       47        0
    //  no simd       59       86        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
            - (right_dual_g1[0] * self[e423])
            - (right_dual_g1[1] * self[e431])
            - (right_dual_g1[2] * self[e412])
            - (right_dual_g1[3] * self[e321]);
        let wedge_g2 = (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz());
        let wedge_g4 = ((right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx())).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (Simd32x4::from(wedge_g0_y) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        5       23        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g0.xyz().with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])))
                - (other.group0().yzxx() * Simd32x3::from(0.0).with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group0(),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])) * other.group0(),
        );
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = other[e321] * self[e321];
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321]);
    }
}
impl ProjectViaOriginOnto<MultiVector> for Horizon {
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
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = self[e321] * other[e321];
        return MultiVector::from_groups(
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
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
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
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       17       34        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0 = (Simd32x3::from(right_dual_g0[3]) * self.group1()).with_w(0.0) + (self.group0().yzx() * right_dual_g0.zxy()).with_w(0.0)
            - (self.group0().zxy() * right_dual_g0.yzx()).with_w(0.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g0.yzx() * other.group1().zxy()) - (wedge_g0.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from([other[e321], other[e321], other[e321], 1.0])
                * wedge_g0.xyz().with_w((wedge_g0[1] * other[e2]) + (wedge_g0[2] * other[e3]) + (wedge_g0[3] * other[e4]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (wedge_g0.wwwx() * other.group1().xyz().with_w(other[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from(other[e321] * -1.0) * self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       12        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0 = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge_g0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       15        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(wedge_g0) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(wedge_g0) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        5       12        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       45       71        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = -(right_dual_g2[0] * self[e23]) - (right_dual_g2[1] * self[e31]) - (right_dual_g2[2] * self[e12]);
        let wedge_g4 = (Simd32x3::from(right_dual_g1[3]) * self.group1()).with_w(0.0) + (self.group0().yzx() * right_dual_g1.zxy()).with_w(0.0)
            - (self.group0().zxy() * right_dual_g1.yzx()).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            (((Simd32x3::from(wedge_g4[3]) * other.group2()) + (other.group3().yzx() * wedge_g4.zxy())).with_w(wedge_g4[2] * other[e43]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (Simd32x3::from(wedge_g0_y) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy()) - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        5        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       16        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(other[e321] * -1.0) * self.group1();
        return Line::from_groups(
            // e41, e42, e43
            (wedge_g0_xyz.yzx() * other.group0().zxy()) - (wedge_g0_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            wedge_g0_xyz * Simd32x3::from(other[e321]) * Simd32x3::from(-1.0),
        );
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        1        6        0
    //    simd4        5        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       26       35        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g1 = (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
            + (right_dual_g0.zxy() * self.group0().yzx()).with_w(0.0)
            - (right_dual_g0.yzx() * self.group0().zxy()).with_w(0.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1.yzx() * other.group1().zxy()) - (wedge_g1.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge_g1.wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((wedge_g1[1] * other[e2]) + (wedge_g1[2] * other[e3]) + (wedge_g1[3] * other[e4]) - (right_dual_g0[3] * other[e321] * self[scalar]))
                - (other.group1().wwwx() * wedge_g1.xyz().with_w(0.0)),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * (Simd32x3::from(right_dual_g0) * self.group1().xyz()).with_w(right_dual_g0 * self[scalar]) * Simd32x4::from(-1.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       18        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g0_w = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(wedge_g0_w) * other.group0()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge_g0_w) * other.group1()).with_w(-(wedge_g0_xyz[0] * other[e23]) - (wedge_g0_xyz[1] * other[e31]) - (wedge_g0_xyz[2] * other[e12])),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       15       25        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0 = (right_dual_g0 * Simd32x4::from(self[scalar]))
            + Simd32x3::from(0.0).with_w(-(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]));
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]) * other.group1()) + Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       32        0
    //    simd3        5       13        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       36       51        0
    //  no simd       70       95        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = (self[scalar] * other[scalar]) - (right_dual_g2[0] * self[e23]) - (right_dual_g2[1] * self[e31]) - (right_dual_g2[2] * self[e12]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g1[3] * self[scalar]);
        let wedge_g2 = right_dual_g2 * Simd32x3::from(self[scalar]);
        let wedge_g4 = (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0)
            + (right_dual_g1.zxy() * self.group0().yzx()).with_w(0.0)
            - (right_dual_g1.yzx() * self.group0().zxy()).with_w(0.0);
        return MultiVector::from_groups(
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
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        7       20        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        let wedge_g1_xyz = Simd32x3::from(right_dual_g0) * self.group1().xyz();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group0().zxy()) - (wedge_g1_xyz.zxy() * other.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(0.0).with_w(right_dual_g0 * self[scalar] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])) - (other.group0().wwwx() * wedge_g1_xyz.with_w(0.0)),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = other.group0().xyz() * self.group1().www();
        return Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]));
    }
}
impl ProjectViaOriginOnto<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       15        0
    //    simd3        3       10        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       40       65        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0_y = (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
            - (right_dual_g0[0] * self[e423])
            - (right_dual_g0[1] * self[e431])
            - (right_dual_g0[2] * self[e412])
            - (right_dual_g0[3] * self[e321]);
        let wedge_g2 = (Simd32x3::from(self[e4]) * right_dual_g0.xyz()) - (Simd32x3::from(right_dual_g0[3]) * self.group1().xyz());
        let wedge_g4 = (Simd32x3::from(right_dual_g0[3]) * self.group3()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
            + (self.group2().yzx() * right_dual_g0.zxy()).with_w(0.0)
            - (self.group2().zxy() * right_dual_g0.yzx()).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4]) - (right_dual_g0[3] * other[e321] * self[scalar]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0_y) * other.group0())
                + (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g2.with_w(-(wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412])))
                - (other.group1().yzxx() * Simd32x3::from(0.0).with_w(wedge_g2[0])),
            // e41, e42, e43
            (wedge_g4.yzx() * other.group1().zxy()) - (wedge_g4.zxy() * other.group1().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g4[3]) * other.group1().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0_y) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd2        0        1        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       26        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * other[e321] * self[scalar], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_dual_g0) * Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_dual_g0) * Simd32x3::from(other[e321]) * self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(right_dual_g0 * other[e321] * self[e321] * -1.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        8        0
    //    simd3        2        8        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       11       32        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_y = -(right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        let wedge_g2 = right_dual_g0 * Simd32x3::from(self[scalar]);
        let wedge_g4_xyz = (right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(wedge_g2[0] * other[e23]) - (wedge_g2[1] * other[e31]) - (wedge_g2[2] * other[e12]), 0.0]),
            // e1, e2, e3, e4
            ((wedge_g4_xyz.zxy() * other.group1().yzx()) - (wedge_g4_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g4_xyz[1] * other[e42]) - (wedge_g4_xyz[2] * other[e43])),
            // e41, e42, e43
            Simd32x3::from(wedge_g0_y) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        3        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       16       42        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0_y = (right_dual_g0[3] * self[scalar]) - (right_dual_g0[0] * self[e23]) - (right_dual_g0[1] * self[e31]) - (right_dual_g0[2] * self[e12]);
        let wedge_g2 = Simd32x3::from(self[scalar]) * right_dual_g0.xyz();
        let wedge_g4_xyz = (right_dual_g0.yzx() * self.group1().zxy()) - (right_dual_g0.zxy() * self.group1().yzx());
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) - (wedge_g2[0] * other[e23]) - (wedge_g2[1] * other[e31]) - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            ((wedge_g4_xyz.zxy() * other.group1().yzx()) - (wedge_g4_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g4_xyz[1] * other[e42]) - (wedge_g4_xyz[2] * other[e43])),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(wedge_g0_y) * other.group1().xyz(),
            // e423, e431, e412, e321
            (wedge_g4_xyz * other.group0().www()).with_w(0.0),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        7       17        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       47       62        0
    //  no simd       91      114        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = (other[scalar] * self[scalar]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
            - (right_dual_g2[0] * self[e23])
            - (right_dual_g2[1] * self[e31])
            - (right_dual_g2[2] * self[e12])
            - (right_dual_g1[0] * self[e423])
            - (right_dual_g1[1] * self[e431])
            - (right_dual_g1[2] * self[e412])
            - (right_dual_g1[3] * self[e321]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(right_dual_g1[3] * self[scalar]);
        let wedge_g2 = (right_dual_g2 * Simd32x3::from(self[scalar])) + (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group1().xyz());
        let wedge_g4 = (Simd32x3::from(right_dual_g1[3]) * self.group3()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0)
            + (right_dual_g2.yzx() * self.group1().zxy()).with_w(0.0)
            + (self.group2().yzx() * right_dual_g1.zxy()).with_w(0.0)
            - (right_dual_g2.zxy() * self.group1().yzx()).with_w(0.0)
            - (self.group2().zxy() * right_dual_g1.yzx()).with_w(0.0);
        return MultiVector::from_groups(
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
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0_y) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        7        0
    //    simd2        0        1        0
    //    simd3        1        7        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       18        0
    //  no simd        8       42        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other[e321] * -1.0;
        let wedge_g2 = Simd32x3::from(right_dual_g0) * self.group1().xyz() * Simd32x3::from(-1.0);
        let wedge_g4_xyz = Simd32x3::from(right_dual_g0) * self.group3();
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual_g0 * self[scalar] * other[e321], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g2.with_w(-(wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412])))
                - (other.group0().yzxx() * Simd32x3::from(0.0).with_w(wedge_g2[0])),
            // e41, e42, e43
            (wedge_g4_xyz.yzx() * other.group0().zxy()) - (wedge_g4_xyz.zxy() * other.group0().yzx()),
            // e23, e31, e12
            wedge_g4_xyz * Simd32x3::from(other[e321]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual_g0 * self[e321] * -1.0) * other.group0(),
        );
    }
}
impl ProjectViaOriginOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       13        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g4_xyz = other.group0().xyz() * self.group0().xx().with_z(self[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(wedge_g4_xyz[0] * other[e1]) + (wedge_g4_xyz[1] * other[e2]) + (wedge_g4_xyz[2] * other[e3]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from((self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
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
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       16        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0 = -(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]) - (right_dual_g0[3] * self[e321]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge_g0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * other[e321] * self[e321]);
    }
}
impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       24        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0 = -(right_dual_g1[0] * self[e423]) - (right_dual_g1[1] * self[e431]) - (right_dual_g1[2] * self[e412]) - (right_dual_g1[3] * self[e321]);
        return MultiVector::from_groups(
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
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0());
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
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        2        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       34        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g0 = (self.group0().wwwx() * right_dual_g0.xyz().with_w(other[e1]))
            + (self.group0().xyz() * right_dual_g0.www() * Simd32x3::from(-1.0)).with_w((other[e2] * self[e2]) + (other[e3] * self[e3]));
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(wedge_g0[3]) * other.group0())
                + (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g0.xyz().with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])))
                - (other.group1().yzxx() * Simd32x3::from(0.0).with_w(wedge_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge_g0[3]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * Simd32x3::from(other[e321] * -1.0) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        7       17        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x3::from(-1.0);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        return Point::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        7       21        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual_g0 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let wedge_g0_xyz = (right_dual_g0.yzx() * self.group0().zxy()) - (right_dual_g0.zxy() * self.group0().yzx());
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((wedge_g0_xyz.zxy() * other.group1().yzx()) - (wedge_g0_xyz.yzx() * other.group1().zxy())).with_w(-(wedge_g0_xyz[1] * other[e42]) - (wedge_g0_xyz[2] * other[e43])),
            // e423, e431, e412, e321
            (wedge_g0_xyz * other.group0().www()).with_w(0.0),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        7       12        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       55       82        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_dual_g1 = Simd32x3::from(0.0).with_w(other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let right_dual_g2 = other.group3() * Simd32x3::from(-1.0);
        let wedge_g0_y = (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]);
        let wedge_g2 = (Simd32x3::from(self[e4]) * right_dual_g1.xyz()) - (Simd32x3::from(right_dual_g1[3]) * self.group0().xyz());
        let wedge_g4 = ((right_dual_g2.yzx() * self.group0().zxy()) - (right_dual_g2.zxy() * self.group0().yzx())).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0_y * other[scalar]) + (wedge_g4[0] * other[e1]) + (wedge_g4[1] * other[e2]) + (wedge_g4[2] * other[e3]) + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12]),
                wedge_g0_y * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (Simd32x4::from(wedge_g0_y) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0_y) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0_y) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0_y) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5       17        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(other[e321] * -1.0) * self.group0().xyz() * Simd32x3::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * wedge_g0.with_w(-(wedge_g0[1] * other[e431]) - (wedge_g0[2] * other[e412])))
                - (other.group0().yzxx() * Simd32x3::from(0.0).with_w(wedge_g0[0])),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])) * other.group0(),
        );
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
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd       13       20        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge_g1_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge_g1_xyz.yzx() * other.group1().zxy()) - (wedge_g1_xyz.zxy() * other.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge_g1_xyz.with_w(0.0).wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w((wedge_g1_xyz[1] * other[e2]) + (wedge_g1_xyz[2] * other[e3]) + (other[e321] * other[e321] * self[scalar]))
                - (other.group1().wwwx() * wedge_g1_xyz.with_w(0.0)),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x3::from(self[scalar]) * other.group1() * Simd32x3::from(-1.0);
        return Scalar::from_groups(/* scalar */ -(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12]));
    }
}
impl ProjectViaOriginOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       22        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(wedge_g0[3]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * wedge_g0.xyz())).with_w(wedge_g0[3] * other[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(wedge_g0[3]) * other.group1()) + Simd32x3::from(0.0).with_w(-(wedge_g0[0] * other[e23]) - (wedge_g0[1] * other[e31]) - (wedge_g0[2] * other[e12])),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       30        0
    //    simd2        0        1        0
    //    simd3        5       10        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       56       86        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge_g0 = Simd32x2::from([1.0, other[scalar] * self[scalar]]) * Simd32x2::from([0.0, 1.0]);
        let wedge_g1 = Simd32x3::from(0.0).with_w(other[e321] * self[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let wedge_g2 = Simd32x3::from(self[scalar]) * other.group3() * Simd32x3::from(-1.0);
        let wedge_g4 = (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (wedge_g0[0] * other[e1234])
                    + (wedge_g0[1] * other[scalar])
                    + (wedge_g4[0] * other[e1])
                    + (wedge_g4[1] * other[e2])
                    + (wedge_g4[2] * other[e3])
                    + (wedge_g4[3] * other[e4])
                    - (wedge_g2[0] * other[e23])
                    - (wedge_g2[1] * other[e31])
                    - (wedge_g2[2] * other[e12])
                    - (wedge_g1[0] * other[e423])
                    - (wedge_g1[1] * other[e431])
                    - (wedge_g1[2] * other[e412])
                    - (wedge_g1[3] * other[e321]),
                wedge_g0[1] * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge_g2[0] * other[e321]) + (wedge_g4[2] * other[e31]) + (wedge_g4[3] * other[e41]),
                (wedge_g2[1] * other[e321]) + (wedge_g4[0] * other[e12]) + (wedge_g4[3] * other[e42]),
                (wedge_g2[2] * other[e321]) + (wedge_g4[1] * other[e23]) + (wedge_g4[3] * other[e43]),
                -(wedge_g2[0] * other[e423]) - (wedge_g2[1] * other[e431]) - (wedge_g2[2] * other[e412]) - (wedge_g4[2] * other[e43]),
            ]) + (wedge_g1 * Simd32x4::from(other[e1234]))
                + (Simd32x4::from(wedge_g0[1]) * other.group1())
                - (wedge_g4.yzxx() * other.group3().zxy().with_w(other[e41]))
                - Simd32x3::from(0.0).with_w(wedge_g4[1] * other[e42]),
            // e41, e42, e43
            (wedge_g2 * Simd32x3::from(other[e1234])) + (Simd32x3::from(wedge_g0[1]) * other.group2()) + (wedge_g4.yzx() * other.group4().zxy())
                - (wedge_g4.zxy() * other.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge_g0[1]) * other.group3()) + (Simd32x3::from(wedge_g4[3]) * other.group4().xyz()) - (Simd32x3::from(other[e321]) * wedge_g4.xyz()),
            // e423, e431, e412, e321
            (wedge_g4 * Simd32x4::from(other[e1234])) + (Simd32x4::from(wedge_g0[1]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * other[e321] * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        return Scalar::from_groups(/* scalar */ (wedge_g0_xyz[0] * other[e1]) + (wedge_g0_xyz[1] * other[e2]) + (wedge_g0_xyz[2] * other[e3]));
    }
}
impl ProjectViaOriginOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * other[scalar] * self[scalar]);
    }
}
