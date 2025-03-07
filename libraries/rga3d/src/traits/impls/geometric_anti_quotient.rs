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
//   Median:         3       6       0
//  Average:         7      11       0
//  Maximum:        74      84       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3      18       0
//  Average:        15      29       0
//  Maximum:       195     218       1
impl std::ops::Div<GeometricAntiQuotientInfix> for AntiScalar {
    type Output = GeometricAntiQuotientInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricAntiQuotientInfix) -> Self::Output {
        GeometricAntiQuotientInfixPartial(self)
    }
}
impl GeometricAntiQuotient<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0())
    }
}
impl GeometricAntiQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       20        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group1(),
        )
    }
}
impl GeometricAntiQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       18        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       24        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       42        0
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
            Simd32x2::from(other_g0) * Simd32x2::from(self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group4(),
        )
    }
}
impl GeometricAntiQuotient<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] / (other[e4]) * -1.0)
    }
}
impl GeometricAntiQuotient<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        8        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0(),
        )
    }
}
impl GeometricAntiQuotient<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0),
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[1] * self[scalar]),
            geometric_anti_product_g0[1] * self[e1234],
        ]))
    }
}
impl GeometricAntiQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        7       25        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * geometric_anti_product_g1.xyz()) + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz()))
                .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e423, e431, e412, e321
            self.group0()
                .yy()
                .with_zw(self[e1234], (geometric_anti_product_g0[3] * self[scalar]) + (geometric_anti_product_g1[3] * self[e1234]))
                * geometric_anti_product_g1.xyz().with_w(1.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        5       18        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            geometric_anti_product_g0 * Simd32x3::from(self[e1234]),
            // e23, e31, e12
            (geometric_anti_product_g0 * Simd32x3::from(self[scalar])) - (Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group1()),
        )
    }
}
impl GeometricAntiQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        7        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        7       28        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(other_g0) * Simd32x4::from(self[e1234]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (geometric_anti_product_g0 * Simd32x4::from(self[scalar])),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        6        0
    //    simd2        0        1        0
    //    simd3        2        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       15       48        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
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
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar])) - (Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            self.group0()
                .yy()
                .with_zw(self[e1234], (geometric_anti_product_g1[3] * self[scalar]) + (geometric_anti_product_g4[3] * self[e1234]))
                * geometric_anti_product_g4.xyz().with_w(1.0),
        )
    }
}
impl GeometricAntiQuotient<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
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
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2       11        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0.xyz() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e423, e431, e412, e321
            geometric_anti_product_g0 * Simd32x4::from(self[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        4       15        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz()) - (Simd32x3::from(geometric_anti_product_g0[0]) * self.group1().xyz()))
                .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e423, e431, e412, e321
            geometric_anti_product_g0
                .yy()
                .with_zw(geometric_anti_product_g0[1], (geometric_anti_product_g0[1] * self[e321]) - (geometric_anti_product_g0[0] * self[e4]))
                * self.group1().xyz().with_w(1.0),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        9       13        0
    // Totals...
    // yes simd       16       21        0
    //  no simd       43       60        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g1[0] * self[e4]) - (geometric_anti_product_g1[2] * self[e431]),
                -(geometric_anti_product_g1[0] * self[e412]) - (geometric_anti_product_g1[1] * self[e4]),
                -(geometric_anti_product_g1[1] * self[e423]) - (geometric_anti_product_g1[2] * self[e4]),
                (geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g1.yzxx() * self.group1().zxyx())
                - (Simd32x4::from(geometric_anti_product_g0[3]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (Simd32x4::from(geometric_anti_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e1]]) * geometric_anti_product_g0.xxy().with_w(geometric_anti_product_g1[0]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e2]]) * geometric_anti_product_g0.zyz().with_w(geometric_anti_product_g1[1]))
                + (geometric_anti_product_g1.yzxz() * self.group0().zxyz())
                - (Simd32x4::from([self[e2], self[e321], self[e321], self[e321]]) * geometric_anti_product_g1.zyz().with_w(geometric_anti_product_g0[3]))
                - (Simd32x4::from([self[e321], self[e3], self[e1], self[e412]]) * geometric_anti_product_g1.xxy().with_w(geometric_anti_product_g0[2]))
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_anti_product_g0.wwwy() * self.group0().xyz().with_w(self[e431])),
        )
    }
}
impl GeometricAntiQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        0        9        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g0.xxy()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product_g1.xxy().with_w(geometric_anti_product_g0[1]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product_g1.zyz().with_w(geometric_anti_product_g0[2]))
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[0] * self[e423]),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product_g0.xxy().with_w(geometric_anti_product_g1[0]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product_g0.zyz().with_w(geometric_anti_product_g1[1]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e412]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3]))
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e1]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        6        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       17       22        0
    //  no simd       50       64        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0.wwyw() * self.group0().xyxw())
                + (Simd32x3::from([self[e2], self[e321], self[e3]]) * geometric_anti_product_g0.zyw()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e321]]) * geometric_anti_product_g0.xxz()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e423]))
                - (self.group1().xyxz() * geometric_anti_product_g1.wwy().with_w(geometric_anti_product_g0[2]))
                - (Simd32x3::from([self[e431], self[e412], self[e412]]) * geometric_anti_product_g1.zxw()).with_w(0.0)
                - (geometric_anti_product_g1.xyz() * self.group0().www()).with_w(geometric_anti_product_g0[1] * self[e431]),
            // e423, e431, e412, e321
            (self.group1().xyxy() * geometric_anti_product_g0.wwy().with_w(geometric_anti_product_g1[1]))
                + (self.group1().yzzz() * geometric_anti_product_g0.zxw().with_w(geometric_anti_product_g1[2]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g0[3] * self[e321])
                        - (geometric_anti_product_g0[1] * self[e2])
                        - (geometric_anti_product_g0[2] * self[e3])
                        - (geometric_anti_product_g1[3] * self[e4]),
                )
                + (geometric_anti_product_g0.xyz() * self.group0().www()).with_w(geometric_anti_product_g1[0] * self[e423])
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       10        0
    //    simd2        4        5        0
    //    simd3       10       22        0
    //    simd4       11        9        0
    // Totals...
    // yes simd       38       46        0
    //  no simd       95      122        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g4[3] * self[e4])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412])
                    - (geometric_anti_product_g1[3] * self[e321]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g4[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g4[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product_g1.xw()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group0())
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy()).with_w(0.0)
                + (geometric_anti_product_g3.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * geometric_anti_product_g3.xxy().with_w(geometric_anti_product_g2[2]))
                - (self.group1().xyzx() * geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g3.zyz()).with_w(0.0)
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g2[1] * self[e431]),
            // e41, e42, e43
            (geometric_anti_product_g4.yzx() * self.group1().zxy())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[1]]) * self.group1().xyx())
                - (Simd32x3::from([geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group1().yzz()),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                + (Simd32x3::from([geometric_anti_product_g1[2], geometric_anti_product_g1[0], geometric_anti_product_g4[3]]) * self.group1().yzz())
                + (Simd32x3::from([geometric_anti_product_g4[3], geometric_anti_product_g4[3], geometric_anti_product_g1[1]]) * self.group1().xyx())
                + (geometric_anti_product_g4.yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                - (Simd32x3::from([geometric_anti_product_g1[3], geometric_anti_product_g1[3], geometric_anti_product_g4[1]]) * self.group0().xyx())
                - (Simd32x3::from([geometric_anti_product_g4[2], geometric_anti_product_g4[0], geometric_anti_product_g1[3]]) * self.group0().yzz())
                - (geometric_anti_product_g1.yzx() * self.group1().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product_g2.xxy().with_w(geometric_anti_product_g3[0]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product_g2.zyz().with_w(geometric_anti_product_g3[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_anti_product_g3[2] * self[e412])
                        - (geometric_anti_product_g2[0] * self[e1])
                        - (geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3]),
                )
                - (geometric_anti_product_g2.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e4]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_anti_product_g0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       22       32        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g0[0] * self[e4]) - (geometric_anti_product_g0[2] * self[e431]),
                -(geometric_anti_product_g0[0] * self[e412]) - (geometric_anti_product_g0[1] * self[e4]),
                -(geometric_anti_product_g0[1] * self[e423]) - (geometric_anti_product_g0[2] * self[e4]),
                (geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412]),
            ]) + (geometric_anti_product_g0.yzxx() * self.group1().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_anti_product_g0[0] * self[e321]) - (geometric_anti_product_g0[2] * self[e2]),
                -(geometric_anti_product_g0[0] * self[e3]) - (geometric_anti_product_g0[1] * self[e321]),
                -(geometric_anti_product_g0[1] * self[e1]) - (geometric_anti_product_g0[2] * self[e321]),
                (geometric_anti_product_g0[2] * self[e3]) + (geometric_anti_product_g0[3] * self[e4]),
            ]) + (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + (geometric_anti_product_g0.wwwy() * self.group1().xyz().with_w(self[e2])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       32        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_anti_product_g0[3]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[e4]) + (geometric_anti_product_g0[2] * self[e431]),
                (geometric_anti_product_g0[0] * self[e412]) + (geometric_anti_product_g0[1] * self[e4]),
                (geometric_anti_product_g0[1] * self[e423]) + (geometric_anti_product_g0[2] * self[e4]),
                -(geometric_anti_product_g0[2] * self[e412]) - (geometric_anti_product_g0[3] * self[e321]),
            ]) - (geometric_anti_product_g0.yzxy() * self.group1().zxyy())
                - (geometric_anti_product_g0.wwwx() * self.group0().xyz().with_w(self[e423])),
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       14        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * (Simd32x4::from(other_g0) * other.group1()).xyz().with_w(other_g0 * other[e4] * -1.0) * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * Simd32x3::from(other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43]) * other.group0() * Simd32x3::from(-1.0))
                .with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
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
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       11        0
    //  no simd        7       25        0
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
            Simd32x2::from([other_g0 * self[e321] * other[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e321]) * other.group2() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * (Simd32x4::from(other_g0) * other.group4()).xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * self[e321] * other[e1234]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / (other[e4]))
    }
}
impl GeometricAntiQuotient<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321])
                * Simd32x3::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412])
                * other.group0().xyz()
                * Simd32x3::from(-1.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / (other[e4]))
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
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
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       11        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        3        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       31       49        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group1())
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0())
                + (self.group0().yzx() * geometric_anti_product_g0.zxy())
                + (self.group1().yzx() * geometric_anti_product_g1.zxy()))
            .with_w(geometric_anti_product_g1[2] * self[e43] * -1.0)
                - (geometric_anti_product_g1.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * geometric_anti_product_g0.yzx()).with_w(geometric_anti_product_g1[0] * self[e41]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_anti_product_g0[3] * self[e41]) + (geometric_anti_product_g1[2] * self[e42]),
                (geometric_anti_product_g0[3] * self[e42]) + (geometric_anti_product_g1[0] * self[e43]),
                (geometric_anti_product_g0[3] * self[e43]) + (geometric_anti_product_g1[1] * self[e41]),
                -(geometric_anti_product_g0[1] * self[e42])
                    - (geometric_anti_product_g0[2] * self[e43])
                    - (geometric_anti_product_g1[0] * self[e23])
                    - (geometric_anti_product_g1[1] * self[e31])
                    - (geometric_anti_product_g1[2] * self[e12]),
            ]) - (self.group0().zxy() * geometric_anti_product_g1.yzx()).with_w(geometric_anti_product_g0[0] * self[e41]),
        )
    }
}
impl GeometricAntiQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        1        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       21       39        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_anti_product_g0.zxy() * self.group0().yzx()) - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
                .with_w(-(geometric_anti_product_g0[0] * self[e41]) - (geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e31]) + (geometric_anti_product_g1[2] * self[e42]),
                (geometric_anti_product_g0[0] * self[e12]) + (geometric_anti_product_g1[0] * self[e43]),
                (geometric_anti_product_g0[1] * self[e23]) + (geometric_anti_product_g1[1] * self[e41]),
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e23])
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       31       52        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e42]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[0] * self[e43]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[1] * self[e41]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e31])
                    + (geometric_anti_product_g0[3] * self[e23])
                    + (geometric_anti_product_g1[2] * self[e42])
                    + (geometric_anti_product_g1[3] * self[e41]),
                (geometric_anti_product_g0[0] * self[e12])
                    + (geometric_anti_product_g0[3] * self[e31])
                    + (geometric_anti_product_g1[0] * self[e43])
                    + (geometric_anti_product_g1[3] * self[e42]),
                (geometric_anti_product_g0[1] * self[e23])
                    + (geometric_anti_product_g0[3] * self[e12])
                    + (geometric_anti_product_g1[1] * self[e41])
                    + (geometric_anti_product_g1[3] * self[e43]),
                -(geometric_anti_product_g0[0] * self[e23])
                    - (geometric_anti_product_g0[1] * self[e31])
                    - (geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) - (geometric_anti_product_g1.yzxx() * self.group0().zxy().with_w(self[e41]))
                - (self.group1().zxy() * geometric_anti_product_g0.yzx()).with_w(geometric_anti_product_g1[1] * self[e42]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       18        0
    //    simd2        3        4        0
    //    simd3       10       19        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       64       99        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g2[0] * self[e23]) - (geometric_anti_product_g2[1] * self[e31]) - (geometric_anti_product_g2[2] * self[e12]),
                0.0,
            ]) - (Simd32x2::from(self[e41]) * Simd32x2::from([geometric_anti_product_g3[0], geometric_anti_product_g2[0]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([geometric_anti_product_g3[1], geometric_anti_product_g2[1]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([geometric_anti_product_g3[2], geometric_anti_product_g2[2]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g1[3]) * self.group1())
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0())
                + (self.group0().yzx() * geometric_anti_product_g1.zxy())
                + (self.group1().yzx() * geometric_anti_product_g4.zxy()))
            .with_w(geometric_anti_product_g4[2] * self[e43] * -1.0)
                - (geometric_anti_product_g4.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * geometric_anti_product_g1.yzx()).with_w(geometric_anti_product_g4[0] * self[e41]),
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
            Simd32x4::from([
                (geometric_anti_product_g1[3] * self[e41]) + (geometric_anti_product_g4[2] * self[e42]),
                (geometric_anti_product_g1[3] * self[e42]) + (geometric_anti_product_g4[0] * self[e43]),
                (geometric_anti_product_g1[3] * self[e43]) + (geometric_anti_product_g4[1] * self[e41]),
                -(geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43])
                    - (geometric_anti_product_g4[0] * self[e23])
                    - (geometric_anti_product_g4[1] * self[e31])
                    - (geometric_anti_product_g4[2] * self[e12]),
            ]) - (self.group0().zxy() * geometric_anti_product_g4.yzx()).with_w(geometric_anti_product_g1[0] * self[e41]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        7        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
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
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       25        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e31]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[0] * self[e12]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[1] * self[e23]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            (self.group0().yzx() * geometric_anti_product_g0.zxy()).with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12]))
                - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd       10       23        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
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
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4       14        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32        7        7        0
    //    simd3        4        5        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       17       22        0
    //  no simd       43       62        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0.xxyw() * self.group0().wzxw())
                + ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                    + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * geometric_anti_product_g0.zyz())
                    + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * geometric_anti_product_g1.zyz())
                    + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * geometric_anti_product_g1.xxy()))
                .with_w(geometric_anti_product_g1[2] * self[e43] * -1.0)
                - (geometric_anti_product_g1.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxyx() * geometric_anti_product_g0.yzx().with_w(geometric_anti_product_g1[0])),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[3]) * self.group0().xyz().with_w(self[scalar]))
                + (Simd32x4::from([self[e42], self[e1234], self[e1234], 1.0])
                    * geometric_anti_product_g1.zyz().with_w(
                        -(geometric_anti_product_g0[1] * self[e42])
                            - (geometric_anti_product_g0[2] * self[e43])
                            - (geometric_anti_product_g1[0] * self[e23])
                            - (geometric_anti_product_g1[1] * self[e31])
                            - (geometric_anti_product_g1[2] * self[e12]),
                    ))
                + (geometric_anti_product_g1.xxyw() * self.group0().wzxw())
                - (self.group0().zxyx() * geometric_anti_product_g1.yzx().with_w(geometric_anti_product_g0[0])),
        )
    }
}
impl GeometricAntiQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       30       48        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[e1234]) + (geometric_anti_product_g0[2] * self[e42]),
                (geometric_anti_product_g0[0] * self[e43]) + (geometric_anti_product_g0[1] * self[e1234]),
                (geometric_anti_product_g0[1] * self[e41]) + (geometric_anti_product_g0[2] * self[e1234]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (self.group0().zxyx() * geometric_anti_product_g0.yzx().with_w(geometric_anti_product_g0[0])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[scalar])
                    + (geometric_anti_product_g0[2] * self[e31])
                    + (geometric_anti_product_g1[0] * self[e1234])
                    + (geometric_anti_product_g1[2] * self[e42]),
                (geometric_anti_product_g0[0] * self[e12])
                    + (geometric_anti_product_g0[1] * self[scalar])
                    + (geometric_anti_product_g1[0] * self[e43])
                    + (geometric_anti_product_g1[1] * self[e1234]),
                (geometric_anti_product_g0[1] * self[e23])
                    + (geometric_anti_product_g0[2] * self[scalar])
                    + (geometric_anti_product_g1[1] * self[e41])
                    + (geometric_anti_product_g1[2] * self[e1234]),
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) - (self.group1().zxyx() * geometric_anti_product_g0.yzx().with_w(geometric_anti_product_g0[0]))
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        0
    //    simd3        0        2        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       43       64        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e42]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[1] * self[e1234]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[2] * self[e1234]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) + (geometric_anti_product_g0.xxyw() * self.group0().wzxw())
                - (geometric_anti_product_g0.yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[3] * self[e23])
                    + (geometric_anti_product_g1[0] * self[e1234])
                    + (geometric_anti_product_g1[2] * self[e42])
                    + (geometric_anti_product_g1[3] * self[e41]),
                (geometric_anti_product_g0[3] * self[e31])
                    + (geometric_anti_product_g1[0] * self[e43])
                    + (geometric_anti_product_g1[1] * self[e1234])
                    + (geometric_anti_product_g1[3] * self[e42]),
                (geometric_anti_product_g0[3] * self[e12])
                    + (geometric_anti_product_g1[1] * self[e41])
                    + (geometric_anti_product_g1[2] * self[e1234])
                    + (geometric_anti_product_g1[3] * self[e43]),
                -(geometric_anti_product_g0[2] * self[e12])
                    - (geometric_anti_product_g1[0] * self[e41])
                    - (geometric_anti_product_g1[1] * self[e42])
                    - (geometric_anti_product_g1[2] * self[e43]),
            ]) + (geometric_anti_product_g0.xxyw() * self.group1().wzxw())
                + (geometric_anti_product_g0.zyz() * self.group1().yww()).with_w(geometric_anti_product_g1[3] * self[e1234])
                - (geometric_anti_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()).with_w(geometric_anti_product_g0[1] * self[e31]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       14        0
    //    simd2        4        5        0
    //    simd3       14       20        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       38       49        0
    //  no simd       88      124        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
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
            (self.group0().xyxw() * geometric_anti_product_g4.ww().with_zw(geometric_anti_product_g1[1], geometric_anti_product_g1[3]))
                + (Simd32x3::from([
                    geometric_anti_product_g1[3] * self[e23],
                    geometric_anti_product_g1[3] * self[e31],
                    geometric_anti_product_g4[3] * self[e43],
                ]) + (Simd32x3::from([self[e42], self[e1234], self[e12]]) * geometric_anti_product_g1.zyw())
                    + (Simd32x3::from([self[e1234], self[e43], self[e1234]]) * geometric_anti_product_g1.xxz())
                    + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * geometric_anti_product_g4.zyz())
                    + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * geometric_anti_product_g4.xxy()))
                .with_w(geometric_anti_product_g4[2] * self[e43] * -1.0)
                - (geometric_anti_product_g4.yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxyx() * geometric_anti_product_g1.yzx().with_w(geometric_anti_product_g4[0])),
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
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * geometric_anti_product_g4.xyz().with_w(
                    -(geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g4[0] * self[e23])
                        - (geometric_anti_product_g4[1] * self[e31])
                        - (geometric_anti_product_g4[2] * self[e12]),
                ))
                + (self.group0().xyxw() * geometric_anti_product_g1.ww().with_zw(geometric_anti_product_g4[1], geometric_anti_product_g4[3]))
                + (geometric_anti_product_g4.zx().with_zw(geometric_anti_product_g1[3], geometric_anti_product_g1[3]) * self.group0().yzz().with_w(self[scalar]))
                - (self.group0().zxyx() * geometric_anti_product_g4.yzx().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        9        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
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
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       13       18        0
    //  no simd       22       33        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[scalar]) + (geometric_anti_product_g0[2] * self[e31]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[0] * self[e12]) + (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[1] * self[e23]) + (geometric_anti_product_g0[2] * self[scalar]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * geometric_anti_product_g0
                    .xyz()
                    .with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12])))
                + (geometric_anti_product_g0.zxyw() * self.group0().yzxw())
                - (geometric_anti_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       28        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
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
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd2        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        8       27        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
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
            geometric_anti_product_g0
                .yy()
                .with_zw(geometric_anti_product_g0[1], (geometric_anti_product_g0[1] * self[e321]) - (geometric_anti_product_g0[0] * self[e4]))
                * self.group4().xyz().with_w(1.0),
        )
    }
}
impl GeometricAntiQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd2        4        4        0
    //    simd3       14       19        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       34       44        0
    //  no simd       84      110        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e4])
                    - (geometric_anti_product_g0[1] * self[e431])
                    - (geometric_anti_product_g0[2] * self[e412])
                    - (geometric_anti_product_g0[3] * self[e321]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g1[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g1[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g1[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product_g0.xw()),
            // e1, e2, e3, e4
            (self.group0().xx().with_zw(self[scalar], self[e1234]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]))
                + ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group3())
                    + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group2())
                    + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * geometric_anti_product_g0.xxy())
                    + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * geometric_anti_product_g0.zyz())
                    + (self.group3().yzx() * geometric_anti_product_g1.zxy()))
                .with_w(geometric_anti_product_g1[2] * self[e43] * -1.0)
                - (geometric_anti_product_g1.yzxy() * self.group3().zxy().with_w(self[e42]))
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()).with_w(geometric_anti_product_g1[0] * self[e41]),
            // e41, e42, e43
            (geometric_anti_product_g1.yzx() * self.group4().zxy())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g1.xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g1.zyz()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g0.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g0.zyz())
                + (geometric_anti_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g1.zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g1.xxy())
                - (geometric_anti_product_g0.yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e42], self[e43], self[e43], 1.0])
                * geometric_anti_product_g1.zx().with_zw(
                    geometric_anti_product_g0[3],
                    -(geometric_anti_product_g0[1] * self[e42])
                        - (geometric_anti_product_g0[2] * self[e43])
                        - (geometric_anti_product_g1[0] * self[e23])
                        - (geometric_anti_product_g1[1] * self[e31])
                        - (geometric_anti_product_g1[2] * self[e12]),
                ))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]))
                + (geometric_anti_product_g0.ww().with_zw(geometric_anti_product_g1[1], geometric_anti_product_g1[3]) * self.group2().xyx().with_w(self[e1234]))
                - (self.group2().zxy() * geometric_anti_product_g1.yzx()).with_w(geometric_anti_product_g0[0] * self[e41]),
        )
    }
}
impl GeometricAntiQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd2        3        3        0
    //    simd3        7       18        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       24       33        0
    //  no simd       65       84        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
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
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product_g1.xxy().with_w(geometric_anti_product_g0[1]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product_g1.zyz().with_w(geometric_anti_product_g0[2]))
                - (geometric_anti_product_g0.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g0[0] * self[e423]),
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
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * geometric_anti_product_g0.xxy().with_w(geometric_anti_product_g1[0]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * geometric_anti_product_g0.zyz().with_w(geometric_anti_product_g1[1]))
                + Simd32x3::from(0.0).with_w((geometric_anti_product_g1[2] * self[e412]) - (geometric_anti_product_g0[1] * self[e2]) - (geometric_anti_product_g0[2] * self[e3]))
                - (geometric_anti_product_g0.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g0[0] * self[e1]),
        )
    }
}
impl GeometricAntiQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        8        0
    //    simd2        4        4        0
    //    simd3       10       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       34       40        0
    //  no simd       91      112        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g1[3] * self[e1234])
                    - (geometric_anti_product_g0[0] * self[e23])
                    - (geometric_anti_product_g0[1] * self[e31])
                    - (geometric_anti_product_g0[2] * self[e12]),
                0.0,
            ]) + (Simd32x2::from(geometric_anti_product_g0[3]) * self.group0())
                - (Simd32x2::from(self[e41]) * Simd32x2::from([geometric_anti_product_g1[0], geometric_anti_product_g0[0]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([geometric_anti_product_g1[1], geometric_anti_product_g0[1]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([geometric_anti_product_g1[2], geometric_anti_product_g0[2]])),
            // e1, e2, e3, e4
            (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * geometric_anti_product_g0.xxyw())
                + (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g0.zyz()).with_w(0.0)
                + (geometric_anti_product_g1.yzx() * self.group4().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * geometric_anti_product_g1.xxy().with_w(geometric_anti_product_g0[1]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * geometric_anti_product_g1.zyz().with_w(geometric_anti_product_g0[2]))
                - (geometric_anti_product_g0.yzxx() * self.group1().zxy().with_w(self[e423]))
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz()).with_w(0.0),
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
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * geometric_anti_product_g0.zyz().with_w(geometric_anti_product_g1[0]))
                + (self.group4().xyzy() * geometric_anti_product_g0.www().with_w(geometric_anti_product_g1[1]))
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
    //           add/sub      mul      div
    //      f32       21       20        0
    //    simd2        8        9        0
    //    simd3       22       40        0
    //    simd4       23       15        0
    // Totals...
    // yes simd       74       84        0
    //  no simd      195      218        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g4[3] * self[e4])
                    - (geometric_anti_product_g3[0] * self[e41])
                    - (geometric_anti_product_g3[1] * self[e42])
                    - (geometric_anti_product_g3[2] * self[e43])
                    - (geometric_anti_product_g1[1] * self[e431])
                    - (geometric_anti_product_g1[2] * self[e412])
                    - (geometric_anti_product_g1[3] * self[e321]),
                0.0,
            ]) + (geometric_anti_product_g0 * Simd32x2::from(self[e1234]))
                + (Simd32x2::from(geometric_anti_product_g4[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g4[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g4[2]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(geometric_anti_product_g2[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_anti_product_g2[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_anti_product_g2[2]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([self[e423], self[e4]]) * geometric_anti_product_g1.xw()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group1())
                + (self.group0().xx().with_zw(self[scalar], self[e1234]) * geometric_anti_product_g4.xyz().with_w(geometric_anti_product_g1[3]))
                + (Simd32x3::from(geometric_anti_product_g1[3]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[e1234], self[e43], self[e41]]) * geometric_anti_product_g1.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g2.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e42], self[e1234], self[e1234]]) * geometric_anti_product_g1.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g2.xxy()).with_w(0.0)
                + (geometric_anti_product_g3.yzx() * self.group4().zxy()).with_w(0.0)
                + (self.group3().yzx() * geometric_anti_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * geometric_anti_product_g3.xxy().with_w(geometric_anti_product_g2[2]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], geometric_anti_product_g4[0]]) * geometric_anti_product_g3.zyz().with_w(self[e41]))
                - (geometric_anti_product_g4.yzxz() * self.group3().zxy().with_w(self[e43]))
                - (self.group4().xyzx() * geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (geometric_anti_product_g2.yzx() * self.group1().zxy()).with_w(geometric_anti_product_g2[1] * self[e431])
                - (self.group2().zxy() * geometric_anti_product_g1.yzx()).with_w(geometric_anti_product_g4[1] * self[e42]),
            // e41, e42, e43
            (geometric_anti_product_g2 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group2())
                + (geometric_anti_product_g2.zxy() * self.group2().yzx())
                + (geometric_anti_product_g4.yzx() * self.group4().zxy())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g4.xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g4.zyz())
                - (geometric_anti_product_g2.yzx() * self.group2().zxy()),
            // e23, e31, e12
            (geometric_anti_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_anti_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_anti_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_anti_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_anti_product_g4[3]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * geometric_anti_product_g1.xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * geometric_anti_product_g1.zyz())
                + (geometric_anti_product_g2.zxy() * self.group3().yzx())
                + (geometric_anti_product_g3.zxy() * self.group2().yzx())
                + (geometric_anti_product_g4.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * geometric_anti_product_g4.zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * geometric_anti_product_g4.xxy())
                - (geometric_anti_product_g2.yzx() * self.group3().zxy())
                - (geometric_anti_product_g3.yzx() * self.group2().zxy())
                - (geometric_anti_product_g1.yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group4())
                + (Simd32x4::from([self[e4], self[e412], self[e423], geometric_anti_product_g4[3]]) * geometric_anti_product_g2.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * geometric_anti_product_g2.zyz().with_w(geometric_anti_product_g3[0]))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * geometric_anti_product_g4.xyz().with_w(geometric_anti_product_g1[3]))
                + (geometric_anti_product_g1.ww().with_zw(geometric_anti_product_g4[1], self[e431]) * self.group2().xyx().with_w(geometric_anti_product_g3[1]))
                + (geometric_anti_product_g4.zx().with_zw(geometric_anti_product_g1[3], self[e412]) * self.group2().yzz().with_w(geometric_anti_product_g3[2]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_anti_product_g2[1] * self[e2])
                        - (geometric_anti_product_g2[2] * self[e3])
                        - (geometric_anti_product_g1[0] * self[e41])
                        - (geometric_anti_product_g1[1] * self[e42])
                        - (geometric_anti_product_g1[2] * self[e43])
                        - (geometric_anti_product_g4[0] * self[e23])
                        - (geometric_anti_product_g4[1] * self[e31])
                        - (geometric_anti_product_g4[2] * self[e12]),
                )
                - (geometric_anti_product_g2.yzx() * self.group4().zxy()).with_w(geometric_anti_product_g0[0] * self[e4])
                - (self.group2().zxy() * geometric_anti_product_g4.yzx()).with_w(geometric_anti_product_g2[0] * self[e1]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        1
    //  no simd        0       25        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_anti_product_g0) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0) * self.group3().with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(geometric_anti_product_g0) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0) * self.group2().with_w(self[scalar]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd2        3        4        0
    //    simd3        5        8        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       43       62        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([geometric_anti_product_g0[3] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]))
                + (Simd32x2::from(geometric_anti_product_g0[0]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(geometric_anti_product_g0[1]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(geometric_anti_product_g0[2]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[scalar]) + (geometric_anti_product_g0[2] * self[e31]) + (geometric_anti_product_g0[3] * self[e41]),
                (geometric_anti_product_g0[0] * self[e12]) + (geometric_anti_product_g0[1] * self[scalar]) + (geometric_anti_product_g0[3] * self[e42]),
                (geometric_anti_product_g0[1] * self[e23]) + (geometric_anti_product_g0[2] * self[scalar]) + (geometric_anti_product_g0[3] * self[e43]),
                -(geometric_anti_product_g0[1] * self[e42]) - (geometric_anti_product_g0[2] * self[e43]),
            ]) - (geometric_anti_product_g0.yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (geometric_anti_product_g0.yzx() * self.group4().zxy())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g0.zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz()) + (geometric_anti_product_g0.yzx() * self.group1().zxy())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())
                - (geometric_anti_product_g0.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (geometric_anti_product_g0 * Simd32x4::from(self[e1234]))
                + (self.group2().yzx() * geometric_anti_product_g0.zxy()).with_w(-(geometric_anti_product_g0[1] * self[e31]) - (geometric_anti_product_g0[2] * self[e12]))
                - (geometric_anti_product_g0.yzxx() * self.group2().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd2        0        1        0
    //    simd3        6       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       53        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(geometric_anti_product_g0[0] * self[e423])
                    - (geometric_anti_product_g0[1] * self[e431])
                    - (geometric_anti_product_g0[2] * self[e412])
                    - (geometric_anti_product_g0[3] * self[e321]),
                geometric_anti_product_g0[3] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group3())
                + (Simd32x3::from(self[e1234]) * geometric_anti_product_g0.xyz())
                + (self.group2().yzx() * geometric_anti_product_g0.zxy())
                - (self.group2().zxy() * geometric_anti_product_g0.yzx()))
            .with_w(geometric_anti_product_g0[3] * self[e1234]),
            // e41, e42, e43
            Simd32x3::from(geometric_anti_product_g0[3]) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz()) + (geometric_anti_product_g0.zxy() * self.group4().yzx())
                - (Simd32x3::from(geometric_anti_product_g0[3]) * self.group1().xyz())
                - (geometric_anti_product_g0.yzx() * self.group4().zxy()),
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<DualNum> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[1] * self[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[0] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       24        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]),
        )
    }
}
impl GeometricAntiQuotient<Line> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       15        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e4]) * other.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e4]) * other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       32        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[3]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd2        0        3        0
    //    simd3        0        7        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        7       17        0
    //  no simd        7       55        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([geometric_anti_product_g4[3], geometric_anti_product_g1[3]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4])
                * (Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0)).with_w(geometric_anti_product_g0[1])
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * geometric_anti_product_g4.xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4])
                * (Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0)).with_w(geometric_anti_product_g0[0])
                * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] / (other[e4]))
    }
}
impl GeometricAntiQuotient<Plane> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       11        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4]) * geometric_anti_product_g0.xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4]),
        )
    }
}
impl GeometricAntiQuotient<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyz() * geometric_anti_product_g0.xx().with_z(geometric_anti_product_g0[0]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_anti_product_g0[1]) * self.group0(),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       23       40        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_anti_product_g0[3] * self[e423]) - (geometric_anti_product_g1[2] * self[e431]),
                -(geometric_anti_product_g0[3] * self[e431]) - (geometric_anti_product_g1[0] * self[e412]),
                -(geometric_anti_product_g0[3] * self[e412]) - (geometric_anti_product_g1[1] * self[e423]),
                (geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412]),
            ]) + (geometric_anti_product_g1.yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_anti_product_g0[2] * self[e431]) + (geometric_anti_product_g1[3] * self[e423]),
                (geometric_anti_product_g0[0] * self[e412]) + (geometric_anti_product_g1[3] * self[e431]),
                (geometric_anti_product_g0[1] * self[e423]) + (geometric_anti_product_g1[3] * self[e412]),
                -(geometric_anti_product_g0[2] * self[e412]) - (geometric_anti_product_g0[3] * self[e321]),
            ]) - (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                - (self.group0().wwwy() * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[1])),
        )
    }
}
impl GeometricAntiQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        0        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       36        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_anti_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_anti_product_g0[0] * self[e321]) + (geometric_anti_product_g1[1] * self[e412]),
                (geometric_anti_product_g0[1] * self[e321]) + (geometric_anti_product_g1[2] * self[e423]),
                (geometric_anti_product_g0[2] * self[e321]) + (geometric_anti_product_g1[0] * self[e431]),
                -(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            (self.group0().yzxx() * geometric_anti_product_g0.zxy().with_w(geometric_anti_product_g1[0]))
                + (geometric_anti_product_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412])),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd3        1        4        0
    //    simd4        4        8        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       23       48        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz()) + (geometric_anti_product_g1.yzx() * self.group0().zxy()))
                .with_w(geometric_anti_product_g0[2] * self[e412] * -1.0)
                - (self.group0().xyzy() * geometric_anti_product_g1.www().with_w(geometric_anti_product_g0[1]))
                - (self.group0().yzxx() * geometric_anti_product_g1.zxy().with_w(geometric_anti_product_g0[0])),
            // e423, e431, e412, e321
            (geometric_anti_product_g0.zxyw() * self.group0().yzxw())
                + (self.group0().xyzx() * geometric_anti_product_g0.www().with_w(geometric_anti_product_g1[0]))
                + (geometric_anti_product_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((geometric_anti_product_g1[1] * self[e431]) + (geometric_anti_product_g1[2] * self[e412])),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       11        0
    //    simd2        0        1        0
    //    simd3        6       15        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       86        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
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
            ((geometric_anti_product_g2 * Simd32x3::from(self[e321])) + (geometric_anti_product_g3.yzx() * self.group0().zxy()))
                .with_w(geometric_anti_product_g2[2] * self[e412] * -1.0)
                - (self.group0().xyzx() * geometric_anti_product_g0.xx().with_zw(geometric_anti_product_g0[0], geometric_anti_product_g2[0]))
                - (self.group0().yzxy() * geometric_anti_product_g3.zxy().with_w(geometric_anti_product_g2[1])),
            // e41, e42, e43
            (geometric_anti_product_g4.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_anti_product_g4[3]) * self.group0().xyz()) + (geometric_anti_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_anti_product_g4.xyz())
                - (geometric_anti_product_g1.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_anti_product_g0[1]) * self.group0())
                + (self.group0().yzxx() * geometric_anti_product_g2.zxy().with_w(geometric_anti_product_g3[0]))
                + (geometric_anti_product_g2.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((geometric_anti_product_g3[1] * self[e431]) + (geometric_anti_product_g3[2] * self[e412])),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_anti_product_g0) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        1        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd       10       22        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx() * Simd32x3::from(-1.0))
                    .with_w((geometric_anti_product_g0[1] * self[e431]) + (geometric_anti_product_g0[2] * self[e412])),
            // e23, e31, e12, scalar
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_anti_product_g0.xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        6       24        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().xyz() * geometric_anti_product_g0.www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0.zxy() * self.group0().yzx())
                .with_w(-(geometric_anti_product_g0[1] * self[e431]) - (geometric_anti_product_g0[2] * self[e412]) - (geometric_anti_product_g0[3] * self[e321]))
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / other[e1234]) * self.group0())
    }
}
impl GeometricAntiQuotient<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x2::from(f32::powi(other[e1234], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_anti_product_g0[1]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[0] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       36        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from(-1.0);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * geometric_anti_product_g1.xyz().with_w(geometric_anti_product_g0[3]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_anti_product_g0[3] * self[e1]) - (geometric_anti_product_g1[2] * self[e2]),
                -(geometric_anti_product_g0[3] * self[e2]) - (geometric_anti_product_g1[0] * self[e3]),
                -(geometric_anti_product_g0[3] * self[e3]) - (geometric_anti_product_g1[1] * self[e1]),
                (geometric_anti_product_g1[2] * self[e3]) + (geometric_anti_product_g1[3] * self[e4]),
            ]) + (geometric_anti_product_g1.yzxy() * self.group0().zxyy())
                + (self.group0().wwwx() * geometric_anti_product_g0.xyz().with_w(geometric_anti_product_g1[0])),
        )
    }
}
impl GeometricAntiQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       27        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43];
        let geometric_anti_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_anti_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(other_g0) * Simd32x3::from(self[e4]) * other.group1() * Simd32x3::from(-1.0)).with_w(0.0)
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
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd3        3        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       36        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234];
        let geometric_anti_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[3]) * self.group0().xyz()) + (geometric_anti_product_g0.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz())
                - (geometric_anti_product_g0.yzx() * self.group0().zxy()))
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
    //           add/sub      mul      div
    //      f32       13       10        0
    //    simd2        0        2        0
    //    simd3        6       14        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       31       68        0
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
        let geometric_anti_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        let geometric_anti_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_anti_product_g4 = Simd32x4::from(other_g0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_anti_product_g4[0] * self[e1])
                    + (geometric_anti_product_g4[1] * self[e2])
                    + (geometric_anti_product_g4[2] * self[e3])
                    + (geometric_anti_product_g4[3] * self[e4]),
                geometric_anti_product_g1[3] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(geometric_anti_product_g0[1]) * self.group0().xyz())
                + (geometric_anti_product_g2.zxy() * self.group0().yzx())
                + (Simd32x3::from(other_g0) * Simd32x3::from(self[e4]) * other.group3())
                - (geometric_anti_product_g2.yzx() * self.group0().zxy()))
            .with_w(geometric_anti_product_g0[1] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * geometric_anti_product_g4.xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * geometric_anti_product_g1.xyz()) + (geometric_anti_product_g4.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_anti_product_g1[3]) * self.group0().xyz())
                - (geometric_anti_product_g4.zxy() * self.group0().yzx()),
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
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       12        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = 1.0 / other[e4] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0 * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_anti_product_g0) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       23        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_anti_product_g0.xyz() * self.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_anti_product_g0.yzxx() * self.group0().zxyx())
                + (geometric_anti_product_g0.zxy() * self.group0().yzx() * Simd32x3::from(-1.0))
                    .with_w((geometric_anti_product_g0[1] * self[e2]) + (geometric_anti_product_g0[2] * self[e3]) + (geometric_anti_product_g0[3] * self[e4])),
        )
    }
}
impl GeometricAntiQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product_g0 = Simd32x4::from(f32::powi(other[e4], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_anti_product_g0[3] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_anti_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / (other[e1234]))
    }
}
impl GeometricAntiQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       10        0
    fn geometric_anti_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e4] * other[e4] + other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * (Simd32x4::from(other_g0) * other.group1()).xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * other[e4] * self[scalar] * -1.0),
        )
    }
}
impl GeometricAntiQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_anti_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * Simd32x3::from(other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43]) * other.group0() * Simd32x3::from(-1.0),
        )
    }
}
impl GeometricAntiQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn geometric_anti_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar])
                * Simd32x4::from(other[e41] * other[e41] + other[e42] * other[e42] + other[e43] * other[e43] + other[e1234] * other[e1234])
                * other.group0()
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricAntiQuotient<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        4        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       11        0
    //  no simd        7       26        0
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
            Simd32x2::from([other_g0 * other[e1234] * self[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * (Simd32x4::from(other_g0) * other.group4()).xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group2() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other_g0 * other[e4] * self[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricAntiQuotient<Origin> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] / (other[e4]) * -1.0)
    }
}
impl GeometricAntiQuotient<Plane> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn geometric_anti_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * Simd32x3::from(other[e423] * other[e423] + other[e431] * other[e431] + other[e412] * other[e412]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiQuotient<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_anti_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] / (other[e4]) * -1.0)
    }
}
