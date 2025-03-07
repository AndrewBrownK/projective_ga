// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 99
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5      11       0
//  Average:        14      23       0
//  Maximum:       140     163       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        11      26       0
//  Average:        35      51       0
//  Maximum:       373     394       0
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
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[e1234], 2));
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * Simd32x2::from([self[e1234] * other[scalar], self[e1234] * other[e1234]]),
        );
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
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group1(),
        );
    }
}
impl AntiSandwich<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321]);
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
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(self[e1234]), 2) * other.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(self[e1234]), 2) * other.group1(),
        );
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
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group1(),
        );
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
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::powi(Simd32x2::from(self[e1234]), 2) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group1(),
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(self[e1234]), 2) * other.group2(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(self[e1234]), 2) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(self[e1234]), 2) * other.group4(),
        );
    }
}
impl AntiSandwich<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4]);
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * Simd32x4::from([self[e1234] * other[e423], self[e1234] * other[e431], self[e1234] * other[e412], self[e1234] * other[e321]]),
        );
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * Simd32x4::from([self[e1234] * other[e1], self[e1234] * other[e2], self[e1234] * other[e3], self[e1234] * other[e4]]),
        );
    }
}
impl AntiSandwich<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * self[e1234] * other[scalar]);
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
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
            geometric_anti_product_g0[1] * self[e1234],
        ]));
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
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0_y * self[scalar]) + (self[e1234] * self[e1234] * other[scalar]) + (other[e1234] * self[scalar] * self[e1234]),
            geometric_anti_product_g0_y * self[e1234],
        ]));
    }
}
impl AntiSandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       26        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = self[e1234] * other[e4];
        let geometric_anti_product_g1 = self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321])) * other.group1().xyz().with_w(1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(self[e1234]) * other.group0().xyz())))
                - (Simd32x3::from(self[scalar]) * geometric_anti_product_g1.xyz()))
            .with_w(geometric_anti_product_g0_w * self[e1234]),
            // e423, e431, e412, e321
            self.group0()
                .yy()
                .with_zw(self[e1234], (geometric_anti_product_g1[3] * self[e1234]) - (geometric_anti_product_g0_w * self[scalar]))
                * geometric_anti_product_g1.xyz().with_w(1.0),
        );
    }
}
impl AntiSandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321]);
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
        return Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g0 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(self[e1234]) * ((Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()))),
        );
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
        return Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(self[e1234]) * ((Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()))),
        );
    }
}
impl AntiSandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd3        4       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       16       50        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = self[e1234] * other[e1234];
        let geometric_anti_product_g1_w = self[e1234] * other[e4];
        let geometric_anti_product_g2 = Simd32x3::from(self[e1234]) * other.group2();
        let geometric_anti_product_g4 = self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321])) * other.group4().xyz().with_w(1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_y * self[scalar]) + (self[e1234] * self[e1234] * other[scalar]) + (self[scalar] * self[e1234] * other[e1234]),
                geometric_anti_product_g0_y * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * ((Simd32x3::from(self[scalar]) * other.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group1().xyz())))
                - (Simd32x3::from(self[scalar]) * geometric_anti_product_g4.xyz()))
            .with_w(geometric_anti_product_g1_w * self[e1234]),
            // e41, e42, e43
            geometric_anti_product_g2 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(self[e1234]) * ((Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()))),
            // e423, e431, e412, e321
            self.group0()
                .yy()
                .with_zw(self[e1234], (geometric_anti_product_g4[3] * self[e1234]) - (geometric_anti_product_g1_w * self[scalar]))
                * geometric_anti_product_g4.xyz().with_w(1.0),
        );
    }
}
impl AntiSandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4]);
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x4::from(self[e1234]) * other.group0();
        return Plane::from_groups(
            // e423, e431, e412, e321
            self.group0().yy().with_zw(self[e1234], geometric_anti_product_g1[3] * self[e1234]) * geometric_anti_product_g1.xyz().with_w(1.0),
        );
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e1234]) * other.group0();
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz()).with_w(geometric_anti_product_g0[3] * self[e1234]),
        );
    }
}
impl AntiSandwich<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * self[e1234] * other[scalar]);
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
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       11        0
    //  no simd       11       24        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[3] * self[e321])
                    - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                    - (anti_reverse_g0[3] * geometric_anti_product_g1[3]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0[2], geometric_anti_product_g1[2]]))
                - (Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[3]]) * anti_reverse_g0.xw()),
        );
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd2        4        4        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       29        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(other[scalar]) * self.group1().xyz());
        let geometric_anti_product_g0_w = other[e1234] * self[e4];
        let geometric_anti_product_g1 = other.group0().yy().with_zw(other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4])) * self.group1().xyz().with_w(1.0);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_w * self[e321])
                    - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                    - (anti_reverse_g0[3] * geometric_anti_product_g1[3]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0_xyz[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0_xyz[2], geometric_anti_product_g1[2]]))
                - (Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0_w]) * anti_reverse_g0.xw()),
        );
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        4        5        0
    //    simd4       15       18        0
    // Totals...
    // yes simd       27       38        0
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
            + (Simd32x4::from([self[e4], self[e412], self[e423], self[e1]]) * other.group0().xxy().with_w(other[e423]))
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e2]]) * other.group0().zyz().with_w(other[e431]))
            + (other.group1().yzxz() * self.group0().zxyz())
            - (Simd32x4::from([self[e2], self[e321], self[e321], self[e321]]) * other.group1().zyz().with_w(other[e4]))
            - (Simd32x4::from([self[e321], self[e3], self[e1], self[e412]]) * other.group1().xxy().with_w(other[e3]))
            - (other.group0().yzxx() * self.group1().zxyx())
            - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.xxyw() * geometric_anti_product_g0.wzxw())
                + ((Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                    + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                    + (anti_reverse_g0.zyz() * geometric_anti_product_g0.yww())
                    + (geometric_anti_product_g1.yzx() * self.group1().zxy()))
                .with_w(geometric_anti_product_g0[2] * self[e412] * -1.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
            // e423, e431, e412, e321
            (Simd32x4::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]))
                + (Simd32x4::from([self[e412], self[e431], self[e412], 1.0])
                    * geometric_anti_product_g0.yww().with_w(
                        -(anti_reverse_g0[1] * geometric_anti_product_g0[1])
                            - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                            - (geometric_anti_product_g1[0] * self[e423])
                            - (geometric_anti_product_g1[1] * self[e431])
                            - (geometric_anti_product_g1[2] * self[e412]),
                    ))
                + (geometric_anti_product_g0.wzxw() * self.group1().xxyw())
                - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(anti_reverse_g0[0])),
        );
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
        return Horizon::from_groups(
            // e321
            -(geometric_anti_product_g1[0] * self[e423])
                - (geometric_anti_product_g1[1] * self[e431])
                - (geometric_anti_product_g1[2] * self[e412])
                - (geometric_anti_product_g1[3] * self[e4]),
        );
    }
}
impl AntiSandwich<Line> for Flector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3       10       15        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       63       76        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]),
            (self[e1] * other[e42]) + (self[e431] * other[e23]),
            0.0,
        ]) + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
            - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
            - (other.group0().yzx() * self.group0().zxy()).with_w(self[e423] * other[e41]);
        let geometric_anti_product_g1 = (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group0().xxy().with_w(other[e23]))
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group0().zyz().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
            - (other.group0().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Line::from_groups(
            // e41, e42, e43
            (geometric_anti_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group1().xxy())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3]]) * self.group1().zyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g0.xxy())
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3]]) * anti_reverse_g0.zyz())
                + (geometric_anti_product_g0.zxy() * self.group1().yzx())
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz())
                - (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group1().zyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g0[0]]) * self.group1().xxy())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()),
        );
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       23        0
    //    simd3        0        3        0
    //    simd4       18       17        0
    // Totals...
    // yes simd       31       43        0
    //  no simd       85      100        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]) - (self[e431] * other[e12]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]) - (self[e412] * other[e23]),
            (self[e3] * other[e1234]) + (self[e431] * other[e23]) - (self[e412] * other[scalar]),
            0.0,
        ]) + (self.group0().xyxw() * other.group0().wwyw())
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            - (self.group1().xyxz() * other.group1().wwy().with_w(other[e43]))
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
            - (other.group1().xyz() * self.group0().www()).with_w(self[e431] * other[e42]);
        let geometric_anti_product_g1 = (self.group1().xyxy() * other.group0().wwy().with_w(other[e31]))
            + (self.group1().yzzz() * other.group0().zxw().with_w(other[e12]))
            + Simd32x3::from(0.0).with_w((self[e321] * other[e1234]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
            + (other.group0().xyz() * self.group0().www()).with_w(self[e423] * other[e23])
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g0[3] * self[e423]) - (geometric_anti_product_g1[1] * self[e412]),
                -(geometric_anti_product_g0[3] * self[e431]) - (geometric_anti_product_g1[2] * self[e423]),
                -(geometric_anti_product_g0[3] * self[e412]) - (geometric_anti_product_g1[0] * self[e431]),
                (geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g1.zxyx() * self.group1().yzxx())
                - (Simd32x4::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3])),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]))
                + (Simd32x4::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g0[0]])
                    * anti_reverse_g0.xxy().with_w(self[e423]))
                + (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3], geometric_anti_product_g0[1]])
                    * anti_reverse_g0.zyz().with_w(self[e431]))
                + (geometric_anti_product_g0.zxyz() * self.group1().yzxz())
                - (Simd32x4::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g1[3]])
                    * self.group1().zyz().with_w(anti_reverse_g0[3]))
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g0[0], geometric_anti_product_g1[2]])
                    * self.group1().xxy().with_w(anti_reverse_g0[2]))
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxyx())
                - (anti_reverse_g0.wwwy() * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[1])),
        );
    }
}
impl AntiSandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       32        0
    //    simd2        8        8        0
    //    simd3       24       34        0
    //    simd4       15       12        0
    // Totals...
    // yes simd       66       86        0
    //  no simd      167      198        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from([self[e423], self[e4]]) * other.group1().xw());
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e2] * other[e43]) + (self[e412] * other[e31]) - (self[e431] * other[e12]),
            (self[e3] * other[e41]) + (self[e423] * other[e12]) - (self[e4] * other[e31]),
            (self[e1] * other[e42]) + (self[e431] * other[e23]) - (self[e4] * other[e12]),
            0.0,
        ]) + (Simd32x4::from(other[e1234]) * self.group0())
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * other.group3().xxy().with_w(other[e43]))
            - (self.group1().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
            - (other.group2().yzx() * self.group0().zxy()).with_w(self[e431] * other[e42]);
        let geometric_anti_product_g2 = (self.group1().zxy() * other.group4().yzx())
            - (Simd32x3::from(self[e4]) * other.group4().xyz())
            - (Simd32x3::from([other[e4], other[e4], other[e431]]) * self.group1().xyx())
            - (Simd32x3::from([other[e412], other[e423], other[e4]]) * self.group1().yzz());
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (Simd32x3::from([other[e3], other[e1], other[e321]]) * self.group1().yzz())
            + (Simd32x3::from([other[e321], other[e321], other[e2]]) * self.group1().xyx())
            + (self.group0().zxy() * other.group4().yzx())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (Simd32x3::from([other[e4], other[e4], other[e431]]) * self.group0().xyx())
            - (Simd32x3::from([other[e412], other[e423], other[e4]]) * self.group0().yzz())
            - (self.group1().zxy() * other.group1().yzx());
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group2().xxy().with_w(other[e23]))
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group2().zyz().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
            - (other.group2().yzx() * self.group1().zxy()).with_w(self[e4] * other[scalar]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g0[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g0[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g0.xw()),
            // e1, e2, e3, e4
            (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group1().xyz().with_w(anti_reverse_g0[3]))
                + ((Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g0.xyz())
                    + (Simd32x3::from([anti_reverse_g0[2], anti_reverse_g0[0], self[e321]]) * geometric_anti_product_g2.yzz())
                    + (Simd32x3::from([self[e412], self[e423], anti_reverse_g0[3]]) * geometric_anti_product_g3.yzz())
                    + (Simd32x3::from([self[e321], self[e321], anti_reverse_g0[1]]) * geometric_anti_product_g2.xyx())
                    + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g0[3]).with_z(self[e431])))
                .with_w(geometric_anti_product_g2[2] * self[e412] * -1.0)
                - (self.group1().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (geometric_anti_product_g2.zxy() * anti_reverse_g0.yzx()).with_w(geometric_anti_product_g2[0] * self[e423]),
            // e41, e42, e43
            (geometric_anti_product_g4.zxy() * self.group1().yzx())
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group1().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group1().zyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g0.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g0.zyz())
                + (geometric_anti_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group1().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group1().xxy())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e412], self[e423], anti_reverse_g0[3], 1.0])
                * geometric_anti_product_g2.yzz().with_w(
                    -(geometric_anti_product_g2[1] * anti_reverse_g0[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g0[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                ))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group1().xyz().with_w(anti_reverse_g0[3]))
                + (anti_reverse_g0.ww().with_zw(self[e431], self[e321]) * geometric_anti_product_g2.xyx().with_w(geometric_anti_product_g0[1]))
                - (geometric_anti_product_g2.zxy() * self.group1().yzx()).with_w(geometric_anti_product_g2[0] * anti_reverse_g0[0]),
        );
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        4        5        0
    //    simd4        3        8        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       24       49        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.xxyw() * geometric_anti_product_g0.wzxw())
                + ((Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                    + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                    + (anti_reverse_g0.zyz() * geometric_anti_product_g0.yww())
                    + (geometric_anti_product_g1.yzx() * self.group1().zxy()))
                .with_w(geometric_anti_product_g0[2] * self[e412] * -1.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
        );
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       36       53        0
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]))
                + (Simd32x4::from([self[e412], self[e431], self[e412], 1.0])
                    * geometric_anti_product_g0.yww().with_w(
                        -(anti_reverse_g0[1] * geometric_anti_product_g0[1])
                            - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                            - (geometric_anti_product_g1[0] * self[e423])
                            - (geometric_anti_product_g1[1] * self[e431])
                            - (geometric_anti_product_g1[2] * self[e412]),
                    ))
                + (geometric_anti_product_g0.wzxw() * self.group1().xxyw())
                - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(anti_reverse_g0[0])),
        );
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        4        5        0
    //    simd4        5        8        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       36       57        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from([
            (self[e4] * other[e1]) + (self[e431] * other[e3]),
            (self[e4] * other[e2]) + (self[e412] * other[e1]),
            (self[e4] * other[e3]) + (self[e423] * other[e2]),
            -(self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (self.group1().zxyy() * other.group0().yzxy())
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.xxyw() * geometric_anti_product_g0.wzxw())
                + ((Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                    + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                    + (anti_reverse_g0.zyz() * geometric_anti_product_g0.yww())
                    + (geometric_anti_product_g1.yzx() * self.group1().zxy()))
                .with_w(geometric_anti_product_g0[2] * self[e412] * -1.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
        );
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       22        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar]) * self.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(self[e4] * other[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Scalar::from_groups(
            // scalar
            (geometric_anti_product_g0_xyz[0] * self[e423]) + (geometric_anti_product_g0_xyz[1] * self[e431]) + (geometric_anti_product_g0_xyz[2] * self[e412])
                - (anti_reverse_g0[0] * geometric_anti_product_g1[0])
                - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                - (anti_reverse_g0[3] * geometric_anti_product_g1[3]),
        );
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
    //    simd3        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       21        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other[e1234]) * self.group1();
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(anti_reverse_g1[0] * geometric_anti_product_g0[0]) - (anti_reverse_g1[1] * geometric_anti_product_g0[1]) - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
                0.0,
            ]) - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g0[2]])),
        );
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd2        3        3        0
    //    simd3        1        5        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       11       24        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1());
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(anti_reverse_g1[0] * geometric_anti_product_g0[0]) - (anti_reverse_g1[1] * geometric_anti_product_g0[1]) - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
                0.0,
            ]) - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g0[2]])),
        );
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd3        2       11        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       25       42        0
    //  no simd       62       79        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from([
            (other[e3] * self[e42]) + (other[e412] * self[e31]),
            (other[e1] * self[e43]) + (other[e423] * self[e12]),
            (other[e2] * self[e41]) + (other[e431] * self[e23]),
        ]) + (Simd32x3::from(other[e4]) * self.group1())
            + (Simd32x3::from(other[e321]) * self.group0()))
        .with_w(other[e412] * self[e43] * -1.0)
            - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxy() * other.group0().yzx()).with_w(other[e423] * self[e41]);
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e4] * self[e41]) + (other[e412] * self[e42]),
            (other[e4] * self[e42]) + (other[e423] * self[e43]),
            (other[e4] * self[e43]) + (other[e431] * self[e41]),
            -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        ]) - (self.group0().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]);
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g0[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[1]])
                    * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                - (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3], geometric_anti_product_g1[2]])
                    * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1[0]),
            // e423, e431, e412, e321
            (Simd32x4::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[0]])
                * anti_reverse_g0.xxy().with_w(anti_reverse_g1[0]))
                + (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3], geometric_anti_product_g1[1]])
                    * anti_reverse_g0.zyz().with_w(anti_reverse_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g1[2]) - (anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
                )
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g0[0]),
        );
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0();
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        return Horizon::from_groups(
            // e321
            -(anti_reverse_g0[0] * geometric_anti_product_g0_xyz[0])
                - (anti_reverse_g0[1] * geometric_anti_product_g0_xyz[1])
                - (anti_reverse_g0[2] * geometric_anti_product_g0_xyz[2]),
        );
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
        return Line::from_groups(
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
        );
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       56       78        0
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
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (anti_reverse_g0[0] * geometric_anti_product_g0[3]) + (anti_reverse_g0[2] * geometric_anti_product_g0[1]),
                (anti_reverse_g0[0] * geometric_anti_product_g0[2]) + (anti_reverse_g0[1] * geometric_anti_product_g0[3]),
                (anti_reverse_g0[1] * geometric_anti_product_g0[0]) + (anti_reverse_g0[2] * geometric_anti_product_g0[3]),
                -(anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
            ]) - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(anti_reverse_g0[0])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (anti_reverse_g0[0] * geometric_anti_product_g1[3])
                    + (anti_reverse_g0[2] * geometric_anti_product_g1[1])
                    + (anti_reverse_g1[0] * geometric_anti_product_g0[3])
                    + (anti_reverse_g1[2] * geometric_anti_product_g0[1]),
                (anti_reverse_g0[0] * geometric_anti_product_g1[2])
                    + (anti_reverse_g0[1] * geometric_anti_product_g1[3])
                    + (anti_reverse_g1[0] * geometric_anti_product_g0[2])
                    + (anti_reverse_g1[1] * geometric_anti_product_g0[3]),
                (anti_reverse_g0[1] * geometric_anti_product_g1[0])
                    + (anti_reverse_g0[2] * geometric_anti_product_g1[3])
                    + (anti_reverse_g1[1] * geometric_anti_product_g0[0])
                    + (anti_reverse_g1[2] * geometric_anti_product_g0[3]),
                -(anti_reverse_g0[2] * geometric_anti_product_g1[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g0[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g0[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
            ]) - (geometric_anti_product_g1.zxyx() * anti_reverse_g0.yzx().with_w(anti_reverse_g0[0]))
                - (anti_reverse_g1.yzx() * geometric_anti_product_g0.zxy()).with_w(anti_reverse_g0[1] * geometric_anti_product_g1[1]),
        );
    }
}
impl AntiSandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd2        6        6        0
    //    simd3       16       29        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       49       72        0
    //  no simd      120      151        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = (Simd32x3::from([
            (self[e42] * other[e3]) + (self[e31] * other[e412]),
            (self[e43] * other[e1]) + (self[e12] * other[e423]),
            (self[e41] * other[e2]) + (self[e23] * other[e431]),
        ]) + (Simd32x3::from(other[e4]) * self.group1())
            + (Simd32x3::from(other[e321]) * self.group0()))
        .with_w(self[e43] * other[e412] * -1.0)
            - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxy() * other.group1().yzx()).with_w(self[e41] * other[e423]);
        let geometric_anti_product_g2 =
            (Simd32x3::from(other[e1234]) * self.group0()) + (self.group0().yzx() * other.group2().zxy()) - (self.group0().zxy() * other.group2().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group0())
            + (Simd32x3::from(other[e1234]) * self.group1())
            + (self.group0().yzx() * other.group3().zxy())
            + (self.group1().yzx() * other.group2().zxy())
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
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(anti_reverse_g1[0] * geometric_anti_product_g2[0]) - (anti_reverse_g1[1] * geometric_anti_product_g2[1]) - (anti_reverse_g1[2] * geometric_anti_product_g2[2]),
                0.0,
            ]) - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]])),
            // e1, e2, e3, e4
            (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[1]])
                    * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[2]])
                    * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
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
            (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[0]])
                * anti_reverse_g0.xxy().with_w(anti_reverse_g1[0]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[1]])
                    * anti_reverse_g0.zyz().with_w(anti_reverse_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g4[2]) - (anti_reverse_g0[1] * geometric_anti_product_g1[1]) - (anti_reverse_g0[2] * geometric_anti_product_g1[2]),
                )
                - (anti_reverse_g0.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        8        0
    //    simd4        5        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       20       33        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4]) * self.group1();
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([0.0, geometric_anti_product_g0_xyz[2], geometric_anti_product_g0_xyz[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g0_xyz[1], 0.0, 0.0]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(0.0)
                - (Simd32x4::from([0.0, geometric_anti_product_g1_xyz[2], geometric_anti_product_g1_xyz[0], geometric_anti_product_g1_xyz[1]])
                    * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                - (Simd32x4::from([geometric_anti_product_g1_xyz[1], 0.0, 0.0, geometric_anti_product_g1_xyz[2]]) * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0_xyz.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1_xyz[0]),
        );
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd3        0        3        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       27       42        0
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
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from([geometric_anti_product_g0[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[0]])
                * anti_reverse_g0.xxy().with_w(anti_reverse_g1[0]))
                + (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3], geometric_anti_product_g1[1]])
                    * anti_reverse_g0.zyz().with_w(anti_reverse_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g1[2]) - (anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
                )
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g0[0]),
        );
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0       10        0
    //    simd4        7        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       30       42        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (self.group0().zxy() * other.group0().yzx()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        let geometric_anti_product_g1_w = -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]);
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([geometric_anti_product_g1_w, geometric_anti_product_g0[2], geometric_anti_product_g0[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1_w, geometric_anti_product_g1_w]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g1_xyz[1],
                    geometric_anti_product_g0[3],
                    geometric_anti_product_g0[3],
                    geometric_anti_product_g1_xyz[2],
                ]) * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                - (Simd32x4::from([
                    geometric_anti_product_g0[3],
                    geometric_anti_product_g1_xyz[2],
                    geometric_anti_product_g1_xyz[0],
                    geometric_anti_product_g1_xyz[1],
                ]) * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1_xyz[0]),
        );
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(other[scalar]) * self.group0();
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            -(anti_reverse_g0[0] * geometric_anti_product_g1[0]) - (anti_reverse_g0[1] * geometric_anti_product_g1[1]) - (anti_reverse_g0[2] * geometric_anti_product_g1[2]),
        );
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
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       11       28        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (anti_reverse_g1[3] * geometric_anti_product_g0[3])
                    - (anti_reverse_g1[0] * geometric_anti_product_g0[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g0[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
                0.0,
            ]) + (Simd32x2::from(anti_reverse_g0[3]) * Simd32x2::from([geometric_anti_product_g1[3], geometric_anti_product_g0[3]]))
                - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g0[2]])),
        );
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       15       32        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1());
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (anti_reverse_g1[3] * geometric_anti_product_g0[3])
                    - (anti_reverse_g1[0] * geometric_anti_product_g0[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g0[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
                0.0,
            ]) + (Simd32x2::from(anti_reverse_g0[3]) * Simd32x2::from([geometric_anti_product_g1[3], geometric_anti_product_g0[3]]))
                - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g0[2]])),
        );
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd3        3        9        0
    //    simd4       17       14        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       87      105        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group0().xxyw() * self.group0().wzxw())
            + (Simd32x3::from([
                (other[e3] * self[e42]) + (other[e412] * self[e31]),
                (other[e2] * self[e1234]) + (other[e423] * self[e12]),
                (other[e3] * self[e1234]) + (other[e431] * self[e23]),
            ]) + (Simd32x3::from(other[e4]) * self.group1().xyz())
                + (Simd32x3::from(other[e321]) * self.group0().xyz())
                + (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(other[e412] * self[e43] * -1.0)
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.wwyw() * geometric_anti_product_g0.xyxw())
                + (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g0[2]]) * anti_reverse_g0.zyw()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g1[3]]) * anti_reverse_g0.xxz()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (geometric_anti_product_g1.xyxz() * anti_reverse_g1.wwy().with_w(anti_reverse_g0[2]))
                - (geometric_anti_product_g1.yzz() * anti_reverse_g1.zxw()).with_w(0.0)
                - (anti_reverse_g1.xyz() * geometric_anti_product_g0.www()).with_w(anti_reverse_g0[1] * geometric_anti_product_g1[1]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1.xyxy() * anti_reverse_g0.wwy().with_w(anti_reverse_g1[1]))
                + (geometric_anti_product_g1.yzzz() * anti_reverse_g0.zxw().with_w(anti_reverse_g1[2]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g0[3] * geometric_anti_product_g1[3])
                        - (anti_reverse_g0[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                        - (anti_reverse_g1[3] * geometric_anti_product_g0[3]),
                )
                + (anti_reverse_g0.xyz() * geometric_anti_product_g0.www()).with_w(anti_reverse_g1[0] * geometric_anti_product_g1[0])
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        );
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0().xyz();
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Horizon::from_groups(
            // e321
            (anti_reverse_g0[3] * other[e321] * self[e1234])
                - (geometric_anti_product_g0_xyz[0] * anti_reverse_g0[0])
                - (geometric_anti_product_g0_xyz[1] * anti_reverse_g0[1])
                - (geometric_anti_product_g0_xyz[2] * anti_reverse_g0[2]),
        );
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3       10       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       58       80        0
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz())
                + (anti_reverse_g0.xxy() * geometric_anti_product_g0.wzx())
                + (anti_reverse_g0.zyz() * geometric_anti_product_g0.yww())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g0.xyz())
                + (anti_reverse_g0.xxy() * geometric_anti_product_g1.wzx())
                + (anti_reverse_g0.zyz() * geometric_anti_product_g1.yww())
                + (anti_reverse_g1.xxy() * geometric_anti_product_g0.wzx())
                + (anti_reverse_g1.zyz() * geometric_anti_product_g0.yww())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g0.zxy()),
        );
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        4        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       44       66        0
    //  no simd       80      104        0
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
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (anti_reverse_g0[2] * geometric_anti_product_g0[1]) + (anti_reverse_g0[3] * geometric_anti_product_g0[0]),
                (anti_reverse_g0[1] * geometric_anti_product_g0[3]) + (anti_reverse_g0[3] * geometric_anti_product_g0[1]),
                (anti_reverse_g0[2] * geometric_anti_product_g0[3]) + (anti_reverse_g0[3] * geometric_anti_product_g0[2]),
                -(anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
            ]) + (anti_reverse_g0.xxyw() * geometric_anti_product_g0.wzxw())
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (anti_reverse_g0[3] * geometric_anti_product_g1[0])
                    + (anti_reverse_g1[0] * geometric_anti_product_g0[3])
                    + (anti_reverse_g1[2] * geometric_anti_product_g0[1])
                    + (anti_reverse_g1[3] * geometric_anti_product_g0[0]),
                (anti_reverse_g0[3] * geometric_anti_product_g1[1])
                    + (anti_reverse_g1[0] * geometric_anti_product_g0[2])
                    + (anti_reverse_g1[1] * geometric_anti_product_g0[3])
                    + (anti_reverse_g1[3] * geometric_anti_product_g0[1]),
                (anti_reverse_g0[3] * geometric_anti_product_g1[2])
                    + (anti_reverse_g1[1] * geometric_anti_product_g0[0])
                    + (anti_reverse_g1[2] * geometric_anti_product_g0[3])
                    + (anti_reverse_g1[3] * geometric_anti_product_g0[2]),
                -(anti_reverse_g0[2] * geometric_anti_product_g1[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g0[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g0[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
            ]) + (anti_reverse_g0.xxyw() * geometric_anti_product_g1.wzxw())
                + (anti_reverse_g0.zyz() * geometric_anti_product_g1.yww()).with_w(anti_reverse_g1[3] * geometric_anti_product_g0[3])
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxyx())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g0.zxy()).with_w(anti_reverse_g0[1] * geometric_anti_product_g1[1]),
        );
    }
}
impl AntiSandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd2        8        8        0
    //    simd3       23       32        0
    //    simd4       17       16        0
    // Totals...
    // yes simd       64       81        0
    //  no simd      169      201        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]), 0.0])
            + (Simd32x2::from(self[e1234]) * other.group0())
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = (self.group0().xyxw() * other.group4().ww().with_zw(other[e2], other[e4]))
            + (Simd32x3::from([
                (self[e42] * other[e3]) + (self[e31] * other[e412]),
                (self[e43] * other[e1]) + (self[e12] * other[e423]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]),
            ]) + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group4().xyz())
                + (Simd32x3::from(other[e4]) * self.group1().xyz()))
            .with_w(self[e43] * other[e412] * -1.0)
            - (self.group0().zxyx() * other.group1().yzx().with_w(other[e423]))
            - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]));
        let geometric_anti_product_g2 =
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().xxy() * self.group0().wzx()) + (other.group2().zyz() * self.group0().yww())
                - (other.group2().yzx() * self.group0().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group0().xyz())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz())
            + (other.group2().xxy() * self.group1().wzx())
            + (other.group2().zyz() * self.group1().yww())
            + (other.group3().xxy() * self.group0().wzx())
            + (other.group3().zyz() * self.group0().yww())
            - (other.group2().yzx() * self.group1().zxy())
            - (other.group3().yzx() * self.group0().zxy());
        let geometric_anti_product_g4 = (self.group0().xyxw() * other.group1().ww().with_zw(other[e431], other[e321]))
            + (other.group4().zx().with_zw(other[e4], other[e4]) * self.group0().yzz().with_w(self[scalar]))
            + (Simd32x3::from(self[e1234]) * other.group4().xyz())
                .with_w(-(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            - (self.group0().zxyx() * other.group4().yzx().with_w(other[e1]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * anti_reverse_g1[3])
                    - (geometric_anti_product_g3[0] * anti_reverse_g0[0])
                    - (geometric_anti_product_g3[1] * anti_reverse_g0[1])
                    - (geometric_anti_product_g3[2] * anti_reverse_g0[2]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(anti_reverse_g0[3]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([anti_reverse_g1[0], anti_reverse_g0[0]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([anti_reverse_g1[1], anti_reverse_g0[1]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([anti_reverse_g1[2], anti_reverse_g0[2]])),
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[3]]) * anti_reverse_g0.xxyw())
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[1]])
                    * anti_reverse_g1.xxy().with_w(anti_reverse_g0[1]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[2]])
                    * anti_reverse_g1.zyz().with_w(anti_reverse_g0[2]))
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g4[0]))
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz()).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g0.xyz())
                + (geometric_anti_product_g2.xyx() * anti_reverse_g0.wwy())
                + (geometric_anti_product_g2.yzz() * anti_reverse_g0.zxw())
                - (geometric_anti_product_g2.zxy() * anti_reverse_g0.yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * anti_reverse_g0.xyz())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz())
                + (geometric_anti_product_g2.xyx() * anti_reverse_g1.wwy())
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zxw())
                + (geometric_anti_product_g3.xyx() * anti_reverse_g0.wwy())
                + (geometric_anti_product_g3.yzz() * anti_reverse_g0.zxw())
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx())
                - (geometric_anti_product_g3.zxy() * anti_reverse_g0.yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[3]]) * anti_reverse_g0.xxyw())
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g0.zyz().with_w(anti_reverse_g1[0]))
                + (geometric_anti_product_g4.xyzy() * anti_reverse_g0.www().with_w(anti_reverse_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                        - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                        - (anti_reverse_g1[3] * geometric_anti_product_g1[3]),
                )
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g4.zxy().with_w(geometric_anti_product_g1[0])),
        );
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        5        0
    //    simd4        7        7        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       28       44        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e1234]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.wwyw() * geometric_anti_product_g0.xyxw())
                + (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g0[2]]) * anti_reverse_g0.zyw()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g1[3]]) * anti_reverse_g0.xxz()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (geometric_anti_product_g1.xyxz() * anti_reverse_g1.wwy().with_w(anti_reverse_g0[2]))
                - (geometric_anti_product_g1.yzz() * anti_reverse_g1.zxw()).with_w(0.0)
                - (anti_reverse_g1.xyz() * geometric_anti_product_g0.www()).with_w(anti_reverse_g0[1] * geometric_anti_product_g1[1]),
        );
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        0        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       39       56        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (self[e41] * other[e321]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]),
            (self[e42] * other[e321]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]),
            (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[scalar] * other[e412]),
            -(self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = (self.group0().yzxw() * other.group0().zxyw())
            + (Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_anti_product_g1.xyxy() * anti_reverse_g0.wwy().with_w(anti_reverse_g1[1]))
                + (geometric_anti_product_g1.yzzz() * anti_reverse_g0.zxw().with_w(anti_reverse_g1[2]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g0[3] * geometric_anti_product_g1[3])
                        - (anti_reverse_g0[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                        - (anti_reverse_g1[3] * geometric_anti_product_g0[3]),
                )
                + (anti_reverse_g0.xyz() * geometric_anti_product_g0.www()).with_w(anti_reverse_g1[0] * geometric_anti_product_g1[0])
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        );
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        3       11        0
    //    simd4        7        4        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz =
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy())
                - (self.group0().zxy() * other.group0().yzx());
        let geometric_anti_product_g0_w = self[e1234] * other[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0().xyz();
        let geometric_anti_product_g1_w = (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0.wwyw() * geometric_anti_product_g0_xyz.xyx().with_w(geometric_anti_product_g0_w))
                + (Simd32x3::from([geometric_anti_product_g1_w, geometric_anti_product_g0_xyz[2], geometric_anti_product_g1_w]) * anti_reverse_g0.xxz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1_w, geometric_anti_product_g0_xyz[2]]) * anti_reverse_g0.zyw()).with_w(0.0)
                + (geometric_anti_product_g1_xyz.zxy() * anti_reverse_g1.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0_xyz.zxy().with_w(geometric_anti_product_g1_xyz[0]))
                - (Simd32x3::from(geometric_anti_product_g0_w) * anti_reverse_g1.xyz()).with_w(geometric_anti_product_g1_xyz[1] * anti_reverse_g0[1])
                - (geometric_anti_product_g1_xyz.xyx() * anti_reverse_g1.wwy()).with_w(geometric_anti_product_g1_w * anti_reverse_g0[2])
                - (geometric_anti_product_g1_xyz.yzz() * anti_reverse_g1.zxw()).with_w(0.0),
        );
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x4::from(other[scalar]) * self.group0();
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        return Scalar::from_groups(
            // scalar
            (anti_reverse_g0[3] * geometric_anti_product_g1[3])
                - (anti_reverse_g0[0] * geometric_anti_product_g1[0])
                - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                - (anti_reverse_g0[2] * geometric_anti_product_g1[2]),
        );
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
    //      f32       14       18        0
    //    simd2        8        9        0
    //    simd3        0        6        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       62       90        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other[e1234]) * self.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other[e1234]) * self.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other[e1234]) * self.group4();
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0[1]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1[3] * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd2        8        8        0
    //    simd3        2       10        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       70       99        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]);
        let geometric_anti_product_g0_y = other[e1234] * self[e1234];
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(other[scalar]) * self.group4().xyz());
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(other[e1234]) * self.group2();
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3());
        let geometric_anti_product_g4 = other.group0().yy().with_zw(other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4])) * self.group4().xyz().with_w(1.0);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234]) + (geometric_anti_product_g1_w * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0_y) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1_xyz[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1_xyz[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1_xyz[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1_w]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1_w, geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0_y))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1_w, geometric_anti_product_g1_w, geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (Simd32x3::from(geometric_anti_product_g0_y) * self.group4().xyz()).with_w(geometric_anti_product_g0_x * anti_reverse_g1[3])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1_w * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0]),
        );
    }
}
impl AntiSandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       42        0
    //    simd2       12       12        0
    //    simd3       35       55        0
    //    simd4       29       17        0
    // Totals...
    // yes simd      100      126        0
    //  no simd      269      299        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from([self[e423], self[e4]]) * other.group0().xw());
        let geometric_anti_product_g1 = (self.group0().xx().with_zw(self[scalar], self[e1234]) * other.group1().xyz().with_w(other[e4]))
            + (Simd32x3::from([
                (other[e3] * self[e42]) + (other[e412] * self[e31]),
                (other[e1] * self[e43]) + (other[e423] * self[e12]),
                (other[e2] * self[e41]) + (other[e431] * self[e23]),
            ]) + (Simd32x3::from(other[e4]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group0().xyz()))
            .with_w(other[e412] * self[e43] * -1.0)
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
            other[e423] * self[e43],
            other[e4] * self[e43],
            -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        ]) + (self.group0().yy().with_zw(self[e1234], self[scalar]) * other.group1().xyz().with_w(other[e4]))
            + (other.group0().ww().with_zw(other[e431], other[e321]) * self.group2().xyx().with_w(self[e1234]))
            - (self.group2().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[2]])
                    * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4.xyzx() * self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g1.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0[1]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1[3] * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       20        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       16       35        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]);
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e321]) * self.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other[e321]) * self.group4().xyz();
        let geometric_anti_product_g4_w = other[e321] * self[e1234];
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234])
                    + (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (geometric_anti_product_g4_w * anti_reverse_g1[3])
                    - (anti_reverse_g2[0] * geometric_anti_product_g3[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g3[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g3[2]),
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
                (geometric_anti_product_g4_w * self[e1234]) + (geometric_anti_product_g0[0] * anti_reverse_g1[3]) + (geometric_anti_product_g0[1] * self[e321])
                    - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                    - (geometric_anti_product_g3[0] * self[e423])
                    - (geometric_anti_product_g3[1] * self[e431])
                    - (geometric_anti_product_g3[2] * self[e412]),
            ),
        );
    }
}
impl AntiSandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd2        3        3        0
    //    simd3       29       48        0
    //    simd4       22       11        0
    // Totals...
    // yes simd       61       78        0
    //  no simd      188      210        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e43] * self[e2]) + (other[e31] * self[e412]),
            (other[e41] * self[e3]) + (other[e12] * self[e423]),
            (other[e42] * self[e1]) + (other[e23] * self[e431]),
            0.0,
        ]) + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
            - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
            - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e423]);
        let geometric_anti_product_g2 =
            (Simd32x3::from(self[e1234]) * other.group0()) + (other.group0().zxy() * self.group2().yzx()) - (other.group0().yzx() * self.group2().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(self[scalar]) * other.group0())
            + (Simd32x3::from(self[e1234]) * other.group1())
            + (other.group0().zxy() * self.group3().yzx())
            + (other.group1().zxy() * self.group2().yzx())
            - (other.group0().yzx() * self.group3().zxy())
            - (other.group1().yzx() * self.group2().zxy());
        let geometric_anti_product_g4 = (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group0().xxy().with_w(other[e23]))
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group0().zyz().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
            - (other.group0().yzx() * self.group4().zxy()).with_w(other[e41] * self[e1]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[2]])
                    * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4.xyzx() * self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g1.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiSandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       34        0
    //    simd2       12       12        0
    //    simd3       32       52        0
    //    simd4       33       21        0
    // Totals...
    // yes simd      100      119        0
    //  no simd      275      298        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]), 0.0])
            + (Simd32x2::from(other[e1234]) * self.group0())
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = Simd32x4::from([
            (other[e43] * self[e2]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            0.0,
        ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * other.group0().xxyw())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
            - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e423]))
            - (Simd32x3::from(other[scalar]) * self.group4().xyz()).with_w(0.0);
        let geometric_anti_product_g2 =
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (self.group2().xyx() * other.group0().wwy()) + (self.group2().yzz() * other.group0().zxw())
                - (self.group2().zxy() * other.group0().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(self[scalar]) * other.group0().xyz())
            + (Simd32x3::from(self[e1234]) * other.group1().xyz())
            + (self.group2().xyx() * other.group1().wwy())
            + (self.group2().yzz() * other.group1().zxw())
            + (self.group3().xyx() * other.group0().wwy())
            + (self.group3().yzz() * other.group0().zxw())
            - (self.group2().zxy() * other.group1().yzx())
            - (self.group3().zxy() * other.group0().yzx());
        let geometric_anti_product_g4 = (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * other.group0().xxyw())
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * other.group0().zyz().with_w(other[e23]))
            + (self.group4().xyzy() * other.group0().www().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
            - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1]));
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[2]])
                    * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4.xyzx() * self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g1.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0[1]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1[3] * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       52        0
    //    simd2       16       16        0
    //    simd3       44       70        0
    //    simd4       43       25        0
    // Totals...
    // yes simd      140      163        0
    //  no simd      373      394        0
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
            (other[e3] * self[e42]) + (other[e43] * self[e2]) + (other[e31] * self[e412]) + (other[e412] * self[e31]),
            (other[e1] * self[e43]) + (other[e41] * self[e3]) + (other[e12] * self[e423]) + (other[e423] * self[e12]),
            (other[e2] * self[e41]) + (other[e42] * self[e1]) + (other[e23] * self[e431]) + (other[e431] * self[e23]),
            0.0,
        ]) + (Simd32x4::from(other[e1234]) * self.group1())
            + (self.group0().xx().with_zw(self[scalar], self[e1234]) * other.group4().xyz().with_w(other[e4]))
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
            + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * other.group3().xxy().with_w(other[e43]))
            - (Simd32x4::from([self[e431], self[e4], self[e4], other[e423]]) * other.group3().zyz().with_w(self[e41]))
            - (other.group4().yzxz() * self.group3().zxy().with_w(self[e43]))
            - (self.group4().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
            - (other.group2().yzx() * self.group1().zxy()).with_w(other[e42] * self[e431])
            - (self.group2().zxy() * other.group1().yzx()).with_w(other[e431] * self[e42]);
        let geometric_anti_product_g2 = (Simd32x3::from(other[e1234]) * self.group2())
            + (Simd32x3::from(self[e1234]) * other.group2())
            + (other.group2().zxy() * self.group2().yzx())
            + (other.group4().yzx() * self.group4().zxy())
            - (Simd32x3::from(other[e4]) * self.group4().xyz())
            - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group4().xxy())
            - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group4().zyz())
            - (other.group2().yzx() * self.group2().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
            + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz())
            + (other.group2().zxy() * self.group3().yzx())
            + (other.group3().zxy() * self.group2().yzx())
            + (other.group4().yzx() * self.group1().zxy())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group4().zyz())
            - (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group4().xxy())
            - (other.group2().yzx() * self.group3().zxy())
            - (other.group3().yzx() * self.group2().zxy())
            - (other.group1().yzx() * self.group4().zxy());
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group4())
            + (Simd32x4::from([self[e4], self[e412], self[e423], other[e321]]) * other.group2().xxy().with_w(self[e1234]))
            + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * other.group2().zyz().with_w(other[e23]))
            + (self.group0().yy().with_zw(self[e1234], self[scalar]) * other.group4().xyz().with_w(other[e4]))
            + (other.group1().ww().with_zw(other[e431], self[e431]) * self.group2().xyx().with_w(other[e31]))
            + (other.group4().zx().with_zw(other[e4], self[e412]) * self.group2().yzz().with_w(other[e12]))
            + Simd32x3::from(0.0).with_w(
                -(other[e1] * self[e41])
                    - (other[e2] * self[e42])
                    - (other[e3] * self[e43])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            )
            - (other.group2().yzx() * self.group4().zxy()).with_w(other[scalar] * self[e4])
            - (self.group2().zxy() * other.group4().yzx()).with_w(other[e41] * self[e1]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[2]])
                    * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4.xyzx() * self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g1.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0[1]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1[3] * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        2        0
    //    simd3       22       40        0
    //    simd4       15        9        0
    // Totals...
    // yes simd       37       53        0
    //  no simd      126      162        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group3().with_w(self[e1234]);
        let geometric_anti_product_g2 = Simd32x3::from(other[e4]) * self.group4().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g4[2]])
                    * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4.xyzx() * self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]]) * anti_reverse_g1.xxy())
                + (Simd32x3::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd2       11       12        0
    //    simd3        5       11        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       49       68        0
    //  no simd      103      132        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x2::from([self[e4] * other[e321], 1.0]) * Simd32x2::from([1.0, 0.0]))
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
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from([geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * anti_reverse_g1.xw()),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from([geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0], self[e321]])
                    * anti_reverse_g2.xxy().with_w(geometric_anti_product_g0[1]))
                + (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[0]])
                    * anti_reverse_g2.zyz().with_w(anti_reverse_g3[0]))
                + (geometric_anti_product_g0.yy().with_zw(geometric_anti_product_g0[1], geometric_anti_product_g0[0]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (anti_reverse_g1.ww().with_zw(self[e431], geometric_anti_product_g4[1]) * geometric_anti_product_g2.xyx().with_w(anti_reverse_g3[1]))
                + (self.group4().zx().with_zw(anti_reverse_g1[3], geometric_anti_product_g4[2]) * geometric_anti_product_g2.yzz().with_w(anti_reverse_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[0] * self[e423])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g1[3] * self[scalar])
                - (geometric_anti_product_g2.zxy() * self.group4().yzx()).with_w(anti_reverse_g2[0] * geometric_anti_product_g1[0]),
        );
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd2        0        1        0
    //    simd3       28       47        0
    //    simd4       15        7        0
    // Totals...
    // yes simd       49       67        0
    //  no simd      150      183        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            self[e4] * other[e4],
        ]) * Simd32x2::from([1.0, -1.0]);
        let geometric_anti_product_g1 =
            ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx()))
            .with_w(self[e1234] * other[e4]);
        let geometric_anti_product_g2 = Simd32x3::from(other[e4]) * self.group4().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().yzx() * other.group0().zxy())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group4().zxy() * other.group0().yzx());
        let geometric_anti_product_g4_xyz = Simd32x3::from(other[e4]) * self.group2();
        let geometric_anti_product_g4_w = (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g0[1]) * self.group4().xyz().with_w(anti_reverse_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4_w, geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (Simd32x3::from([anti_reverse_g1[2], anti_reverse_g1[0], self[e321]]) * geometric_anti_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4_w, geometric_anti_product_g4_w]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], anti_reverse_g1[3]]) * geometric_anti_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], anti_reverse_g1[1]]) * geometric_anti_product_g2.xyx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4_xyz.zxy()).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g4_xyz[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3], self[e423]])
                    * anti_reverse_g3.zyz().with_w(geometric_anti_product_g2[0]))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4_xyz[2],
                    geometric_anti_product_g4_xyz[0],
                    geometric_anti_product_g4_xyz[2],
                ]) * anti_reverse_g3.xxy().with_w(anti_reverse_g2[2]))
                - (self.group4().yzxz() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[2]))
                - (self.group0().xx().with_zw(self[scalar], anti_reverse_g2[0]) * geometric_anti_product_g4_xyz.with_w(geometric_anti_product_g4_w))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4_xyz[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4_xyz.zxy() * self.group4().yzx())
                - (geometric_anti_product_g4_xyz * Simd32x3::from(anti_reverse_g1[3]))
                - (Simd32x3::from([geometric_anti_product_g4_xyz[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * self.group4().zyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4_xyz[2], geometric_anti_product_g4_xyz[0]]) * self.group4().xxy())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (geometric_anti_product_g4_xyz * Simd32x3::from(self[e321]))
                + (Simd32x3::from([geometric_anti_product_g4_xyz[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g1.zyz())
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g4_xyz[2], geometric_anti_product_g4_xyz[0]]) * anti_reverse_g1.xxy())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g1.zxy() * self.group4().yzx())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from([geometric_anti_product_g4_w, geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * self.group4().xxy())
                - (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4_w, geometric_anti_product_g4_w]) * self.group4().zyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4_xyz.zxy() * anti_reverse_g1.yzx()),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       26        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       22       51        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([self[e1234] * other[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]);
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[scalar]) * self.group4().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other[scalar]) * self.group2();
        let geometric_anti_product_g4 = Simd32x3::from(0.0).with_w(self[e4] * other[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234])
                    + (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    - (anti_reverse_g2[0] * geometric_anti_product_g3[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g3[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g3[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g4[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2])
                    - (anti_reverse_g1[3] * geometric_anti_product_g4[3]),
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
                (geometric_anti_product_g0[0] * anti_reverse_g1[3])
                    + (geometric_anti_product_g0[1] * self[e321])
                    + (anti_reverse_g3[0] * geometric_anti_product_g4[0])
                    + (anti_reverse_g3[1] * geometric_anti_product_g4[1])
                    + (anti_reverse_g3[2] * geometric_anti_product_g4[2])
                    + (geometric_anti_product_g4[3] * self[e1234])
                    - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0])
                    - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                    - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                    - (geometric_anti_product_g3[0] * self[e423])
                    - (geometric_anti_product_g3[1] * self[e431])
                    - (geometric_anti_product_g3[2] * self[e412]),
            ),
        );
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
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[e4], 2));
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4] * -1.0) * Simd32x2::from([other[scalar] * self[e4] * -1.0, other[e1234] * self[e4]]) * Simd32x2::from(-1.0),
        );
    }
}
impl AntiSandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]);
        let anti_reverse_g0 = self[e4] * -1.0;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e423, e431, e412, e321
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]),
        );
    }
}
impl AntiSandwich<Horizon> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * f32::powi(self[e4], 2) * -1.0);
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_reverse_g0 = self[e4] * -1.0;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_reverse_g0) * Simd32x3::from(self[e4]) * other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(anti_reverse_g0) * Simd32x3::from(self[e4]) * other.group1(),
        );
    }
}
impl AntiSandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       33        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let anti_reverse_g0 = self[e4] * -1.0;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_reverse_g0) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]) * Simd32x4::from(-1.0),
        );
    }
}
impl AntiSandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        4        0
    //    simd3        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       54        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(self[e4]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from([1.0, -1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(self[e4]) * other.group3().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g4 = Simd32x4::from(self[e4]) * other.group2().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let anti_reverse_g0 = self[e4] * -1.0;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_reverse_g0) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(anti_reverse_g0) * (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(geometric_anti_product_g0[1]),
            // e41, e42, e43
            Simd32x3::from(anti_reverse_g0) * geometric_anti_product_g4.xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(anti_reverse_g0) * geometric_anti_product_g1.xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(anti_reverse_g0) * (Simd32x3::from(self[e4]) * other.group4().xyz() * Simd32x3::from(-1.0)).with_w(geometric_anti_product_g0[0]),
        );
    }
}
impl AntiSandwich<Origin> for Origin {
    type Output = Origin;
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e4] * f32::powi(self[e4], 2));
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e4] * other[e321]),
        );
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
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(self[e4] * other[e4] * -1.0),
        );
    }
}
impl AntiSandwich<Scalar> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e4] * self[e4] * other[scalar] * -1.0);
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
        return AntiScalar::from_groups(
            // e1234
            (geometric_anti_product_g0[0] * self[e423]) + (geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]),
        );
    }
}
impl AntiSandwich<DualNum> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        2        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       16        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = self.group0().xyz() * other.group0().xx().with_z(other[scalar]) * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group0();
        return DualNum::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0_xyz[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0_xyz[2], geometric_anti_product_g1[2]])),
        );
    }
}
impl AntiSandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        0        1        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       36        0
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
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            (geometric_anti_product_g0.yzxw() * self.group0().zxyw())
                + (self.group0().xyz() * geometric_anti_product_g0.www()).with_w(-(geometric_anti_product_g1[1] * self[e431]) - (geometric_anti_product_g1[2] * self[e412]))
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0])),
        );
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
        return Horizon::from_groups(
            // e321
            -(geometric_anti_product_g1[0] * self[e423]) - (geometric_anti_product_g1[1] * self[e431]) - (geometric_anti_product_g1[2] * self[e412]),
        );
    }
}
impl AntiSandwich<Line> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        5        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       28       46        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]),
            -(other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = (Simd32x4::from([
            other[e42] * self[e412],
            other[e43] * self[e423],
            other[e41] * self[e431],
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        return Line::from_groups(
            // e41, e42, e43
            (geometric_anti_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz()) + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()),
        );
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       26        0
    //    simd3        1        1        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       17       35        0
    //  no simd       40       61        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from([other[e31] * self[e412], other[e12] * self[e423], other[e23] * self[e431]])
            + (Simd32x3::from(self[e321]) * other.group0().xyz()))
        .with_w(other[e43] * self[e412] * -1.0)
            - (self.group0().xyzy() * other.group1().www().with_w(other[e42]))
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = (Simd32x4::from([
            other[e42] * self[e412],
            other[e43] * self[e423],
            other[e41] * self[e431],
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().zxyw() * self.group0().yzxw())
            + (self.group0().xyzx() * other.group0().www().with_w(other[e23]));
        return Motor::from_groups(
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
        );
    }
}
impl AntiSandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       31        0
    //    simd2        3        4        0
    //    simd3       11       16        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       35       59        0
    //  no simd       81      119        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]);
        let geometric_anti_product_g1 = (Simd32x3::from([other[e31] * self[e412], other[e12] * self[e423], other[e23] * self[e431]])
            + (Simd32x3::from(self[e321]) * other.group2()))
        .with_w(other[e43] * self[e412] * -1.0)
            - (self.group0().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
            - (self.group0().yzxy() * other.group3().zxy().with_w(other[e42]));
        let geometric_anti_product_g2 =
            (other.group4().yzx() * self.group0().zxy()) - (Simd32x3::from(other[e4]) * self.group0().xyz()) - (other.group4().zxy() * self.group0().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().zxy() * self.group0().yzx())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (other.group1().yzx() * self.group0().zxy());
        let geometric_anti_product_g4 = (Simd32x4::from([
            other[e42] * self[e412],
            other[e43] * self[e423],
            other[e41] * self[e431],
            (other[e31] * self[e431]) + (other[e12] * self[e412]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (Simd32x4::from(other[e1234]) * self.group0())
            + (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([geometric_anti_product_g1[3] * self[e321], 1.0]) * Simd32x2::from([1.0, 0.0]))
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
            (Simd32x4::from((other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412])) * self.group0())
                + (geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(-(geometric_anti_product_g3[1] * self[e431]) - (geometric_anti_product_g3[2] * self[e412]))
                - (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        );
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       16        0
    //  no simd       11       26        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(other[e4] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_xyz[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0_xyz[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0_xyz[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0_xyz[1] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0_xyz[0])),
        );
    }
}
impl AntiSandwich<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        3        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       17       32        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([
            other[e412] * self[e431],
            other[e423] * self[e412],
            other[e431] * self[e423],
            (other[e431] * self[e431]) + (other[e412] * self[e412]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz());
        return Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_anti_product_g0.yzxw() * self.group0().zxyw())
                + (self.group0().xyz() * geometric_anti_product_g0.www())
                    .with_w(-(geometric_anti_product_g1_xyz[1] * self[e431]) - (geometric_anti_product_g1_xyz[2] * self[e412]))
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1_xyz[0])),
        );
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       17       31        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = self.group0().xyz() * other.group0().www() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from([
            self[e431] * other[e3],
            self[e412] * other[e1],
            self[e423] * other[e2],
            -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (self.group0().zxyx() * other.group0().yzxx());
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0_xyz[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0_xyz[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0_xyz[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0_xyz[1] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0_xyz[0])),
        );
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar]) * self.group0().xyz() * Simd32x3::from(-1.0);
        return Scalar::from_groups(
            // scalar
            (geometric_anti_product_g0_xyz[0] * self[e423]) + (geometric_anti_product_g0_xyz[1] * self[e431]) + (geometric_anti_product_g0_xyz[2] * self[e412]),
        );
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
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[e4], 2));
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       17        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(other[scalar] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(anti_reverse_g0[0] * geometric_anti_product_g1[0])
                    - (anti_reverse_g0[1] * geometric_anti_product_g1[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                    - (anti_reverse_g0[3] * geometric_anti_product_g1[3]),
                anti_reverse_g0[3] * other[e1234] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
        );
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from([
            -(other[e4] * self[e1]) - (other[e412] * self[e2]),
            -(other[e4] * self[e2]) - (other[e423] * self[e3]),
            -(other[e4] * self[e3]) - (other[e431] * self[e1]),
            (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) + (other.group1().yzxy() * self.group0().zxyy())
            + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from(geometric_anti_product_g0[3]) * anti_reverse_g0.xyz())
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()))
            .with_w(anti_reverse_g0[3] * geometric_anti_product_g0[3]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz()).with_w(
                (anti_reverse_g0[3] * geometric_anti_product_g1[3])
                    - (anti_reverse_g0[0] * geometric_anti_product_g0[0])
                    - (anti_reverse_g0[1] * geometric_anti_product_g0[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
            ),
        );
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * f32::powi(self[e4], 2) * -1.0);
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3       10        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       17       34        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0();
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g1_xyz * Simd32x3::from(anti_reverse_g0[3]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * anti_reverse_g0.xyz()) + (geometric_anti_product_g1_xyz.yzx() * anti_reverse_g0.zxy())
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g1_xyz.zxy() * anti_reverse_g0.yzx()),
        );
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e1234] * self[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0().xyz();
        let geometric_anti_product_g1_w = -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_reverse_g0[3]) * geometric_anti_product_g1_xyz.with_w(geometric_anti_product_g0_w) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0_w * anti_reverse_g0[0]) + (geometric_anti_product_g1_xyz[1] * anti_reverse_g0[2]),
                (geometric_anti_product_g0_w * anti_reverse_g0[1]) + (geometric_anti_product_g1_xyz[2] * anti_reverse_g0[0]),
                (geometric_anti_product_g0_w * anti_reverse_g0[2]) + (geometric_anti_product_g1_xyz[0] * anti_reverse_g0[1]),
                -(geometric_anti_product_g1_w * anti_reverse_g0[3]) - (geometric_anti_product_g1_xyz[2] * anti_reverse_g0[2]),
            ]) - (anti_reverse_g0.yzxy() * geometric_anti_product_g1_xyz.zxy().with_w(geometric_anti_product_g1_w))
                - (anti_reverse_g0.wwwx()
                    * ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group0().zxy() * self.group0().yzx())
                        - (Simd32x3::from(self[e4]) * other.group1().xyz())
                        - (other.group0().yzx() * self.group0().zxy()))
                    .with_w(geometric_anti_product_g1_xyz[0])),
        );
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd2        0        2        0
    //    simd3       12       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       24       48        0
    //  no simd       48       97        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            other[e4] * self[e4],
        ]) * Simd32x2::from([1.0, -1.0]);
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(self[e4]) * other.group4().xyz() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4_xyz = Simd32x3::from(self[e4]) * other.group2();
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (anti_reverse_g0[3] * other[scalar] * self[e4])
                    + (anti_reverse_g0[3] * other[e41] * self[e1])
                    + (anti_reverse_g0[3] * other[e42] * self[e2])
                    + (anti_reverse_g0[3] * other[e43] * self[e3])
                    - (geometric_anti_product_g4_xyz[0] * anti_reverse_g0[0])
                    - (geometric_anti_product_g4_xyz[1] * anti_reverse_g0[1])
                    - (geometric_anti_product_g4_xyz[2] * anti_reverse_g0[2]),
                geometric_anti_product_g1_w * anti_reverse_g0[3],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g0.xyz())
                + (Simd32x3::from(anti_reverse_g0[3])
                    * ((Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group4().yzx() * self.group0().zxy())
                        - (Simd32x3::from(other[e4]) * self.group0().xyz())
                        - (other.group4().zxy() * self.group0().yzx())))
                + (geometric_anti_product_g2.yzx() * anti_reverse_g0.zxy())
                - (geometric_anti_product_g2.zxy() * anti_reverse_g0.yzx()))
            .with_w(geometric_anti_product_g0[1] * anti_reverse_g0[3]),
            // e41, e42, e43
            geometric_anti_product_g4_xyz * Simd32x3::from(anti_reverse_g0[3]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1_w) * anti_reverse_g0.xyz()) + (geometric_anti_product_g4_xyz.yzx() * anti_reverse_g0.zxy())
                - (Simd32x3::from(anti_reverse_g0[3])
                    * ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().zxy() * self.group0().yzx())
                        - (Simd32x3::from(self[e4]) * other.group3())
                        - (other.group2().yzx() * self.group0().zxy())))
                - (geometric_anti_product_g4_xyz.zxy() * anti_reverse_g0.yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g2 * Simd32x3::from(anti_reverse_g0[3])).with_w(
                (geometric_anti_product_g0[0] * anti_reverse_g0[3])
                    - (geometric_anti_product_g2[0] * anti_reverse_g0[0])
                    - (geometric_anti_product_g2[1] * anti_reverse_g0[1])
                    - (geometric_anti_product_g2[2] * anti_reverse_g0[2]),
            ),
        );
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        9       25        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(other[e4] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * anti_reverse_g0.xyz()) + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy())
                - (Simd32x3::from(anti_reverse_g0[3]) * Simd32x3::from(other[e4]) * self.group0().xyz()))
            .with_w(anti_reverse_g0[3] * geometric_anti_product_g0[3]),
        );
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        6       24        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = other.group0().xyz() * self.group0().www() * Simd32x3::from(-1.0);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Plane::from_groups(/* e423, e431, e412, e321 */ (geometric_anti_product_g0_xyz * Simd32x3::from(anti_reverse_g0[3])).with_w(
            (anti_reverse_g0[3] * other.group0().yzxx()[3] * self.group0().zxyx()[3])
                + (anti_reverse_g0[3] * other[e431] * self[e2])
                + (anti_reverse_g0[3] * other[e412] * self[e3])
                + (anti_reverse_g0[3] * other[e321] * self[e4])
                - (geometric_anti_product_g0_xyz[0] * anti_reverse_g0[0])
                - (geometric_anti_product_g0_xyz[1] * anti_reverse_g0[1])
                - (geometric_anti_product_g0_xyz[2] * anti_reverse_g0[2]),
        ));
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        4        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       12       28        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(other[e4] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(anti_reverse_g0[3]) * ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * anti_reverse_g0.xyz())
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()))
            .with_w(anti_reverse_g0[3] * geometric_anti_product_g0[3]),
        );
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e4] * self[e4] * other[scalar] * -1.0);
    }
}
