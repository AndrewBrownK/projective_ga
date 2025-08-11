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
//   Median:         3      11       0     N/A
//  Average:         7      15       0     N/A
//  Maximum:        94     115       3     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         6      21       0       0
//  Average:        14      29       0       0
//  Maximum:       179     213       4       3
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
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
    //      f32        0       16        0        3
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       19        0      N/A
    //  no simd        8       28        0        3
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234])
                * Simd32x4::from([f32::powi(other[e423], 3), f32::powi(other[e431], 3), f32::powi(other[e412], 3), other[e423] * other[e423] * other[e321]]))
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
    //      f32        0        2        1        0
    //    simd3        0        1        1      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        9        4        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234] * -1.0 / other[e4]) * (other.group0().xyz() / Simd32x4::from(other[e4]).xyz()).with_w(1.0),
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
    //      add/sub      mul      div      pow
    // f32        1        5        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = 1.0 / other[e1234];
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0_y * self[scalar]) + (other[scalar] * self[e1234] / (other[e1234] * other[e1234])),
            geometric_anti_product_g0_y * self[e1234],
        ]))
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        7       25        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g1.xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz()))
                .with_w(self[e1234] * geometric_anti_product_g0[3]),
            // e423, e431, e412, e321
            (geometric_anti_product_g1.xyz() * Simd32x2::from(self[e1234]).with_z(self[e1234]))
                .with_w((self[scalar] * geometric_anti_product_g0[3]) + (self[e1234] * geometric_anti_product_g1[3])),
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
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        7       24        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0 * Simd32x4::from(self[scalar])) + -(Simd32x3::from(other_g0 * self[e1234]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        7        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       11       27        0      N/A
    //  no simd       15       48        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
                .with_w(self[e1234] * geometric_anti_product_g1[3]),
            // e41, e42, e43
            geometric_anti_product_g2 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar])) - (Simd32x3::from(other_g0 * self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            (geometric_anti_product_g4.xyz() * Simd32x2::from(self[e1234]).with_z(self[e1234]))
                .with_w((self[scalar] * geometric_anti_product_g1[3]) + (self[e1234] * geometric_anti_product_g4[3])),
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
    //      f32        0        7        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        6        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       34        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (geometric_anti_product_g0.xyz() * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412, e321
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        1        0
    //    simd3        0        3        1      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        2      N/A
    //  no simd        0       15        4        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 =
            (other.group0().xyz() * Simd32x3::from(-1.0) / (Simd32x4::from(other[e4]).xyz() * Simd32x4::from(other[e4]).xyz())).with_w(-1.0 / other[e4]);
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * geometric_anti_product_g0[3]),
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
    //      f32        1        5        2        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        8        2      N/A
    //  no simd        4       14        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = other[scalar] / (other[e1234] * other[e1234]);
        let geometric_anti_product_g0_y = 1.0 / other[e1234];
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_y) * self.group0().xyz()) - (Simd32x3::from(geometric_anti_product_g0_x) * self.group1().xyz()))
                .with_w(geometric_anti_product_g0_y * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0_y) * self.group1().xyz()).with_w((geometric_anti_product_g0_y * self[e321]) - (geometric_anti_product_g0_x * self[e4])),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       15        0        0
    //    simd2        4        6        0      N/A
    //    simd4        7        8        0      N/A
    // Totals...
    // yes simd       21       29        0      N/A
    //  no simd       46       59        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, -(geometric_anti_product_g1[1] * self[e423]) - (geometric_anti_product_g1[2] * self[e4]), 0.0])
                + (geometric_anti_product_g1.yzxx() * self.group1().zxyx())
                + (-(Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]]))
                    - (Simd32x2::from([self[e431], self[e4]]) * geometric_anti_product_g1.zy()))
                .with_zw(0.0, 0.0)
                - (Simd32x4::from(geometric_anti_product_g0[3]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_anti_product_g1.yzxz() * self.group0().zxyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e4], self[e412]]))
                    + (Simd32x2::from([self[e431], self[e4]]) * geometric_anti_product_g0.zy())
                    - (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e321], self[e3]]))
                    - (Simd32x2::from([self[e2], self[e321]]) * geometric_anti_product_g1.zy()))
                .with_zw(
                    (geometric_anti_product_g0[1] * self[e423]) + (geometric_anti_product_g0[2] * self[e4])
                        - (geometric_anti_product_g1[1] * self[e1])
                        - (geometric_anti_product_g1[2] * self[e321]),
                    (geometric_anti_product_g1[0] * self[e1]) + (geometric_anti_product_g1[1] * self[e2])
                        - (geometric_anti_product_g0[2] * self[e412])
                        - (geometric_anti_product_g0[3] * self[e321]),
                )
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_anti_product_g0.wwwy() * self.group0().xyz().with_w(self[e431])),
        )
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        7       11        0      N/A
    // Totals...
    // yes simd       13       21        0      N/A
    //  no simd       27       43        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy())
                + (geometric_anti_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g1.xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g1.zyz())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0.zyz())
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()))
            .with_w(
                (geometric_anti_product_g1[0] * self[e423]) + (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g0[0] * self[e1])
                    - (geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3]),
            ),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       26        0        0
    //    simd2        2        4        0      N/A
    //    simd3        3        2        0      N/A
    //    simd4        2        6        0      N/A
    // Totals...
    // yes simd       26       38        0      N/A
    //  no simd       40       64        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e2]) + (geometric_anti_product_g1[1] * self[e412])
                    - (geometric_anti_product_g0[1] * self[e3])
                    - (geometric_anti_product_g1[2] * self[e431]),
                (geometric_anti_product_g0[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423])
                    - (geometric_anti_product_g0[2] * self[e1])
                    - (geometric_anti_product_g1[1] * self[e4]),
                (geometric_anti_product_g0[1] * self[e1]) + (geometric_anti_product_g0[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431])
                    - (geometric_anti_product_g0[0] * self[e2])
                    - (geometric_anti_product_g1[1] * self[e423])
                    - (geometric_anti_product_g1[2] * self[e4]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e321], self[e3]]))
                    - (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]])))
                .with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()))
            .with_w(geometric_anti_product_g0[3] * self[e4]),
            // e423, e431, e412, e321
            (self.group1().xyzz() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[2]))
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e4], self[e412]]))
                    + (Simd32x2::from([self[e431], self[e4]]) * geometric_anti_product_g0.zy()))
                .with_zw(
                    (geometric_anti_product_g0[1] * self[e423]) + (geometric_anti_product_g0[2] * self[e4]),
                    (geometric_anti_product_g1[0] * self[e423]) + (geometric_anti_product_g1[1] * self[e431])
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
    //      f32       31       46        0        0
    //    simd2        4        7        0      N/A
    //    simd3       14       16        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       50       72        0      N/A
    //  no simd       85      120        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
                    - (geometric_anti_product_g1[2] * self[e412])
                    - (geometric_anti_product_g1[3] * self[e321]),
                geometric_anti_product_g1[3] * self[e4] * -1.0,
            ]) + (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g4[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g4[2]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy())
                + (geometric_anti_product_g3.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group1().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g3.xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g3.zyz())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_anti_product_g4[1] * self[e412]) - (geometric_anti_product_g4[2] * self[e431]),
                (geometric_anti_product_g4[2] * self[e423]) - (geometric_anti_product_g4[1] * self[e4]),
                (geometric_anti_product_g4[0] * self[e431]) - (geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[2] * self[e4]),
            ]) + -(Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_anti_product_g1[2] * self[e431]) + (geometric_anti_product_g4[1] * self[e3])
                    - (geometric_anti_product_g1[1] * self[e412])
                    - (geometric_anti_product_g4[2] * self[e2]),
                (geometric_anti_product_g1[1] * self[e4]) + (geometric_anti_product_g4[2] * self[e1])
                    - (geometric_anti_product_g1[2] * self[e423])
                    - (geometric_anti_product_g4[1] * self[e321]),
                (geometric_anti_product_g1[1] * self[e423]) + (geometric_anti_product_g1[2] * self[e4]) + (geometric_anti_product_g4[0] * self[e2])
                    - (geometric_anti_product_g1[0] * self[e431])
                    - (geometric_anti_product_g4[1] * self[e1])
                    - (geometric_anti_product_g4[2] * self[e321]),
            ]) + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group1().xyz())
                + ((Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]]))
                    - (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e321], self[e3]])))
                .with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + ((Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g2.xxy())
                    + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g2.zyz())
                    - (geometric_anti_product_g2.yzx() * self.group1().zxy()))
                .with_w(
                    (geometric_anti_product_g3[0] * self[e423]) + (geometric_anti_product_g3[1] * self[e431])
                        - (geometric_anti_product_g0[0] * self[e4])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3]),
                ),
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
    //      f32        5       17        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        0        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       12       26        0      N/A
    //  no simd       30       47        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from([
                (geometric_anti_product_g0[1] * self[e412]) - (geometric_anti_product_g0[2] * self[e431]),
                (geometric_anti_product_g0[2] * self[e423]) - (geometric_anti_product_g0[1] * self[e4]),
                (geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g0[1] * self[e423]) - (geometric_anti_product_g0[2] * self[e4]),
            ]) + -(Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0))
            .with_w(geometric_anti_product_g0[0] * self[e423]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, -(geometric_anti_product_g0[1] * self[e1]) - (geometric_anti_product_g0[2] * self[e321]), 0.0])
                + (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + (geometric_anti_product_g0.wwwy() * self.group1().xyz().with_w(self[e2]))
                + (-(Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e321], self[e3]]))
                    - (Simd32x2::from([self[e2], self[e321]]) * geometric_anti_product_g0.zy()))
                .with_zw(0.0, 0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        2        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       10        2      N/A
    //  no simd        9       23        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(-1.0 / (other[e4] * other[e4])) * other.group0().xyz();
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0_w * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            ((Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0_xyz.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0_xyz.zyz())
                - (Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz())
                - (geometric_anti_product_g0_xyz.yzx() * self.group1().zxy()))
            .with_w(0.0),
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
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] / other[e1234])
    }
}
impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       14        0        3
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3       18        0      N/A
    //  no simd        9       26        0        3
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (-(Simd32x3::from(self[e321]) * Simd32x3::from([f32::powi(other[e423], 3), f32::powi(other[e431], 3), f32::powi(other[e412], 3)]))
                - (Simd32x3::from(self[e321])
                    * Simd32x3::from([
                        other[e431] * other[e431] * other[e423],
                        other[e423] * other[e423] * other[e431],
                        other[e423] * other[e423] * other[e412],
                    ]))
                - (Simd32x3::from(self[e321])
                    * Simd32x3::from([
                        other[e412] * other[e412] * other[e423],
                        other[e412] * other[e412] * other[e431],
                        other[e431] * other[e431] * other[e412],
                    ]))
                - (Simd32x3::from(other[e4] * other[e4] * self[e321]) * other.group1().xyz()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        0        3
    //    simd3        2        7        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       21        0        3
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (-(Simd32x3::powi(other.group0(), 3) * Simd32x3::from(self[e321]))
                - (Simd32x3::powi(other.group0().yxx(), 2) * Simd32x3::from(self[e321]) * other.group0())
                - (Simd32x3::powi(other.group0().zzy(), 2) * Simd32x3::from(self[e321]) * other.group0()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        3
    //    simd2        2        3        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       13       19        0        3
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from([
            0.0,
            0.0,
            -f32::powi(other[e43], 3) - (other[e41] * other[e41] * other[e43]) - (other[e42] * other[e42] * other[e43]),
            0.0,
        ]) + ((-Simd32x2::powi(other.group0().xy(), 3)
            - (Simd32x2::from(other[e43] * other[e43]) * other.group0().xy())
            - (Simd32x2::powi(other.group0().yx(), 2) * other.group0().xy()))
        .with_z(0.0)
            - (Simd32x3::from(other[e1234] * other[e1234]) * other.group0().xyz()))
        .with_w(0.0);
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
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
            + other[e423] * other[e423]
            + other[e431] * other[e431]
            + other[e412] * other[e412];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other_g0 * other[e4] * self[e321], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * other[e1234] * self[e321]),
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
    //      f32        0       12        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        6       21        0        3
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from(self[e321]) * Simd32x3::from([f32::powi(other[e423], 3), f32::powi(other[e431], 3), f32::powi(other[e412], 3)]))
                - (Simd32x3::from(self[e321])
                    * Simd32x3::from([
                        other[e431] * other[e431] * other[e423],
                        other[e423] * other[e423] * other[e431],
                        other[e423] * other[e423] * other[e412],
                    ]))
                - (Simd32x3::from(self[e321])
                    * Simd32x3::from([
                        other[e412] * other[e412] * other[e423],
                        other[e412] * other[e412] * other[e431],
                        other[e431] * other[e431] * other[e412],
                    ])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / other[e4])
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
    //      f32        0        2        2        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        3       11        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = 1.0 / other[e1234];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0_y) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0_y) * self.group1()) + (Simd32x3::from(other[scalar] / (other[e1234] * other[e1234])) * self.group0()),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        7        9        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       24       40        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((self.group0().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g0[1]))
                + (self.group0().yzz() * geometric_anti_product_g0.zx().with_z(geometric_anti_product_g1[3]))
                + (self.group1().xyx() * Simd32x2::from(geometric_anti_product_g0[3]).with_z(geometric_anti_product_g1[1]))
                + (self.group1().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g0[3]))
                - (self.group0().zxy() * geometric_anti_product_g0.yzx())
                - (self.group1().zxy() * geometric_anti_product_g1.yzx()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((self.group0().xyx() * Simd32x2::from(geometric_anti_product_g0[3]).with_z(geometric_anti_product_g1[1]))
                + (self.group0().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g0[3]))
                - (self.group0().zxy() * geometric_anti_product_g1.yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       29        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_anti_product_g0.zxy() * self.group0().yzx()) - (geometric_anti_product_g0.yzx() * self.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_anti_product_g0.zxy() * self.group1().yzx()) + (geometric_anti_product_g1.zxy() * self.group0().yzx())
                - (geometric_anti_product_g0.yzx() * self.group1().zxy())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        7        9        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       24       47        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group0().xyx() * geometric_anti_product_g0.wwy()) + (self.group0().yzz() * geometric_anti_product_g0.zxw())
                - (self.group0().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            ((self.group0().xyx() * geometric_anti_product_g1.wwy())
                + (self.group0().yzz() * geometric_anti_product_g1.zxw())
                + (self.group1().xyx() * geometric_anti_product_g0.wwy())
                + (self.group1().yzz() * geometric_anti_product_g0.zxw())
                - (self.group0().zxy() * geometric_anti_product_g1.yzx())
                - (self.group1().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       14        0        0
    //    simd2        3        4        0      N/A
    //    simd3       14       20        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       26       40        0      N/A
    //  no simd       57       90        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
            ((self.group0().xyx() * Simd32x2::from(geometric_anti_product_g4[3]).with_z(geometric_anti_product_g1[1]))
                + (self.group0().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g4[3]))
                + (self.group1().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g4[1]))
                + (self.group1().yzz() * geometric_anti_product_g4.zx().with_z(geometric_anti_product_g1[3]))
                - (self.group0().zxy() * geometric_anti_product_g1.yzx())
                - (self.group1().zxy() * geometric_anti_product_g4.yzx()))
            .with_w(0.0),
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
            ((self.group0().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g4[1]))
                + (self.group0().yzz() * geometric_anti_product_g4.zx().with_z(geometric_anti_product_g1[3]))
                - (self.group0().zxy() * geometric_anti_product_g4.yzx()))
            .with_w(0.0),
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
    //      f32        0        7        0        0
    //    simd3        3        5        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        5       16        0      N/A
    //  no simd       17       38        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0()) + (self.group1().yzx() * geometric_anti_product_g0.zxy())
                - (self.group1().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((self.group0().yzx() * geometric_anti_product_g0.zxy()) - (self.group0().zxy() * geometric_anti_product_g0.yzx())).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        2        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4       11        2      N/A
    //  no simd        8       21        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(-1.0 / (other[e4] * other[e4])) * other.group0().xyz();
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_w) * self.group1()) + (geometric_anti_product_g0_xyz.zxy() * self.group0().yzx())
                - (geometric_anti_product_g0_xyz.yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0_w) * self.group0())
                .with_w(-(geometric_anti_product_g0_xyz[0] * self[e41]) - (geometric_anti_product_g0_xyz[1] * self[e42]) - (geometric_anti_product_g0_xyz[2] * self[e43])),
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
    //      f32        0        2        2        0
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        4       14        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_y = 1.0 / other[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0_y) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product_g0_y) * self.group1()) + (Simd32x4::from(other[scalar] / (other[e1234] * other[e1234])) * self.group0()),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       22        0        0
    //    simd2        2        4        0      N/A
    //    simd3        3        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       23       32        0      N/A
    //  no simd       40       52        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e42]) + (geometric_anti_product_g1[2] * self[e31])
                    - (geometric_anti_product_g0[1] * self[e43])
                    - (geometric_anti_product_g1[1] * self[e12]),
                (geometric_anti_product_g0[1] * self[e1234]) + (geometric_anti_product_g1[1] * self[scalar])
                    - (geometric_anti_product_g0[2] * self[e41])
                    - (geometric_anti_product_g1[2] * self[e23]),
                (geometric_anti_product_g0[1] * self[e41])
                    + (geometric_anti_product_g0[2] * self[e1234])
                    + (geometric_anti_product_g1[1] * self[e23])
                    + (geometric_anti_product_g1[2] * self[scalar])
                    - (geometric_anti_product_g0[0] * self[e42])
                    - (geometric_anti_product_g1[0] * self[e31]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * self.group0().wz()) + (Simd32x2::from(geometric_anti_product_g1[0]) * self.group1().wz())).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, (geometric_anti_product_g1[2] * self[e1234]) - (geometric_anti_product_g1[0] * self[e42]), 0.0])
                + (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0().xyz().with_w(self[scalar]))
                + (geometric_anti_product_g1.xxyw() * self.group0().wzxw())
                + ((geometric_anti_product_g1.zy() * self.group0().yw()) - (geometric_anti_product_g1.yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        7       11        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       23       38        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_anti_product_g0.xxy() * self.group0().wzx()) + (geometric_anti_product_g0.zyz() * self.group0().yww())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_anti_product_g0.xxy() * self.group1().wzx())
                + (geometric_anti_product_g0.zyz() * self.group1().yww())
                + (geometric_anti_product_g1.xxy() * self.group0().wzx())
                + (geometric_anti_product_g1.zyz() * self.group0().yww())
                - (geometric_anti_product_g0.yzx() * self.group1().zxy())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       18        0        0
    //    simd2        4        6        0      N/A
    //    simd3        4        3        0      N/A
    //    simd4        1        5        0      N/A
    // Totals...
    // yes simd       20       32        0      N/A
    //  no simd       35       59        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e42]) - (geometric_anti_product_g0[1] * self[e43]),
                (geometric_anti_product_g0[1] * self[e1234]) - (geometric_anti_product_g0[2] * self[e41]),
                (geometric_anti_product_g0[1] * self[e41]) + (geometric_anti_product_g0[2] * self[e1234]) - (geometric_anti_product_g0[0] * self[e42]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + (Simd32x2::from(geometric_anti_product_g0[0]) * self.group0().wz()).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0.xxyw() * self.group1().wzxw())
                + ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                    + ((Simd32x2::from(geometric_anti_product_g1[0]) * self.group0().wz())
                        + (geometric_anti_product_g0.zy() * self.group1().yw())
                        + (geometric_anti_product_g1.zy() * self.group0().yw())
                        - (geometric_anti_product_g0.yz() * self.group1().zx())
                        - (geometric_anti_product_g1.yz() * self.group0().zx()))
                    .with_z(
                        (geometric_anti_product_g0[2] * self[scalar]) + (geometric_anti_product_g1[1] * self[e41]) + (geometric_anti_product_g1[2] * self[e1234])
                            - (geometric_anti_product_g0[0] * self[e31])
                            - (geometric_anti_product_g1[0] * self[e42]),
                    ))
                .with_w(geometric_anti_product_g1[3] * self[e1234]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       35        0        0
    //    simd2        5        8        0      N/A
    //    simd3       13       16        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       44       63        0      N/A
    //  no simd       84      115        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
                    - (geometric_anti_product_g2[2] * self[e12])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43]),
                geometric_anti_product_g2[0] * self[e41] * -1.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                - (Simd32x2::from([self[e23], self[e42]]) * geometric_anti_product_g2.xy())
                - (Simd32x2::from([self[e31], self[e43]]) * geometric_anti_product_g2.yz()),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_anti_product_g1[2] * self[e42]) + (geometric_anti_product_g4[2] * self[e31])
                    - (geometric_anti_product_g1[1] * self[e43])
                    - (geometric_anti_product_g4[1] * self[e12]),
                (geometric_anti_product_g1[1] * self[e1234]) + (geometric_anti_product_g4[1] * self[scalar])
                    - (geometric_anti_product_g1[2] * self[e41])
                    - (geometric_anti_product_g4[2] * self[e23]),
                (geometric_anti_product_g1[1] * self[e41])
                    + (geometric_anti_product_g1[2] * self[e1234])
                    + (geometric_anti_product_g4[1] * self[e23])
                    + (geometric_anti_product_g4[2] * self[scalar])
                    - (geometric_anti_product_g1[0] * self[e42])
                    - (geometric_anti_product_g4[0] * self[e31]),
            ]) + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz())
                + ((Simd32x2::from(geometric_anti_product_g1[0]) * self.group0().wz()) + (Simd32x2::from(geometric_anti_product_g4[0]) * self.group1().wz())).with_z(0.0))
            .with_w(geometric_anti_product_g1[3] * self[e1234]),
            // e41, e42, e43
            (Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (geometric_anti_product_g2.xxy() * self.group0().wzx())
                + (geometric_anti_product_g2.zyz() * self.group0().yww())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0().xyz())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group1().xyz())
                + (geometric_anti_product_g2.xxy() * self.group1().wzx())
                + (geometric_anti_product_g2.zyz() * self.group1().yww())
                + (geometric_anti_product_g3.xxy() * self.group0().wzx())
                + (geometric_anti_product_g3.zyz() * self.group0().yww())
                - (geometric_anti_product_g2.yzx() * self.group1().zxy())
                - (geometric_anti_product_g3.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, (geometric_anti_product_g4[2] * self[e1234]) - (geometric_anti_product_g4[0] * self[e42]), 0.0])
                + (Simd32x4::from(geometric_anti_product_g1[3]) * self.group0().xyz().with_w(self[scalar]))
                + (geometric_anti_product_g4.xxyw() * self.group0().wzxw())
                + ((geometric_anti_product_g4.zy() * self.group0().yw()) - (geometric_anti_product_g4.yz() * self.group0().zx())).with_zw(0.0, 0.0),
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
    //      f32        6       18        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        1        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       13       27        0      N/A
    //  no simd       28       45        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                (geometric_anti_product_g0[1] * self[e23]) + (geometric_anti_product_g0[2] * self[scalar]) - (geometric_anti_product_g0[0] * self[e31]),
                0.0,
            ]) + ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * self.group1().wz()) + (geometric_anti_product_g0.zy() * self.group1().yw())
                    - (geometric_anti_product_g0.yz() * self.group1().zx()))
                .with_z(0.0))
            .with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e42]) - (geometric_anti_product_g0[1] * self[e43]),
                (geometric_anti_product_g0[1] * self[e1234]) - (geometric_anti_product_g0[2] * self[e41]),
                (geometric_anti_product_g0[1] * self[e41]) + (geometric_anti_product_g0[2] * self[e1234]) - (geometric_anti_product_g0[0] * self[e42]),
            ]) + (Simd32x2::from(geometric_anti_product_g0[0]) * self.group0().wz()).with_z(0.0))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        2        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        6       14        2      N/A
    //  no simd       12       26        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(-1.0 / (other[e4] * other[e4])) * other.group0().xyz();
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_w) * self.group1().xyz())
                + (geometric_anti_product_g0_xyz.xxy() * self.group0().wzx())
                + (geometric_anti_product_g0_xyz.zyz() * self.group0().yww())
                - (geometric_anti_product_g0_xyz.yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product_g0_w * self[e1234]),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()).with_w(
                (geometric_anti_product_g0_w * self[scalar])
                    - (geometric_anti_product_g0_xyz[0] * self[e41])
                    - (geometric_anti_product_g0_xyz[1] * self[e42])
                    - (geometric_anti_product_g0_xyz[2] * self[e43]),
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
    //      f32        2        8        2        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       14        2      N/A
    //  no simd        8       26        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_x = other[scalar] / (other[e1234] * other[e1234]);
        let geometric_anti_product_g0_y = 1.0 / other[e1234];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0_x * self[e1234]) + (geometric_anti_product_g0_y * self[scalar]),
                geometric_anti_product_g0_y * self[e1234],
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0_y) * self.group1().xyz()) - (Simd32x3::from(geometric_anti_product_g0_x) * self.group4().xyz()))
                .with_w(geometric_anti_product_g0_y * self[e4]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0_y) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0_x) * self.group2()) + (Simd32x3::from(geometric_anti_product_g0_y) * self.group3()),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0_y) * self.group4().xyz()).with_w((geometric_anti_product_g0_y * self[e321]) - (geometric_anti_product_g0_x * self[e4])),
        )
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       23       36        0        0
    //    simd2        4        6        0      N/A
    //    simd3       15       15        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       42       59        0      N/A
    //  no simd       76      101        0        0
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
                    - (geometric_anti_product_g0[2] * self[e412])
                    - (geometric_anti_product_g0[3] * self[e321]),
                geometric_anti_product_g0[3] * self[e4] * -1.0,
            ]) + (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g1[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g1[2]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (self.group2().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g0[1]))
                + (self.group2().yzz() * geometric_anti_product_g0.zx().with_z(geometric_anti_product_g1[3]))
                + (self.group3().xyx() * Simd32x2::from(geometric_anti_product_g0[3]).with_z(geometric_anti_product_g1[1]))
                + (self.group3().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g0[3]))
                - (self.group2().zxy() * geometric_anti_product_g0.yzx())
                - (self.group3().zxy() * geometric_anti_product_g1.yzx()))
            .with_w(self[e1234] * geometric_anti_product_g0[3]),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_anti_product_g1[1] * self[e412]) - (geometric_anti_product_g1[2] * self[e431]),
                (geometric_anti_product_g1[2] * self[e423]) - (geometric_anti_product_g1[1] * self[e4]),
                (geometric_anti_product_g1[0] * self[e431]) - (geometric_anti_product_g1[1] * self[e423]) - (geometric_anti_product_g1[2] * self[e4]),
            ]) + -(Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e431]) + (geometric_anti_product_g1[1] * self[e3])
                    - (geometric_anti_product_g0[1] * self[e412])
                    - (geometric_anti_product_g1[2] * self[e2]),
                (geometric_anti_product_g0[1] * self[e4]) + (geometric_anti_product_g1[2] * self[e1])
                    - (geometric_anti_product_g0[2] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e321]),
                (geometric_anti_product_g0[1] * self[e423]) + (geometric_anti_product_g0[2] * self[e4]) + (geometric_anti_product_g1[0] * self[e2])
                    - (geometric_anti_product_g0[0] * self[e431])
                    - (geometric_anti_product_g1[1] * self[e1])
                    - (geometric_anti_product_g1[2] * self[e321]),
            ]) + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e4], self[e412]]))
                    - (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e321], self[e3]])))
                .with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz())
                + (self.group2().xyx() * Simd32x2::from(geometric_anti_product_g0[3]).with_z(geometric_anti_product_g1[1]))
                + (self.group2().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g0[3]))
                - (self.group2().zxy() * geometric_anti_product_g1.yzx()))
            .with_w((self[scalar] * geometric_anti_product_g0[3]) + (self[e1234] * geometric_anti_product_g1[3])),
        )
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd2        3        3        0      N/A
    //    simd3       14       20        0      N/A
    // Totals...
    // yes simd       25       36        0      N/A
    //  no simd       56       79        0        0
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
            ((Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy())
                + (geometric_anti_product_g1.yzx() * self.group4().zxy())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g1.xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g1.zyz())
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()))
            .with_w(0.0),
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
            ((Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0.zyz())
                - (geometric_anti_product_g0.yzx() * self.group4().zxy()))
            .with_w(
                (geometric_anti_product_g1[0] * self[e423]) + (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g0[0] * self[e1])
                    - (geometric_anti_product_g0[1] * self[e2])
                    - (geometric_anti_product_g0[2] * self[e3]),
            ),
        )
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       21       31        0        0
    //    simd2        4        6        0      N/A
    //    simd3       13       14        0      N/A
    //    simd4        3        7        0      N/A
    // Totals...
    // yes simd       41       58        0      N/A
    //  no simd       80      113        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1234] * geometric_anti_product_g1[3])
                    - (self[e43] * geometric_anti_product_g1[2])
                    - (self[e23] * geometric_anti_product_g0[0])
                    - (self[e31] * geometric_anti_product_g0[1])
                    - (self[e12] * geometric_anti_product_g0[2]),
                self[e41] * geometric_anti_product_g0[0] * -1.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * self.group0())
                - (Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[1]]) * self.group2().xy())
                - (Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[2]]) * self.group2().yz()),
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e2]) + (geometric_anti_product_g1[1] * self[e412])
                    - (geometric_anti_product_g0[1] * self[e3])
                    - (geometric_anti_product_g1[2] * self[e431]),
                (geometric_anti_product_g0[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423])
                    - (geometric_anti_product_g0[2] * self[e1])
                    - (geometric_anti_product_g1[1] * self[e4]),
                (geometric_anti_product_g0[1] * self[e1]) + (geometric_anti_product_g0[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431])
                    - (geometric_anti_product_g0[0] * self[e2])
                    - (geometric_anti_product_g1[1] * self[e423])
                    - (geometric_anti_product_g1[2] * self[e4]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                + ((Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e321], self[e3]]))
                    - (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]])))
                .with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz()))
            .with_w(geometric_anti_product_g0[3] * self[e4]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (self.group2().xyx() * geometric_anti_product_g0.wwy())
                + (self.group2().yzz() * geometric_anti_product_g0.zxw())
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_anti_product_g0.xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz())
                + (self.group2().xyx() * geometric_anti_product_g1.wwy())
                + (self.group2().yzz() * geometric_anti_product_g1.zxw())
                + (self.group3().xyx() * geometric_anti_product_g0.wwy())
                + (self.group3().yzz() * geometric_anti_product_g0.zxw())
                - (self.group2().zxy() * geometric_anti_product_g1.yzx())
                - (self.group3().zxy() * geometric_anti_product_g0.yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * geometric_anti_product_g0.xxyw())
                + (self.group4().xyzy() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[1]))
                + (Simd32x2::from([self[e431], self[e4]]) * geometric_anti_product_g0.zy()).with_zw(
                    geometric_anti_product_g0[2] * self[e4],
                    (geometric_anti_product_g1[0] * self[e423])
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
    //      f32       50       64        0        0
    //    simd2        5        8        0      N/A
    //    simd3       37       39        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       94      115        0      N/A
    //  no simd      179      213        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g4[2] * self[e3]) + (geometric_anti_product_g4[3] * self[e4])
                    - (geometric_anti_product_g2[1] * self[e31])
                    - (geometric_anti_product_g2[2] * self[e12])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g1[0] * self[e423])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412])
                    - (geometric_anti_product_g1[3] * self[e321]),
                (geometric_anti_product_g4[0] * self[e423])
                    - (geometric_anti_product_g2[0] * self[e41])
                    - (geometric_anti_product_g2[1] * self[e42])
                    - (geometric_anti_product_g1[3] * self[e4]),
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from([self[e1], self[e431]]) * geometric_anti_product_g4.xy())
                + (Simd32x2::from([self[e2], self[e412]]) * geometric_anti_product_g4.yz())
                - (Simd32x2::from([self[e23], self[e43]]) * geometric_anti_product_g2.xz()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + ((Simd32x3::from(self[scalar]) * geometric_anti_product_g4.xyz())
                    + (Simd32x3::from(self[e1234]) * geometric_anti_product_g1.xyz())
                    + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz())
                    + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy())
                    + (geometric_anti_product_g3.yzx() * self.group4().zxy())
                    + (self.group2().xyx() * Simd32x2::from(geometric_anti_product_g4[3]).with_z(geometric_anti_product_g1[1]))
                    + (self.group2().yzz() * geometric_anti_product_g1.zx().with_z(geometric_anti_product_g4[3]))
                    + (self.group3().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g4[1]))
                    + (self.group3().yzz() * geometric_anti_product_g4.zx().with_z(geometric_anti_product_g1[3]))
                    - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group4().xyz())
                    - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g3.xxy())
                    - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g3.zyz())
                    - (geometric_anti_product_g2.yzx() * self.group1().zxy())
                    - (self.group2().zxy() * geometric_anti_product_g1.yzx())
                    - (self.group3().zxy() * geometric_anti_product_g4.yzx()))
                .with_w(self[e1234] * geometric_anti_product_g1[3]),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_anti_product_g4[1] * self[e412]) - (geometric_anti_product_g4[2] * self[e431]),
                (geometric_anti_product_g4[2] * self[e423]) - (geometric_anti_product_g4[1] * self[e4]),
                (geometric_anti_product_g4[0] * self[e431]) - (geometric_anti_product_g4[1] * self[e423]) - (geometric_anti_product_g4[2] * self[e4]),
            ]) + (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group2())
                + (geometric_anti_product_g2.zxy() * self.group2().yzx())
                + -(Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (geometric_anti_product_g2.yzx() * self.group2().zxy()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_anti_product_g1[2] * self[e431]) + (geometric_anti_product_g4[1] * self[e3])
                    - (geometric_anti_product_g1[1] * self[e412])
                    - (geometric_anti_product_g4[2] * self[e2]),
                (geometric_anti_product_g1[1] * self[e4]) + (geometric_anti_product_g4[2] * self[e1])
                    - (geometric_anti_product_g1[2] * self[e423])
                    - (geometric_anti_product_g4[1] * self[e321]),
                (geometric_anti_product_g1[1] * self[e423]) + (geometric_anti_product_g1[2] * self[e4]) + (geometric_anti_product_g4[0] * self[e2])
                    - (geometric_anti_product_g1[0] * self[e431])
                    - (geometric_anti_product_g4[1] * self[e1])
                    - (geometric_anti_product_g4[2] * self[e321]),
            ]) + (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                + (geometric_anti_product_g2.zxy() * self.group3().yzx())
                + (geometric_anti_product_g3.zxy() * self.group2().yzx())
                + ((Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e4], self[e412]]))
                    - (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e321], self[e3]])))
                .with_z(0.0)
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (geometric_anti_product_g2.yzx() * self.group3().zxy())
                - (geometric_anti_product_g3.yzx() * self.group2().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + ((Simd32x3::from(self[e1234]) * geometric_anti_product_g4.xyz())
                    + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g2.xxy())
                    + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g2.zyz())
                    + (self.group2().xyx() * Simd32x2::from(geometric_anti_product_g1[3]).with_z(geometric_anti_product_g4[1]))
                    + (self.group2().yzz() * geometric_anti_product_g4.zx().with_z(geometric_anti_product_g1[3]))
                    - (geometric_anti_product_g2.yzx() * self.group4().zxy())
                    - (self.group2().zxy() * geometric_anti_product_g4.yzx()))
                .with_w(
                    (self[scalar] * geometric_anti_product_g1[3])
                        + (self[e1234] * geometric_anti_product_g4[3])
                        + (geometric_anti_product_g3[0] * self[e423])
                        + (geometric_anti_product_g3[1] * self[e431])
                        + (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g0[0] * self[e4])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (self[e41] * geometric_anti_product_g1[0])
                        - (self[e42] * geometric_anti_product_g1[1])
                        - (self[e43] * geometric_anti_product_g1[2])
                        - (self[e23] * geometric_anti_product_g4[0])
                        - (self[e31] * geometric_anti_product_g4[1])
                        - (self[e12] * geometric_anti_product_g4[2]),
                ),
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
    //      f32        8       23        0        0
    //    simd2        3        5        0      N/A
    //    simd3        8        8        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       21       40        0      N/A
    //  no simd       46       73        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_anti_product_g0[3] * self[e4], 0.0])
                + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g0.xyz())
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group2())
                + (self.group3().yzx() * geometric_anti_product_g0.zxy())
                - (self.group3().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_anti_product_g0[1] * self[e412]) - (geometric_anti_product_g0[2] * self[e431]),
                (geometric_anti_product_g0[2] * self[e423]) - (geometric_anti_product_g0[1] * self[e4]),
                (geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g0[1] * self[e423]) - (geometric_anti_product_g0[2] * self[e4]),
            ]) + -(Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_anti_product_g0[1] * self[e3]) - (geometric_anti_product_g0[2] * self[e2]),
                (geometric_anti_product_g0[2] * self[e1]) - (geometric_anti_product_g0[1] * self[e321]),
                (geometric_anti_product_g0[0] * self[e2]) - (geometric_anti_product_g0[1] * self[e1]) - (geometric_anti_product_g0[2] * self[e321]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz())
                + -(Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e321], self[e3]])).with_z(0.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz()) + (self.group2().yzx() * geometric_anti_product_g0.zxy())
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(self[e1234] * geometric_anti_product_g0[3]),
        )
    }
}
impl GeometricAntiQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        2        0
    //    simd3        6       11        0      N/A
    // Totals...
    // yes simd       12       26        2      N/A
    //  no simd       24       48        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(-1.0 / (other[e4] * other[e4])) * other.group0().xyz();
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g0_w * self[e321])
                    - (geometric_anti_product_g0_xyz[0] * self[e423])
                    - (geometric_anti_product_g0_xyz[1] * self[e431])
                    - (geometric_anti_product_g0_xyz[2] * self[e412]),
                geometric_anti_product_g0_w * self[e4] * -1.0,
            ]),
            // e1, e2, e3, e4
            ((geometric_anti_product_g0_xyz * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0_w) * self.group3())
                + (geometric_anti_product_g0_xyz.zxy() * self.group2().yzx())
                - (geometric_anti_product_g0_xyz.yzx() * self.group2().zxy()))
            .with_w(geometric_anti_product_g0_w * self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0_w * -1.0) * self.group4().xyz(),
            // e23, e31, e12
            (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0_xyz.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0_xyz.zyz())
                - (Simd32x3::from(geometric_anti_product_g0_w) * self.group1().xyz())
                - (geometric_anti_product_g0_xyz.yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_anti_product_g0_w) * self.group2()).with_w(
                (geometric_anti_product_g0_w * self[scalar])
                    - (geometric_anti_product_g0_xyz[0] * self[e41])
                    - (geometric_anti_product_g0_xyz[1] * self[e42])
                    - (geometric_anti_product_g0_xyz[2] * self[e43]),
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
    //      add/sub      mul      div      pow
    // f32        0        5        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e4] / other[e1234]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4] * -1.0 / (other[e1234] * other[e1234])),
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
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
    //      f32        0        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       28        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
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
    //      f32        0        2        1        0
    //    simd3        0        3        1      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0       11        4        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e4] / other[e4]),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e4] * -1.0) * other.group0().xyz() / (Simd32x4::from(other[e4]).xyz() * Simd32x4::from(other[e4]).xyz())).with_w(0.0),
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
    //      f32        0        5        3        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        9        3      N/A
    //  no simd        0       19        3        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, other[scalar] / (other[e1234] * other[e1234]), 0.0])
                * (self.group0().xyz() * Simd32x2::from(other[scalar] * -1.0 / (other[e1234] * other[e1234])).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(1.0 / other[e1234]) * self.group0(),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       22       35        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from([
                (geometric_anti_product_g1[1] * self[e412]) - (geometric_anti_product_g1[2] * self[e431]),
                (geometric_anti_product_g1[2] * self[e423]) - (geometric_anti_product_g1[0] * self[e412]),
                (geometric_anti_product_g1[0] * self[e431]) - (geometric_anti_product_g1[1] * self[e423]),
            ]) - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()))
            .with_w(geometric_anti_product_g1[0] * self[e423]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (geometric_anti_product_g0[1] * self[e423]) - (geometric_anti_product_g0[0] * self[e431]), 0.0])
                + ((Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                    + ((geometric_anti_product_g0.zx() * self.group0().yz()) - (geometric_anti_product_g0.yz() * self.group0().zx())).with_z(0.0)
                    - (Simd32x3::from(self[e321]) * geometric_anti_product_g1.xyz()))
                .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd       11       27        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((geometric_anti_product_g0 * Simd32x3::from(self[e321])) + (geometric_anti_product_g1.yzx() * self.group0().zxy())
                - (geometric_anti_product_g1.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((geometric_anti_product_g0.zxy() * self.group0().yzx()) - (geometric_anti_product_g0.yzx() * self.group0().zxy())).with_w(geometric_anti_product_g1[0] * self[e423]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       11       19        0      N/A
    //  no simd       28       44        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (geometric_anti_product_g1[0] * self[e431]) - (geometric_anti_product_g1[1] * self[e423]), 0.0])
                + ((Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                    + ((geometric_anti_product_g1.yz() * self.group0().zx()) - (geometric_anti_product_g1.zx() * self.group0().yz())).with_z(0.0)
                    - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()))
                .with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, geometric_anti_product_g0[0] * self[e431] * -1.0, 0.0])
                + (geometric_anti_product_g0.zxyw() * self.group0().yzxw())
                + (self.group0().xyzx() * Simd32x3::from(geometric_anti_product_g0[3]).with_w(geometric_anti_product_g1[0]))
                + -(geometric_anti_product_g0.yz() * self.group0().zx()).with_zw(0.0, 0.0),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       30        0        0
    //    simd2        0        1        0      N/A
    //    simd3        6       10        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       26       45        0      N/A
    //  no simd       44       78        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
            ((geometric_anti_product_g2 * Simd32x3::from(self[e321])) + (geometric_anti_product_g3.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group0().xyz())
                - (geometric_anti_product_g3.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_anti_product_g4[1] * self[e412]) - (geometric_anti_product_g4[2] * self[e431]),
                (geometric_anti_product_g4[2] * self[e423]) - (geometric_anti_product_g4[0] * self[e412]),
                (geometric_anti_product_g4[0] * self[e431]) - (geometric_anti_product_g4[1] * self[e423]),
            ]) - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_anti_product_g1[2] * self[e431]) - (geometric_anti_product_g1[1] * self[e412]),
                (geometric_anti_product_g1[0] * self[e412]) - (geometric_anti_product_g1[2] * self[e423]),
                (geometric_anti_product_g1[1] * self[e423]) - (geometric_anti_product_g1[0] * self[e431]),
            ]) + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group0())
                + (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                + -(geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(0.0),
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
    //      f32        1       10        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        5       18        0      N/A
    //  no simd       14       36        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_anti_product_g0.yz() * self.group0().zx()) - (geometric_anti_product_g0.zx() * self.group0().yz())).with_zw(
                (geometric_anti_product_g0[0] * self[e431]) - (geometric_anti_product_g0[1] * self[e423]),
                geometric_anti_product_g0[0] * self[e423],
            ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        2        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        1       10        2      N/A
    //  no simd        3       18        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_xyz = Simd32x3::from(-1.0 / (other[e4] * other[e4])) * other.group0().xyz();
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_anti_product_g0_w * -1.0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_anti_product_g0_xyz.zxy() * self.group0().yzx()) - (geometric_anti_product_g0_xyz.yzx() * self.group0().zxy()))
                .with_w(geometric_anti_product_g0_w * self[e321] * -1.0),
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
    //      f32        0        4        2        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        5        2      N/A
    //  no simd        0        8        2        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(1.0 / other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4] * -1.0 / (other[e1234] * other[e1234])),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd       18       33        0        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0 * -1.0) * other.group0();
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, geometric_anti_product_g1[1] * self[e1] * -1.0, 0.0])
                + (geometric_anti_product_g1.yzxy() * self.group0().zxyy())
                + (self.group0().wwwx() * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[0]))
                + (-(geometric_anti_product_g1.zx() * self.group0().yz()).with_z(0.0) - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       10       23        0        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other_g0 * self[e4]) * other.group1()) + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x4::from(self[e4]).xyz())
                .with_w(-(geometric_anti_product_g0[0] * self[e1]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3])),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       11       22        0      N/A
    //  no simd       15       40        0        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_anti_product_g0[2] * self[e2]) - (geometric_anti_product_g0[1] * self[e3]),
                (geometric_anti_product_g0[0] * self[e3]) - (geometric_anti_product_g0[2] * self[e1]),
                (geometric_anti_product_g0[1] * self[e1]) - (geometric_anti_product_g0[0] * self[e2]),
            ]) + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz()))
            .with_w(geometric_anti_product_g0[3] * self[e4]),
            // e423, e431, e412, e321
            (geometric_anti_product_g0.xyz() * Simd32x4::from(self[e4]).xyz()).with_w(
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
    //      f32       16       29        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        9        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       21       41        0      N/A
    //  no simd       31       66        0        0
    fn geometric_anti_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1234] * other[e1234]
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
            Simd32x3::from([
                (geometric_anti_product_g4[1] * self[e3]) - (geometric_anti_product_g4[2] * self[e2]),
                (geometric_anti_product_g4[2] * self[e1]) - (geometric_anti_product_g4[0] * self[e3]),
                (geometric_anti_product_g4[0] * self[e2]) - (geometric_anti_product_g4[1] * self[e1]),
            ]) + (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz()),
            // e423, e431, e412, e321
            (geometric_anti_product_g2 * Simd32x4::from(self[e4]).xyz()).with_w(
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
    //      f32        1       10        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        4       18        0      N/A
    //  no simd       11       36        0        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = (Simd32x4::from([other[e431] * other[e431], other[e423] * other[e423], other[e423] * other[e423], other[e431] * other[e431]])
            * other.group0())
            + (other.group0() * Simd32x2::from(other[e412] * other[e412]).with_zw(other[e431] * other[e431], other[e412] * other[e412]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0.xyz() * Simd32x4::from(self[e4]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_anti_product_g0.yz() * self.group0().zx()) - (geometric_anti_product_g0.zx() * self.group0().yz())).with_zw(
                (geometric_anti_product_g0[0] * self[e2]) - (geometric_anti_product_g0[1] * self[e1]),
                geometric_anti_product_g0[0] * self[e1],
            ),
        )
    }
}
impl GeometricAntiQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        2        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        7        2      N/A
    //  no simd        3       11        2        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0_w = -1.0 / other[e4];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0_w * self[e4] * -1.0),
            // e23, e31, e12, scalar
            (-(Simd32x3::from(geometric_anti_product_g0_w) * self.group0().xyz()) - (Simd32x3::from(self[e4] / (other[e4] * other[e4])) * other.group0().xyz())).with_w(0.0),
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
    // f32        0        1        1        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / other[e1234])
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
    //      f32        0       10        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       13        0      N/A
    //  no simd        6       19        0        3
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            -(Simd32x3::from([other[e42] * other[e42] * self[scalar], other[e41] * other[e41] * self[scalar], other[e41] * other[e41] * self[scalar]]) * other.group0())
                - (other.group0() * Simd32x2::from(other[e43] * other[e43] * self[scalar]).with_z(other[e42] * other[e42] * self[scalar]))
                - (Simd32x3::powi(other.group0(), 3) * Simd32x3::from(self[scalar])),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       14        0        3
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3       18        0      N/A
    //  no simd        9       26        0        3
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (-(Simd32x3::from(self[scalar]) * Simd32x3::from([f32::powi(other[e41], 3), f32::powi(other[e42], 3), f32::powi(other[e43], 3)]))
                - (Simd32x3::from(self[scalar])
                    * Simd32x3::from([other[e42] * other[e42] * other[e41], other[e41] * other[e41] * other[e42], other[e41] * other[e41] * other[e43]]))
                - (Simd32x3::from(self[scalar])
                    * Simd32x3::from([other[e43] * other[e43] * other[e41], other[e43] * other[e43] * other[e42], other[e42] * other[e42] * other[e43]]))
                - (Simd32x3::from(other[e1234] * other[e1234] * self[scalar]) * other.group0().xyz()))
            .with_w(0.0),
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
            + other[e41] * other[e41]
            + other[e42] * other[e42]
            + other[e43] * other[e43]
            + other[e4] * other[e4]
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
    //      f32        0       12        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        6       21        0        3
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * Simd32x3::from([f32::powi(other[e423], 3), f32::powi(other[e431], 3), f32::powi(other[e412], 3)]))
                + (Simd32x3::from(self[scalar])
                    * Simd32x3::from([
                        other[e431] * other[e431] * other[e423],
                        other[e423] * other[e423] * other[e431],
                        other[e423] * other[e423] * other[e412],
                    ]))
                + (Simd32x3::from(self[scalar])
                    * Simd32x3::from([
                        other[e412] * other[e412] * other[e423],
                        other[e412] * other[e412] * other[e431],
                        other[e431] * other[e431] * other[e412],
                    ])))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * -1.0 / other[e4])
    }
}
