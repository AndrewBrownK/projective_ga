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
//  Minimum:         0       1       0     N/A
//   Median:         3      10       0     N/A
//  Average:         7      15       0     N/A
//  Maximum:        80     100       2     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         4      20       0       0
//  Average:        18      30       0       0
//  Maximum:       205     221       2       3
impl std::ops::Div<GeometricAntiQuotientInfix> for AntiScalar {
    type Output = GeometricAntiQuotientInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] / other[e1234])
    }
}
impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        4        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234] / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]),
        )
    }
}
impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       15        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * self[e1234] * -1.0) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * self[e1234]) * other.group1(),
        )
    }
}
impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        2       13        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group1(),
        )
    }
}
impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       22        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0 * self[e1234]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0 * self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       21        0      N/A
    //  no simd        7       32        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_g0 * self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * self[e1234] * -1.0) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * self[e1234]) * other.group4(),
        )
    }
}
impl GeometricAntiQuotient<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * -1.0 / other[e4])
    }
}
impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       22        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       25        0      N/A
    //  no simd        8       34        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from([
                other[e423] * other[e423] * self[e1234],
                other[e431] * other[e431] * self[e1234],
                other[e412] * other[e412] * self[e1234],
                other[e423] * other[e423] * self[e1234],
            ]) * other.group0())
                + (Simd32x4::from([
                    other[e431] * other[e431] * self[e1234],
                    other[e423] * other[e423] * self[e1234],
                    other[e423] * other[e423] * self[e1234],
                    other[e431] * other[e431] * self[e1234],
                ]) * other.group0())
                + (other.group0()
                    * Simd32x2::from(other[e412] * other[e412] * self[e1234]).with_zw(other[e431] * other[e431] * self[e1234], other[e412] * other[e412] * self[e1234])),
        )
    }
}
impl GeometricAntiQuotient<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        9        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234] * -1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for DualNum {
    type Output = GeometricAntiQuotientInfixPartial<DualNum>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        2        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        1        6        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
            geometric_anti_product_g0[1] * self[e1234],
        ]))
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd        7       25        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g1.xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz()))
                .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e423, e431, e412, e321
            (Simd32x2::from(self[e1234]) * geometric_anti_product_g1.xy()).with_zw(
                geometric_anti_product_g1[2] * self[e1234],
                (geometric_anti_product_g0[3] * self[scalar]) + (geometric_anti_product_g1[3] * self[e1234]),
            ),
        )
    }
}
impl GeometricAntiQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        5       17        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g0 * Simd32x3::from(self[scalar])) - (Simd32x3::from(other_g0 * self[e1234]) * other.group1()),
        )
    }
}
impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       11       26        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x4::from(self[scalar])) + Simd32x3::from(0.0).with_w(other_g0 * self[e1234] * other[scalar])
                - (Simd32x3::from(other_g0 * self[e1234]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       18        0        0
    //    simd2        0        2        0      N/A
    //    simd3        2        6        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       11       28        0      N/A
    //  no simd       15       48        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
                geometric_anti_product_g0[1] * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g4.xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz()))
                .with_w(geometric_anti_product_g1[3] * self[e1234]),
            // e41, e42, e43
            geometric_anti_product_g2 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar])) - (Simd32x3::from(other_g0 * self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            (Simd32x2::from(self[e1234]) * geometric_anti_product_g4.xy()).with_zw(
                geometric_anti_product_g4[2] * self[e1234],
                (geometric_anti_product_g1[3] * self[scalar]) + (geometric_anti_product_g4[3] * self[e1234]),
            ),
        )
    }
}
impl GeometricAntiQuotient<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[e1234]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[scalar]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       16        0      N/A
    //  no simd        8       30        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * geometric_anti_product_g0.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0       13        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[scalar]),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Flector {
    type Output = GeometricAntiQuotientInfixPartial<Flector>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        8        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e1234];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl GeometricAntiQuotient<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd2        0        1        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        8        2      N/A
    //  no simd        4       15        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz()) - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group1().xyz()))
                .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz()).with_w((geometric_anti_product_g0[1] * self[e321]) - (geometric_anti_product_g0[0] * self[e4])),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       17       23        0      N/A
    //  no simd       53       61        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g1.yzxx() * self.group1().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (Simd32x4::from(geometric_anti_product_g0[3]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g1.xxy()).with_w(0.0)
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g1.zyz()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4]) * Simd32x4::from([geometric_anti_product_g0[0], geometric_anti_product_g0[1], geometric_anti_product_g0[2], geometric_anti_product_g1[3]]))
                + (geometric_anti_product_g1.yzxx() * self.group0().zxyx())
                + (geometric_anti_product_g1.wwwy() * self.group1().xyz().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e3]) - (geometric_anti_product_g0[2] * self[e412]))
                + (geometric_anti_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0().xyz().with_w(self[e321]))
                - (Simd32x4::from([geometric_anti_product_g1[0], geometric_anti_product_g1[1], geometric_anti_product_g1[2], geometric_anti_product_g0[1]]) * self.group1().wwwy())
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       12        0        0
    //    simd3        0        9        0      N/A
    //    simd4        8        2        0      N/A
    // Totals...
    // yes simd       12       23        0      N/A
    //  no simd       36       47        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g1 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g0[2] * self[e412])
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[1] * self[e431]),
            // e423, e431, e412, e321
            (self.group1().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e412]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3]))
                + (geometric_anti_product_g0 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g1[1] * self[e431])
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e1]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4       11       12        0      N/A
    // Totals...
    // yes simd       17       24        0      N/A
    //  no simd       50       68        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[1], geometric_anti_product_g0[1]]) * self.group1().yzxy())
                - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e423]))
                - (self.group1().xyzz() * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g0[2]))
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (Simd32x4::from([geometric_anti_product_g0[2], geometric_anti_product_g0[0], geometric_anti_product_g0[1], geometric_anti_product_g1[0]]) * self.group1().yzxx())
                + (self.group1().xyzy() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g1[2] * self[e412])
                        - (geometric_anti_product_g0[1] * self[e2])
                        - (geometric_anti_product_g0[2] * self[e3])
                        - (geometric_anti_product_g1[3] * self[e4]),
                )
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       27        0        0
    //    simd2        4        9        0      N/A
    //    simd3       12       17        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       42       60        0      N/A
    //  no simd      103      124        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g4[3] * self[e4])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g4[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g4[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(geometric_anti_product_g1[3]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy()).with_w(0.0)
                + (geometric_anti_product_g3.yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().xyzx() * Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (self.group1().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (geometric_anti_product_g3 * Simd32x3::from(self[e4])).with_w(0.0)
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (geometric_anti_product_g4.yzx() * self.group1().zxy()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g4.zx() * self.group1().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g4[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g1.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[0] * self[e2]) - (geometric_anti_product_g1[0] * self[e431]) - (geometric_anti_product_g4[1] * self[e1]))
                + (geometric_anti_product_g4.yz() * self.group0().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (geometric_anti_product_g4.zx() * self.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (self.group1().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3]),
                )
                + (geometric_anti_product_g2 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g3[1] * self[e431])
                - (geometric_anti_product_g2.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e4]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        1      N/A
    //  no simd        0       11        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0 * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product_g0 * -1.0) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        4        0      N/A
    //    simd4        9        6        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       38       51        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]))
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0.zyz()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product_g0[3]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                + (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[1] * self[e2]) + (geometric_anti_product_g0[2] * self[e3]))
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd3        0        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        5       11        2      N/A
    //  no simd       17       29        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0[3] * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]))
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                - (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0().xyz().with_w(self[e321]))
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx()),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Horizon {
    type Output = GeometricAntiQuotientInfixPartial<Horizon>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] / other[e1234])
    }
}
impl GeometricAntiQuotient<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e1234] * self[e321] * 1.0 / (other[e1234] * other[e1234]))
    }
}
impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       15        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * (Simd32x4::from(other_g0) * other.group1()).xyz().with_w(other_g0 * other[e4] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2        9        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(-(other[e41] * other[e41] * self[e321]) - (other[e42] * other[e42] * self[e321]) - (other[e43] * other[e43] * self[e321])) * other.group0())
                .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234])
            * other.group0()
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e321]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       22        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other_g0 * self[e321] * other[e4], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * self[e321] * other[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / other[e4])
    }
}
impl GeometricAntiQuotient<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        6       17        0        3
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from(self[e321]) * Simd32x3::from([f32::powi(other[e423], 3), f32::powi(other[e431], 3), f32::powi(other[e412], 3)]))
                - (Simd32x3::from(self[e321] * other[e423]) * Simd32x3::from([other[e431] * other[e431], other[e423] * other[e431], other[e423] * other[e412]]))
                - (Simd32x3::from(self[e321] * other[e412]) * Simd32x3::from([other[e423] * other[e412], other[e431] * other[e412], other[e431] * other[e431]])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        5        1        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e4] * (-1.0 / (other[e4] * other[e4])) * -1.0)
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Line {
    type Output = GeometricAntiQuotientInfixPartial<Line>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        6        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e1234];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl GeometricAntiQuotient<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        3       12        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0[1]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0()) + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1()),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        6        0      N/A
    //    simd4        9        5        0      N/A
    // Totals...
    // yes simd       16       24        0      N/A
    //  no simd       43       51        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e43]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                + (self.group1().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g0[1],
                    geometric_anti_product_g0[2],
                    geometric_anti_product_g0[0],
                    geometric_anti_product_g1[1] * self[e42],
                ]) * self.group0().zxy().with_w(1.0))
                - (geometric_anti_product_g1.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(
                -(geometric_anti_product_g0[0] * self[e41])
                    - (geometric_anti_product_g0[1] * self[e42])
                    - (geometric_anti_product_g0[2] * self[e43])
                    - (geometric_anti_product_g1[1] * self[e31])
                    - (geometric_anti_product_g1[2] * self[e12]),
            ) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g1.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        0        8        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       30       38        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[0] * self[e41]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ) + (geometric_anti_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                + (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e23])
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4        9        7        0      N/A
    // Totals...
    // yes simd       16       23        0      N/A
    //  no simd       43       56        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (self.group0().zxyx() * geometric_anti_product_g0.yzxx()),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                -(geometric_anti_product_g0[1] * self[e31])
                    - (geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                + (self.group1().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (self.group0().zxyx() * geometric_anti_product_g1.yzxx())
                - (self.group1().zxyx() * geometric_anti_product_g0.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       22        0        0
    //    simd2        3        4        0      N/A
    //    simd3        7       17        0      N/A
    //    simd4        9        5        0      N/A
    // Totals...
    // yes simd       32       48        0      N/A
    //  no simd       76      101        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g3[0] * self[e41]) - (geometric_anti_product_g3[1] * self[e42]) - (geometric_anti_product_g3[2] * self[e43]),
                0.0,
            ]) - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((geometric_anti_product_g4[2] * self[e43]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                + (self.group1().yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g1[1],
                    geometric_anti_product_g1[2],
                    geometric_anti_product_g1[0],
                    geometric_anti_product_g4[1] * self[e42],
                ]) * self.group0().zxy().with_w(1.0))
                - (geometric_anti_product_g4.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[1]) * self.group0()) + (geometric_anti_product_g2.zxy() * self.group0().yzx())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1())
                + (geometric_anti_product_g2.zxy() * self.group1().yzx())
                + (geometric_anti_product_g3.zxy() * self.group0().yzx())
                - (geometric_anti_product_g2.yzx() * self.group1().zxy())
                - (geometric_anti_product_g3.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(
                -(geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43])
                    - (geometric_anti_product_g4[1] * self[e31])
                    - (geometric_anti_product_g4[2] * self[e12]),
            ) + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g4.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        1      N/A
    //  no simd        0        7        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_anti_product_g0) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0) * self.group0()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        3        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd        9       23        0      N/A
    //  no simd       30       44        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12]))
                + (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g0.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        2        0
    //    simd3        0        5        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4       10        2      N/A
    //  no simd       10       23        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1()).with_w(0.0) + (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (self.group0().zxy() * geometric_anti_product_g0.yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0())
                .with_w(-(geometric_anti_product_g0[0] * self[e41]) - (geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43])),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Motor {
    type Output = GeometricAntiQuotientInfixPartial<Motor>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        8        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
        )
    }
}
impl GeometricAntiQuotient<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        4       15        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0[1]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product_g0[0]) * self.group0()) + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1()),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        7        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       19       27        0      N/A
    //  no simd       55       62        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e43]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                + (geometric_anti_product_g1.xxy() * self.group1().wzx()).with_w(0.0)
                + (geometric_anti_product_g1.zyz() * self.group1().yww()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g0[1], geometric_anti_product_g0[2], geometric_anti_product_g0[0], geometric_anti_product_g1[0]]) * self.group0().zxyx())
                - (geometric_anti_product_g1.yzxy() * self.group1().zxy().with_w(self[e42])),
            // e423, e431, e412, e321
            (self.group0() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[3]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g0[1] * self[e42])
                        - (geometric_anti_product_g0[2] * self[e43])
                        - (geometric_anti_product_g1[0] * self[e23])
                        - (geometric_anti_product_g1[1] * self[e31])
                        - (geometric_anti_product_g1[2] * self[e12]),
                )
                + (geometric_anti_product_g1.xxy() * self.group0().wzx()).with_w(geometric_anti_product_g0[3] * self[scalar])
                + (geometric_anti_product_g1.zyz() * self.group0().yww()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g0[0]]) * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        8        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       42       47        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (geometric_anti_product_g0.xxy() * self.group0().wzx()).with_w(0.0)
                + (geometric_anti_product_g0.zyz() * self.group0().yww()).with_w(0.0)
                - (self.group0().zxyx() * geometric_anti_product_g0.yzx().with_w(geometric_anti_product_g0[0])),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(
                -(geometric_anti_product_g0[1] * self[e31])
                    - (geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ) + (geometric_anti_product_g0.xxy() * self.group1().wzx()).with_w(0.0)
                + (geometric_anti_product_g0.zyz() * self.group1().yww()).with_w(0.0)
                + (geometric_anti_product_g1.xxy() * self.group0().wzx()).with_w(0.0)
                + (geometric_anti_product_g1.zyz() * self.group0().yww()).with_w(0.0)
                - (self.group0().zxyx() * geometric_anti_product_g1.yzx().with_w(geometric_anti_product_g1[0]))
                - (self.group1().zxyx() * geometric_anti_product_g0.yzx().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       19       26        0      N/A
    //  no simd       55       68        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g0[1] * self[e31])
                        - (geometric_anti_product_g0[2] * self[e12])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43]),
                )
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                + (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_anti_product_g1.yzxx() * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       23        0        0
    //    simd2        4        5        0      N/A
    //    simd3       10       21        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       40       56        0      N/A
    //  no simd      100      124        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
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
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g4[2] * self[e43]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                + (geometric_anti_product_g4.xxy() * self.group1().wzx()).with_w(0.0)
                + (geometric_anti_product_g4.zyz() * self.group1().yww()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[1], geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g4[0]]) * self.group0().zxyx())
                - (geometric_anti_product_g4.yzxy() * self.group1().zxy().with_w(self[e42])),
            // e41, e42, e43
            (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (geometric_anti_product_g2.zxy() * self.group0().yzx())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0().xyz())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz())
                + (geometric_anti_product_g2.zxy() * self.group1().yzx())
                + (geometric_anti_product_g3.zxy() * self.group0().yzx())
                - (geometric_anti_product_g2.yzx() * self.group1().zxy())
                - (geometric_anti_product_g3.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (self.group0() * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g4[3]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g4[0] * self[e23])
                        - (geometric_anti_product_g4[1] * self[e31])
                        - (geometric_anti_product_g4[2] * self[e12]),
                )
                + (geometric_anti_product_g4.xxy() * self.group0().wzx()).with_w(geometric_anti_product_g1[3] * self[scalar])
                + (geometric_anti_product_g4.zyz() * self.group0().yww()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g4[1], geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g1[0]]) * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        1      N/A
    //  no simd        0        9        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1().xyz().with_w(self[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group0().xyz().with_w(self[scalar]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        4        0      N/A
    //    simd4        9        6        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       38       51        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.xxy() * self.group1().wzx()).with_w(0.0)
                + (geometric_anti_product_g0.zyz() * self.group1().yww()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12]))
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        2        0
    //    simd2        0        1        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       15        2      N/A
    //  no simd       15       29        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z(geometric_anti_product_g0[0] * self[e42] * -1.0)
                - (geometric_anti_product_g0.yz() * self.group0().zx()).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(
                (geometric_anti_product_g0[3] * self[scalar])
                    - (geometric_anti_product_g0[0] * self[e41])
                    - (geometric_anti_product_g0[1] * self[e42])
                    - (geometric_anti_product_g0[2] * self[e43]),
            ),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for MultiVector {
    type Output = GeometricAntiQuotientInfixPartial<MultiVector>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        1      N/A
    //  no simd        0       16        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e1234];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group4(),
        )
    }
}
impl GeometricAntiQuotient<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        2        0
    //    simd2        0        1        0      N/A
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       14        2      N/A
    //  no simd        8       27        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
                geometric_anti_product_g0[1] * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz()) - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz()))
                .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0[1]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * self.group2()) + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group3()),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0[1]) * self.group4().xyz()).with_w((geometric_anti_product_g0[1] * self[e321]) - (geometric_anti_product_g0[0] * self[e4])),
        )
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       22        0        0
    //    simd2        4        8        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4       12        8        0      N/A
    // Totals...
    // yes simd       40       52        0      N/A
    //  no simd      104      112        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e4])
                    - (geometric_anti_product_g0[0] * self[e423])
                    - (geometric_anti_product_g0[1] * self[e431])
                    - (geometric_anti_product_g0[2] * self[e412]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g1[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g1[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(geometric_anti_product_g0[3]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e43]) * -1.0)
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * geometric_anti_product_g1.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * geometric_anti_product_g1.zyz()).with_w(0.0)
                + (self.group2().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g0[1],
                    geometric_anti_product_g0[2],
                    geometric_anti_product_g0[0],
                    geometric_anti_product_g1[1] * self[e42],
                ]) * self.group2().zxy().with_w(1.0))
                - (geometric_anti_product_g1.yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (geometric_anti_product_g1.yzx() * self.group4().zxy()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[1] * self[e423]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                - (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group4().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[0] * self[e2]) - (geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g1[1] * self[e1]))
                + (geometric_anti_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz())
                - (geometric_anti_product_g0.yz() * self.group4().zx()).with_z(0.0)
                - (geometric_anti_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[3]) * self.group2().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g0[0] * self[e41])
                        - (geometric_anti_product_g0[1] * self[e42])
                        - (geometric_anti_product_g0[2] * self[e43])
                        - (geometric_anti_product_g1[1] * self[e31])
                        - (geometric_anti_product_g1[2] * self[e12]),
                )
                + (self.group2().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g1.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd2        3        3        0      N/A
    //    simd3        7       18        0      N/A
    //    simd4        8        2        0      N/A
    // Totals...
    // yes simd       24       38        0      N/A
    //  no simd       65       83        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g1[0] * self[e41]) - (geometric_anti_product_g1[1] * self[e42]) - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group4().zxy()).with_w(0.0)
                - (self.group4().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g1 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g0[2] * self[e412])
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[1] * self[e431]),
            // e41, e42, e43
            (geometric_anti_product_g0 * Simd32x3::from(self[e1234])) + (geometric_anti_product_g0.zxy() * self.group2().yzx())
                - (geometric_anti_product_g0.yzx() * self.group2().zxy()),
            // e23, e31, e12
            (geometric_anti_product_g0 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g1 * Simd32x3::from(self[e1234]))
                + (geometric_anti_product_g0.zxy() * self.group3().yzx())
                + (geometric_anti_product_g1.zxy() * self.group2().yzx())
                - (geometric_anti_product_g0.yzx() * self.group3().zxy())
                - (geometric_anti_product_g1.yzx() * self.group2().zxy()),
            // e423, e431, e412, e321
            (self.group4().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e412]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3]))
                + (geometric_anti_product_g0 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g1[1] * self[e431])
                - (geometric_anti_product_g0.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g0[0] * self[e1]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd2        4        4        0      N/A
    //    simd3       10       16        0      N/A
    //    simd4       11       12        0      N/A
    // Totals...
    // yes simd       34       44        0      N/A
    //  no simd       91      116        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e1234])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * self.group0())
                - (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (geometric_anti_product_g0 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group4().zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[1], geometric_anti_product_g0[1]]) * self.group4().yzxy())
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e423]))
                - (self.group4().xyzz() * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g0[2]))
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz()).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group2())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (self.group2().yzx() * geometric_anti_product_g0.zxy())
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group3())
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group2())
                + (Simd32x3::from(self[scalar]) * geometric_anti_product_g0.xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz())
                + (self.group2().yzx() * geometric_anti_product_g1.zxy())
                + (self.group3().yzx() * geometric_anti_product_g0.zxy())
                - (self.group2().zxy() * geometric_anti_product_g1.yzx())
                - (self.group3().zxy() * geometric_anti_product_g0.yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (Simd32x4::from([geometric_anti_product_g0[2], geometric_anti_product_g0[0], geometric_anti_product_g0[1], geometric_anti_product_g1[0]]) * self.group4().yzxx())
                + (self.group4().xyzy() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g1[2] * self[e412])
                        - (geometric_anti_product_g0[1] * self[e2])
                        - (geometric_anti_product_g0[2] * self[e3])
                        - (geometric_anti_product_g1[3] * self[e4]),
                )
                - (geometric_anti_product_g0.yzxx() * self.group4().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       39        0        0
    //    simd2        8       15        0      N/A
    //    simd3       24       32        0      N/A
    //    simd4       23       14        0      N/A
    // Totals...
    // yes simd       80      100        0      N/A
    //  no simd      205      221        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g4[3] * self[e4])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g4[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g4[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from(geometric_anti_product_g1[3]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            (geometric_anti_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * geometric_anti_product_g4.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * geometric_anti_product_g4.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy()).with_w(0.0)
                + (geometric_anti_product_g3.yzx() * self.group4().zxy()).with_w(0.0)
                + (self.group2().yzx() * geometric_anti_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([
                    geometric_anti_product_g1[1],
                    geometric_anti_product_g1[2],
                    geometric_anti_product_g1[0],
                    geometric_anti_product_g4[2] * self[e43],
                ]) * self.group2().zxy().with_w(1.0))
                - (geometric_anti_product_g4.yzxx() * self.group3().zxy().with_w(self[e41]))
                - (self.group4().xyzx() * Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (self.group4().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1]))
                - (geometric_anti_product_g3 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g4[1] * self[e42])
                - (geometric_anti_product_g2.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g2[2] * self[e412]),
            // e41, e42, e43
            (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group2())
                + (geometric_anti_product_g2.zxy() * self.group2().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[0] * self[e431]) - (geometric_anti_product_g4[1] * self[e423]))
                + (geometric_anti_product_g4.yz() * self.group4().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g2.yzx() * self.group2().zxy())
                - (geometric_anti_product_g4.zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + (geometric_anti_product_g2.zxy() * self.group3().yzx())
                + (geometric_anti_product_g3.zxy() * self.group2().yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_anti_product_g1[1] * self[e423]) + (geometric_anti_product_g4[0] * self[e2])
                        - (geometric_anti_product_g1[0] * self[e431])
                        - (geometric_anti_product_g4[1] * self[e1]),
                )
                + (geometric_anti_product_g1.zx() * self.group4().yz()).with_z(0.0)
                + (geometric_anti_product_g4.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g2.yzx() * self.group3().zxy())
                - (geometric_anti_product_g3.yzx() * self.group2().zxy())
                - (geometric_anti_product_g1.yz() * self.group4().zx()).with_z(0.0)
                - (geometric_anti_product_g4.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g4 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (Simd32x4::from(geometric_anti_product_g1[3]) * self.group2().with_w(self[scalar]))
                + (Simd32x4::from([
                    geometric_anti_product_g4[2],
                    geometric_anti_product_g4[0],
                    geometric_anti_product_g4[1],
                    geometric_anti_product_g3[2] * self[e412],
                ]) * self.group2().yzx().with_w(1.0))
                + (self.group4().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g1[0] * self[e41])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g4[1] * self[e31])
                        - (geometric_anti_product_g4[2] * self[e12]),
                )
                + (geometric_anti_product_g2 * Simd32x3::from(self[e4])).with_w(geometric_anti_product_g3[1] * self[e431])
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g4.yzxx())
                - (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g0[0] * self[e4]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        1        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        9        1      N/A
    //  no simd        0       20        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product_g0 * -1.0) * Simd32x2::from([self[e321], self[e4]]),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group3().with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0 * -1.0) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0 * -1.0) * self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group2().with_w(self[scalar]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       20        0        0
    //    simd2        3        5        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        9        6        0      N/A
    // Totals...
    // yes simd       21       40        0      N/A
    //  no simd       65       81        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product_g0[3] * self[e4], 0.0])
                + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]))
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (geometric_anti_product_g0.yzx() * self.group4().zxy()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g0[1] * self[e423]) * -1.0)
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g0.zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz())
                + (geometric_anti_product_g0.yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g0[1] * self[e1]) * -1.0)
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g0.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12]))
                + (self.group2().yzx() * geometric_anti_product_g0.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_anti_product_g0.yzxx()),
        )
    }
}
impl GeometricAntiQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        2        0
    //    simd2        0        1        0      N/A
    //    simd3        7       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       27        2      N/A
    //  no simd       27       51        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g0[0] * self[e423])
                    - (geometric_anti_product_g0[1] * self[e431])
                    - (geometric_anti_product_g0[2] * self[e412])
                    - (geometric_anti_product_g0[3] * self[e321]),
                geometric_anti_product_g0[3] * self[e4] * -1.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group3())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (self.group2().yzx() * geometric_anti_product_g0.zxy())
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0[3] * -1.0) * self.group4().xyz(),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group4().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g0[0] * self[e431]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (geometric_anti_product_g0.yz() * self.group4().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group2()).with_w(
                (geometric_anti_product_g0[3] * self[scalar])
                    - (geometric_anti_product_g0[0] * self[e41])
                    - (geometric_anti_product_g0[1] * self[e42])
                    - (geometric_anti_product_g0[2] * self[e43]),
            ),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Origin {
    type Output = GeometricAntiQuotientInfixPartial<Origin>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] / other[e1234])
    }
}
impl GeometricAntiQuotient<DualNum> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0        6        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[1] * self[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[0] * self[e4] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       22        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]),
        )
    }
}
impl GeometricAntiQuotient<Line> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        8        0      N/A
    //  no simd        2       12        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[e4]) * other.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e4] * -1.0) * other.group0()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        7        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd        3       35        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * (geometric_anti_product_g1.xyz() * Simd32x3::from(-1.0)).with_w(geometric_anti_product_g0[3]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        3        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       22        0      N/A
    //  no simd        7       45        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * (Simd32x3::from(other_g0) * other.group3()).with_w(geometric_anti_product_g0[1]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * geometric_anti_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4] * -1.0) * (Simd32x3::from(other_g0) * other.group2()).with_w(geometric_anti_product_g0[0]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] / other[e4])
    }
}
impl GeometricAntiQuotient<Plane> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       17        0      N/A
    //  no simd        8       28        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4] * -1.0) * geometric_anti_product_g0.xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4]),
        )
    }
}
impl GeometricAntiQuotient<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        2      N/A
    //  no simd        0       13        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Plane {
    type Output = GeometricAntiQuotientInfixPartial<Plane>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        4        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0       11        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_anti_product_g0[0] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0[1]) * self.group0(),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       33       41        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g1.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e321])
                    * Simd32x4::from([geometric_anti_product_g1[0], geometric_anti_product_g1[1], geometric_anti_product_g1[2], geometric_anti_product_g0[3]]))
                - (geometric_anti_product_g0.yzxx() * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       24       32        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]))
                + (geometric_anti_product_g0 * Simd32x3::from(self[e321])).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        3        0      N/A
    //    simd4        7        8        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       32       49        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((geometric_anti_product_g0[2] * self[e412]) * -1.0)
                + (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g1[1], geometric_anti_product_g0[0]]) * self.group0().yzxx())
                - (self.group0().xyzy() * Simd32x3::from(geometric_anti_product_g1[3]).with_w(geometric_anti_product_g0[1])),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0())
                + (Simd32x4::from([geometric_anti_product_g0[2], geometric_anti_product_g0[0], geometric_anti_product_g0[1], geometric_anti_product_g1[0]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]))
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       26        0        0
    //    simd2        0        3        0      N/A
    //    simd3        7       10        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd       27       45        0      N/A
    //  no simd       62       86        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412])
                    - (geometric_anti_product_g1[3] * self[e321]),
                (geometric_anti_product_g4[0] * self[e423]) + (geometric_anti_product_g4[1] * self[e431]) + (geometric_anti_product_g4[2] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((geometric_anti_product_g2[2] * self[e412]) * -1.0)
                + (geometric_anti_product_g2 * Simd32x3::from(self[e321])).with_w(0.0)
                + (geometric_anti_product_g3.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().xyzx() * Simd32x2::from(geometric_anti_product_g0[0]).with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (self.group0().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1])),
            // e41, e42, e43
            (geometric_anti_product_g4.yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[1] * self[e423]) * -1.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.zx() * self.group0().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz())
                + (geometric_anti_product_g1.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g1[0] * self[e431]) * -1.0)
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group0())
                + (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g3[1] * self[e431]) + (geometric_anti_product_g3[2] * self[e412]))
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        5        1      N/A
    //  no simd        0        7        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_anti_product_g0 * -1.0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[e321] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       13        0        0
    //    simd3        1        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        6       20        0      N/A
    //  no simd       20       38        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]))
                - (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        2        0
    //    simd3        0        3        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4       10        2      N/A
    //  no simd       10       22        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_anti_product_g0[3] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]) - (geometric_anti_product_g0[3] * self[e321]))
                + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group0().zxyx()),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Point {
    type Output = GeometricAntiQuotientInfixPartial<Point>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        4        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd2        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0        9        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(1.0 / other[e1234]) * Simd32x2::from([other[scalar] / other[e1234], 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0[1]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[0] * self[e4] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       20       34        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4]) * Simd32x4::from([geometric_anti_product_g0[0], geometric_anti_product_g0[1], geometric_anti_product_g0[2], geometric_anti_product_g1[3]]))
                + (geometric_anti_product_g1.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[1] * self[e2]) + (geometric_anti_product_g1[2] * self[e3]))
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       12       23        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[e4]) * other.group1()).with_w(0.0) + (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x3::from(self[e4]))
                .with_w(-(geometric_anti_product_g0[0] * self[e1]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3])),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       20        0      N/A
    //  no simd       18       41        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                + Simd32x2::from(0.0).with_z(geometric_anti_product_g0[0] * self[e2] * -1.0)
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                - (geometric_anti_product_g0.yz() * self.group0().zx()).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()).with_w(
                -(geometric_anti_product_g0[0] * self[e1])
                    - (geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3])
                    - (geometric_anti_product_g1[3] * self[e4]),
            ),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       25        0        0
    //    simd2        0        3        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       21       39        0      N/A
    //  no simd       35       66        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        let geometric_anti_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g4[0] * self[e1])
                    + (geometric_anti_product_g4[1] * self[e2])
                    + (geometric_anti_product_g4[2] * self[e3])
                    + (geometric_anti_product_g4[3] * self[e4]),
                geometric_anti_product_g1[3] * self[e4] * -1.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (Simd32x3::from(other_g0 * self[e4]) * other.group3())
                + (geometric_anti_product_g2.zxy() * self.group0().yzx())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * geometric_anti_product_g4.xyz(),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + Simd32x2::from(0.0).with_z((geometric_anti_product_g4[0] * self[e2]) - (geometric_anti_product_g4[1] * self[e1]))
                + (geometric_anti_product_g4.yz() * self.group0().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.zx() * self.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g2 * Simd32x3::from(self[e4])).with_w(
                -(geometric_anti_product_g0[0] * self[e4])
                    - (geometric_anti_product_g2[0] * self[e1])
                    - (geometric_anti_product_g2[1] * self[e2])
                    - (geometric_anti_product_g2[2] * self[e3]),
            ),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        5        1      N/A
    //  no simd        0        7        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[e4] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_anti_product_g0 * -1.0) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        6       21        0      N/A
    //  no simd       18       37        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e423] * other[e423], other[e431] * other[e431], other[e412] * other[e412], other[e423] * other[e423]])
            * other.group0())
            + (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4] * -1.0) * geometric_anti_product_g0.xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g0[1] * self[e2]) + (geometric_anti_product_g0[2] * self[e3]) + (geometric_anti_product_g0[3] * self[e4]))
                - (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        7        2      N/A
    //  no simd        3       16        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(-1.0 / other[e4]) * (Simd32x3::from(1.0 / other[e4]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()) - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())).with_w(0.0),
        )
    }
}
impl std::ops::Div<GeometricAntiQuotientInfix> for Scalar {
    type Output = GeometricAntiQuotientInfixPartial<Scalar>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / other[e1234])
    }
}
impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        1        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e1234] * self[scalar] * 1.0 / (other[e1234] * other[e1234]))
    }
}
impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       11        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[scalar]) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * other[e4] * self[scalar] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2        9        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(-(other[e41] * other[e41] * self[scalar]) - (other[e42] * other[e42] * self[scalar]) - (other[e43] * other[e43] * self[scalar])) * other.group0(),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       16        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(
                (other[e41] * other[e41] * self[scalar])
                    + (other[e42] * other[e42] * self[scalar])
                    + (other[e43] * other[e43] * self[scalar])
                    + (other[e1234] * other[e1234] * self[scalar]),
            ) * other.group0()
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       22        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e4] * other[e4]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other_g0 * other[e1234] * self[scalar], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[scalar]) * other.group4().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[scalar] * -1.0) * other.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * other[e4] * self[scalar] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * -1.0 / other[e4])
    }
}
impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        3
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        8       17        0        3
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::powi(other.group0().xyz(), 3) * Simd32x3::from(self[scalar])).with_w(0.0)
                + (Simd32x3::from(other[e423] * self[scalar]) * other.group0().yxx() * other.group0().yyz()).with_w(0.0)
                + (Simd32x3::from(other[e412] * self[scalar]) * other.group0().xyy() * other.group0().zzy()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        1        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e4] * self[scalar] * (-1.0 / (other[e4] * other[e4])))
    }
}
