// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 99
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5      10       0
//  Average:        17      26       0
//  Maximum:       162     202       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        11      22       0
//  Average:        36      49       0
//  Maximum:       362     404       0
impl std::ops::Div<AntiSandwichInfix> for AntiScalar {
    type Output = AntiSandwichInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234] * self[e1234])
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234] * self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234] * self[e1234]) * other.group1(),
        )
    }
}
impl AntiSandwich<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321])
    }
}
impl AntiSandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234] * self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234] * self[e1234]) * other.group1(),
        )
    }
}
impl AntiSandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234] * self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234] * self[e1234]) * other.group1(),
        )
    }
}
impl AntiSandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234] * self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234] * self[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234] * self[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234] * self[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234] * self[e1234]) * other.group4(),
        )
    }
}
impl AntiSandwich<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4])
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234] * self[e1234] * other[scalar])
    }
}
impl std::ops::Div<AntiSandwichInfix> for DualNum {
    type Output = AntiSandwichInfixPartial<DualNum>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e1234]) * self.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
            geometric_anti_product_g0[1] * self[e1234],
        ]))
    }
}
impl AntiSandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = other[e1234] * self[e1234];
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0_y * self[scalar]) + (self[e1234] * self[e1234] * other[scalar]) + (other[e1234] * self[scalar] * self[e1234]),
            geometric_anti_product_g0_y * self[e1234],
        ]))
    }
}
impl AntiSandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       22        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = self[e1234] * other[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e1234]) * other.group1().xyz();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234] * self[e1234]) * other.group0().xyz()) + (Simd32x3::from(self[scalar] * self[e1234]) * other.group1().xyz())
                - (geometric_anti_product_g1_xyz * Simd32x3::from(self[scalar])))
            .with_w(geometric_anti_product_g0_w * self[e1234]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1_xyz * Simd32x3::from(self[e1234]))
                .with_w((self[e1234] * self[e1234] * other[e321]) + (self[scalar] * self[e1234] * other[e4]) - (geometric_anti_product_g0_w * self[scalar])),
        )
    }
}
impl AntiSandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321])
    }
}
impl AntiSandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(self[e1234]) * other.group0();
        Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g0 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(self[e1234]) * ((Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()))),
        )
    }
}
impl AntiSandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e1234]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(self[e1234]) * ((Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()))),
        )
    }
}
impl AntiSandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd3        4       10        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       16       44        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = self[e1234] * other[e1234];
        let geometric_anti_product_g1_w = self[e1234] * other[e4];
        let geometric_anti_product_g2 = Simd32x3::from(self[e1234]) * other.group2();
        let geometric_anti_product_g4_xyz = Simd32x3::from(self[e1234]) * other.group4().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_y * self[scalar]) + (self[e1234] * self[e1234] * other[scalar]) + (self[scalar] * self[e1234] * other[e1234]),
                geometric_anti_product_g0_y * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234] * self[e1234]) * other.group1().xyz()) + (Simd32x3::from(self[scalar] * self[e1234]) * other.group4().xyz())
                - (geometric_anti_product_g4_xyz * Simd32x3::from(self[scalar])))
            .with_w(geometric_anti_product_g1_w * self[e1234]),
            // e41, e42, e43
            geometric_anti_product_g2 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(self[e1234] * self[e1234]) * other.group3())
                + (Simd32x3::from(self[scalar] * self[e1234]) * other.group2()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4_xyz * Simd32x3::from(self[e1234]))
                .with_w((self[e1234] * self[e1234] * other[e321]) + (self[scalar] * self[e1234] * other[e4]) - (geometric_anti_product_g1_w * self[scalar])),
        )
    }
}
impl AntiSandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4])
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234] * self[e1234] * other[scalar])
    }
}
impl std::ops::Div<AntiSandwichInfix> for Flector {
    type Output = AntiSandwichInfixPartial<Flector>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd       11       20        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[3] * self[e321])
                    - (geometric_anti_product_g1[1] * self[e2])
                    - (geometric_anti_product_g1[2] * self[e3])
                    - (geometric_anti_product_g1[3] * self[e4]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0[2], geometric_anti_product_g1[2]]))
                - (Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[3]]) * self.group0().xw()),
        )
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd2        4        4        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       24        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(other[scalar]) * self.group1().xyz());
        let geometric_anti_product_g0_w = other[e1234] * self[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e1234]) * self.group1().xyz();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_xyz[2] * self[e412])
                    - (geometric_anti_product_g1_xyz[0] * self[e1])
                    - (geometric_anti_product_g1_xyz[1] * self[e2])
                    - (geometric_anti_product_g1_xyz[2] * self[e3]),
                0.0,
            ]) + (Simd32x2::from([geometric_anti_product_g0_w, geometric_anti_product_g1_xyz[0]]) * self.group1().wx())
                + (Simd32x2::from([geometric_anti_product_g0_xyz[0], geometric_anti_product_g1_xyz[1]]) * self.group1().xy())
                + (Simd32x2::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1_xyz[2]]) * self.group1().yz())
                - (Simd32x2::from(self[e4]) * Simd32x2::from([(other[e1234] * self[e321]) - (other[scalar] * self[e4]), geometric_anti_product_g0_w])),
        )
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       38        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       35       54        0
    //  no simd       80      102        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            -(other[e423] * self[e4]) - (other[e412] * self[e431]),
            -(other[e423] * self[e412]) - (other[e431] * self[e4]),
            -(other[e431] * self[e423]) - (other[e412] * self[e4]),
            (other[e431] * self[e431]) + (other[e412] * self[e412]),
        ]) + (other.group1().yzxx() * self.group1().zxyx())
            - (Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]));
        let geometric_anti_product_g1 = (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
            + (Simd32x4::from([self[e4], self[e412], self[e423], other[e423] * self[e1]]) * other.group0().xxy().with_w(1.0))
            + (Simd32x4::from([self[e431], self[e4], self[e4], other[e431] * self[e2]]) * other.group0().zyz().with_w(1.0))
            + (other.group1().yzxz() * self.group0().zxyz())
            - (Simd32x4::from([self[e2], self[e321], self[e321], other[e4] * self[e321]]) * other.group1().zyz().with_w(1.0))
            - (Simd32x4::from([self[e321], self[e3], self[e1], other[e3] * self[e412]]) * other.group1().xxy().with_w(1.0))
            - (other.group0().yzxx() * self.group1().zxyx())
            - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e3])
                    + (geometric_anti_product_g0[3] * self[e1])
                    + (geometric_anti_product_g1[0] * self[e4])
                    + (geometric_anti_product_g1[1] * self[e412])
                    + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[2] * self[e1])
                    + (geometric_anti_product_g0[3] * self[e2])
                    + (geometric_anti_product_g1[1] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e423])
                    + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[2] * self[e321])
                    + (geometric_anti_product_g0[3] * self[e3])
                    + (geometric_anti_product_g1[0] * self[e431])
                    + (geometric_anti_product_g1[2] * self[e4])
                    + (geometric_anti_product_g1[3] * self[e412]),
                geometric_anti_product_g0[2] * self[e412] * -1.0,
            ]) + (geometric_anti_product_g0.xyxw() * self.group1().ww().with_zw(self[e2], self[e4]))
                - (geometric_anti_product_g0.zxyx() * self.group0().yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g0[3] * self[e423],
                geometric_anti_product_g0[3] * self[e431],
                geometric_anti_product_g0[3] * self[e412],
                -(geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g0.xyxw() * self.group0().ww().with_zw(self[e431], self[e321]))
                + (self.group1().zx().with_zw(self[e4], geometric_anti_product_g1[3] * self[e4]) * geometric_anti_product_g0.yzz().with_w(1.0))
                - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(self[e1])),
        )
    }
}
impl AntiSandwich<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]);
        Horizon::from_groups(
            // e321
            -(geometric_anti_product_g1[0] * self[e423])
                - (geometric_anti_product_g1[1] * self[e431])
                - (geometric_anti_product_g1[2] * self[e412])
                - (geometric_anti_product_g1[3] * self[e4]),
        )
    }
}
impl AntiSandwich<Line> for Flector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd3       10       16        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       62       72        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
            (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
            0.0,
        ]) - (self.group1().yzxz() * other.group1().zxy().with_w(other[e43]))
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(self[e431] * other[e42])
            - (other.group0().yzx() * self.group0().zxy()).with_w(self[e423] * other[e41]);
        let geometric_anti_product_g1 = (self.group1().yzxy() * other.group0().zxy().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
            + (Simd32x3::from(self[e4]) * other.group0()).with_w(self[e423] * other[e23])
            - (other.group0().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]);
        Line::from_groups(
            // e41, e42, e43
            (geometric_anti_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_anti_product_g1.xyx())
                - (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_anti_product_g1.yzz()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + (geometric_anti_product_g0.zxy() * self.group1().yzx())
                + (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g1.yzz() * self.group0().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_anti_product_g0.xyx())
                - (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_anti_product_g0.yzz())
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd3        0        2        0
    //    simd4       17       16        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       84      100        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
            (self[e3] * other[e1234]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e412] * other[scalar]),
            0.0,
        ]) + (self.group0().xyxw() * other.group0().wwyw())
            - (self.group1().xyxz() * other.group1().wwy().with_w(other[e43]))
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
            - (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(self[e431] * other[e42]);
        let geometric_anti_product_g1 = (self.group1().xyxy() * other.group0().wwy().with_w(other[e31]))
            + (self.group1().yzzz() * other.group0().zxw().with_w(other[e12]))
            + Simd32x3::from(0.0).with_w((self[e321] * other[e1234]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
            + (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(self[e423] * other[e23])
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g1[0] * self[e4]) - (geometric_anti_product_g1[1] * self[e412]),
                -(geometric_anti_product_g1[1] * self[e4]) - (geometric_anti_product_g1[2] * self[e423]),
                -(geometric_anti_product_g1[0] * self[e431]) - (geometric_anti_product_g1[2] * self[e4]),
                (geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g1.zxyx() * self.group1().yzxx())
                - (Simd32x4::from(geometric_anti_product_g0[3]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0.zxyx() * self.group1().yzxx())
                + (geometric_anti_product_g0.wwwy() * self.group0().xyz().with_w(self[e431]))
                + (self.group0().zx().with_zw(self[e321], geometric_anti_product_g0[3] * self[e321]) * geometric_anti_product_g1.yzz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e2], geometric_anti_product_g0[2] * self[e412]) * geometric_anti_product_g1.xyx().with_w(1.0))
                - (Simd32x4::from(geometric_anti_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                - (geometric_anti_product_g1.zxyz() * self.group0().yzxz())
                - (self.group0().ww().with_zw(self[e431], geometric_anti_product_g1[0] * self[e1]) * geometric_anti_product_g0.xyx().with_w(1.0))
                - (self.group1().zx().with_zw(self[e4], geometric_anti_product_g1[1] * self[e2]) * geometric_anti_product_g0.yzz().with_w(1.0)),
        )
    }
}
impl AntiSandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       58        0
    //    simd2        8        8        0
    //    simd3       20       30        0
    //    simd4       14        8        0
    // Totals...
    // yes simd       76      104        0
    //  no simd      166      196        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from([self[e423], self[e4]]) * other.group1().xw());
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
            (self[e3] * other[e1234]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e412] * other[scalar]),
            0.0,
        ]) + (self.group0().xyxw() * Simd32x2::from(other[e1234]).with_zw(other[e42], other[e1234]))
            - (self.group1().xyxz() * Simd32x2::from(other[scalar]).with_zw(other[e31], other[e43]))
            - (Simd32x3::from(self[e4]) * other.group3()).with_w(self[e431] * other[e42])
            - (other.group2().yzx() * self.group0().zxy()).with_w(self[e423] * other[e41]);
        let geometric_anti_product_g2 = (self.group1().zxy() * other.group4().yzx())
            - (Simd32x3::from(self[e4]) * other.group4().xyz())
            - (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
            - (self.group1().yzz() * other.group4().zx().with_z(other[e4]));
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (self.group0().zxy() * other.group4().yzx())
            + (self.group1().xyx() * Simd32x2::from(other[e321]).with_z(other[e2]))
            + (self.group1().yzz() * other.group1().zx().with_z(other[e321]))
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
            - (self.group0().yzz() * other.group4().zx().with_z(other[e4]))
            - (self.group1().zxy() * other.group1().yzx());
        let geometric_anti_product_g4 = (self.group1().xyxy() * Simd32x2::from(other[e1234]).with_zw(other[e42], other[e31]))
            + (self.group1().yzzz() * other.group2().zx().with_zw(other[e1234], other[e12]))
            + Simd32x3::from(0.0).with_w((self[e321] * other[e1234]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
            + (Simd32x3::from(self[e4]) * other.group2()).with_w(self[e423] * other[e23])
            - (other.group2().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e321])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group0().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e1])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412]),
                (geometric_anti_product_g0[1] * self[e2])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423]),
                (geometric_anti_product_g0[1] * self[e3])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4]),
                geometric_anti_product_g2[2] * self[e412] * -1.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group1().xyz().with_w(1.0))
                - (self.group1().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (geometric_anti_product_g2.zxy() * self.group0().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (geometric_anti_product_g4.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_anti_product_g4.xyx())
                - (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_anti_product_g4.yzz()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                + (geometric_anti_product_g1.zxy() * self.group1().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group0().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_anti_product_g1.xyx())
                - (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_anti_product_g1.yzz())
                - (geometric_anti_product_g4.zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g2[1] * self[e412],
                geometric_anti_product_g2[2] * self[e423],
                geometric_anti_product_g2[2] * self[e4],
                -(geometric_anti_product_g2[1] * self[e2])
                    - (geometric_anti_product_g2[2] * self[e3])
                    - (geometric_anti_product_g3[0] * self[e423])
                    - (geometric_anti_product_g3[1] * self[e431])
                    - (geometric_anti_product_g3[2] * self[e412]),
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group1().xyz().with_w(1.0))
                + (self.group0().ww().with_zw(self[e431], geometric_anti_product_g0[1] * self[e321]) * geometric_anti_product_g2.xyx().with_w(1.0))
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e1]),
        )
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       19        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       24       39        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4] * -1.0) * self.group0().xyz().with_w(self[e321]);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e3])
                    + (geometric_anti_product_g0[3] * self[e1])
                    + (geometric_anti_product_g1[0] * self[e4])
                    + (geometric_anti_product_g1[1] * self[e412])
                    + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[2] * self[e1])
                    + (geometric_anti_product_g0[3] * self[e2])
                    + (geometric_anti_product_g1[1] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e423])
                    + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[2] * self[e321])
                    + (geometric_anti_product_g0[3] * self[e3])
                    + (geometric_anti_product_g1[0] * self[e431])
                    + (geometric_anti_product_g1[2] * self[e4])
                    + (geometric_anti_product_g1[3] * self[e412]),
                geometric_anti_product_g0[2] * self[e412] * -1.0,
            ]) + (geometric_anti_product_g0.xyxw() * self.group1().ww().with_zw(self[e2], self[e4]))
                - (geometric_anti_product_g0.zxyx() * self.group0().yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
        )
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       25        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       36       49        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            -(self[e4] * other[e423]) - (self[e431] * other[e412]),
            -(self[e4] * other[e431]) - (self[e412] * other[e423]),
            -(self[e4] * other[e412]) - (self[e423] * other[e431]),
            (self[e431] * other[e431]) + (self[e412] * other[e412]),
        ]) + (self.group1().zxyx() * other.group0().yzxx());
        let geometric_anti_product_g1 = Simd32x4::from([
            -(self[e2] * other[e412]) - (self[e321] * other[e423]),
            -(self[e3] * other[e423]) - (self[e321] * other[e431]),
            -(self[e1] * other[e431]) - (self[e321] * other[e412]),
            (self[e3] * other[e412]) + (self[e4] * other[e321]),
        ]) + (self.group0().zxyx() * other.group0().yzxx())
            + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2]));
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g0[3] * self[e423],
                geometric_anti_product_g0[3] * self[e431],
                geometric_anti_product_g0[3] * self[e412],
                -(geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g0.xyxw() * self.group0().ww().with_zw(self[e431], self[e321]))
                + (self.group1().zx().with_zw(self[e4], geometric_anti_product_g1[3] * self[e4]) * geometric_anti_product_g0.yzz().with_w(1.0))
                - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(self[e1])),
        )
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       34        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       28       40        0
    //  no simd       43       58        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]);
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e4] * other[e1]) + (self[e431] * other[e3]),
            (self[e4] * other[e2]) + (self[e412] * other[e1]),
            (self[e4] * other[e3]) + (self[e423] * other[e2]),
            -(self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (self.group1().zxyy() * other.group0().yzxy())
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e3])
                    + (geometric_anti_product_g0[3] * self[e1])
                    + (geometric_anti_product_g1[0] * self[e4])
                    + (geometric_anti_product_g1[1] * self[e412])
                    + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[2] * self[e1])
                    + (geometric_anti_product_g0[3] * self[e2])
                    + (geometric_anti_product_g1[1] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e423])
                    + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[2] * self[e321])
                    + (geometric_anti_product_g0[3] * self[e3])
                    + (geometric_anti_product_g1[0] * self[e431])
                    + (geometric_anti_product_g1[2] * self[e4])
                    + (geometric_anti_product_g1[3] * self[e412]),
                geometric_anti_product_g0[2] * self[e412] * -1.0,
            ]) + (geometric_anti_product_g0.xyxw() * self.group1().ww().with_zw(self[e2], self[e4]))
                - (geometric_anti_product_g0.zxyx() * self.group0().yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(
                (geometric_anti_product_g0[3] * self[e321]) + (geometric_anti_product_g1[3] * self[e4])
                    - (geometric_anti_product_g0[0] * self[e1])
                    - (geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412]),
            ),
        )
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       22        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group1().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_anti_product_g0_xyz[0] * self[e4]) - (geometric_anti_product_g0_xyz[1] * self[e412]),
                -(geometric_anti_product_g0_xyz[1] * self[e4]) - (geometric_anti_product_g0_xyz[2] * self[e423]),
                -(geometric_anti_product_g0_xyz[0] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e4]),
                (geometric_anti_product_g0_xyz[1] * self[e431]) + (geometric_anti_product_g0_xyz[2] * self[e412]),
            ]) + (self.group1().yzxx() * geometric_anti_product_g0_xyz.zxy().with_w(geometric_anti_product_g0_xyz[0]))
                - (Simd32x4::from(self[e4] * other[scalar] * -1.0) * self.group1().xyz().with_w(self[e4])),
        )
    }
}
impl std::ops::Div<AntiSandwichInfix> for Line {
    type Output = AntiSandwichInfixPartial<Line>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd2        3        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        8       15        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g1[0] * self[e41]) - (geometric_anti_product_g1[1] * self[e42]) - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd2        3        3        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6        9        0
    //  no simd       11       18        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1());
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g1[0] * self[e41]) - (geometric_anti_product_g1[1] * self[e42]) - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        0        6        0
    //    simd4        9        3        0
    // Totals...
    // yes simd       33       52        0
    //  no simd       60       73        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]),
            (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]),
            other[e412] * self[e43] * -1.0,
        ]) - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxy() * other.group0().yzx()).with_w(other[e423] * self[e41]);
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e4] * self[e41]) + (other[e412] * self[e42]),
            (other[e4] * self[e42]) + (other[e423] * self[e43]),
            (other[e4] * self[e43]) + (other[e431] * self[e41]),
            -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        ]) - (self.group0().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e43]) + (geometric_anti_product_g1[2] * self[e31]) + (geometric_anti_product_g1[3] * self[e41]),
                (geometric_anti_product_g0[2] * self[e41]) + (geometric_anti_product_g1[0] * self[e12]) + (geometric_anti_product_g1[3] * self[e42]),
                (geometric_anti_product_g0[0] * self[e42]) + (geometric_anti_product_g1[1] * self[e23]) + (geometric_anti_product_g1[3] * self[e43]),
                0.0,
            ]) - (geometric_anti_product_g1.yzxz() * self.group1().zxy().with_w(self[e43]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1()).with_w(geometric_anti_product_g1[1] * self[e42])
                - (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(geometric_anti_product_g1[0] * self[e41]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1.yzxy() * self.group0().zxy().with_w(self[e31]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e12]) - (geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(geometric_anti_product_g1[0] * self[e23])
                - (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(geometric_anti_product_g0[0] * self[e41]),
        )
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0();
        Horizon::from_groups(
            // e321
            -(geometric_anti_product_g0_xyz[0] * self[e41]) - (geometric_anti_product_g0_xyz[1] * self[e42]) - (geometric_anti_product_g0_xyz[2] * self[e43]),
        )
    }
}
impl AntiSandwich<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        7       14        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            other[e43] * self[e42],
            other[e41] * self[e43],
            other[e42] * self[e41],
            -(other[e42] * self[e42]) - (other[e43] * self[e43]),
        ]) - (other.group0().yzx() * self.group0().zxy()).with_w(other[e41] * self[e41]);
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e43] * self[e31]) + (other[e12] * self[e42]),
            (other[e41] * self[e12]) + (other[e23] * self[e43]),
            (other[e42] * self[e23]) + (other[e31] * self[e41]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e23])
            - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]);
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (anti_reverse_g0.xxy() * geometric_anti_product_g0.wzx()) + (anti_reverse_g0.zyz() * geometric_anti_product_g0.yww())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()),
            // e23, e31, e12
            (anti_reverse_g0.xxy() * geometric_anti_product_g1.wzx())
                + (anti_reverse_g0.zyz() * geometric_anti_product_g1.yww())
                + (anti_reverse_g1.xxy() * geometric_anti_product_g0.wzx())
                + (anti_reverse_g1.zyz() * geometric_anti_product_g0.yww())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g0.zxy()),
        )
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       38       56        0
    //  no simd       56       72        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e41] * other[e1234]) + (self[e42] * other[e43]),
            (self[e42] * other[e1234]) + (self[e43] * other[e41]),
            (self[e41] * other[e42]) + (self[e43] * other[e1234]),
            -(self[e42] * other[e42]) - (self[e43] * other[e43]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e41] * other[scalar]) + (self[e42] * other[e12]) + (self[e23] * other[e1234]) + (self[e31] * other[e43]),
            (self[e42] * other[scalar]) + (self[e43] * other[e23]) + (self[e31] * other[e1234]) + (self[e12] * other[e41]),
            (self[e41] * other[e31]) + (self[e43] * other[scalar]) + (self[e23] * other[e42]) + (self[e12] * other[e1234]),
            -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        ]) - (other.group1().yzxx() * self.group0().zxy().with_w(self[e41]))
            - (self.group1().zxy() * other.group0().yzx()).with_w(self[e42] * other[e31]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e43]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[2] * self[e41]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[0] * self[e42]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (geometric_anti_product_g0.zxyx() * self.group0().yzx().with_w(self[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e12])
                    + (geometric_anti_product_g0[3] * self[e23])
                    + (geometric_anti_product_g1[1] * self[e43])
                    + (geometric_anti_product_g1[3] * self[e41]),
                (geometric_anti_product_g0[2] * self[e23])
                    + (geometric_anti_product_g0[3] * self[e31])
                    + (geometric_anti_product_g1[2] * self[e41])
                    + (geometric_anti_product_g1[3] * self[e42]),
                (geometric_anti_product_g0[0] * self[e31])
                    + (geometric_anti_product_g0[3] * self[e12])
                    + (geometric_anti_product_g1[0] * self[e42])
                    + (geometric_anti_product_g1[3] * self[e43]),
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(self[e23]))
                - (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl AntiSandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       51        0
    //    simd2        6        6        0
    //    simd3       14       24        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       57       86        0
    //  no simd      118      155        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e41] * other[e321]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e31] * other[e412]),
            (self[e42] * other[e321]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[e12] * other[e423]),
            (self[e41] * other[e2]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]),
            self[e43] * other[e412] * -1.0,
        ]) - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxy() * other.group1().yzx()).with_w(self[e41] * other[e423]);
        let geometric_anti_product_g2 = (self.group0().xyx() * Simd32x2::from(other[e1234]).with_z(other[e42])) + (self.group0().yzz() * other.group2().zx().with_z(other[e1234]))
            - (self.group0().zxy() * other.group2().yzx());
        let geometric_anti_product_g3 = (self.group0().xyx() * Simd32x2::from(other[scalar]).with_z(other[e31]))
            + (self.group0().yzz() * other.group3().zx().with_z(other[scalar]))
            + (self.group1().xyx() * Simd32x2::from(other[e1234]).with_z(other[e42]))
            + (self.group1().yzz() * other.group2().zx().with_z(other[e1234]))
            - (self.group0().zxy() * other.group3().yzx())
            - (self.group1().zxy() * other.group2().yzx());
        let geometric_anti_product_g4 = Simd32x4::from([
            (self[e41] * other[e4]) + (self[e42] * other[e412]),
            (self[e42] * other[e4]) + (self[e43] * other[e423]),
            (self[e41] * other[e431]) + (self[e43] * other[e4]),
            -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
        ]) - (self.group0().zxy() * other.group4().yzx()).with_w(self[e41] * other[e1]);
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(anti_reverse_g1[0] * geometric_anti_product_g2[0]) - (anti_reverse_g1[1] * geometric_anti_product_g2[1]) - (anti_reverse_g1[2] * geometric_anti_product_g2[2]),
                0.0,
            ]) - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g0[0] * geometric_anti_product_g4[3]) + (anti_reverse_g0[2] * geometric_anti_product_g1[1]) + (anti_reverse_g1[1] * geometric_anti_product_g4[2]),
                (anti_reverse_g0[0] * geometric_anti_product_g1[2]) + (anti_reverse_g0[1] * geometric_anti_product_g4[3]) + (anti_reverse_g1[2] * geometric_anti_product_g4[0]),
                (anti_reverse_g0[1] * geometric_anti_product_g1[0]) + (anti_reverse_g0[2] * geometric_anti_product_g4[3]) + (anti_reverse_g1[0] * geometric_anti_product_g4[1]),
                0.0,
            ]) - (Simd32x4::from([
                geometric_anti_product_g1[3],
                geometric_anti_product_g4[2],
                geometric_anti_product_g4[0],
                anti_reverse_g0[1] * geometric_anti_product_g4[1],
            ]) * anti_reverse_g1.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g0[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g1.zyz().with_w(1.0))
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g4[0]),
            // e41, e42, e43
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[1])) + (anti_reverse_g0.zxy() * geometric_anti_product_g2.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g1 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (anti_reverse_g0.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g1.zxy() * geometric_anti_product_g2.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g2.zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from([
                geometric_anti_product_g1[3],
                geometric_anti_product_g4[2],
                geometric_anti_product_g4[0],
                anti_reverse_g1[0] * geometric_anti_product_g4[0],
            ]) * anti_reverse_g0.xxy().with_w(1.0))
                + (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g1[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g0.zyz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g4[2]) - (anti_reverse_g0[1] * geometric_anti_product_g1[1]) - (anti_reverse_g0[2] * geometric_anti_product_g1[2]),
                )
                - (anti_reverse_g0.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1[0]),
        )
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       10        0
    //    simd3        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       11       22        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4]) * self.group1();
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_xyz[1] * self[e43]) + (geometric_anti_product_g1_xyz[2] * self[e31]),
                (geometric_anti_product_g0_xyz[2] * self[e41]) + (geometric_anti_product_g1_xyz[0] * self[e12]),
                (geometric_anti_product_g0_xyz[0] * self[e42]) + (geometric_anti_product_g1_xyz[1] * self[e23]),
                geometric_anti_product_g1_xyz[2] * self[e43] * -1.0,
            ]) - (geometric_anti_product_g0_xyz.zxy() * self.group0().yzx()).with_w(geometric_anti_product_g1_xyz[0] * self[e41])
                - (geometric_anti_product_g1_xyz.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g1_xyz[1] * self[e42]),
        )
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       27       36        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e41] * other[e321]) + (self[e31] * other[e412]),
            (self[e42] * other[e321]) + (self[e12] * other[e423]),
            (self[e43] * other[e321]) + (self[e23] * other[e431]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            self[e42] * other[e412],
            self[e43] * other[e423],
            self[e41] * other[e431],
            -(self[e31] * other[e431]) - (self[e12] * other[e412]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_anti_product_g1.yzxy() * self.group0().zxy().with_w(self[e31]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e12]) - (geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(geometric_anti_product_g1[0] * self[e23])
                - (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(geometric_anti_product_g0[0] * self[e41]),
        )
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        7        0
    //    simd4        5        0        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       36        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (self.group0().zxy() * other.group0().yzx()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        let geometric_anti_product_g1_w = -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g1_w * self[e41]) + (geometric_anti_product_g1_xyz[2] * self[e31]) + (geometric_anti_product_g0[1] * self[e43]),
                (geometric_anti_product_g1_w * self[e42]) + (geometric_anti_product_g1_xyz[0] * self[e12]) + (geometric_anti_product_g0[2] * self[e41]),
                (geometric_anti_product_g1_w * self[e43]) + (geometric_anti_product_g1_xyz[1] * self[e23]) + (geometric_anti_product_g0[0] * self[e42]),
                0.0,
            ]) - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1()).with_w(geometric_anti_product_g1_xyz[2] * self[e43])
                - (geometric_anti_product_g1_xyz.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g1_xyz[0] * self[e41])
                - (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(geometric_anti_product_g1_xyz[1] * self[e42]),
        )
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(other[scalar]) * self.group0();
        Scalar::from_groups(
            // scalar
            -(geometric_anti_product_g1[0] * self[e41]) - (geometric_anti_product_g1[1] * self[e42]) - (geometric_anti_product_g1[2] * self[e43]),
        )
    }
}
impl std::ops::Div<AntiSandwichInfix> for Motor {
    type Output = AntiSandwichInfixPartial<Motor>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd       11       20        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e1234])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       11        0
    //  no simd       15       24        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1());
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e1234])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        2        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       84       97        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42]),
            (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43]),
            other[e412] * self[e43] * -1.0,
        ]) + (other.group0().xxyw() * self.group0().wzxw())
            - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxyx() * other.group0().yzx().with_w(other[e423]));
        let geometric_anti_product_g1 = Simd32x4::from([
            other[e412] * self[e42],
            other[e431] * self[e1234],
            other[e412] * self[e1234],
            -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        ]) + (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]))
            + (other.group1().xxyw() * self.group0().wzxw())
            - (self.group0().zxyx() * other.group1().yzx().with_w(other[e1]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e43]) + (geometric_anti_product_g1[2] * self[e31]) + (geometric_anti_product_g1[3] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e12]),
                (geometric_anti_product_g0[2] * self[e41]) + (geometric_anti_product_g1[0] * self[e12]) + (geometric_anti_product_g1[3] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e23]),
                (geometric_anti_product_g0[2] * self[e1234]) + (geometric_anti_product_g1[1] * self[e23]) + (geometric_anti_product_g1[3] * self[e43])
                    - (geometric_anti_product_g1[2] * self[scalar]),
                0.0,
            ]) + (geometric_anti_product_g0.xyxw() * self.group0().wwyw())
                - (geometric_anti_product_g1.xyxz() * self.group1().wwy().with_w(self[e43]))
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(geometric_anti_product_g1[1] * self[e42]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1.xyxy() * self.group0().wwy().with_w(self[e31]))
                + (geometric_anti_product_g1.yzzz() * self.group0().zxw().with_w(self[e12]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g1[3] * self[e1234])
                        - (geometric_anti_product_g0[1] * self[e42])
                        - (geometric_anti_product_g0[2] * self[e43])
                        - (geometric_anti_product_g0[3] * self[scalar]),
                )
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(geometric_anti_product_g1[0] * self[e23])
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0().xyz();
        Horizon::from_groups(
            // e321
            (other[e321] * self[e1234] * self[e1234])
                - (geometric_anti_product_g0_xyz[0] * self[e41])
                - (geometric_anti_product_g0_xyz[1] * self[e42])
                - (geometric_anti_product_g0_xyz[2] * self[e43]),
        )
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3       10       13        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       58       72        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e41] * self[e1234]) + (other[e43] * self[e42]),
            (other[e41] * self[e43]) + (other[e42] * self[e1234]),
            (other[e42] * self[e41]) + (other[e43] * self[e1234]),
            -(other[e42] * self[e42]) - (other[e43] * self[e43]),
        ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e41] * self[scalar]) + (other[e43] * self[e31]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]),
            (other[e41] * self[e12]) + (other[e42] * self[scalar]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]),
            (other[e42] * self[e23]) + (other[e43] * self[scalar]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) - (self.group1().zxyx() * other.group0().yzx().with_w(other[e41]))
            - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + (geometric_anti_product_g0.xyx() * self.group0().wwy())
                + (geometric_anti_product_g0.yzz() * self.group0().zxw())
                - (geometric_anti_product_g0.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                + (geometric_anti_product_g0.xyx() * self.group1().wwy())
                + (geometric_anti_product_g0.yzz() * self.group1().zxw())
                + (geometric_anti_product_g1.xyx() * self.group0().wwy())
                + (geometric_anti_product_g1.yzz() * self.group0().zxw())
                - (geometric_anti_product_g0.zxy() * self.group1().yzx())
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()),
        )
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        4        0
    //    simd4       12        8        0
    // Totals...
    // yes simd       44       64        0
    //  no simd       80       96        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e43] * self[e42]) + (other[e1234] * self[e41]),
            (other[e42] * self[e1234]) + (other[e1234] * self[e42]),
            (other[e43] * self[e1234]) + (other[e1234] * self[e43]),
            -(other[e42] * self[e42]) - (other[e43] * self[e43]),
        ]) + (other.group0().xxyw() * self.group0().wzxw())
            - (other.group0().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]) + (other[scalar] * self[e41]),
            (other[e1234] * self[e31]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]) + (other[scalar] * self[e42]),
            (other[e1234] * self[e12]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) + (other.group0().xxyw() * self.group1().wzxw())
            + (other.group0().zyz() * self.group1().yww()).with_w(other[scalar] * self[e1234])
            - (other.group0().yzxx() * self.group1().zxyx())
            - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e43]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[2] * self[e41]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[2] * self[e1234]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) + (geometric_anti_product_g0.xyxw() * self.group0().wwyw())
                - (geometric_anti_product_g0.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[3] * self[e23])
                    + (geometric_anti_product_g1[0] * self[e1234])
                    + (geometric_anti_product_g1[1] * self[e43])
                    + (geometric_anti_product_g1[3] * self[e41]),
                (geometric_anti_product_g0[3] * self[e31])
                    + (geometric_anti_product_g1[1] * self[e1234])
                    + (geometric_anti_product_g1[2] * self[e41])
                    + (geometric_anti_product_g1[3] * self[e42]),
                (geometric_anti_product_g0[3] * self[e12])
                    + (geometric_anti_product_g1[0] * self[e42])
                    + (geometric_anti_product_g1[2] * self[e1234])
                    + (geometric_anti_product_g1[3] * self[e43]),
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) + (geometric_anti_product_g0.xyxw() * self.group1().wwyw())
                + (geometric_anti_product_g0.yzz() * self.group1().zxw()).with_w(geometric_anti_product_g1[3] * self[e1234])
                - (geometric_anti_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl AntiSandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       55        0
    //    simd2        8       11        0
    //    simd3       20       23        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       76      101        0
    //  no simd      166      194        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
            + (Simd32x2::from(self[e1234]) * other.group0())
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e1234] * other[e1]) + (self[e23] * other[e4]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]),
            (self[e43] * other[e1]) + (self[e1234] * other[e2]) + (self[e31] * other[e4]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]),
            (self[e43] * other[e321]) + (self[e1234] * other[e3]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]),
            self[e43] * other[e412] * -1.0,
        ]) + (self.group0().xyxw() * other.group4().ww().with_zw(other[e2], other[e4]))
            - (self.group0().zxyx() * other.group1().yzx().with_w(other[e423]))
            - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]));
        let geometric_anti_product_g2 = (Simd32x3::from(self[e1234]) * other.group2())
            + (self.group0().yzz() * other.group2().zx().with_z(other[e1234]))
            + (Simd32x2::from(other[e1234]) * self.group0().xy()).with_z(self[e41] * other[e42])
            - (other.group2().yzx() * self.group0().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (self.group0().yzz() * other.group3().zx().with_z(other[scalar]))
            + (self.group1().yzz() * other.group2().zx().with_z(other[e1234]))
            + (Simd32x2::from(other[scalar]) * self.group0().xy()).with_z(self[e41] * other[e31])
            + (Simd32x2::from(other[e1234]) * self.group1().xy()).with_z(self[e23] * other[e42])
            - (other.group2().yzx() * self.group1().zxy())
            - (other.group3().yzx() * self.group0().zxy());
        let geometric_anti_product_g4 = Simd32x4::from([
            self[e1234] * other[e423],
            self[e1234] * other[e431],
            self[e1234] * other[e412],
            -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
        ]) + (self.group0().xyxw() * other.group1().ww().with_zw(other[e431], other[e321]))
            + (other.group4().zx().with_zw(other[e4], self[scalar] * other[e4]) * self.group0().yzz().with_w(1.0))
            - (self.group0().zxyx() * other.group4().yzx().with_w(other[e1]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g1[1] * self[e43]) + (geometric_anti_product_g4[2] * self[e31]) + (geometric_anti_product_g4[3] * self[e41])
                    - (geometric_anti_product_g4[1] * self[e12]),
                (geometric_anti_product_g1[2] * self[e41]) + (geometric_anti_product_g4[0] * self[e12]) + (geometric_anti_product_g4[3] * self[e42])
                    - (geometric_anti_product_g4[2] * self[e23]),
                (geometric_anti_product_g1[2] * self[e1234]) + (geometric_anti_product_g4[1] * self[e23]) + (geometric_anti_product_g4[3] * self[e43])
                    - (geometric_anti_product_g4[2] * self[scalar]),
                0.0,
            ]) + (geometric_anti_product_g1.xyxw() * self.group0().wwyw())
                - (geometric_anti_product_g4.xyxz() * self.group1().wwy().with_w(self[e43]))
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g4[0]))
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(geometric_anti_product_g4[1] * self[e42]),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (geometric_anti_product_g2.xyx() * self.group0().wwy())
                + (geometric_anti_product_g2.yzz() * self.group0().zxw())
                - (geometric_anti_product_g2.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0().xyz())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz())
                + (geometric_anti_product_g2.xyx() * self.group1().wwy())
                + (geometric_anti_product_g2.yzz() * self.group1().zxw())
                + (geometric_anti_product_g3.xyx() * self.group0().wwy())
                + (geometric_anti_product_g3.yzz() * self.group0().zxw())
                - (geometric_anti_product_g2.zxy() * self.group1().yzx())
                - (geometric_anti_product_g3.zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4.xyxy() * self.group0().wwy().with_w(self[e31]))
                + (geometric_anti_product_g4.yzzz() * self.group0().zxw().with_w(self[e12]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g4[3] * self[e1234])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(geometric_anti_product_g4[0] * self[e23])
                - (self.group0().yzxx() * geometric_anti_product_g4.zxy().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       19        0
    //  no simd       25       36        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e1234]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[1] * self[e43]) + (geometric_anti_product_g1[2] * self[e31]) + (geometric_anti_product_g1[3] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e12]),
                (geometric_anti_product_g0[2] * self[e41]) + (geometric_anti_product_g1[0] * self[e12]) + (geometric_anti_product_g1[3] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e23]),
                (geometric_anti_product_g0[2] * self[e1234]) + (geometric_anti_product_g1[1] * self[e23]) + (geometric_anti_product_g1[3] * self[e43])
                    - (geometric_anti_product_g1[2] * self[scalar]),
                0.0,
            ]) + (geometric_anti_product_g0.xyxw() * self.group0().wwyw())
                - (geometric_anti_product_g1.xyxz() * self.group1().wwy().with_w(self[e43]))
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(geometric_anti_product_g1[1] * self[e42]),
        )
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        0        1        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       39       48        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e41] * other[e321]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]),
            (self[e42] * other[e321]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]),
            (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[scalar] * other[e412]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            self[e1234] * other[e423],
            self[e1234] * other[e431],
            self[e1234] * other[e412],
            -(self[e31] * other[e431]) - (self[e12] * other[e412]),
        ]) + (self.group0().yzxw() * other.group0().zxyw())
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_anti_product_g1.xyxy() * self.group0().wwy().with_w(self[e31]))
                + (geometric_anti_product_g1.yzzz() * self.group0().zxw().with_w(self[e12]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g1[3] * self[e1234])
                        - (geometric_anti_product_g0[1] * self[e42])
                        - (geometric_anti_product_g0[2] * self[e43])
                        - (geometric_anti_product_g0[3] * self[scalar]),
                )
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(geometric_anti_product_g1[0] * self[e23])
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd2        0        1        0
    //    simd3        3        6        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       37       48        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz =
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy())
                - (self.group0().zxy() * other.group0().yzx());
        let geometric_anti_product_g0_w = self[e1234] * other[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0().xyz();
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_xyz[0] * self[e1234]) + (geometric_anti_product_g0_xyz[1] * self[e43]) + (geometric_anti_product_g1_xyz[2] * self[e31])
                    - (geometric_anti_product_g1_xyz[1] * self[e12]),
                (geometric_anti_product_g0_xyz[1] * self[e1234]) + (geometric_anti_product_g0_xyz[2] * self[e41]) + (geometric_anti_product_g1_xyz[0] * self[e12])
                    - (geometric_anti_product_g1_xyz[2] * self[e23]),
                (geometric_anti_product_g0_xyz[0] * self[e42]) + (geometric_anti_product_g0_xyz[2] * self[e1234]) + (geometric_anti_product_g1_xyz[1] * self[e23])
                    - (geometric_anti_product_g1_xyz[2] * self[scalar]),
                0.0,
            ]) + (self.group0()
                * Simd32x3::from((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])).with_w(geometric_anti_product_g0_w))
                - (self.group0().yzxy() * geometric_anti_product_g0_xyz.zxy().with_w(geometric_anti_product_g1_xyz[1]))
                - (Simd32x2::from(geometric_anti_product_g0_w) * self.group1().xy()).with_zw(geometric_anti_product_g0_w * self[e12], geometric_anti_product_g1_xyz[0] * self[e41])
                - (geometric_anti_product_g1_xyz.xyx() * self.group1().wwy()).with_w(geometric_anti_product_g1_xyz[2] * self[e43]),
        )
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x4::from(other[scalar]) * self.group0();
        Scalar::from_groups(
            // scalar
            (geometric_anti_product_g1[3] * self[e1234])
                - (geometric_anti_product_g1[0] * self[e41])
                - (geometric_anti_product_g1[1] * self[e42])
                - (geometric_anti_product_g1[2] * self[e43]),
        )
    }
}
impl std::ops::Div<AntiSandwichInfix> for MultiVector {
    type Output = AntiSandwichInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd2        8        9        0
    //    simd3        0        5        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       30       43        0
    //  no simd       62       83        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other[e1234]) * self.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other[e1234]) * self.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other[e1234]) * self.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4.xyxz() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e12]))
                + (geometric_anti_product_g4.yzzw() * self.group2().zx().with_zw(self[e1234], self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], geometric_anti_product_g0[1] * self[e321]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], geometric_anti_product_g4[0] * self[e23]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g1[0] * self[e41])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group2()).with_w(geometric_anti_product_g4[1] * self[e31])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(geometric_anti_product_g2[0] * self[e1])
                - (self.group2().yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g2[1] * self[e2]),
        )
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd2        8        8        0
    //    simd3        2        9        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       34       52        0
    //  no simd       70       93        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]);
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(other[scalar]) * self.group4().xyz());
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(other[e1234]) * self.group2();
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3());
        let geometric_anti_product_g4_xyz = Simd32x3::from(other[e1234]) * self.group4().xyz();
        let geometric_anti_product_g4_w = (other[e1234] * self[e321]) - (other[scalar] * self[e4]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g4_xyz[0] * self[e1])
                    - (geometric_anti_product_g4_xyz[1] * self[e2])
                    - (geometric_anti_product_g4_xyz[2] * self[e3]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from([geometric_anti_product_g1_w, geometric_anti_product_g4_xyz[0]]) * self.group4().wx())
                + (Simd32x2::from([geometric_anti_product_g1_xyz[0], geometric_anti_product_g4_xyz[1]]) * self.group4().xy())
                + (Simd32x2::from([geometric_anti_product_g1_xyz[1], geometric_anti_product_g4_xyz[2]]) * self.group4().yz())
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from(self[e4]) * Simd32x2::from([geometric_anti_product_g4_w, geometric_anti_product_g1_w])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(self[e1234]).with_zw(self[e42], geometric_anti_product_g4_xyz[1] * self[e31]) * geometric_anti_product_g4_xyz.xyx().with_w(1.0))
                + (self.group2().zx().with_zw(self[e1234], geometric_anti_product_g4_xyz[2] * self[e12]) * geometric_anti_product_g4_xyz.yzz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], geometric_anti_product_g0[1] * self[e321]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], geometric_anti_product_g4_xyz[0] * self[e23]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g1_xyz[1] * self[e42])
                        - (geometric_anti_product_g1_xyz[2] * self[e43])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (Simd32x3::from(geometric_anti_product_g1_w) * self.group2()).with_w(geometric_anti_product_g4_w * self[e1234])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(geometric_anti_product_g1_w * self[scalar])
                - (geometric_anti_product_g4_xyz.zxy() * self.group2().yzx()).with_w(geometric_anti_product_g1_xyz[0] * self[e41]),
        )
    }
}
impl AntiSandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       86        0
    //    simd2       12       13        0
    //    simd3       32       44        0
    //    simd4       22       15        0
    // Totals...
    // yes simd      120      158        0
    //  no simd      262      304        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from([self[e423], self[e4]]) * other.group0().xw());
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
            (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42]),
            (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43]),
            other[e412] * self[e43] * -1.0,
        ]) + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e1234]]) * other.group0().xxyw())
            - (other.group1().yzxy() * self.group3().zxy().with_w(self[e42]))
            - (self.group2().zxy() * other.group0().yzx()).with_w(other[e423] * self[e41]);
        let geometric_anti_product_g2 = (other.group1().yzx() * self.group4().zxy())
            - (Simd32x3::from(other[e4]) * self.group4().xyz())
            - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
            - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz());
        let geometric_anti_product_g3 = (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group0().xxy())
            + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group0().zyz())
            + (other.group1().yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group1().zyz())
            - (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group1().xxy())
            - (other.group0().yzx() * self.group4().zxy());
        let geometric_anti_product_g4 = Simd32x4::from([
            other[e412] * self[e42],
            other[e431] * self[e1234],
            other[e412] * self[e1234],
            -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        ]) + (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
            + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e1234]]) * other.group1().xxyw())
            - (self.group2().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412])
                    + (geometric_anti_product_g1[0] * self[e1234]),
                (anti_reverse_g2[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (anti_reverse_g2[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g1[3] * self[e1234])
                    * self.group1().xyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    anti_reverse_g2[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g2[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    geometric_anti_product_g0[1] * self[e321],
                ]) * anti_reverse_g2.xxy().with_w(1.0))
                + (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g3[0] * geometric_anti_product_g4[0],
                ]) * anti_reverse_g2.zyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], anti_reverse_g3[1] * geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], anti_reverse_g3[2] * geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[1] * geometric_anti_product_g1[1]),
        )
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       14       24        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = other[e321] * self[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e321]) * self.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other[e321]) * self.group4().xyz();
        let geometric_anti_product_g4_w = other[e321] * self[e1234];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (geometric_anti_product_g4_w * self[e4])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(
                (geometric_anti_product_g0_x * self[e4]) + (geometric_anti_product_g4_w * self[e1234])
                    - (geometric_anti_product_g1_xyz[0] * self[e41])
                    - (geometric_anti_product_g1_xyz[1] * self[e42])
                    - (geometric_anti_product_g1_xyz[2] * self[e43])
                    - (geometric_anti_product_g3[0] * self[e423])
                    - (geometric_anti_product_g3[1] * self[e431])
                    - (geometric_anti_product_g3[2] * self[e412]),
            ),
        )
    }
}
impl AntiSandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       53        0
    //    simd2        3        4        0
    //    simd3       29       39        0
    //    simd4       14        9        0
    // Totals...
    // yes simd       77      105        0
    //  no simd      180      214        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412]),
            (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431]),
            0.0,
        ]) - (Simd32x4::from([self[e4], self[e412], self[e423], other[e42] * self[e431]]) * other.group1().xxy().with_w(1.0))
            - (Simd32x4::from([self[e431], self[e4], self[e4], other[e43] * self[e412]]) * other.group1().zyz().with_w(1.0))
            - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e423]);
        let geometric_anti_product_g2 = (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group0().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group0().zyz())
            - (other.group0().yzx() * self.group2().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from([self[scalar], self[e12], self[e23]]) * other.group0().xxy())
            + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group1().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group1().zyz())
            + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * other.group0().zyz())
            - (other.group0().yzx() * self.group3().zxy())
            - (other.group1().yzx() * self.group2().zxy());
        let geometric_anti_product_g4 = (Simd32x4::from([self[e4], self[e412], self[e423], other[e23] * self[e423]]) * other.group0().xxy().with_w(1.0))
            + (Simd32x4::from([self[e431], self[e4], self[e4], other[e31] * self[e431]]) * other.group0().zyz().with_w(1.0))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
            - (other.group0().yzx() * self.group4().zxy()).with_w(other[e41] * self[e1]);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412])
                    + (geometric_anti_product_g1[0] * self[e1234]),
                (anti_reverse_g2[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (anti_reverse_g2[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g1[3] * self[e1234])
                    * self.group1().xyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    anti_reverse_g2[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g2[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiSandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       78        0
    //    simd2       12       13        0
    //    simd3       32       42        0
    //    simd4       24       19        0
    // Totals...
    // yes simd      118      152        0
    //  no simd      266      306        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
            + (Simd32x2::from(other[e1234]) * self.group0())
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e43] * self[e2]) + (other[e1234] * self[e1]) + (other[e31] * self[e412]) - (other[scalar] * self[e423]),
            (other[e42] * self[e321]) + (other[e1234] * self[e2]) + (other[e12] * self[e423]) - (other[scalar] * self[e431]),
            (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) - (other[scalar] * self[e412]),
            0.0,
        ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * other.group0().xxyw())
            - (Simd32x4::from([self[e4], self[e412], self[e423], other[e42] * self[e431]]) * other.group1().xxy().with_w(1.0))
            - (Simd32x4::from([self[e431], self[e4], self[e4], other[e43] * self[e412]]) * other.group1().zyz().with_w(1.0))
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e423]));
        let geometric_anti_product_g2 = (Simd32x3::from(other[e1234]) * self.group2())
            + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group0().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group0().zyz())
            - (self.group2().zxy() * other.group0().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * other.group0().xxy())
            + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group1().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group1().zyz())
            + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * other.group0().zyz())
            - (self.group2().zxy() * other.group1().yzx())
            - (self.group3().zxy() * other.group0().yzx());
        let geometric_anti_product_g4 = (Simd32x4::from([other[e1234], other[e1234], other[e1234], other[e31]]) * self.group4().xyzy())
            + (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * other.group0().xxyw())
            + (Simd32x4::from([self[e431], self[e4], self[e4], other[e23] * self[e423]]) * other.group0().zyz().with_w(1.0))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
            - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1]));
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412])
                    + (geometric_anti_product_g1[0] * self[e1234]),
                (anti_reverse_g2[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (anti_reverse_g2[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g1[3] * self[e1234])
                    * self.group1().xyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    anti_reverse_g2[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g2[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    geometric_anti_product_g0[1] * self[e321],
                ]) * anti_reverse_g2.xxy().with_w(1.0))
                + (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g3[0] * geometric_anti_product_g4[0],
                ]) * anti_reverse_g2.zyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], anti_reverse_g3[1] * geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], anti_reverse_g3[2] * geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[1] * geometric_anti_product_g1[1]),
        )
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70      104        0
    //    simd2       16       17        0
    //    simd3       44       58        0
    //    simd4       32       23        0
    // Totals...
    // yes simd      162      202        0
    //  no simd      362      404        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([
            (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                - (other[e2] * self[e431])
                - (other[e3] * self[e412])
                - (other[e4] * self[e321])
                - (other[e23] * self[e41])
                - (other[e31] * self[e42])
                - (other[e12] * self[e43]),
            0.0,
        ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            + (Simd32x2::from(self[e1234]) * other.group0())
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]))
            - (Simd32x2::from([self[e423], self[e4]]) * other.group1().xw());
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e3] * self[e42])
                + (other[e4] * self[e23])
                + (other[e41] * self[e321])
                + (other[e43] * self[e2])
                + (other[e31] * self[e412])
                + (other[e423] * self[scalar])
                + (other[e412] * self[e31])
                + (other[e321] * self[e41]),
            (other[e2] * self[e1234])
                + (other[e4] * self[e31])
                + (other[e41] * self[e3])
                + (other[e42] * self[e321])
                + (other[e12] * self[e423])
                + (other[e423] * self[e12])
                + (other[e431] * self[scalar])
                + (other[e321] * self[e42]),
            (other[e3] * self[e1234])
                + (other[e4] * self[e12])
                + (other[e42] * self[e1])
                + (other[e43] * self[e321])
                + (other[e23] * self[e431])
                + (other[e431] * self[e23])
                + (other[e412] * self[scalar])
                + (other[e321] * self[e43]),
            0.0,
        ]) + (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from([self[e1234], self[e43], self[e41], self[e1234]]) * other.group1().xxyw())
            - (Simd32x4::from([self[e4], self[e412], self[e423], other[e423] * self[e41]]) * other.group3().xxy().with_w(1.0))
            - (Simd32x4::from([self[e431], self[e4], self[e4], other[e431] * self[e42]]) * other.group3().zyz().with_w(1.0))
            - (other.group4().yzxz() * self.group3().zxy().with_w(self[e43]))
            - (self.group4().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
            - (other.group2().yzx() * self.group1().zxy()).with_w(other[e43] * self[e412])
            - (self.group2().zxy() * other.group1().yzx()).with_w(other[e42] * self[e431]);
        let geometric_anti_product_g2 = (Simd32x3::from(other[e1234]) * self.group2())
            + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group2().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group2().zyz())
            + (other.group4().yzx() * self.group4().zxy())
            - (Simd32x3::from(other[e4]) * self.group4().xyz())
            - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group4().xxy())
            - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group4().zyz())
            - (other.group2().yzx() * self.group2().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * other.group2().xxy())
            + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * other.group3().xxy())
            + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
            + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * other.group3().zyz())
            + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * other.group2().zyz())
            + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz())
            + (other.group4().yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group4().zyz())
            - (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group4().xxy())
            - (other.group2().yzx() * self.group3().zxy())
            - (other.group3().yzx() * self.group2().zxy())
            - (other.group1().yzx() * self.group4().zxy());
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group4())
            + (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
            + (Simd32x4::from([self[e1234], self[e43], self[e41], other[e12] * self[e412]]) * other.group4().xxy().with_w(1.0))
            + (Simd32x4::from([self[e4], self[e412], self[e423], other[e23] * self[e423]]) * other.group2().xxy().with_w(1.0))
            + (Simd32x4::from([self[e42], self[e1234], self[e1234], self[e1234]]) * other.group4().zyzw())
            + (Simd32x4::from([self[e431], self[e4], self[e4], other[e31] * self[e431]]) * other.group2().zyz().with_w(1.0))
            + Simd32x3::from(0.0).with_w(
                -(other[e2] * self[e42])
                    - (other[e3] * self[e43])
                    - (other[e41] * self[e1])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            )
            - (other.group2().yzx() * self.group4().zxy()).with_w(other[scalar] * self[e4])
            - (self.group2().zxy() * other.group4().yzx()).with_w(other[e1] * self[e41]);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412])
                    + (geometric_anti_product_g1[0] * self[e1234]),
                (anti_reverse_g2[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (anti_reverse_g2[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g1[3] * self[e1234])
                    * self.group1().xyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    anti_reverse_g2[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g2[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    geometric_anti_product_g0[1] * self[e321],
                ]) * anti_reverse_g2.xxy().with_w(1.0))
                + (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g3[0] * geometric_anti_product_g4[0],
                ]) * anti_reverse_g2.zyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], anti_reverse_g3[1] * geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], anti_reverse_g3[2] * geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[1] * geometric_anti_product_g1[1]),
        )
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd2        8       10        0
    //    simd3       22       30        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       66       90        0
    //  no simd      142      181        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e4] * -1.0) * Simd32x2::from([self[e321], self[e4]]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group3().with_w(self[e1234]);
        let geometric_anti_product_g2 = Simd32x3::from(other[e4] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = Simd32x3::from(other[e4] * -1.0) * self.group1().xyz();
        let geometric_anti_product_g4 = Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412])
                    + (geometric_anti_product_g1[0] * self[e1234]),
                (anti_reverse_g2[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (anti_reverse_g2[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g1[3] * self[e1234])
                    * self.group1().xyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    anti_reverse_g2[1] * geometric_anti_product_g4[1],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    anti_reverse_g2[2] * geometric_anti_product_g4[2],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd2       11       11        0
    //    simd3        5       10        0
    //    simd4       11        8        0
    // Totals...
    // yes simd       49       68        0
    //  no simd      103      123        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([self[e4] * other[e321], 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[scalar] * other[e423]) + (self[e41] * other[e321]) + (self[e31] * other[e412]),
            (self[scalar] * other[e431]) + (self[e42] * other[e321]) + (self[e12] * other[e423]),
            (self[scalar] * other[e412]) + (self[e43] * other[e321]) + (self[e23] * other[e431]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41]));
        let geometric_anti_product_g2 =
            (self.group4().zxy() * other.group0().yzx()) - (Simd32x3::from(self[e4]) * other.group0().xyz()) - (self.group4().yzx() * other.group0().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().zxy() * other.group0().yzx())
            - (Simd32x3::from(self[e321]) * other.group0().xyz())
            - (self.group1().yzx() * other.group0().zxy());
        let geometric_anti_product_g4 = Simd32x4::from([
            self[e42] * other[e412],
            self[e43] * other[e423],
            self[e41] * other[e431],
            -(self[e31] * other[e431]) - (self[e12] * other[e412]),
        ]) + (Simd32x4::from(self[e1234]) * other.group0())
            - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4.xyxz() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e12]))
                + (geometric_anti_product_g4.yzzw() * self.group2().zx().with_zw(self[e1234], self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g0[1]).with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0] * self[e4]) * self.group4().xyz().with_w(1.0))
                + (self.group1().ww().with_zw(self[e431], geometric_anti_product_g0[1] * self[e321]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], geometric_anti_product_g4[0] * self[e23]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g1[0] * self[e41])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g1[3] * self[scalar]),
                )
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group2()).with_w(geometric_anti_product_g4[1] * self[e31])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(geometric_anti_product_g2[0] * self[e1])
                - (self.group2().yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g2[1] * self[e2]),
        )
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       62        0
    //    simd2        8       10        0
    //    simd3       28       42        0
    //    simd4       16        9        0
    // Totals...
    // yes simd       93      123        0
    //  no simd      205      244        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            self[e4] * other[e4],
        ]) * Simd32x2::from([1.0, -1.0]);
        let geometric_anti_product_g1_xyz =
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx());
        let geometric_anti_product_g1_w = self[e1234] * other[e4];
        let geometric_anti_product_g2 = Simd32x3::from(other[e4] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().yzx() * other.group0().zxy())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group4().zxy() * other.group0().yzx());
        let geometric_anti_product_g4 =
            (Simd32x3::from(other[e4]) * self.group2()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]));
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (geometric_anti_product_g4[1] * self[e2])
                    - (geometric_anti_product_g4[2] * self[e3])
                    - (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from([geometric_anti_product_g1_w, geometric_anti_product_g4[0]]) * self.group4().wx())
                + (Simd32x2::from([geometric_anti_product_g1_xyz[0], geometric_anti_product_g4[1]]) * self.group4().xy())
                + (Simd32x2::from([geometric_anti_product_g1_xyz[1], geometric_anti_product_g4[2]]) * self.group4().yz())
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1_w]) * self.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (anti_reverse_g2[0] * geometric_anti_product_g4[3])
                    + (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[1])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g1_xyz[0] * self[e1234])
                    + (geometric_anti_product_g2[0] * self[e321])
                    + (geometric_anti_product_g2[1] * self[e3])
                    + (geometric_anti_product_g3[0] * self[e4])
                    + (geometric_anti_product_g3[1] * self[e412]),
                (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[2])
                    + (anti_reverse_g2[1] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[0])
                    + (geometric_anti_product_g1_xyz[1] * self[e1234])
                    + (geometric_anti_product_g2[1] * self[e321])
                    + (geometric_anti_product_g2[2] * self[e1])
                    + (geometric_anti_product_g3[1] * self[e4])
                    + (geometric_anti_product_g3[2] * self[e423]),
                (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[0])
                    + (anti_reverse_g2[2] * geometric_anti_product_g4[3])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[1])
                    + (geometric_anti_product_g1_xyz[2] * self[e1234])
                    + (geometric_anti_product_g2[0] * self[e2])
                    + (geometric_anti_product_g2[2] * self[e321])
                    + (geometric_anti_product_g3[0] * self[e431])
                    + (geometric_anti_product_g3[2] * self[e4]),
                0.0,
            ]) + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g1_w * self[e1234]) * self.group4().xyz().with_w(1.0))
                - (geometric_anti_product_g4.yzxz() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[2]))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (Simd32x2::from(self[scalar]) * geometric_anti_product_g4.xy()).with_zw(geometric_anti_product_g4[2] * self[scalar], geometric_anti_product_g2[2] * self[e412])
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1_w)).with_w(anti_reverse_g2[0] * geometric_anti_product_g4[0])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g1_w) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g4.yzz() * self.group4().zx().with_z(self[e4])),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1_w) * self.group1().xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1_xyz.zxy() * self.group4().yzx())
                + (geometric_anti_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_anti_product_g4.yzz() * self.group1().zx().with_z(self[e321]))
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g1_xyz.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1_xyz.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_anti_product_g4.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (geometric_anti_product_g4.yzxx() * anti_reverse_g2.zxy().with_w(anti_reverse_g3[0]))
                + (self.group1().ww().with_zw(self[e431], anti_reverse_g3[1] * geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(1.0))
                + (self.group4().zx().with_zw(self[e4], anti_reverse_g3[2] * geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g1_w)).with_w(geometric_anti_product_g0[0] * self[e4])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1_w * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0]),
        )
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       20        0
    //    simd3        7       19        0
    //    simd4        7        0        0
    // Totals...
    // yes simd       28       39        0
    //  no simd       63       77        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = self[e1234] * other[scalar];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = Simd32x3::from(other[scalar]) * self.group2();
        let geometric_anti_product_g4_w = self[e4] * other[scalar] * -1.0;
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (geometric_anti_product_g4_w * self[e4])
                    - (anti_reverse_g2[0] * geometric_anti_product_g3[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g3[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g3[2]),
                0.0,
            ]),
            // e1, e2, e3, e4
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g4_w)).with_w(0.0)
                + (geometric_anti_product_g1_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g0_x) * self.group4().xyz()).with_w(0.0)
                + (anti_reverse_g2.zxy() * geometric_anti_product_g1_xyz.yzx()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(self[e4])).with_w(0.0)
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(0.0)
                - (geometric_anti_product_g3.zxy() * self.group4().yzx()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0_x))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (geometric_anti_product_g1_xyz.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g4_w) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (geometric_anti_product_g1_xyz.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                - (geometric_anti_product_g1_xyz.yzz() * self.group4().zx().with_z(self[e4])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(
                (geometric_anti_product_g0_x * self[e4]) + (geometric_anti_product_g4_w * self[e1234])
                    - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                    - (geometric_anti_product_g3[0] * self[e423])
                    - (geometric_anti_product_g3[1] * self[e431])
                    - (geometric_anti_product_g3[2] * self[e412]),
            ),
        )
    }
}
impl std::ops::Div<AntiSandwichInfix> for Origin {
    type Output = AntiSandwichInfixPartial<Origin>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Origin {
    type Output = AntiScalar;
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e4] * self[e4])
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        5        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([other[scalar] * self[e4] * -1.0, other[e1234] * self[e4]]),
        )
    }
}
impl AntiSandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       18        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]);
        let anti_reverse_g0 = self[e4] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]),
        )
    }
}
impl AntiSandwich<Horizon> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[e4] * self[e4] * -1.0)
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self[e4] * -1.0;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_reverse_g0 * self[e4] * -1.0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_reverse_g0 * self[e4]) * other.group1(),
        )
    }
}
impl AntiSandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       26        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4]) * (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let anti_reverse_g0 = self[e4] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]),
        )
    }
}
impl AntiSandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        3        0
    //    simd3        0        4        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       50        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(self[e4]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from([1.0, -1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * (other.group3() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let geometric_anti_product_g4 = Simd32x4::from(self[e4]) * other.group2().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let anti_reverse_g0 = self[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_reverse_g0 * -1.0) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_reverse_g0) * Simd32x4::from([self[e4], self[e4], self[e4], geometric_anti_product_g0[1]]) * other.group1().xyz().with_w(1.0),
            // e41, e42, e43
            Simd32x3::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_reverse_g0)
                * Simd32x4::from([self[e4], self[e4], self[e4], geometric_anti_product_g0[0]])
                * (other.group4().xyz() * Simd32x3::from(-1.0)).with_w(1.0),
        )
    }
}
impl AntiSandwich<Origin> for Origin {
    type Output = Origin;
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * self[e4] * self[e4])
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4] * -1.0) * other.group0().xyz()).with_w(self[e4] * other[e321]),
        )
    }
}
impl AntiSandwich<Point> for Origin {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(self[e4] * other[e4] * -1.0),
        )
    }
}
impl AntiSandwich<Scalar> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e4] * self[e4] * other[scalar] * -1.0)
    }
}
impl std::ops::Div<AntiSandwichInfix> for Plane {
    type Output = AntiSandwichInfixPartial<Plane>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        AntiScalar::from_groups(
            // e1234
            (geometric_anti_product_g0[0] * self[e423]) + (geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]),
        )
    }
}
impl AntiSandwich<DualNum> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       14        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group0().xyz();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group0();
        DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0_xyz[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0_xyz[2], geometric_anti_product_g1[2]])),
        )
    }
}
impl AntiSandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            -(other[e4] * self[e423]) - (other[e412] * self[e431]),
            -(other[e4] * self[e431]) - (other[e423] * self[e412]),
            -(other[e4] * self[e412]) - (other[e431] * self[e423]),
            (other[e431] * self[e431]) + (other[e412] * self[e412]),
        ]) + (other.group1().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e3] * self[e431]) + (other[e321] * self[e423]),
            (other[e1] * self[e412]) + (other[e321] * self[e431]),
            (other[e2] * self[e423]) + (other[e321] * self[e412]),
            -(other[e3] * self[e412]) - (other[e4] * self[e321]),
        ]) - (other.group0().yzxx() * self.group0().zxyx())
            - (self.group0().wwwy() * other.group1().xyz().with_w(other[e2]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g0[3] * self[e423],
                geometric_anti_product_g0[3] * self[e431],
                geometric_anti_product_g0[3] * self[e412],
                -(geometric_anti_product_g1[1] * self[e431]) - (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g0.yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl AntiSandwich<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(other[e321]) * self.group0().xyz();
        Horizon::from_groups(
            // e321
            -(geometric_anti_product_g1[0] * self[e423]) - (geometric_anti_product_g1[1] * self[e431]) - (geometric_anti_product_g1[2] * self[e412]),
        )
    }
}
impl AntiSandwich<Line> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        5        7        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       28       45        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            other[e42] * self[e412] * -1.0,
            other[e43] * self[e423] * -1.0,
            other[e41] * self[e431] * -1.0,
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) + (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        Line::from_groups(
            // e41, e42, e43
            (geometric_anti_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz()) + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()),
        )
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       39        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            other[e43] * self[e412] * -1.0,
        ]) - (self.group0().xyzy() * Simd32x3::from(other[scalar]).with_w(other[e42]))
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = Simd32x4::from([
            other[e42] * self[e412] * -1.0,
            other[e43] * self[e423] * -1.0,
            other[e41] * self[e431] * -1.0,
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) + (Simd32x4::from([other[e1234], other[e1234], other[e1234], other[e23]]) * self.group0().xyzx())
            + (other.group0().zxyw() * self.group0().yzxw());
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g0[3] * self[e423]) - (geometric_anti_product_g1[1] * self[e412]),
                -(geometric_anti_product_g0[3] * self[e431]) - (geometric_anti_product_g1[2] * self[e423]),
                -(geometric_anti_product_g0[3] * self[e412]) - (geometric_anti_product_g1[0] * self[e431]),
                (geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g1.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_anti_product_g0[1] * self[e412]) - (geometric_anti_product_g1[3] * self[e423]),
                -(geometric_anti_product_g0[2] * self[e423]) - (geometric_anti_product_g1[3] * self[e431]),
                -(geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g1[3] * self[e412]),
                (geometric_anti_product_g0[2] * self[e412]) + (geometric_anti_product_g0[3] * self[e321]),
            ]) + (geometric_anti_product_g0.zxyx() * self.group0().yzxx())
                + (self.group0().wwwy() * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[1])),
        )
    }
}
impl AntiSandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       40        0
    //    simd2        3        3        0
    //    simd3       10       14        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       37       64        0
    //  no simd       81      116        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            other[e43] * self[e412] * -1.0,
        ]) - (self.group0().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
            - (self.group0().yzxy() * other.group3().zxy().with_w(other[e42]));
        let geometric_anti_product_g2 =
            (other.group4().yzx() * self.group0().zxy()) - (Simd32x3::from(other[e4]) * self.group0().xyz()) - (other.group4().zxy() * self.group0().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().zxy() * self.group0().yzx())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (other.group1().yzx() * self.group0().zxy());
        let geometric_anti_product_g4 = Simd32x4::from([
            other[e42] * self[e412] * -1.0,
            other[e43] * self[e423] * -1.0,
            other[e41] * self[e431] * -1.0,
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) + (Simd32x4::from(other[e1234]) * self.group0())
            + (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product_g1[3] * self[e321], 0.0])
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_x * self[e423]) + (geometric_anti_product_g2[0] * self[e321]) + (geometric_anti_product_g3[1] * self[e412]),
                (geometric_anti_product_g0_x * self[e431]) + (geometric_anti_product_g2[1] * self[e321]) + (geometric_anti_product_g3[2] * self[e423]),
                (geometric_anti_product_g0_x * self[e412]) + (geometric_anti_product_g2[2] * self[e321]) + (geometric_anti_product_g3[0] * self[e431]),
                -(geometric_anti_product_g2[1] * self[e431]) - (geometric_anti_product_g2[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0])),
            // e41, e42, e43
            (geometric_anti_product_g4.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz()) + (geometric_anti_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g2[1] * self[e412],
                geometric_anti_product_g2[2] * self[e423],
                geometric_anti_product_g2[0] * self[e431],
                -(geometric_anti_product_g3[1] * self[e431]) - (geometric_anti_product_g3[2] * self[e412]),
            ]) + (Simd32x4::from((other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412])) * self.group0())
                - (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       15        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4] * -1.0) * self.group0().xyz();
        let geometric_anti_product_g1_w = other[e4] * self[e321] * -1.0;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (geometric_anti_product_g1_w * self[e423]) + (geometric_anti_product_g0_xyz[0] * self[e321]),
            (geometric_anti_product_g1_w * self[e431]) + (geometric_anti_product_g0_xyz[1] * self[e321]),
            (geometric_anti_product_g1_w * self[e412]) + (geometric_anti_product_g0_xyz[2] * self[e321]),
            -(geometric_anti_product_g0_xyz[0] * self[e423]) - (geometric_anti_product_g0_xyz[1] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e412]),
        ]))
    }
}
impl AntiSandwich<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            other[e412] * self[e431] * -1.0,
            other[e423] * self[e412] * -1.0,
            other[e431] * self[e423] * -1.0,
            (other[e431] * self[e431]) + (other[e412] * self[e412]),
        ]) + (other.group0().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz());
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                geometric_anti_product_g0[3] * self[e423],
                geometric_anti_product_g0[3] * self[e431],
                geometric_anti_product_g0[3] * self[e412],
                -(geometric_anti_product_g1_xyz[1] * self[e431]) - (geometric_anti_product_g1_xyz[2] * self[e412]),
            ]) + (geometric_anti_product_g0.yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1_xyz[0])),
        )
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       17       29        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4] * -1.0) * self.group0().xyz();
        let geometric_anti_product_g1 = Simd32x4::from([
            self[e431] * other[e3],
            self[e412] * other[e1],
            self[e423] * other[e2],
            -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (self.group0().zxyx() * other.group0().yzxx());
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_xyz[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0_xyz[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0_xyz[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0_xyz[1] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0_xyz[0])),
        )
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        7        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group0().xyz();
        Scalar::from_groups(
            // scalar
            (geometric_anti_product_g0_xyz[0] * self[e423]) + (geometric_anti_product_g0_xyz[1] * self[e431]) + (geometric_anti_product_g0_xyz[2] * self[e412]),
        )
    }
}
impl std::ops::Div<AntiSandwichInfix> for Point {
    type Output = AntiSandwichInfixPartial<Point>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Point {
    type Output = AntiScalar;
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e4] * self[e4])
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e4] * self[e4]) * other.group0() * Simd32x2::from([1.0, -1.0]))
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       24       41        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]);
        let geometric_anti_product_g1 = Simd32x4::from([
            -(other[e4] * self[e1]) - (other[e412] * self[e2]),
            -(other[e4] * self[e2]) - (other[e423] * self[e3]),
            -(other[e4] * self[e3]) - (other[e431] * self[e1]),
            (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) + (other.group1().yzxy() * self.group0().zxyy())
            + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]));
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g0.yzx() * self.group0().zxy())
                - (geometric_anti_product_g0.zxy() * self.group0().yzx()))
            .with_w(geometric_anti_product_g0[3] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()).with_w(
                (geometric_anti_product_g1[3] * self[e4])
                    - (geometric_anti_product_g0[0] * self[e1])
                    - (geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3]),
            ),
        )
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[e4] * self[e4] * -1.0)
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        3        9        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       17       28        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0();
        Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g1_xyz * Simd32x3::from(self[e4] * -1.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()) + (geometric_anti_product_g1_xyz.yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g1_xyz.zxy() * self.group0().yzx()),
        )
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        3        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       24       41        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e1234] * self[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * geometric_anti_product_g1_xyz.with_w(geometric_anti_product_g0_w),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0_w * self[e1]) + (geometric_anti_product_g1_xyz[1] * self[e3]),
                (geometric_anti_product_g0_w * self[e2]) + (geometric_anti_product_g1_xyz[2] * self[e1]),
                (geometric_anti_product_g0_w * self[e3]) + (geometric_anti_product_g1_xyz[0] * self[e2]),
                -(geometric_anti_product_g1_xyz[1] * self[e2]) - (geometric_anti_product_g1_xyz[2] * self[e3]),
            ]) - (Simd32x4::from(self[e4])
                * ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group0().zxy() * self.group0().yzx())
                    - (Simd32x3::from(self[e4]) * other.group1().xyz())
                    - (other.group0().yzx() * self.group0().zxy()))
                .with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])))
                - (self.group0().yzxx() * geometric_anti_product_g1_xyz.zxy().with_w(geometric_anti_product_g1_xyz[0])),
        )
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd2        0        1        0
    //    simd3       12       22        0
    // Totals...
    // yes simd       24       53        0
    //  no simd       48       98        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = other[e4] * self[e4] * -1.0;
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(self[e4] * -1.0) * other.group4().xyz();
        let geometric_anti_product_g4_xyz = Simd32x3::from(self[e4]) * other.group2();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e4] * self[e4] * other[scalar]) + (other[e41] * self[e1] * self[e4]) + (other[e42] * self[e2] * self[e4]) + (other[e43] * self[e3] * self[e4])
                    - (geometric_anti_product_g4_xyz[0] * self[e1])
                    - (geometric_anti_product_g4_xyz[1] * self[e2])
                    - (geometric_anti_product_g4_xyz[2] * self[e3]),
                geometric_anti_product_g1_w * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e4] * self[e4]) * other.group1().xyz())
                + (geometric_anti_product_g2.yzx() * self.group0().zxy())
                + (Simd32x3::from(self[e4]) * other.group4().yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product_g0_y) * self.group0().xyz())
                - (Simd32x3::from(other[e4] * self[e4]) * self.group0().xyz())
                - (geometric_anti_product_g2.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * other.group4().zxy() * self.group0().yzx()))
            .with_w(geometric_anti_product_g0_y * self[e4] * -1.0),
            // e41, e42, e43
            geometric_anti_product_g4_xyz * Simd32x3::from(self[e4] * -1.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1_w) * self.group0().xyz())
                + (Simd32x3::from(self[e4] * self[e4]) * other.group3())
                + (geometric_anti_product_g4_xyz.yzx() * self.group0().zxy())
                + (Simd32x3::from(self[e4]) * other.group2().yzx() * self.group0().zxy())
                - (Simd32x3::from(other[e1234] * self[e4]) * self.group0().xyz())
                - (geometric_anti_product_g4_xyz.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * other.group2().zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g2 * Simd32x3::from(self[e4])).with_w(
                (self[e4] * self[e4] * other[e321]) + (other[e423] * self[e1] * self[e4]) + (other[e431] * self[e2] * self[e4]) + (other[e412] * self[e3] * self[e4])
                    - (geometric_anti_product_g2[0] * self[e1])
                    - (geometric_anti_product_g2[1] * self[e2])
                    - (geometric_anti_product_g2[2] * self[e3]),
            ),
        )
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       10        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e4] * self[e4] * -1.0;
        Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()) - (Simd32x3::from(other[e4] * self[e4]) * self.group0().xyz()))
                .with_w(geometric_anti_product_g0_w * self[e4]),
        )
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       14        0
    //    simd3        0        5        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       17       33        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        let geometric_anti_product_g1 = Simd32x4::from([
            other[e412] * self[e2] * -1.0,
            other[e423] * self[e3] * -1.0,
            other[e431] * self[e1] * -1.0,
            (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) + (other.group0().yzxx() * self.group0().zxyx());
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz()).with_w(0.0) + (geometric_anti_product_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0)
                - (geometric_anti_product_g0_xyz.zxy() * self.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g0_xyz * Simd32x3::from(self[e4])).with_w(
                (geometric_anti_product_g1[3] * self[e4])
                    - (geometric_anti_product_g0_xyz[0] * self[e1])
                    - (geometric_anti_product_g0_xyz[1] * self[e2])
                    - (geometric_anti_product_g0_xyz[2] * self[e3]),
            ),
        )
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        2        3        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       13        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e4] * self[e4] * -1.0;
        Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()) + (Simd32x3::from(self[e4] * self[e4]) * other.group0().xyz())
                - (Simd32x3::from(other[e4] * self[e4]) * self.group0().xyz()))
            .with_w(geometric_anti_product_g0_w * self[e4]),
        )
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e4] * self[e4] * other[scalar] * -1.0)
    }
}
