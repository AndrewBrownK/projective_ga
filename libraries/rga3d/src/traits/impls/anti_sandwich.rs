// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 99
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0     N/A
//   Median:         5      10       0     N/A
//  Average:        14      21       0     N/A
//  Maximum:       146     171       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         8      21       0       0
//  Average:        40      48       0       0
//  Maximum:       396     394       0       0
impl std::ops::Div<AntiSandwichInfix> for AntiScalar {
    type Output = AntiSandwichInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234] * self[e1234])
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321])
    }
}
impl AntiSandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       21        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4])
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
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
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        5        0        0
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
    //      add/sub      mul      div      pow
    // f32        2        7        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4       14        0      N/A
    //  no simd        8       24        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * self[e1234] * other[e321])
    }
}
impl AntiSandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        6        0      N/A
    // no simd        6       18        0        0
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
    //          add/sub      mul      div      pow
    //   simd4        2        6        0      N/A
    // no simd        8       24        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4       18        0        0
    //    simd3        4       10        0      N/A
    // Totals...
    // yes simd        8       28        0      N/A
    //  no simd       16       48        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * self[e1234] * other[e4])
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234] * self[e1234]) * other.group0())
    }
}
impl AntiSandwich<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd2        4        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       11       20        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[0] * self[e1])
                    + (geometric_anti_product_g1[1] * self[e2])
                    + (geometric_anti_product_g1[2] * self[e3])
                    + (geometric_anti_product_g1[3] * self[e4]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[e321], self[e4]]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0[0], geometric_anti_product_g1[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0[1], geometric_anti_product_g1[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0[2], geometric_anti_product_g1[2]])),
        )
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        4        4        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        9       15        0      N/A
    //  no simd       15       25        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(other[scalar]) * self.group1().xyz());
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e1234]) * self.group1().xyz();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1_xyz[0] * self[e1])
                    + (geometric_anti_product_g1_xyz[1] * self[e2])
                    + (geometric_anti_product_g1_xyz[2] * self[e3])
                    + (other[e1234] * self[e4] * self[e321])
                    - (self[e4] * self[e4] * other[scalar]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g0_xyz[0], geometric_anti_product_g1_xyz[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g0_xyz[1], geometric_anti_product_g1_xyz[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g0_xyz[2], geometric_anti_product_g1_xyz[2]]))
                + (Simd32x2::from(other[e1234] * self[e4]) * Simd32x2::from([self[e321], self[e4]])),
        )
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        0       11        0      N/A
    //    simd4       24       14        0      N/A
    // Totals...
    // yes simd       30       37        0      N/A
    //  no simd      102      101        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group1().yzxx() * self.group1().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
            - (Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]))
            - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy()).with_w(0.0)
            - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz()).with_w(0.0);
        let geometric_anti_product_g1 = (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
            + (other.group1().yzxx() * self.group0().zxyx())
            + (other.group1().wwwy() * self.group1().xyz().with_w(self[e2]))
            + Simd32x3::from(0.0).with_w((other[e412] * self[e3]) - (other[e3] * self[e412]))
            + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
            - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
            - (other.group0().yzxx() * self.group1().zxyx())
            - (self.group1().wwwy() * other.group1().xyz().with_w(other[e2]))
            - (other.group1().zxy() * self.group0().yzx()).with_w(0.0);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g0[3]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[2] * self[e412]) * -1.0)
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x3::from(anti_reverse_g0[3]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g0[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                        - (geometric_anti_product_g1[0] * self[e423])
                        - (geometric_anti_product_g1[1] * self[e431])
                        - (geometric_anti_product_g1[2] * self[e412]),
                )
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(anti_reverse_g0[3] * geometric_anti_product_g1[3])
                - (geometric_anti_product_g0.zxyx() * self.group1().yzx().with_w(anti_reverse_g0[0])),
        )
    }
}
impl AntiSandwich<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd2        0        4        0      N/A
    //    simd3       12       15        0      N/A
    //    simd4        8        2        0      N/A
    // Totals...
    // yes simd       24       33        0      N/A
    //  no simd       72       73        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group0().zyz()).with_w(0.0)
            + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group0().xxy()).with_w(0.0)
            + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]))
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(other[e43] * self[e412])
            - (other.group0().yzx() * self.group0().zxy()).with_w(other[e42] * self[e431]);
        let geometric_anti_product_g1 = (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
            + (Simd32x3::from(self[e4]) * other.group0()).with_w(other[e31] * self[e431])
            - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e1]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g1.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (geometric_anti_product_g1.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g0.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[1] * self[e1]) - (geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g1[0] * self[e2]))
                + (geometric_anti_product_g1.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (geometric_anti_product_g0.yz() * self.group1().zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0       10        0      N/A
    //    simd4       23       14        0      N/A
    // Totals...
    // yes simd       30       34        0      N/A
    //  no simd       99       96        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from(other[e1234]) * self.group0())
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            + (self.group1().zxy() * other.group1().yzx()).with_w(0.0)
            - (self.group1().xyxy() * other.group1().wwy().with_w(other[e42]))
            - (self.group1().yzzz() * other.group1().zxw().with_w(other[e43]))
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
            - (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0);
        let geometric_anti_product_g1 = (other.group0() * Simd32x3::from(self[e4]).with_w(self[e321]))
            + (self.group1().xyxx() * other.group0().wwy().with_w(other[e23]))
            + (self.group1().yzzy() * other.group0().zxw().with_w(other[e31]))
            + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]))
                + (geometric_anti_product_g1.zxyx() * self.group1().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                - (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (geometric_anti_product_g1 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (geometric_anti_product_g0.zxyx() * self.group1().yzxx())
                + (geometric_anti_product_g1.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g0[1] * self[e431])
                        + (geometric_anti_product_g0[2] * self[e412])
                        + (geometric_anti_product_g1[1] * self[e2])
                        + (geometric_anti_product_g1[2] * self[e3]),
                )
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl AntiSandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       33        0        0
    //    simd2        8       16        0      N/A
    //    simd3       24       30        0      N/A
    //    simd4       23       11        0      N/A
    // Totals...
    // yes simd       72       90        0      N/A
    //  no simd      197      199        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]));
        let geometric_anti_product_g1 = (Simd32x4::from(other[e1234]) * self.group0())
            + (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group2().zyz()).with_w(0.0)
            + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group2().xxy()).with_w(0.0)
            + (other.group3().yzx() * self.group1().zxy()).with_w(0.0)
            - (self.group1().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
            - (self.group1().yzxy() * other.group3().zxy().with_w(other[e42]))
            - (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
            - (other.group2().yzx() * self.group0().zxy()).with_w(other[e43] * self[e412]);
        let geometric_anti_product_g2 = (self.group1().zxy() * other.group4().yzx()) + Simd32x2::from(0.0).with_z((self[e423] * other[e431]) * -1.0)
            - (Simd32x3::from(self[e4]) * other.group4().xyz())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group1().yz() * other.group4().zx()).with_z(0.0);
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (Simd32x3::from(other[e321]) * self.group1().xyz())
            + (self.group0().zxy() * other.group4().yzx())
            + Simd32x2::from(0.0).with_z((self[e423] * other[e2]) - (self[e1] * other[e431]) - (self[e431] * other[e1]))
            + (self.group1().yz() * other.group1().zx()).with_z(0.0)
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (Simd32x3::from(other[e4]) * self.group0().xyz())
            - (self.group0().yz() * other.group4().zx()).with_z(0.0)
            - (self.group1().zx() * other.group1().yz()).with_z(0.0);
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group1())
            + (self.group1().yzxx() * other.group2().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
            + (Simd32x3::from(self[e4]) * other.group2()).with_w(other[e31] * self[e431])
            - (other.group2().yzx() * self.group1().zxy()).with_w(other[scalar] * self[e4]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g0[0] * geometric_anti_product_g4[0])
                    - (anti_reverse_g0[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g0[2] * geometric_anti_product_g4[2]),
                0.0,
            ]) + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g0[3]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]])),
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g0[1]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g2[2] * self[e412]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group1().xyz()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g0[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g0.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g0[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group1().zx().with_z(anti_reverse_g0[3])).with_w(0.0)
                - (self.group1().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g2.zxy() * anti_reverse_g0.yzx()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (geometric_anti_product_g4.zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[0] * self[e431]) * -1.0)
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (geometric_anti_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g0.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g0.zxy() * geometric_anti_product_g4.yzx())
                + Simd32x2::from(0.0)
                    .with_z((geometric_anti_product_g1[1] * self[e423]) - (anti_reverse_g0[0] * geometric_anti_product_g4[1]) - (geometric_anti_product_g1[0] * self[e431]))
                + (geometric_anti_product_g1.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group1().xyz())
                - (anti_reverse_g0.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (Simd32x4::from(anti_reverse_g0[3]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g2[0] * anti_reverse_g0[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g0[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g0[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (geometric_anti_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        5        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       32       43        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4] * -1.0) * self.group0().xyz().with_w(self[e321]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g0[3]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[2] * self[e412]) * -1.0)
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
        )
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       16       19        0      N/A
    //  no simd       49       48        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (self.group1().zxyx() * other.group0().yzxx()) + Simd32x3::from(0.0).with_w((self[e431] * other[e431]) + (self[e412] * other[e412]))
            - (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
            - (self.group1().yzx() * other.group0().zxy()).with_w(0.0);
        let geometric_anti_product_g1 = (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
            + (self.group0().zxyx() * other.group0().yzxx())
            + Simd32x3::from(0.0).with_w((self[e2] * other[e431]) + (self[e3] * other[e412]))
            - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            - (self.group0().yzx() * other.group0().zxy()).with_w(0.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[3]) * self.group1())
                + (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g0[1] * self[e2]) + (geometric_anti_product_g0[2] * self[e3])
                        - (geometric_anti_product_g1[1] * self[e431])
                        - (geometric_anti_product_g1[2] * self[e412]),
                )
                - (Simd32x4::from(self[e4]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]))
                - (self.group1().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        0        7        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       49       54        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]);
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]))
            + (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
            + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
            - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
            - (self.group1().zxyx() * other.group0().yzxx());
        let anti_reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g0[3]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[2] * self[e412]) * -1.0)
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_anti_product_g0.zxyx() * anti_reverse_g0.yzx().with_w(self[e423]))
                - (self.group1().yzxy() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[1])),
        )
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group1().xyz();
        Scalar::from_groups(
            // scalar
            (geometric_anti_product_g0_xyz[0] * self[e423]) + (geometric_anti_product_g0_xyz[1] * self[e431]) + (geometric_anti_product_g0_xyz[2] * self[e412])
                - (self[e4] * self[e4] * other[scalar]),
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        8       15        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[0] * self[e41]) + (geometric_anti_product_g1[1] * self[e42]) + (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       11       18        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1());
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[0] * self[e41]) + (geometric_anti_product_g1[1] * self[e42]) + (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
        )
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd3        0       16        0      N/A
    //    simd4       17        4        0      N/A
    // Totals...
    // yes simd       23       35        0      N/A
    //  no simd       74       79        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
            + (self.group0().xyx() * Simd32x2::from(other[e321]).with_z(other[e2])).with_w(0.0)
            + (self.group0().yzz() * other.group0().zx().with_z(other[e321])).with_w(0.0)
            + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group1().yzz() * other.group1().zx().with_z(other[e4])).with_w(0.0)
            - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]))
            - (self.group0().zxy() * other.group0().yzx()).with_w(self[e42] * other[e431]);
        let geometric_anti_product_g1 = Simd32x3::from(0.0)
            .with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group0().yzz() * other.group1().zx().with_z(other[e4])).with_w(0.0)
            - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23]));
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([geometric_anti_product_g0[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g0[2], geometric_anti_product_g0[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (geometric_anti_product_g1.yzxx() * anti_reverse_g1.zxy().with_w(anti_reverse_g0[0]))
                - (anti_reverse_g1 * Simd32x3::from(geometric_anti_product_g0[3])).with_w(anti_reverse_g0[2] * geometric_anti_product_g1[2])
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()).with_w(anti_reverse_g0[1] * geometric_anti_product_g1[1]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1.yzxx() * anti_reverse_g0.zxy().with_w(anti_reverse_g1[0]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g1[2]) - (anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]),
                )
                + (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[3])).with_w(anti_reverse_g1[1] * geometric_anti_product_g1[1])
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g0[0]),
        )
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0();
        Horizon::from_groups(
            // e321
            (geometric_anti_product_g0_xyz[0] * self[e41]) + (geometric_anti_product_g0_xyz[1] * self[e42]) + (geometric_anti_product_g0_xyz[2] * self[e43]),
        )
    }
}
impl AntiSandwich<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        7       17        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       49       60        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43])) + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(other[e41] * self[e41]);
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
            + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
            - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e23])
            - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]);
        let anti_reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[3])) + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g0.zxy()),
            // e23, e31, e12
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g1[3]))
                + (anti_reverse_g1 * Simd32x3::from(geometric_anti_product_g0[3]))
                + (anti_reverse_g0.zxy() * geometric_anti_product_g1.yzx())
                + (anti_reverse_g1.zxy() * geometric_anti_product_g0.yzx())
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy())
                - (anti_reverse_g1.yzx() * geometric_anti_product_g0.zxy()),
        )
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0       12        0      N/A
    //    simd4       18        6        0      N/A
    // Totals...
    // yes simd       26       30        0      N/A
    //  no simd       80       72        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e42]) - (self[e43] * other[e43]))
            + (self.group0().xyx() * other.group0().wwy()).with_w(0.0)
            + (self.group0().yzz() * other.group0().zxw()).with_w(0.0)
            - (self.group0().zxyx() * other.group0().yzxx());
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e31] * other[e42]) - (self[e12] * other[e43]))
            + (self.group0().xyx() * other.group1().wwy()).with_w(0.0)
            + (self.group0().yzz() * other.group1().zxw()).with_w(0.0)
            + (self.group1().xyx() * other.group0().wwy()).with_w(0.0)
            + (self.group1().yzz() * other.group0().zxw()).with_w(0.0)
            - (self.group0().zxyx() * other.group1().yzxx())
            - (self.group1().zxyx() * other.group0().yzxx());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().yzxx() * geometric_anti_product_g0.zxyx())
                + Simd32x3::from(0.0).with_w((self[e42] * geometric_anti_product_g0[1]) + (self[e43] * geometric_anti_product_g0[2]))
                - (self.group0().xxy() * geometric_anti_product_g0.wzx()).with_w(0.0)
                - (self.group0().zyz() * geometric_anti_product_g0.yww()).with_w(0.0),
            // e23, e31, e12, scalar
            (self.group0().yzxx() * geometric_anti_product_g1.zxyx())
                + (self.group1().yzxx() * geometric_anti_product_g0.zxyx())
                + Simd32x3::from(0.0).with_w(
                    (self[e42] * geometric_anti_product_g1[1])
                        + (self[e43] * geometric_anti_product_g1[2])
                        + (self[e31] * geometric_anti_product_g0[1])
                        + (self[e12] * geometric_anti_product_g0[2]),
                )
                - (self.group0().xxy() * geometric_anti_product_g1.wzx()).with_w(0.0)
                - (self.group0().zyz() * geometric_anti_product_g1.yww()).with_w(0.0)
                - (self.group1().xxy() * geometric_anti_product_g0.wzx()).with_w(0.0)
                - (self.group1().zyz() * geometric_anti_product_g0.yww()).with_w(0.0),
        )
    }
}
impl AntiSandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       21        0        0
    //    simd2        6        6        0      N/A
    //    simd3       14       34        0      N/A
    //    simd4       17        4        0      N/A
    // Totals...
    // yes simd       47       65        0      N/A
    //  no simd      132      151        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
            + (self.group0().xyx() * Simd32x2::from(other[e321]).with_z(other[e2])).with_w(0.0)
            + (self.group0().yzz() * other.group1().zx().with_z(other[e321])).with_w(0.0)
            + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group1().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
            - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]))
            - (self.group0().zxy() * other.group1().yzx()).with_w(self[e42] * other[e431]);
        let geometric_anti_product_g2 =
            (Simd32x3::from(other[e1234]) * self.group0()) + (self.group0().yzx() * other.group2().zxy()) - (self.group0().zxy() * other.group2().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group0())
            + (Simd32x3::from(other[e1234]) * self.group1())
            + (self.group0().yzx() * other.group3().zxy())
            + (self.group1().yzx() * other.group2().zxy())
            - (self.group0().zxy() * other.group3().yzx())
            - (self.group1().zxy() * other.group2().yzx());
        let geometric_anti_product_g4 = Simd32x3::from(0.0)
            .with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group0().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
            - (other.group4().yzxx() * self.group0().zxy().with_w(self[e23]));
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
            (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g0.xxy()).with_w(0.0)
                + (anti_reverse_g1.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (geometric_anti_product_g4.yzxx() * anti_reverse_g1.zxy().with_w(anti_reverse_g0[0]))
                - (anti_reverse_g1 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(anti_reverse_g0[2] * geometric_anti_product_g4[2])
                - (anti_reverse_g0.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g0[1] * geometric_anti_product_g4[1]),
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
            (geometric_anti_product_g4.yzxx() * anti_reverse_g0.zxy().with_w(anti_reverse_g1[0]))
                + Simd32x3::from(0.0).with_w(
                    (anti_reverse_g1[2] * geometric_anti_product_g4[2]) - (anti_reverse_g0[1] * geometric_anti_product_g1[1]) - (anti_reverse_g0[2] * geometric_anti_product_g1[2]),
                )
                + (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(anti_reverse_g1[1] * geometric_anti_product_g4[1])
                - (anti_reverse_g0.yzx() * geometric_anti_product_g4.zxy()).with_w(anti_reverse_g0[0] * geometric_anti_product_g1[0]),
        )
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        7        0      N/A
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd       16       24        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4]) * self.group1();
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product_g1_xyz[2] * self[e43])
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(geometric_anti_product_g1_xyz[0] * self[e41])
                + (geometric_anti_product_g0_xyz.zxy() * self.group0().yzx()).with_w(geometric_anti_product_g1_xyz[1] * self[e42])
                - (anti_reverse_g1.zxy() * geometric_anti_product_g1_xyz.yzx()).with_w(0.0)
                - (geometric_anti_product_g0_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       12       17        0      N/A
    //  no simd       36       36        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
            + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
            + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((self[e42] * geometric_anti_product_g0[1]) + (self[e43] * geometric_anti_product_g0[2]) - (self[e12] * geometric_anti_product_g1[2]))
                + (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(self[e41] * geometric_anti_product_g0[0])
                - (geometric_anti_product_g1.yzxx() * self.group0().zxy().with_w(self[e23]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(self[e31] * geometric_anti_product_g1[1]),
        )
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0       11        0      N/A
    //    simd4        8        0        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       34       39        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (self.group0().zxy() * other.group0().yzx()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0();
        let anti_reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product_g1_xyz[2] * self[e43])
                + (anti_reverse_g1.yzx() * geometric_anti_product_g1_xyz.zxy()).with_w(geometric_anti_product_g1_xyz[0] * self[e41])
                + (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(geometric_anti_product_g1_xyz[1] * self[e42])
                - (Simd32x3::from(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])) * self.group0()).with_w(0.0)
                - (Simd32x3::from([geometric_anti_product_g1_xyz[1], geometric_anti_product_g0[3], geometric_anti_product_g0[3]]) * anti_reverse_g1.zyz()).with_w(0.0)
                - (Simd32x3::from([geometric_anti_product_g0[3], geometric_anti_product_g1_xyz[2], geometric_anti_product_g1_xyz[0]]) * anti_reverse_g1.xxy()).with_w(0.0)
                - (self.group0().zxy() * geometric_anti_product_g0.yzx()).with_w(0.0),
        )
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(other[scalar]) * self.group0();
        Scalar::from_groups(
            // scalar
            (geometric_anti_product_g1[0] * self[e41]) + (geometric_anti_product_g1[1] * self[e42]) + (geometric_anti_product_g1[2] * self[e43]),
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd2        4        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       11       20        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other[e1234]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[0] * self[e41])
                    + (geometric_anti_product_g1[1] * self[e42])
                    + (geometric_anti_product_g1[2] * self[e43])
                    + (geometric_anti_product_g1[3] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]]))
                + (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[scalar], self[e1234]])),
        )
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd2        4        4        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       15       24        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e1234]) * self.group0();
        let geometric_anti_product_g1 = (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1());
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[0] * self[e41])
                    + (geometric_anti_product_g1[1] * self[e42])
                    + (geometric_anti_product_g1[2] * self[e43])
                    + (geometric_anti_product_g1[3] * self[e1234]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]]))
                + (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[scalar], self[e1234]])),
        )
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd3        0       13        0      N/A
    //    simd4       23       12        0      N/A
    // Totals...
    // yes simd       32       39        0      N/A
    //  no simd      101      101        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w((other[e412] * self[e43]) * -1.0)
            + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            + (other.group1().xxy() * self.group1().wzx()).with_w(0.0)
            + (other.group1().zyz() * self.group1().yww()).with_w(0.0)
            - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
            - (self.group0().zxyx() * other.group0().yzx().with_w(other[e423]));
        let geometric_anti_product_g1 = (self.group0() * Simd32x3::from(other[e4]).with_w(other[e321]))
            + Simd32x3::from(0.0).with_w(-(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
            + (other.group1().xxy() * self.group0().wzx()).with_w(other[e4] * self[scalar])
            + (other.group1().zyz() * self.group0().yww()).with_w(0.0)
            - (self.group0().zxyx() * other.group1().yzx().with_w(other[e1]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g0[3]))
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (geometric_anti_product_g1.xxyy() * self.group1().wzx().with_w(anti_reverse_g0[1]))
                - (geometric_anti_product_g1.zyzz() * self.group1().yww().with_w(anti_reverse_g0[2])),
            // e423, e431, e412, e321
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[3]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g0[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                        - (geometric_anti_product_g0[3] * self[scalar])
                        - (geometric_anti_product_g1[0] * self[e23])
                        - (geometric_anti_product_g1[1] * self[e31])
                        - (geometric_anti_product_g1[2] * self[e12]),
                )
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g1.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e321]) * self.group0().xyz();
        Horizon::from_groups(
            // e321
            (geometric_anti_product_g0_xyz[0] * self[e41])
                + (geometric_anti_product_g0_xyz[1] * self[e42])
                + (geometric_anti_product_g0_xyz[2] * self[e43])
                + (self[e1234] * self[e1234] * other[e321]),
        )
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd2        0        4        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       27       32        0      N/A
    //  no simd       78       73        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43]))
            + (other.group0().xxy() * self.group0().wzx()).with_w(0.0)
            + (other.group0().zyz() * self.group0().yww()).with_w(0.0)
            - (other.group0().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (other.group0().xxy() * self.group1().wzx()).with_w(0.0)
            + (other.group0().zyz() * self.group1().yww()).with_w(0.0)
            + (other.group1().xxy() * self.group0().wzx()).with_w(0.0)
            + (other.group1().zyz() * self.group0().yww()).with_w(0.0)
            - (other.group0().yzxx() * self.group1().zxyx())
            - (other.group1().yzxx() * self.group0().zxyx());
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g0[0] * self[e42]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (geometric_anti_product_g0.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from(self[scalar]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[1] * self[e41]) - (geometric_anti_product_g0[0] * self[e31]) - (geometric_anti_product_g1[0] * self[e42]))
                + (geometric_anti_product_g1.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g0.yz() * self.group1().zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0       12        0      N/A
    //    simd4       24       14        0      N/A
    // Totals...
    // yes simd       32       38        0      N/A
    //  no simd      104      104        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43]))
            + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (other.group0().yzxx() * self.group0().zxyx());
        let geometric_anti_product_g1 = (Simd32x4::from(self[e1234]) * other.group1())
            + (Simd32x4::from(self[scalar]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
            + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
            - (other.group0().yzxx() * self.group1().zxyx())
            - (other.group1().yzxx() * self.group0().zxyx());
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g0[3]))
                + Simd32x3::from(0.0).with_w(-(anti_reverse_g0[1] * geometric_anti_product_g0[1]) - (anti_reverse_g0[2] * geometric_anti_product_g0[2]))
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxyx()),
            // e23, e31, e12, scalar
            (anti_reverse_g0 * Simd32x4::from(geometric_anti_product_g1[3]))
                + (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[3]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g0[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                        - (anti_reverse_g1[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g1[2] * geometric_anti_product_g0[2]),
                )
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g1.yzx()).with_w(0.0)
                + (anti_reverse_g1.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxyx())
                - (anti_reverse_g1.yzxx() * geometric_anti_product_g0.zxyx()),
        )
    }
}
impl AntiSandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       22        0        0
    //    simd2        8        8        0      N/A
    //    simd3       20       37        0      N/A
    //    simd4       23       12        0      N/A
    // Totals...
    // yes simd       67       79        0      N/A
    //  no simd      184      197        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[e1234] * self[scalar]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
            + (Simd32x2::from(self[e1234]) * other.group0())
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = (self.group0() * Simd32x3::from(other[e321]).with_w(other[e4]))
            + Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
            + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
            + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group1().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
            - (self.group0().zxyx() * other.group1().yzx().with_w(other[e423]))
            - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]));
        let geometric_anti_product_g2 =
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * other.group2()) + (other.group2().zxy() * self.group0().yzx())
                - (other.group2().yzx() * self.group0().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group0().xyz())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz())
            + (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (other.group2().zxy() * self.group1().yzx())
            + (other.group3().zxy() * self.group0().yzx())
            - (other.group2().yzx() * self.group1().zxy())
            - (other.group3().yzx() * self.group0().zxy());
        let geometric_anti_product_g4 = (self.group0() * Simd32x3::from(other[e4]).with_w(other[e321]))
            + Simd32x3::from(0.0).with_w(
                (self[scalar] * other[e4]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            )
            + (Simd32x3::from(self[e1234]) * other.group4().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group4().zxy()).with_w(0.0)
            - (self.group0().zxyx() * other.group4().yzx().with_w(other[e1]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g2[0] * self[e23])
                    + (geometric_anti_product_g2[1] * self[e31])
                    + (geometric_anti_product_g2[2] * self[e12]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(anti_reverse_g0[3]))
                - (Simd32x2::from(anti_reverse_g0[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g0[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g0[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]])),
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g4[3]).with_w(geometric_anti_product_g1[3]))
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g1.yzx()).with_w(0.0)
                + (geometric_anti_product_g4.yzx() * self.group1().zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g4[0]))
                - (geometric_anti_product_g4.xxyy() * self.group1().wzx().with_w(anti_reverse_g0[1]))
                - (geometric_anti_product_g4.zyzz() * self.group1().yww().with_w(anti_reverse_g0[2])),
            // e41, e42, e43
            (geometric_anti_product_g2 * Simd32x3::from(anti_reverse_g0[3]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * anti_reverse_g0.xyz())
                + (geometric_anti_product_g2.yzx() * anti_reverse_g0.zxy())
                - (geometric_anti_product_g2.zxy() * anti_reverse_g0.yzx()),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(anti_reverse_g0[3]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * anti_reverse_g0.xyz())
                + (geometric_anti_product_g2.zxy() * self.group1().yzx())
                + (geometric_anti_product_g3.yzx() * anti_reverse_g0.zxy())
                - (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz())
                - (geometric_anti_product_g2.yzx() * self.group1().zxy())
                - (geometric_anti_product_g3.zxy() * anti_reverse_g0.yzx()),
            // e423, e431, e412, e321
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g4[3]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g0[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g1[3] * self[scalar])
                        - (geometric_anti_product_g4[0] * self[e23])
                        - (geometric_anti_product_g4[1] * self[e31])
                        - (geometric_anti_product_g4[2] * self[e12]),
                )
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g4.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g4.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g4.zxy().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        4        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       28       40        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e1234]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]);
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g0[3]))
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g0.yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                - (geometric_anti_product_g1.xxyy() * self.group1().wzx().with_w(anti_reverse_g0[1]))
                - (geometric_anti_product_g1.zyzz() * self.group1().yww().with_w(anti_reverse_g0[2])),
        )
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       51       52        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
            + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_anti_product_g1 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (anti_reverse_g0 * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[3]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g0[1] * geometric_anti_product_g0[1])
                        - (anti_reverse_g0[2] * geometric_anti_product_g0[2])
                        - (geometric_anti_product_g0[3] * self[scalar])
                        - (geometric_anti_product_g1[0] * self[e23])
                        - (geometric_anti_product_g1[1] * self[e31])
                        - (geometric_anti_product_g1[2] * self[e12]),
                )
                + (Simd32x3::from(anti_reverse_g0[3]) * geometric_anti_product_g1.xyz()).with_w(0.0)
                + (anti_reverse_g0.zxy() * geometric_anti_product_g1.yzx()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       10        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       43       53        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = (Simd32x3::from(self[e1234]) * other.group0().xyz())
            + (Simd32x3::from(other[e4]) * self.group1().xyz())
            + (self.group0().yzx() * other.group0().zxy())
            + Simd32x2::from(0.0).with_z((self[e42] * other[e1]) * -1.0)
            - (self.group0().zx() * other.group0().yz()).with_z(0.0);
        let geometric_anti_product_g0_w = self[e1234] * other[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e4]) * self.group0().xyz();
        let anti_reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Point::from_groups(
            // e1, e2, e3, e4
            (anti_reverse_g0
                * Simd32x3::from((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])).with_w(geometric_anti_product_g0_w))
                + (Simd32x3::from(geometric_anti_product_g0_w) * self.group1().xyz()).with_w(0.0)
                + (geometric_anti_product_g0_xyz.xyx() * anti_reverse_g0.wwy()).with_w(0.0)
                + (geometric_anti_product_g0_xyz.yzz() * anti_reverse_g0.zxw()).with_w(0.0)
                + (geometric_anti_product_g1_xyz.yzx() * self.group1().zxy()).with_w(0.0)
                - (anti_reverse_g0.yzxx() * geometric_anti_product_g0_xyz.zxy().with_w(geometric_anti_product_g1_xyz[0]))
                - (geometric_anti_product_g1_xyz.xxy() * self.group1().wzx()).with_w(geometric_anti_product_g1_xyz[1] * anti_reverse_g0[1])
                - (geometric_anti_product_g1_xyz.zyz() * self.group1().yww()).with_w(geometric_anti_product_g1_xyz[2] * anti_reverse_g0[2]),
        )
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x4::from(other[scalar]) * self.group0();
        Scalar::from_groups(
            // scalar
            (geometric_anti_product_g1[0] * self[e41])
                + (geometric_anti_product_g1[1] * self[e42])
                + (geometric_anti_product_g1[2] * self[e43])
                + (geometric_anti_product_g1[3] * self[e1234]),
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
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd2        8        9        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        8        8        0      N/A
    // Totals...
    // yes simd       30       39        0      N/A
    //  no simd       62       80        0        0
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
                (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g3[0] * self[e41])
                    + (geometric_anti_product_g3[1] * self[e42])
                    + (geometric_anti_product_g3[2] * self[e43])
                    + (geometric_anti_product_g4[0] * self[e1])
                    + (geometric_anti_product_g4[1] * self[e2])
                    + (geometric_anti_product_g4[2] * self[e3])
                    + (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                + (Simd32x2::from(geometric_anti_product_g1[3]) * Simd32x2::from([self[e321], self[e4]]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g2[2] * self[e3])
                        + (self[e41] * geometric_anti_product_g1[0])
                        + (self[e42] * geometric_anti_product_g1[1])
                        + (self[e43] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (self[e31] * geometric_anti_product_g4[1])
                        - (self[e12] * geometric_anti_product_g4[2]),
                )
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g2[0] * self[e1])
                + (self.group2().yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g2[1] * self[e2])
                - (Simd32x4::from(geometric_anti_product_g1[3]) * self.group2().with_w(self[scalar]))
                - (Simd32x4::from(self[e4]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g4.yzxx() * self.group2().zxy().with_w(self[e23]))
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       25        0        0
    //    simd2        8        8        0      N/A
    //    simd3        2       10        0      N/A
    //    simd4        8        5        0      N/A
    // Totals...
    // yes simd       34       48        0      N/A
    //  no simd       70       91        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]);
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(other[scalar]) * self.group4().xyz());
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(other[e1234]) * self.group2();
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3());
        let geometric_anti_product_g4 = (Simd32x3::from(other[e1234]) * self.group4().xyz()).with_w((other[e1234] * self[e321]) - (other[scalar] * self[e4]));
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g2[0] * self[e23])
                    + (geometric_anti_product_g2[1] * self[e31])
                    + (geometric_anti_product_g2[2] * self[e12])
                    + (geometric_anti_product_g4[0] * self[e1])
                    + (geometric_anti_product_g4[1] * self[e2])
                    + (geometric_anti_product_g4[2] * self[e3])
                    + (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g1_w) * Simd32x2::from([self[e321], self[e4]]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1_xyz[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1_xyz[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1_xyz[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g1_w * self[scalar])
                        - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0])
                        - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (self[e31] * geometric_anti_product_g4[1])
                        - (self[e12] * geometric_anti_product_g4[2]),
                )
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g1_w)).with_w(geometric_anti_product_g2[0] * self[e1])
                + (anti_reverse_g2.zxy() * geometric_anti_product_g4.yzx()).with_w(geometric_anti_product_g2[1] * self[e2])
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g2[2] * self[e3])
                - (Simd32x4::from(self[e4]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g4.zxyx() * anti_reverse_g2.yzx().with_w(self[e23]))
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       27       45        0        0
    //    simd2       12       22        0      N/A
    //    simd3       36       49        0      N/A
    //    simd4       35       16        0      N/A
    // Totals...
    // yes simd      110      132        0      N/A
    //  no simd      299      300        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]), 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]));
        let geometric_anti_product_g1 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
            + (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0)
            + (self.group2().xyx() * Simd32x2::from(other[e321]).with_z(other[e2])).with_w(0.0)
            + (self.group2().yzz() * other.group0().zx().with_z(other[e321])).with_w(0.0)
            + (self.group3().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group3().yzz() * other.group1().zx().with_z(other[e4])).with_w(0.0)
            - (other.group1().yzxx() * self.group3().zxy().with_w(self[e41]))
            - (self.group2().zxy() * other.group0().yzx()).with_w(self[e42] * other[e431]);
        let geometric_anti_product_g2 = (other.group1().yzx() * self.group4().zxy()) + Simd32x2::from(0.0).with_z((other[e431] * self[e423]) * -1.0)
            - (Simd32x3::from(other[e4]) * self.group4().xyz())
            - (Simd32x3::from(self[e4]) * other.group1().xyz())
            - (other.group1().zx() * self.group4().yz()).with_z(0.0);
        let geometric_anti_product_g3 = (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from(self[e4]) * other.group0().xyz())
            + (other.group0().zxy() * self.group4().yzx())
            + Simd32x2::from(0.0).with_z((other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]))
            + (other.group1().yz() * self.group1().zx()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group1().xyz())
            - (other.group0().yz() * self.group4().zx()).with_z(0.0)
            - (other.group1().zx() * self.group1().yz()).with_z(0.0);
        let geometric_anti_product_g4 = (Simd32x4::from(self[e1234]) * other.group1())
            + (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
            + Simd32x3::from(0.0).with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
            - (other.group1().yzxx() * self.group2().zxy().with_w(self[e23]));
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g4[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from(anti_reverse_g1[3]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]])),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (geometric_anti_product_g4.xyzx() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g2[0]))
                - (geometric_anti_product_g4.yzxy() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[1]))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(geometric_anti_product_g2[1] * self[e431])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[2] * geometric_anti_product_g4[2])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[0] * self[e431]))
                + (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (anti_reverse_g1[1] * geometric_anti_product_g4[0]) + (geometric_anti_product_g1[1] * self[e423])
                        - (anti_reverse_g1[0] * geometric_anti_product_g4[1])
                        - (geometric_anti_product_g1[0] * self[e431]),
                )
                + (anti_reverse_g1.zx() * geometric_anti_product_g4.yz()).with_z(0.0)
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (Simd32x4::from(anti_reverse_g1[3]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                + (geometric_anti_product_g4.yzxx() * anti_reverse_g2.zxy().with_w(anti_reverse_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                        - (anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(anti_reverse_g3[1] * geometric_anti_product_g4[1])
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(anti_reverse_g3[2] * geometric_anti_product_g4[2])
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(self[scalar] * geometric_anti_product_g1[3]),
        )
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       14       24        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = self[e4] * other[e321];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[e321]) * self.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other[e321]) * self.group4().xyz();
        let geometric_anti_product_g4_w = self[e1234] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234])
                    + (geometric_anti_product_g4_w * self[e4])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    + (geometric_anti_product_g3[0] * self[e41])
                    + (geometric_anti_product_g3[1] * self[e42])
                    + (geometric_anti_product_g3[2] * self[e43]),
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
                (geometric_anti_product_g4_w * self[e1234])
                    + (geometric_anti_product_g1_xyz[0] * self[e41])
                    + (geometric_anti_product_g1_xyz[1] * self[e42])
                    + (geometric_anti_product_g1_xyz[2] * self[e43])
                    - (geometric_anti_product_g0_x * self[e4])
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
    //           add/sub      mul      div      pow
    //      f32        8       19        0        0
    //    simd2        3        9        0      N/A
    //    simd3       31       47        0      N/A
    //    simd4       23        8        0      N/A
    // Totals...
    // yes simd       65       83        0      N/A
    //  no simd      199      210        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_anti_product_g1 = (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group0().zyz()).with_w(0.0)
            + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group0().xxy()).with_w(0.0)
            + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
            - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]))
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(other[e43] * self[e412])
            - (other.group0().yzx() * self.group1().zxy()).with_w(other[e42] * self[e431]);
        let geometric_anti_product_g2 =
            (Simd32x3::from(self[e1234]) * other.group0()) + (other.group0().zxy() * self.group2().yzx()) - (other.group0().yzx() * self.group2().zxy());
        let geometric_anti_product_g3 = (Simd32x3::from(self[scalar]) * other.group0())
            + (Simd32x3::from(self[e1234]) * other.group1())
            + (other.group0().zxy() * self.group3().yzx())
            + (other.group1().zxy() * self.group2().yzx())
            - (other.group0().yzx() * self.group3().zxy())
            - (other.group1().yzx() * self.group2().zxy());
        let geometric_anti_product_g4 = (self.group4().yzxx() * other.group0().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
            + (Simd32x3::from(self[e4]) * other.group0()).with_w(other[e31] * self[e431])
            - (other.group0().yzx() * self.group4().zxy()).with_w(other[e41] * self[e1]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (geometric_anti_product_g4.xyzx() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g2[0]))
                - (geometric_anti_product_g4.yzxy() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[1]))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(geometric_anti_product_g2[1] * self[e431])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[2] * geometric_anti_product_g4[2])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[0] * self[e431]))
                + (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (anti_reverse_g1[1] * geometric_anti_product_g4[0]) + (geometric_anti_product_g1[1] * self[e423])
                        - (anti_reverse_g1[0] * geometric_anti_product_g4[1])
                        - (geometric_anti_product_g1[0] * self[e431]),
                )
                + (anti_reverse_g1.zx() * geometric_anti_product_g4.yz()).with_z(0.0)
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiSandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       36        0        0
    //    simd2       12       18        0      N/A
    //    simd3       34       50        0      N/A
    //    simd4       34       19        0      N/A
    // Totals...
    // yes simd      104      123        0      N/A
    //  no simd      286      298        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([(self[e1234] * other[scalar]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
            + (Simd32x2::from(other[e1234]) * self.group0())
            - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_anti_product_g1 = (other.group0() * Simd32x3::from(self[e321]).with_w(self[e4]))
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
            + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group1().zxy().with_w(self[e423]))
            - (self.group4().xyzz() * Simd32x3::from(other[scalar]).with_w(other[e43]))
            - (self.group4().yzxy() * other.group1().zxy().with_w(other[e42]))
            - (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0);
        let geometric_anti_product_g2 =
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e1234]) * self.group2()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx());
        let geometric_anti_product_g3 = (Simd32x3::from(self[scalar]) * other.group0().xyz())
            + (Simd32x3::from(self[e1234]) * other.group1().xyz())
            + (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[scalar]) * self.group2())
            + (self.group2().yzx() * other.group1().zxy())
            + (self.group3().yzx() * other.group0().zxy())
            - (self.group2().zxy() * other.group1().yzx())
            - (self.group3().zxy() * other.group0().yzx());
        let geometric_anti_product_g4 = (other.group0() * Simd32x3::from(self[e4]).with_w(self[e321]))
            + (self.group4().xyzy() * Simd32x3::from(other[e1234]).with_w(other[e31]))
            + (self.group4().yzxx() * other.group0().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
            - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1]));
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g4[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from(anti_reverse_g1[3]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]])),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (geometric_anti_product_g4.xyzx() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g2[0]))
                - (geometric_anti_product_g4.yzxy() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[1]))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(geometric_anti_product_g2[1] * self[e431])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[2] * geometric_anti_product_g4[2])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[0] * self[e431]))
                + (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (anti_reverse_g1[1] * geometric_anti_product_g4[0]) + (geometric_anti_product_g1[1] * self[e423])
                        - (anti_reverse_g1[0] * geometric_anti_product_g4[1])
                        - (geometric_anti_product_g1[0] * self[e431]),
                )
                + (anti_reverse_g1.zx() * geometric_anti_product_g4.yz()).with_z(0.0)
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (Simd32x4::from(anti_reverse_g1[3]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                + (geometric_anti_product_g4.yzxx() * anti_reverse_g2.zxy().with_w(anti_reverse_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                        - (anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(anti_reverse_g3[1] * geometric_anti_product_g4[1])
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(anti_reverse_g3[2] * geometric_anti_product_g4[2])
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(self[scalar] * geometric_anti_product_g1[3]),
        )
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       36       56        0        0
    //    simd2       16       28        0      N/A
    //    simd3       48       66        0      N/A
    //    simd4       46       21        0      N/A
    // Totals...
    // yes simd      146      171        0      N/A
    //  no simd      396      394        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([
            (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                - (other[e23] * self[e41])
                - (other[e31] * self[e42])
                - (other[e12] * self[e43])
                - (other[e1] * self[e423])
                - (other[e2] * self[e431])
                - (other[e3] * self[e412]),
            0.0,
        ]) + (Simd32x2::from(self[e1234]) * other.group0())
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
            - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]))
            - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]));
        let geometric_anti_product_g1 = (Simd32x4::from(other[e1234]) * self.group1())
            + (Simd32x4::from(self[e1234]) * other.group1())
            + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
            + (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group2().zyz()).with_w(0.0)
            + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group2().xxy()).with_w(0.0)
            + (other.group3().yzx() * self.group4().zxy()).with_w(0.0)
            + (self.group2().xyx() * Simd32x2::from(other[e321]).with_z(other[e2])).with_w(0.0)
            + (self.group2().yzz() * other.group1().zx().with_z(other[e321])).with_w(0.0)
            + (self.group3().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
            + (self.group3().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
            - (other.group4().yzxx() * self.group3().zxy().with_w(self[e41]))
            - (self.group4().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
            - (self.group4().yzxy() * other.group3().zxy().with_w(other[e42]))
            - (Simd32x3::from(self[e4]) * other.group3()).with_w(self[e42] * other[e431])
            - (other.group2().yzx() * self.group1().zxy()).with_w(other[e43] * self[e412])
            - (self.group2().zxy() * other.group1().yzx()).with_w(self[e43] * other[e412]);
        let geometric_anti_product_g2 = (Simd32x3::from(other[e1234]) * self.group2())
            + (Simd32x3::from(self[e1234]) * other.group2())
            + (other.group2().zxy() * self.group2().yzx())
            + Simd32x2::from(0.0).with_z((other[e423] * self[e431]) - (other[e431] * self[e423]))
            + (other.group4().yz() * self.group4().zx()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group4().xyz())
            - (Simd32x3::from(self[e4]) * other.group4().xyz())
            - (other.group2().yzx() * self.group2().zxy())
            - (other.group4().zx() * self.group4().yz()).with_z(0.0);
        let geometric_anti_product_g3 = (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (other.group2().zxy() * self.group3().yzx())
            + (other.group3().zxy() * self.group2().yzx())
            + Simd32x2::from(0.0).with_z((other[e2] * self[e423]) + (other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]))
            + (other.group1().zx() * self.group4().yz()).with_z(0.0)
            + (other.group4().yz() * self.group1().zx()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (other.group2().yzx() * self.group3().zxy())
            - (other.group3().yzx() * self.group2().zxy())
            - (other.group1().yz() * self.group4().zx()).with_z(0.0)
            - (other.group4().zx() * self.group1().yz()).with_z(0.0);
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group4())
            + (Simd32x4::from(self[e1234]) * other.group4())
            + (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
            + (self.group4().yzxx() * other.group2().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w(
                -(other[e41] * self[e1])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (self[e41] * other[e1])
                    - (self[e42] * other[e2])
                    - (self[e43] * other[e3])
                    - (self[e31] * other[e431])
                    - (self[e12] * other[e412]),
            )
            + (Simd32x3::from(self[e4]) * other.group2()).with_w(other[e31] * self[e431])
            + (self.group2().yzx() * other.group4().zxy()).with_w(other[e12] * self[e412])
            - (other.group4().yzxx() * self.group2().zxy().with_w(self[e23]))
            - (other.group2().yzx() * self.group4().zxy()).with_w(other[scalar] * self[e4]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g1[3] * self[e321])
                    - (anti_reverse_g3[0] * geometric_anti_product_g2[0])
                    - (anti_reverse_g3[1] * geometric_anti_product_g2[1])
                    - (anti_reverse_g3[2] * geometric_anti_product_g2[2])
                    - (anti_reverse_g1[0] * geometric_anti_product_g4[0])
                    - (anti_reverse_g1[1] * geometric_anti_product_g4[1])
                    - (anti_reverse_g1[2] * geometric_anti_product_g4[2]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]]))
                - (Simd32x2::from(anti_reverse_g2[0]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(anti_reverse_g2[1]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(anti_reverse_g2[2]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]]))
                - (Simd32x2::from(anti_reverse_g1[3]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]])),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (geometric_anti_product_g4.xyzx() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g2[0]))
                - (geometric_anti_product_g4.yzxy() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[1]))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(geometric_anti_product_g2[1] * self[e431])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[2] * geometric_anti_product_g4[2])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[0] * self[e431]))
                + (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (anti_reverse_g1[1] * geometric_anti_product_g4[0]) + (geometric_anti_product_g1[1] * self[e423])
                        - (anti_reverse_g1[0] * geometric_anti_product_g4[1])
                        - (geometric_anti_product_g1[0] * self[e431]),
                )
                + (anti_reverse_g1.zx() * geometric_anti_product_g4.yz()).with_z(0.0)
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (Simd32x4::from(anti_reverse_g1[3]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                + (geometric_anti_product_g4.yzxx() * anti_reverse_g2.zxy().with_w(anti_reverse_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    -(anti_reverse_g2[0] * geometric_anti_product_g1[0])
                        - (anti_reverse_g2[1] * geometric_anti_product_g1[1])
                        - (anti_reverse_g2[2] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g2[0] * anti_reverse_g1[0])
                        - (geometric_anti_product_g2[1] * anti_reverse_g1[1])
                        - (geometric_anti_product_g2[2] * anti_reverse_g1[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412]),
                )
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(anti_reverse_g3[1] * geometric_anti_product_g4[1])
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(anti_reverse_g3[2] * geometric_anti_product_g4[2])
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                - (anti_reverse_g2.yzx() * geometric_anti_product_g4.zxy()).with_w(self[scalar] * geometric_anti_product_g1[3]),
        )
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd2        0        7        0      N/A
    //    simd3       24       33        0      N/A
    //    simd4       15        8        0      N/A
    // Totals...
    // yes simd       43       60        0      N/A
    //  no simd      136      157        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(other[e4] * -1.0) * Simd32x2::from([self[e321], self[e4]]);
        let geometric_anti_product_g1 = Simd32x4::from(other[e4]) * self.group3().with_w(self[e1234]);
        let geometric_anti_product_g2 = Simd32x3::from(other[e4] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = Simd32x3::from(other[e4] * -1.0) * self.group1().xyz();
        let geometric_anti_product_g4 = Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]]) * anti_reverse_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]]) * anti_reverse_g2.xxy()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (geometric_anti_product_g4.xyzx() * Simd32x3::from(self[scalar]).with_w(anti_reverse_g2[0]))
                - (geometric_anti_product_g4.yzxy() * anti_reverse_g3.zxy().with_w(anti_reverse_g2[1]))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g1[3])).with_w(geometric_anti_product_g2[1] * self[e431])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[2] * geometric_anti_product_g4[2])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[0] * self[e431]))
                + (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0[0]))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0[1]))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (anti_reverse_g1[1] * geometric_anti_product_g4[0]) + (geometric_anti_product_g1[1] * self[e423])
                        - (anti_reverse_g1[0] * geometric_anti_product_g4[1])
                        - (geometric_anti_product_g1[0] * self[e431]),
                )
                + (anti_reverse_g1.zx() * geometric_anti_product_g4.yz()).with_z(0.0)
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (anti_reverse_g1.yz() * geometric_anti_product_g4.zx()).with_z(0.0)
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       27        0        0
    //    simd2       11       13        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4       15        9        0      N/A
    // Totals...
    // yes simd       49       60        0      N/A
    //  no simd      119      122        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from([self[e4] * other[e321], 0.0])
            + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
            + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
            + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]));
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
            + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
            + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
            + (self.group3().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41]));
        let geometric_anti_product_g2 = (self.group4().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((self[e423] * other[e431]) * -1.0)
            - (Simd32x3::from(self[e4]) * other.group0().xyz())
            - (self.group4().yz() * other.group0().zx()).with_z(0.0);
        let geometric_anti_product_g3 =
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((self[e1] * other[e431]) * -1.0)
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().yz() * other.group0().zx()).with_z(0.0);
        let geometric_anti_product_g4 = (Simd32x4::from(self[e1234]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g3[0] * self[e41])
                    + (geometric_anti_product_g3[1] * self[e42])
                    + (geometric_anti_product_g3[2] * self[e43])
                    + (geometric_anti_product_g4[0] * self[e1])
                    + (geometric_anti_product_g4[1] * self[e2])
                    + (geometric_anti_product_g4[2] * self[e3])
                    + (geometric_anti_product_g4[3] * self[e4]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                + (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                + (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                + (Simd32x2::from(geometric_anti_product_g1[3]) * Simd32x2::from([self[e321], self[e4]]))
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g2[2] * self[e3])
                        + (self[e41] * geometric_anti_product_g1[0])
                        + (self[e42] * geometric_anti_product_g1[1])
                        + (self[e43] * geometric_anti_product_g1[2])
                        - (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g3[2] * self[e412])
                        - (self[e31] * geometric_anti_product_g4[1])
                        - (self[e12] * geometric_anti_product_g4[2]),
                )
                + (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g2[0] * self[e1])
                + (self.group2().yzx() * geometric_anti_product_g4.zxy()).with_w(geometric_anti_product_g2[1] * self[e2])
                - (Simd32x4::from(geometric_anti_product_g1[3]) * self.group2().with_w(self[scalar]))
                - (Simd32x4::from(self[e4]) * geometric_anti_product_g2.with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g4.yzxx() * self.group2().zxy().with_w(self[e23]))
                - (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       21        0        0
    //    simd2        0        4        0      N/A
    //    simd3       30       43        0      N/A
    //    simd4       15        6        0      N/A
    // Totals...
    // yes simd       53       74        0      N/A
    //  no simd      158      182        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]);
        let geometric_anti_product_g0_y = self[e4] * other[e4] * -1.0;
        let geometric_anti_product_g1 =
            ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx()))
            .with_w(self[e1234] * other[e4]);
        let geometric_anti_product_g2 = Simd32x3::from(other[e4] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = (Simd32x3::from(self[e4]) * other.group0().xyz())
            + Simd32x2::from(0.0).with_z((self[e423] * other[e2]) - (self[e431] * other[e1]))
            + (self.group4().yz() * other.group0().zx()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group4().zx() * other.group0().yz()).with_z(0.0);
        let geometric_anti_product_g4_xyz = Simd32x3::from(other[e4]) * self.group2();
        let geometric_anti_product_g4_w = (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]);
        let anti_reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        let anti_reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let anti_reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (anti_reverse_g1 * Simd32x4::from(geometric_anti_product_g0_y))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g4_w)).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g0_x) * self.group4().xyz()).with_w(0.0)
                + (anti_reverse_g2.zxy() * geometric_anti_product_g1.yzx()).with_w(0.0)
                + (anti_reverse_g3.yzx() * geometric_anti_product_g4_xyz.zxy()).with_w(0.0)
                + (geometric_anti_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(anti_reverse_g1[1])).with_w(0.0)
                + (geometric_anti_product_g2.yzz() * anti_reverse_g1.zx().with_z(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.xyx() * Simd32x2::from(anti_reverse_g1[3]).with_z(self[e431])).with_w(0.0)
                + (geometric_anti_product_g3.yzz() * self.group4().zx().with_z(anti_reverse_g1[3])).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g4_xyz[1],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g2[1] * self[e431],
                ]) * anti_reverse_g3.zyz().with_w(1.0))
                - (Simd32x4::from([
                    geometric_anti_product_g1[3],
                    geometric_anti_product_g4_xyz[2],
                    geometric_anti_product_g4_xyz[0],
                    anti_reverse_g2[2] * geometric_anti_product_g4_xyz[2],
                ]) * anti_reverse_g3.xxy().with_w(1.0))
                - (self.group4().yzxx() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[0]))
                - (geometric_anti_product_g4_xyz * Simd32x3::from(self[scalar])).with_w(anti_reverse_g2[0] * geometric_anti_product_g4_xyz[0])
                - (anti_reverse_g2.yzx() * geometric_anti_product_g1.zxy()).with_w(anti_reverse_g2[1] * geometric_anti_product_g4_xyz[1])
                - (geometric_anti_product_g2.zxy() * anti_reverse_g1.yzx()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0_y))
                + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (anti_reverse_g2.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4_xyz.zxy() * self.group4().yzx())
                - (geometric_anti_product_g4_xyz * Simd32x3::from(anti_reverse_g1[3]))
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4_xyz.yzx() * self.group4().zxy()),
            // e23, e31, e12
            (anti_reverse_g2 * Simd32x3::from(geometric_anti_product_g0_x))
                + (anti_reverse_g3 * Simd32x3::from(geometric_anti_product_g0_y))
                + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (geometric_anti_product_g4_xyz * Simd32x3::from(self[e321]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * anti_reverse_g1.xyz())
                + (anti_reverse_g2.zxy() * geometric_anti_product_g3.yzx())
                + (anti_reverse_g3.zxy() * geometric_anti_product_g2.yzx())
                + (geometric_anti_product_g4_xyz.yzx() * anti_reverse_g1.zxy())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[1] * self[e423]) - (geometric_anti_product_g1[0] * self[e431]))
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g4_w) * self.group4().xyz())
                - (Simd32x3::from(anti_reverse_g1[3]) * geometric_anti_product_g1.xyz())
                - (anti_reverse_g2.yzx() * geometric_anti_product_g3.zxy())
                - (anti_reverse_g3.yzx() * geometric_anti_product_g2.zxy())
                - (geometric_anti_product_g4_xyz.zxy() * anti_reverse_g1.yzx())
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       20        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd       14       22        0      N/A
    //  no simd       14       26        0        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = self[e1234] * other[scalar];
        let geometric_anti_product_g1_xyz = Simd32x3::from(other[scalar] * -1.0) * self.group4().xyz();
        let geometric_anti_product_g3 = Simd32x3::from(other[scalar]) * self.group2();
        let geometric_anti_product_g4_w = self[e4] * other[scalar] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234])
                    + (geometric_anti_product_g4_w * self[e4])
                    + (geometric_anti_product_g1_xyz[0] * self[e423])
                    + (geometric_anti_product_g1_xyz[1] * self[e431])
                    + (geometric_anti_product_g1_xyz[2] * self[e412])
                    + (geometric_anti_product_g3[0] * self[e41])
                    + (geometric_anti_product_g3[1] * self[e42])
                    + (geometric_anti_product_g3[2] * self[e43]),
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
                (geometric_anti_product_g4_w * self[e1234])
                    + (geometric_anti_product_g1_xyz[0] * self[e41])
                    + (geometric_anti_product_g1_xyz[1] * self[e42])
                    + (geometric_anti_product_g1_xyz[2] * self[e43])
                    - (geometric_anti_product_g0_x * self[e4])
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e4] * self[e4])
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       18        0        0
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[e4] * self[e4] * -1.0)
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       26        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        3        0      N/A
    //    simd3        0        5        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0       18        0      N/A
    //  no simd        0       46        0        0
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
            Simd32x4::from(anti_reverse_g0) * (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(geometric_anti_product_g0[1]),
            // e41, e42, e43
            Simd32x3::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(anti_reverse_g0 * -1.0) * geometric_anti_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_reverse_g0) * (Simd32x3::from(self[e4] * -1.0) * other.group4().xyz()).with_w(geometric_anti_product_g0[0]),
        )
    }
}
impl AntiSandwich<Origin> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * self[e4] * self[e4])
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       10        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4] * -1.0) * other.group0().xyz()).with_w(other[e321] * self[e4]),
        )
    }
}
impl AntiSandwich<Point> for Origin {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       10        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(other[e4] * self[e4] * -1.0),
        )
    }
}
impl AntiSandwich<Scalar> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        2        3        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        4       14        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        8        0      N/A
    //    simd4       14        6        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       60       56        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group1().yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
            - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0)
            - (other.group1().zxy() * self.group0().yzx()).with_w(0.0);
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]))
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]))
            - (other.group0().yzxx() * self.group0().zxyx());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g1[1] * self[e431]) - (geometric_anti_product_g1[2] * self[e412]))
                + (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl AntiSandwich<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       43       44        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
            + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        Line::from_groups(
            // e41, e42, e43
            (geometric_anti_product_g1.zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (geometric_anti_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g0[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g0.yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        0        7        0      N/A
    //    simd4       14        7        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       59       57        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x3::from(0.0).with_w((other[e43] * self[e412]) * -1.0)
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().xyzy() * Simd32x3::from(other[scalar]).with_w(other[e42]))
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_anti_product_g1 = (Simd32x4::from(other[e1234]) * self.group0())
            + (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g1.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]))
                + (geometric_anti_product_g0.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]))
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl AntiSandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       33        0        0
    //    simd2        3        7        0      N/A
    //    simd3       14       19        0      N/A
    //    simd4       18       11        0      N/A
    // Totals...
    // yes simd       37       70        0      N/A
    //  no simd      122      148        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w((other[e43] * self[e412]) * -1.0)
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            + (other.group3().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
            - (self.group0().yzxy() * other.group3().zxy().with_w(other[e42]));
        let geometric_anti_product_g2 = (other.group4().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((other[e431] * self[e423]) * -1.0)
            - (Simd32x3::from(other[e4]) * self.group0().xyz())
            - (other.group4().zx() * self.group0().yz()).with_z(0.0);
        let geometric_anti_product_g3 =
            (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e431]) * -1.0)
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (other.group1().yz() * self.group0().zx()).with_z(0.0);
        let geometric_anti_product_g4 = (Simd32x4::from(other[e1234]) * self.group0())
            + (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]))
            + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
            - (other.group2().yzx() * self.group0().zxy()).with_w(0.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product_g1[3] * self[e321], 0.0])
                + (Simd32x2::from(self[e423]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g4[0]]))
                + (Simd32x2::from(self[e431]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g4[1]]))
                + (Simd32x2::from(self[e412]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g4[2]])),
            // e1, e2, e3, e4
            (geometric_anti_product_g2 * Simd32x3::from(self[e321])).with_w(0.0) + (geometric_anti_product_g3.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(self[e423]) * Simd32x4::from([other[e1] * self[e423], other[e1] * self[e431], geometric_anti_product_g3[1], geometric_anti_product_g2[0]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([geometric_anti_product_g3[2], other[e2] * self[e431], other[e2] * self[e412], geometric_anti_product_g2[1]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([other[e3] * self[e423], geometric_anti_product_g3[0], other[e3] * self[e412], geometric_anti_product_g2[2]]))
                - (Simd32x3::from(other[e4] * self[e321]) * self.group0().xyz()).with_w(0.0)
                - (other.group1().yzx() * self.group0().xyx() * self.group0().yzz()).with_w(0.0),
            // e41, e42, e43
            (geometric_anti_product_g4.zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                + (geometric_anti_product_g1.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz())
                - (geometric_anti_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from([other[e423] * self[e423], other[e431] * self[e431], other[e412] * self[e412], other[e423] * self[e423]]) * self.group0())
                + (Simd32x4::from([other[e431] * self[e431], other[e423] * self[e423], other[e423] * self[e423], other[e431] * self[e431]]) * self.group0())
                + (self.group0() * Simd32x2::from(other[e412] * self[e412]).with_zw(other[e431] * self[e431], other[e412] * self[e412]))
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g3[1] * self[e431]) - (geometric_anti_product_g3[2] * self[e412]))
                + (geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0])),
        )
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        5       15        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4] * -1.0) * self.group0().xyz();
        let geometric_anti_product_g1_w = self[e321] * other[e4] * -1.0;
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
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       28        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group0().yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
            - (other.group0().zxy() * self.group0().yzx()).with_w(0.0);
        let geometric_anti_product_g1_xyz = (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz());
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g1_xyz[1] * self[e431]) - (geometric_anti_product_g1_xyz[2] * self[e412]))
                + (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1_xyz[0])),
        )
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        5        0      N/A
    //    simd4        6        2        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       27       29        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(other[e4] * -1.0) * self.group0().xyz();
        let geometric_anti_product_g1 = Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]))
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (self.group0().zxyx() * other.group0().yzxx());
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0_xyz[1] * self[e431]) - (geometric_anti_product_g0_xyz[2] * self[e412]))
                + (geometric_anti_product_g0_xyz * Simd32x3::from(self[e321])).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0_xyz[0])),
        )
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * self[e4] * other[e1234])
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        5        0        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * self[e4] * other[scalar] * -1.0, self[e4] * self[e4] * other[e1234]]),
        )
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       32       44        0        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]);
        let geometric_anti_product_g1 = (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
            + (other.group1().yzxx() * self.group0().zxyx())
            + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]))
            - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0)
            - (other.group1().zxy() * self.group0().yzx()).with_w(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((geometric_anti_product_g0.zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z(geometric_anti_product_g0[0] * self[e2] * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                - (geometric_anti_product_g0.yz() * self.group0().zx()).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e4] * -1.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4] * -1.0) * geometric_anti_product_g0.xyz()).with_w(
                (geometric_anti_product_g0[0] * self[e1]) + (geometric_anti_product_g0[1] * self[e2]) + (geometric_anti_product_g0[2] * self[e3])
                    - (geometric_anti_product_g1[3] * self[e4]),
            ),
        )
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e4] * self[e4] * other[e321] * -1.0)
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        9        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       17       27        0        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            - (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0();
        Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g1_xyz * Simd32x3::from(self[e4]),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()) + (geometric_anti_product_g1_xyz.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (geometric_anti_product_g1_xyz.yzx() * self.group0().zxy()),
        )
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        6        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       32       41        0        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e1234] * self[e4];
        let geometric_anti_product_g1_xyz = Simd32x3::from(self[e4]) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_anti_product_g1_xyz.with_w(geometric_anti_product_g0_w),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4])
                * ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group0().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z(other[e41] * self[e2] * -1.0)
                    - (Simd32x3::from(self[e4]) * other.group1().xyz())
                    - (other.group0().yz() * self.group0().zx()).with_z(0.0))
                .with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])))
                + (geometric_anti_product_g1_xyz.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1_xyz[1] * self[e2]) + (geometric_anti_product_g1_xyz[2] * self[e3]))
                - (Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()).with_w(0.0)
                - (geometric_anti_product_g1_xyz.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       46        0        0
    //    simd3       12       18        0      N/A
    // Totals...
    // yes simd       24       64        0      N/A
    //  no simd       48      100        0        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = other[e4] * self[e4] * -1.0;
        let geometric_anti_product_g1_w = other[e1234] * self[e4];
        let geometric_anti_product_g2 = Simd32x3::from(self[e4] * -1.0) * other.group4().xyz();
        let geometric_anti_product_g4_xyz = Simd32x3::from(self[e4]) * other.group2();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g4_xyz[0] * self[e1]) + (geometric_anti_product_g4_xyz[1] * self[e2]) + (geometric_anti_product_g4_xyz[2] * self[e3])
                    - (self[e4] * self[e4] * other[scalar])
                    - (other[e41] * self[e1] * self[e4])
                    - (other[e42] * self[e2] * self[e4])
                    - (other[e43] * self[e3] * self[e4]),
                geometric_anti_product_g1_w * self[e4],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e4]) * Simd32x3::from([other[e412] * self[e2], other[e423] * self[e3], other[e431] * self[e1]]))
                + (Simd32x3::from(other[e4] * self[e4]) * self.group0().xyz())
                + (geometric_anti_product_g2.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g0_y) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e431] * self[e3], other[e412] * self[e1], other[e423] * self[e2]]))
                - (Simd32x3::from(self[e4] * self[e4]) * other.group1().xyz())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product_g0_y * self[e4] * -1.0),
            // e41, e42, e43
            geometric_anti_product_g4_xyz * Simd32x3::from(self[e4]),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * Simd32x3::from([other[e43] * self[e2], other[e41] * self[e3], other[e42] * self[e1]]))
                + (Simd32x3::from(other[e1234] * self[e4]) * self.group0().xyz())
                + (geometric_anti_product_g4_xyz.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_anti_product_g1_w) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e42] * self[e3], other[e43] * self[e1], other[e41] * self[e2]]))
                - (Simd32x3::from(self[e4] * self[e4]) * other.group3())
                - (geometric_anti_product_g4_xyz.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (geometric_anti_product_g2 * Simd32x3::from(self[e4] * -1.0)).with_w(
                (geometric_anti_product_g2[0] * self[e1]) + (geometric_anti_product_g2[1] * self[e2]) + (geometric_anti_product_g2[2] * self[e3])
                    - (self[e4] * self[e4] * other[e321])
                    - (other[e423] * self[e1] * self[e4])
                    - (other[e431] * self[e2] * self[e4])
                    - (other[e412] * self[e3] * self[e4]),
            ),
        )
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        3       11        0        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = self[e4] * other[e4] * -1.0;
        Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e4] * other[e4]) * self.group0().xyz()) - (Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()))
                .with_w(geometric_anti_product_g0_w * self[e4] * -1.0),
        )
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd        6       19        0        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(self[e4] * -1.0) * other.group0().xyz();
        Plane::from_groups(/* e423, e431, e412, e321 */ (geometric_anti_product_g0_xyz * Simd32x3::from(self[e4] * -1.0)).with_w(
            (geometric_anti_product_g0_xyz[0] * self[e1]) + (geometric_anti_product_g0_xyz[1] * self[e2]) + (geometric_anti_product_g0_xyz[2] * self[e3])
                - (self[e4] * self[e4] * other[e321])
                - (other[e423] * self[e1] * self[e4])
                - (other[e431] * self[e2] * self[e4])
                - (other[e412] * self[e3] * self[e4]),
        ))
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        6       15        0        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = other[e4] * self[e4] * -1.0;
        Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e4] * self[e4]) * self.group0().xyz())
                - (Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz())
                - (Simd32x3::from(self[e4] * self[e4]) * other.group0().xyz()))
            .with_w(geometric_anti_product_g0_w * self[e4] * -1.0),
        )
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e4] * self[e4] * other[scalar] * -1.0)
    }
}
