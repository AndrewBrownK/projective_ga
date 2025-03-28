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
//   Median:         4      22       0       0
//  Average:        18      30       0       0
//  Maximum:       205     219       2       3
impl std::ops::Div<GeometricQuotientInfix> for AntiScalar {
    type Output = GeometricQuotientInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] / other[scalar])
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       11        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * other[e321] * self[e1234]),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] / other[e321])
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2        9        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(-(other[e23] * other[e23] * self[e1234]) - (other[e31] * other[e31] * self[e1234]) - (other[e12] * other[e12] * self[e1234])) * other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       16        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(
                (other[e23] * other[e23] * self[e1234])
                    + (other[e31] * other[e31] * self[e1234])
                    + (other[e12] * other[e12] * self[e1234])
                    + (other[scalar] * other[scalar] * self[e1234]),
            ) * other.group1()
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl GeometricQuotient<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       22        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other_g0 * other[scalar] * self[e1234]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * other[e321] * self[e1234]),
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] / other[e321])
    }
}
impl GeometricQuotient<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        3
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        8       17        0        3
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            -(Simd32x3::powi(other.group0().xyz(), 3) * Simd32x3::from(self[e1234])).with_w(0.0)
                - (Simd32x3::from(other[e1] * self[e1234]) * other.group0().yxx() * other.group0().yyz()).with_w(0.0)
                - (Simd32x3::from(other[e3] * self[e1234]) * other.group0().xyy() * other.group0().zzy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] / other[scalar])
    }
}
impl std::ops::Div<GeometricQuotientInfix> for DualNum {
    type Output = GeometricQuotientInfixPartial<DualNum>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        1        6        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product_g0[0] * self[scalar],
            (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
        ]))
    }
}
impl GeometricQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd        7       25        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * geometric_product_g0.xyz()).with_w((self[scalar] * geometric_product_g0[3]) - (self[e1234] * geometric_product_g1[3])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz()) - (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz()))
                .with_w(self[scalar] * geometric_product_g1[3]),
        )
    }
}
impl GeometricQuotient<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        4        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e1234] * -1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[scalar]),
        )
    }
}
impl GeometricQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        5       17        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Line::from_groups(
            // e41, e42, e43
            (geometric_product_g1 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other_g0 * self[scalar]) * other.group0()),
            // e23, e31, e12
            geometric_product_g1 * Simd32x3::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       11       26        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g1 * Simd32x4::from(self[e1234])) + Simd32x3::from(0.0).with_w(other_g0 * self[scalar] * other[e1234])
                - (Simd32x3::from(other_g0 * self[scalar]) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            geometric_product_g1 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[0] * self[scalar],
                (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * geometric_product_g1.xyz()).with_w((self[scalar] * geometric_product_g1[3]) - (self[e1234] * geometric_product_g4[3])),
            // e41, e42, e43
            (geometric_product_g3 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other_g0 * self[scalar]) * other.group2()),
            // e23, e31, e12
            geometric_product_g3 * Simd32x3::from(self[scalar]),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g4.xyz()) - (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()))
                .with_w(self[scalar] * geometric_product_g4[3]),
        )
    }
}
impl GeometricQuotient<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        2      N/A
    //  no simd        0       14        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * geometric_product_g0[3] * -1.0),
            // e423, e431, e412, e321
            geometric_product_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       12        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       17        0      N/A
    //  no simd        8       31        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_product_g0 * Simd32x4::from(self[scalar]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234] * -1.0) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        2        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0 / other[scalar]) * self.group0())
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Flector {
    type Output = GeometricQuotientInfixPartial<Flector>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd2        0        1        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        8        2      N/A
    //  no simd        4       15        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz()).with_w((geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz()) + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz()))
                .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       17       23        0      N/A
    //  no simd       53       61        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]))
                + (geometric_product_g1.zxyx() * self.group0().yzxx())
                + (geometric_product_g1.wwwy() * self.group1().xyz().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) - (geometric_product_g0[2] * self[e412]))
                + (geometric_product_g0.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from(geometric_product_g0[3]) * self.group0().xyz().with_w(self[e321]))
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (self.group1().wwwy() * geometric_product_g1.xyz().with_w(geometric_product_g0[1]))
                - (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (Simd32x4::from(self[e321]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        1      N/A
    //  no simd        0       10        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_product_g0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0 * -1.0) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        9        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       42       48        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ) + (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) * -1.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e4])).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd3        0        7        0      N/A
    //    simd4       12        9        0      N/A
    // Totals...
    // yes simd       19       28        0      N/A
    //  no simd       55       69        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g0[1] * self[e2])
                        - (geometric_product_g0[2] * self[e3])
                        - (geometric_product_g1[0] * self[e423])
                        - (geometric_product_g1[1] * self[e431])
                        - (geometric_product_g1[2] * self[e412]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(geometric_product_g0[3] * self[e321])
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            (geometric_product_g1 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) * -1.0)
                + (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_product_g1.zxyy() * self.group1().yzx().with_w(self[e2]))
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       16       28        0        0
    //    simd2        4        9        0      N/A
    //    simd3       12       17        0      N/A
    //    simd4       12        7        0      N/A
    // Totals...
    // yes simd       44       61        0      N/A
    //  no simd      108      125        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g4[3] * self[e4]) - (geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x4::from(self[e321]) * geometric_product_g3.with_w(geometric_product_g0[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g2[1] * self[e2])
                        - (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * self[e423])
                        - (geometric_product_g3[1] * self[e431])
                        - (geometric_product_g3[2] * self[e412]),
                )
                + (geometric_product_g3.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g4[1] * self[e1]) - (geometric_product_g1[1] * self[e423]) - (geometric_product_g4[0] * self[e2]))
                + (geometric_product_g4.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g1.zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) * -1.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + Simd32x3::from(0.0).with_w((geometric_product_g3[2] * self[e3]) * -1.0)
                + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g2.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g3.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e4])).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0]))
                - (geometric_product_g3.zxy() * self.group1().yzx()).with_w(geometric_product_g3[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        2        0
    //    simd3        0        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        5       11        2      N/A
    //  no simd       17       29        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g0[3]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_product_g0.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0[3] * -1.0) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        4        0      N/A
    //    simd4        9        6        0      N/A
    // Totals...
    // yes simd       11       25        0      N/A
    //  no simd       38       51        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]))
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e4])).with_w(0.0)
                - (Simd32x4::from(geometric_product_g0[3]) * self.group0().xyz().with_w(self[e321]))
                - (geometric_product_g0.zxyx() * self.group1().yzxx()),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        8        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[scalar];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Horizon {
    type Output = GeometricQuotientInfixPartial<Horizon>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        5        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0[1] * self[e321]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd        3       23        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321] * -1.0) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]),
        )
    }
}
impl GeometricQuotient<Horizon> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / other[e321])
    }
}
impl GeometricQuotient<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        2       13        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e321] * -1.0) * other.group0()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       28        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       24        0      N/A
    //  no simd        7       46        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321] * -1.0) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * (Simd32x3::from(other_g0 * -1.0) * other.group3()).with_w(geometric_product_g0[1]),
            // e41, e42, e43
            Simd32x3::from(self[e321] * -1.0) * geometric_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * geometric_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * (Simd32x3::from(other_g0 * -1.0) * other.group2()).with_w(geometric_product_g0[0]),
        )
    }
}
impl GeometricQuotient<Plane> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        2        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        2      N/A
    //  no simd        0       14        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e321] * -1.0) * geometric_product_g0.xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321] * -1.0),
        )
    }
}
impl GeometricQuotient<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       14        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       18        0      N/A
    //  no simd        8       29        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321] * -1.0) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] / other[scalar])
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Line {
    type Output = GeometricQuotientInfixPartial<Line>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        3       12        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * self.group0()) + (Simd32x3::from(geometric_product_g0[1]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0[0]) * self.group1(),
        )
    }
}
impl GeometricQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        7        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       13       23        0      N/A
    //  no simd       37       49        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e43] * geometric_product_g0[2]) - (self[e31] * geometric_product_g1[1]) - (self[e12] * geometric_product_g1[2]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(self[e42] * geometric_product_g0[1])
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(self[e23] * geometric_product_g1[0]),
            // e423, e431, e412, e321
            (Simd32x3::from([geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group1().xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]]) * self.group1().zyz()).with_w(0.0)
                + (self.group0().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                - (geometric_product_g0.yzxx() * self.group0().zxy().with_w(self[e23]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0()).with_w(self[e31] * geometric_product_g0[1])
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(self[e12] * geometric_product_g0[2]),
        )
    }
}
impl GeometricQuotient<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        1        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        1      N/A
    //  no simd        0        8        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0 * -1.0) * self.group0()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        0        8        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       30       38        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(
                -(geometric_product_g0[2] * self[e12]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ) + (geometric_product_g0.yzx() * self.group1().zxy()).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (geometric_product_g0.zxy() * self.group1().yzx()).with_w(geometric_product_g0[0] * self[e23])
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g0[1] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]))
                + (geometric_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g1[0] * self[e23]),
        )
    }
}
impl GeometricQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4        9        7        0      N/A
    // Totals...
    // yes simd       16       23        0      N/A
    //  no simd       43       56        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(
                -(self[e42] * geometric_product_g1[1]) - (self[e43] * geometric_product_g1[2]) - (self[e31] * geometric_product_g0[1]) - (self[e12] * geometric_product_g0[2]),
            ) + (self.group0().xxy() * geometric_product_g1.wzx()).with_w(0.0)
                + (self.group0().zyz() * geometric_product_g1.yww()).with_w(0.0)
                + (self.group1().xxy() * geometric_product_g0.wzx()).with_w(0.0)
                + (self.group1().zyz() * geometric_product_g0.yww()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g1.zxyx())
                - (self.group1().yzxx() * geometric_product_g0.zxyx()),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e31] * geometric_product_g1[1]) - (self[e12] * geometric_product_g1[2]))
                + (self.group1().xxy() * geometric_product_g1.wzx()).with_w(0.0)
                + (self.group1().zyz() * geometric_product_g1.yww()).with_w(0.0)
                - (self.group1().yzxx() * geometric_product_g1.zxyx()),
        )
    }
}
impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd2        3        4        0      N/A
    //    simd3        7       18        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       29       47        0      N/A
    //  no simd       70       99        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g3[0] * self[e41]) - (geometric_product_g3[1] * self[e42]) - (geometric_product_g3[2] * self[e43]),
            ]) - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e43] * geometric_product_g1[2]) - (self[e31] * geometric_product_g4[1]) - (self[e12] * geometric_product_g4[2]))
                + (Simd32x3::from(geometric_product_g4[3]) * self.group1()).with_w(self[e42] * geometric_product_g1[1])
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(self[e23] * geometric_product_g4[0]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group1())
                + (geometric_product_g2.yzx() * self.group1().zxy())
                + (geometric_product_g3.yzx() * self.group0().zxy())
                - (geometric_product_g2.zxy() * self.group1().yzx())
                - (geometric_product_g3.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g0[0]) * self.group1()) + (geometric_product_g3.yzx() * self.group1().zxy()) - (geometric_product_g3.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from([geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]]) * self.group1().xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group1().zyz()).with_w(0.0)
                + (self.group0().yzx() * geometric_product_g1.zxy()).with_w(0.0)
                - (geometric_product_g1.yzxx() * self.group0().zxy().with_w(self[e23]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0()).with_w(self[e31] * geometric_product_g1[1])
                - (self.group1().yzx() * geometric_product_g4.zxy()).with_w(self[e12] * geometric_product_g1[2]),
        )
    }
}
impl GeometricQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        2        0
    //    simd3        0        5        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4       10        2      N/A
    //  no simd       10       23        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group1())
                .with_w(-(self[e23] * geometric_product_g0[0]) - (self[e31] * geometric_product_g0[1]) - (self[e12] * geometric_product_g0[2])),
            // e423, e431, e412, e321
            (self.group1().zxy() * geometric_product_g0.yzx()).with_w(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group0()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        3        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd        9       23        0      N/A
    //  no simd       30       44        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e42] * geometric_product_g0[1]) + (self[e43] * geometric_product_g0[2]))
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * geometric_product_g0[1]) - (self[e12] * geometric_product_g0[2]))
                + (Simd32x3::from(geometric_product_g0[3]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                - (geometric_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricQuotient<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        6        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[scalar];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(geometric_product_g0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Motor {
    type Output = GeometricQuotientInfixPartial<Motor>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        4       15        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g0[0]) * self.group0()) + (Simd32x4::from(geometric_product_g0[1]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0[0]) * self.group1(),
        )
    }
}
impl GeometricQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        5        0      N/A
    //    simd4       11        9        0      N/A
    // Totals...
    // yes simd       18       24        0      N/A
    //  no simd       51       61        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43])
                        - (geometric_product_g1[1] * self[e31])
                        - (geometric_product_g1[2] * self[e12])
                        - (geometric_product_g1[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
            // e423, e431, e412, e321
            (self.group1() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[3]))
                + (geometric_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                + (geometric_product_g1.xyx() * self.group1().wwy()).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group1().zxw()).with_w(0.0)
                - (geometric_product_g0.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g0.yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * geometric_product_g1.zxy().with_w(geometric_product_g0[2]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        1        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        1      N/A
    //  no simd        0       16        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl GeometricQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        8        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       15       22        0      N/A
    //  no simd       42       47        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(
                -(geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ) + (geometric_product_g0.xyx() * self.group1().wwy()).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group1().zxw()).with_w(0.0)
                + (geometric_product_g1.xyx() * self.group0().wwy()).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group0().zxw()).with_w(0.0)
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g1.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]))
                + (geometric_product_g1.xyx() * self.group1().wwy()).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group1().zxw()).with_w(0.0)
                - (geometric_product_g1.zxyx() * self.group1().yzxx()),
        )
    }
}
impl GeometricQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4       12       10        0      N/A
    // Totals...
    // yes simd       19       26        0      N/A
    //  no simd       55       68        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g1 * Simd32x4::from(self[e1234]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
                )
                + (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group1().zxy()).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g1.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group1().zxy()).with_w(0.0)
                - (geometric_product_g1.zxyx() * self.group1().yzxx()),
        )
    }
}
impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       20        0        0
    //    simd2        4        5        0      N/A
    //    simd3       10       19        0      N/A
    //    simd4       11        9        0      N/A
    // Totals...
    // yes simd       39       53        0      N/A
    //  no simd       96      123        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[0] * self[e1234]) - (geometric_product_g3[0] * self[e41]) - (geometric_product_g3[1] * self[e42]) - (geometric_product_g3[2] * self[e43]),
            ]) + (geometric_product_g0 * Simd32x2::from(self[scalar]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (geometric_product_g1.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43])
                        - (geometric_product_g4[1] * self[e31])
                        - (geometric_product_g4[2] * self[e12])
                        - (geometric_product_g4[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz()).with_w(0.0)
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g4[0])),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group1().xyz())
                + (geometric_product_g2.yzx() * self.group1().zxy())
                + (geometric_product_g3.yzx() * self.group0().zxy())
                - (geometric_product_g2.zxy() * self.group1().yzx())
                - (geometric_product_g3.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz())
                + (geometric_product_g3.yzx() * self.group1().zxy())
                - (geometric_product_g3.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (self.group1() * Simd32x3::from(geometric_product_g1[3]).with_w(geometric_product_g4[3]))
                + (geometric_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                + (geometric_product_g4.xyx() * self.group1().wwy()).with_w(0.0)
                + (geometric_product_g4.yzz() * self.group1().zxw()).with_w(0.0)
                - (geometric_product_g1.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g1.yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * geometric_product_g4.zxy().with_w(geometric_product_g1[2]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Motor {
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
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(
                -(geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g0[3] * self[e1234]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g0.xyz())
                + (geometric_product_g0.yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z(geometric_product_g0[1] * self[e23] * -1.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                - (geometric_product_g0.zx() * self.group1().yz()).with_z(0.0))
            .with_w(geometric_product_g0[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       15        0        0
    //    simd3        0        3        0      N/A
    //    simd4        9        7        0      N/A
    // Totals...
    // yes simd       10       25        0      N/A
    //  no simd       37       52        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43]))
                - (geometric_product_g0.zxy() * self.group1().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g0[2] * self[e12]) * -1.0)
                + (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_product_g0.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_product_g0.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g0.yzzy() * self.group0().zxw().with_w(self[e31])),
        )
    }
}
impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        2        1      N/A
    //  no simd        0        8        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[scalar];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_product_g0) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0) * self.group1(),
        )
    }
}
impl std::ops::Div<GeometricQuotientInfix> for MultiVector {
    type Output = GeometricQuotientInfixPartial<MultiVector>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        2        0
    //    simd2        0        1        0      N/A
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       14        2      N/A
    //  no simd        8       27        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[0] * self[scalar],
                (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz()).with_w((geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e321])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * self.group2()) + (Simd32x3::from(geometric_product_g0[1]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0[0]) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group4().xyz()) + (Simd32x3::from(geometric_product_g0[1]) * self.group1().xyz()))
                .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       21        0        0
    //    simd2        4        8        0      N/A
    //    simd3       12       15        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       38       51        0      N/A
    //  no simd       99      110        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * self[e4]) - (geometric_product_g0[0] * self[e423]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g0[0], geometric_product_g1[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g0[1], geometric_product_g1[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g0[2], geometric_product_g1[2]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]])),
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group3().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (self[e43] * geometric_product_g0[2]) - (self[e23] * geometric_product_g1[0]) - (self[e31] * geometric_product_g1[1]) - (self[e12] * geometric_product_g1[2]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group3()).with_w(self[e42] * geometric_product_g0[1])
                - (self.group3().yzx() * geometric_product_g0.zxy()).with_w(self[e1234] * geometric_product_g1[3]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * geometric_product_g0.xyz())
                + (geometric_product_g0.yzx() * self.group4().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) - (geometric_product_g0[1] * self[e423]) - (geometric_product_g1[0] * self[e2]))
                + (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (geometric_product_g0.zx() * self.group4().yz()).with_z(0.0)
                - (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((geometric_product_g0[0] * self[e2]) * -1.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                - (geometric_product_g0.yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x3::from([geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group3().xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]]) * self.group3().zyz()).with_w(0.0)
                + (self.group2().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                - (geometric_product_g0.xyzx() * Simd32x3::from(self[e1234]).with_w(self[e23]))
                - (geometric_product_g0.yzxy() * self.group2().zxy().with_w(self[e31]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group2()).with_w(self[e12] * geometric_product_g0[2])
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        1        0
    //    simd2        0        2        0      N/A
    //    simd3        0        3        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0       10        1      N/A
    //  no simd        0       27        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_product_g0) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(geometric_product_g0) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0 * -1.0) * self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * (self.group2() * Simd32x3::from(-1.0)).with_w(self[scalar]),
        )
    }
}
impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       16        0        0
    //    simd2        3        3        0      N/A
    //    simd3        7       18        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       27       39        0      N/A
    //  no simd       71       84        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ) + (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e41, e42, e43
            (geometric_product_g0 * Simd32x3::from(self[scalar]))
                + (geometric_product_g1 * Simd32x3::from(self[e1234]))
                + (geometric_product_g0.yzx() * self.group3().zxy())
                + (geometric_product_g1.yzx() * self.group2().zxy())
                - (geometric_product_g0.zxy() * self.group3().yzx())
                - (geometric_product_g1.zxy() * self.group2().yzx()),
            // e23, e31, e12
            (geometric_product_g1 * Simd32x3::from(self[scalar])) + (geometric_product_g1.yzx() * self.group3().zxy()) - (geometric_product_g1.zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) * -1.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group4().zx().with_z(self[e4])).with_w(0.0)
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group4().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd2        4        4        0      N/A
    //    simd3       10       19        0      N/A
    //    simd4       12        9        0      N/A
    // Totals...
    // yes simd       36       48        0      N/A
    //  no simd       96      117        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[scalar] * geometric_product_g0[3]) - (self[e23] * geometric_product_g0[0]) - (self[e31] * geometric_product_g0[1]) - (self[e12] * geometric_product_g0[2]),
            ]) + (Simd32x2::from(geometric_product_g1[3]) * self.group0())
                - (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x3::from(self[e321]).with_w(self[e4]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g0[1] * self[e2])
                        - (geometric_product_g0[2] * self[e3])
                        - (geometric_product_g1[0] * self[e423])
                        - (geometric_product_g1[1] * self[e431])
                        - (geometric_product_g1[2] * self[e412]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group1().zxy()).with_w(geometric_product_g0[3] * self[e321])
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product_g0.xyz())
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz())
                + (Simd32x3::from(geometric_product_g0[3]) * self.group3())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group2())
                + (self.group2().zxy() * geometric_product_g1.yzx())
                + (self.group3().zxy() * geometric_product_g0.yzx())
                - (self.group2().yzx() * geometric_product_g1.zxy())
                - (self.group3().yzx() * geometric_product_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_product_g1.xyz())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group3())
                + (self.group3().zxy() * geometric_product_g1.yzx())
                - (self.group3().yzx() * geometric_product_g1.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g1 * Simd32x3::from(self[e4]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) * -1.0)
                + (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz()).with_w(0.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group4().zxy()).with_w(0.0)
                - (geometric_product_g1.zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       39        0        0
    //    simd2        8       15        0      N/A
    //    simd3       24       34        0      N/A
    //    simd4       23       12        0      N/A
    // Totals...
    // yes simd       80      100        0      N/A
    //  no simd      205      219        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g4[3] * self[e4])
                    - (geometric_product_g3[0] * self[e41])
                    - (geometric_product_g3[1] * self[e42])
                    - (geometric_product_g3[2] * self[e43])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from(self[e321]) * geometric_product_g3.with_w(geometric_product_g0[1]))
                + (geometric_product_g1.yzxx() * self.group3().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g2[1] * self[e2])
                        - (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * self[e423])
                        - (geometric_product_g3[1] * self[e431])
                        - (geometric_product_g3[2] * self[e412])
                        - (self[e23] * geometric_product_g4[0])
                        - (self[e31] * geometric_product_g4[1])
                        - (self[e12] * geometric_product_g4[2]),
                )
                + (Simd32x3::from(geometric_product_g4[3]) * self.group3()).with_w(self[e43] * geometric_product_g1[2])
                + (geometric_product_g3.yzx() * self.group1().zxy()).with_w(self[e42] * geometric_product_g1[1])
                - (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(self[e1234] * geometric_product_g4[3]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_product_g4[3]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                + (geometric_product_g2.yzx() * self.group3().zxy())
                + (geometric_product_g3.yzx() * self.group2().zxy())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * self[e431]) + (geometric_product_g4[1] * self[e1]) - (geometric_product_g1[1] * self[e423]) - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * self.group4().zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz())
                - (geometric_product_g2.zxy() * self.group3().yzx())
                - (geometric_product_g3.zxy() * self.group2().yzx())
                - (geometric_product_g1.zx() * self.group4().yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group3())
                + (geometric_product_g3.yzx() * self.group3().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) - (geometric_product_g1[0] * self[e2]))
                + (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (geometric_product_g3.zxy() * self.group3().yzx())
                - (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group4())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]]) * self.group3().xxy()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group3().zyz()).with_w(0.0)
                + (geometric_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g2.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                + (geometric_product_g3.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
                + (geometric_product_g3.yzz() * self.group4().zx().with_z(self[e4])).with_w(0.0)
                + (self.group2().yzx() * geometric_product_g1.zxy()).with_w(0.0)
                - (geometric_product_g1.xyzx() * Simd32x3::from(self[e1234]).with_w(self[e23]))
                - (geometric_product_g1.yzxy() * self.group2().zxy().with_w(self[e31]))
                - (self.group1().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group2()).with_w(geometric_product_g3[2] * self[e3])
                - (geometric_product_g3.zxy() * self.group4().yzx()).with_w(geometric_product_g3[1] * self[e2])
                - (self.group3().yzx() * geometric_product_g4.zxy()).with_w(self[e12] * geometric_product_g1[2]),
        )
    }
}
impl GeometricQuotient<Plane> for MultiVector {
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
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[3] * self[e321] * -1.0,
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group3()).with_w(
                -(self[e1234] * geometric_product_g0[3]) - (self[e23] * geometric_product_g0[0]) - (self[e31] * geometric_product_g0[1]) - (self[e12] * geometric_product_g0[2]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group4().xyz())
                + (geometric_product_g0.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g0[0] * self[e2]) * -1.0)
                - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                - (geometric_product_g0.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0[3] * -1.0) * self.group1().xyz(),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g0.xyz()) + (self.group3().zxy() * geometric_product_g0.yzx())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group2())
                - (self.group3().yzx() * geometric_product_g0.zxy()))
            .with_w(self[scalar] * geometric_product_g0[3]),
        )
    }
}
impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       26        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        9        7        0      N/A
    // Totals...
    // yes simd       22       43        0      N/A
    //  no simd       63       82        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
                -(geometric_product_g0[0] * self[e423]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group3().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e42] * geometric_product_g0[1]) + (self[e43] * geometric_product_g0[2]))
                - (self.group3().yzx() * geometric_product_g0.zxy()).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product_g0.xyz())
                + (geometric_product_g0.yzx() * self.group4().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g0[1] * self[e423]) * -1.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (geometric_product_g0.zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((geometric_product_g0[0] * self[e2]) * -1.0)
                - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                - (geometric_product_g0.yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((self[e12] * geometric_product_g0[2]) * -1.0)
                + (Simd32x3::from(geometric_product_g0[3]) * self.group3()).with_w(0.0)
                + (self.group2().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                - (geometric_product_g0.xyzx() * Simd32x3::from(self[e1234]).with_w(self[e23]))
                - (geometric_product_g0.yzxy() * self.group2().zxy().with_w(self[e31])),
        )
    }
}
impl GeometricQuotient<Scalar> for MultiVector {
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
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[scalar];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_product_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(geometric_product_g0) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * self.group4(),
        )
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Origin {
    type Output = GeometricQuotientInfixPartial<Origin>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] / other[scalar])
    }
}
impl GeometricQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       14        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * (Simd32x4::from(other_g0) * other.group0()).xyz().with_w(other_g0 * other[e321] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * -1.0 / other[e321])
    }
}
impl GeometricQuotient<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2        9        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(-(other[e23] * other[e23] * self[e4]) - (other[e31] * other[e31] * self[e4]) - (other[e12] * other[e12] * self[e4])) * other.group1()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x4::from(other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar])
            * other.group1()
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g1[3] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * geometric_product_g1.xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       16        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       22        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other_g0 * other[e321] * self[e4] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[e4]) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e4] * -1.0) * other.group3()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * -1.0 / other[e321])
    }
}
impl GeometricQuotient<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       11        0      N/A
    //  no simd        6       17        0        3
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3)]))
                + (Simd32x3::from(other[e1] * self[e4]) * Simd32x3::from([other[e2] * other[e2], other[e1] * other[e2], other[e1] * other[e3]]))
                + (Simd32x3::from(other[e3] * self[e4]) * Simd32x3::from([other[e1] * other[e3], other[e2] * other[e3], other[e2] * other[e2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] / other[scalar])
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Plane {
    type Output = GeometricQuotientInfixPartial<Plane>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd2        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        8        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0[1] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0[0]) * self.group0(),
        )
    }
}
impl GeometricQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        5        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       20       34        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(self[e321]) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]))
                - (geometric_product_g0.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]),
        )
    }
}
impl GeometricQuotient<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        1      N/A
    //  no simd        0        6        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_product_g0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e321] * -1.0),
        )
    }
}
impl GeometricQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        5        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       12       23        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x3::from(self[e321]))
                .with_w(-(geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412])),
            // e423, e431, e412, e321
            (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x3::from(other_g0 * self[e321]) * other.group0()).with_w(0.0)
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       11       20        0      N/A
    //  no simd       19       40        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * geometric_product_g1.xyz()).with_w(
                (geometric_product_g0[3] * self[e321]) - (geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e431]) - (geometric_product_g1[1] * self[e423]))
                + (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0)
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0))
            .with_w(geometric_product_g1[3] * self[e321]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       25        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       10        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       20       39        0      N/A
    //  no simd       34       67        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g4[3] * self[e321] * -1.0,
                -(geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]) - (geometric_product_g1[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g3 * Simd32x3::from(self[e321])).with_w(
                (geometric_product_g0[1] * self[e321]) - (geometric_product_g3[0] * self[e423]) - (geometric_product_g3[1] * self[e431]) - (geometric_product_g3[2] * self[e412]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                + (geometric_product_g1.yzx() * self.group0().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e423]) * -1.0)
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz())
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * geometric_product_g1.xyz(),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz()) + (geometric_product_g3.yzx() * self.group0().zxy())
                - (Simd32x3::from(other_g0 * self[e321]) * other.group2())
                - (geometric_product_g3.zxy() * self.group0().yzx()))
            .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        2        0
    //    simd3        1        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        7        2      N/A
    //  no simd        3       16        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321] * -1.0),
        )
    }
}
impl GeometricQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       15        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        6       21        0      N/A
    //  no simd       18       37        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]))
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (geometric_product_g0.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321] * -1.0) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        4        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0 / other[scalar]) * self.group0())
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Point {
    type Output = GeometricQuotientInfixPartial<Point>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0       10        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(1.0 / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0[0]) * self.group0(),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       12       18        0      N/A
    //  no simd       33       41        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]))
                + (geometric_product_g1.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]))
                - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        1        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        1      N/A
    //  no simd        0        6        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e4]),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_product_g0 * -1.0) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       24       32        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]))
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]))
                + (geometric_product_g1 * Simd32x3::from(self[e4])).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        7        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       33       48        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g1[3]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]))
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]))
                + (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       20        0        0
    //    simd2        3        7        0      N/A
    //    simd3        7       10        0      N/A
    //    simd4        7        5        0      N/A
    // Totals...
    // yes simd       27       42        0      N/A
    //  no simd       65       84        0        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g2 = Simd32x3::from(other_g0 * -1.0) * other.group2();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, geometric_product_g4[3] * self[e4]])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + Simd32x3::from(0.0).with_w(-(geometric_product_g2[1] * self[e2]) - (geometric_product_g2[2] * self[e3]))
                + (geometric_product_g3.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                + Simd32x2::from(0.0).with_z((geometric_product_g4[1] * self[e1]) - (geometric_product_g4[0] * self[e2]))
                + (geometric_product_g4.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g4.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g1.zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) * -1.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(geometric_product_g3[1] * self[e2]) - (geometric_product_g3[2] * self[e3]))
                + (geometric_product_g3 * Simd32x3::from(self[e4])).with_w(0.0)
                + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0])),
        )
    }
}
impl GeometricQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        2        0
    //    simd3        0        3        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4       10        2      N/A
    //  no simd       10       22        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(-1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0.zxyx() * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e4]))
                - (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_product_g0[3] * -1.0) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       13        0        0
    //    simd3        1        3        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd        6       20        0      N/A
    //  no simd       20       38        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e1] * other[e1], other[e2] * other[e2], other[e3] * other[e3], other[e1] * other[e1]]) * other.group0())
            + (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * geometric_product_g0.xyz()) - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        1        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        1        1      N/A
    //  no simd        0        4        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0 / other[scalar]) * self.group0())
    }
}
impl std::ops::Div<GeometricQuotientInfix> for Scalar {
    type Output = GeometricQuotientInfixPartial<Scalar>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        4        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar] / other[scalar]) * Simd32x2::from([1.0, other[e1234] / other[scalar]]),
        )
    }
}
impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       15        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * self[scalar] * -1.0) * other.group1(),
        )
    }
}
impl GeometricQuotient<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        1        0
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * -1.0 / other[e321])
    }
}
impl GeometricQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        2       13        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[scalar] * -1.0) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[scalar] * -1.0) * other.group1(),
        )
    }
}
impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       10        0      N/A
    //  no simd        3       22        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0 * self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0 * self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Scalar {
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
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e321] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other_g0 * self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0 * self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_g0 * self[scalar] * -1.0) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * self[scalar] * -1.0) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * self[scalar] * -1.0) * other.group4(),
        )
    }
}
impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        9        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar] * -1.0 / other[e321]) * (Simd32x3::from(1.0 / other[e321]) * other.group0().xyz()).with_w(1.0),
        )
    }
}
impl GeometricQuotient<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       22        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       25        0      N/A
    //  no simd        8       34        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                other[e1] * other[e1] * self[scalar],
                other[e2] * other[e2] * self[scalar],
                other[e3] * other[e3] * self[scalar],
                other[e1] * other[e1] * self[scalar],
            ]) * other.group0())
                + (Simd32x4::from([
                    other[e2] * other[e2] * self[scalar],
                    other[e1] * other[e1] * self[scalar],
                    other[e1] * other[e1] * self[scalar],
                    other[e2] * other[e2] * self[scalar],
                ]) * other.group0())
                + (other.group0() * Simd32x2::from(other[e3] * other[e3] * self[scalar]).with_zw(other[e2] * other[e2] * self[scalar], other[e3] * other[e3] * self[scalar])),
        )
    }
}
impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / other[scalar])
    }
}
