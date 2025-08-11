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
//   Median:         6      22       0       0
//  Average:        14      29       0       0
//  Maximum:       179     213       4       3
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
    //      f32        0       10        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       13        0      N/A
    //  no simd        6       19        0        3
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            -(Simd32x3::from([other[e31] * other[e31] * self[e1234], other[e23] * other[e23] * self[e1234], other[e23] * other[e23] * self[e1234]]) * other.group1())
                - (other.group1() * Simd32x2::from(other[e12] * other[e12] * self[e1234]).with_z(other[e31] * other[e31] * self[e1234]))
                - (Simd32x3::powi(other.group1(), 3) * Simd32x3::from(self[e1234])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       14        0        3
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3       18        0      N/A
    //  no simd        9       26        0        3
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x3::from(self[e1234]) * Simd32x3::from([f32::powi(other[e23], 3), f32::powi(other[e31], 3), f32::powi(other[e12], 3)]))
                - (Simd32x3::from(self[e1234])
                    * Simd32x3::from([other[e31] * other[e31] * other[e23], other[e23] * other[e23] * other[e31], other[e23] * other[e23] * other[e12]]))
                - (Simd32x3::from(self[e1234])
                    * Simd32x3::from([other[e12] * other[e12] * other[e23], other[e12] * other[e12] * other[e31], other[e31] * other[e31] * other[e12]]))
                - (Simd32x3::from(other[scalar] * other[scalar] * self[e1234]) * other.group1().xyz()))
            .with_w(0.0),
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
    //      f32        0       12        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        6       21        0        3
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (-(Simd32x3::from(self[e1234]) * Simd32x3::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3)]))
                - (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e2] * other[e2] * other[e1], other[e1] * other[e1] * other[e2], other[e1] * other[e1] * other[e3]]))
                - (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e3] * other[e3] * other[e1], other[e3] * other[e3] * other[e2], other[e2] * other[e2] * other[e3]])))
            .with_w(0.0),
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
    //      add/sub      mul      div      pow
    // f32        1        5        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product_g0_x * self[scalar],
            (geometric_product_g0_x * self[e1234]) + (other[e1234] * self[scalar] / (other[scalar] * other[scalar])),
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
            (geometric_product_g0.xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar]))
                .with_w((self[scalar] * geometric_product_g0[3]) - (self[e1234] * geometric_product_g1[3])),
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
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        7       24        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g1 * Simd32x4::from(self[e1234])) + -(Simd32x3::from(other_g0 * self[scalar]) * other.group0().xyz()).with_w(0.0),
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
            (geometric_product_g1.xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar]))
                .with_w((self[scalar] * geometric_product_g1[3]) - (self[e1234] * geometric_product_g4[3])),
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
    //      f32        0        3        1        0
    //    simd3        0        3        1      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        7        2      N/A
    //  no simd        0       16        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 =
            (other.group0().xyz() * Simd32x3::from(-1.0) / (Simd32x4::from(other[e321]).xyz() * Simd32x4::from(other[e321]).xyz())).with_w(-1.0 / other[e321]);
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
    //      f32        0        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        6        0      N/A
    // Totals...
    // yes simd        2       16        0      N/A
    //  no simd        8       38        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_product_g0 * Simd32x4::from(self[scalar]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, self[e1234], 0.0])
                * (geometric_product_g0.xyz() * Simd32x2::from(self[e1234] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
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
    //      f32        1        5        2        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        8        2      N/A
    //  no simd        4       14        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        let geometric_product_g0_y = other[e1234] / (other[scalar] * other[scalar]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_x) * self.group0().xyz()).with_w((geometric_product_g0_x * self[e4]) + (geometric_product_g0_y * self[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0_x) * self.group1().xyz()) + (Simd32x3::from(geometric_product_g0_y) * self.group0().xyz()))
                .with_w(geometric_product_g0_x * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd2        4        6        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       20       31        0      N/A
    //  no simd       41       60        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_product_g1.zxyz() * self.group0().yzxz())
                + ((Simd32x2::from(self[e4]) * geometric_product_g0.xy()) + (geometric_product_g0.yz() * self.group1().zx())
                    - (Simd32x2::from(self[e321]) * geometric_product_g1.xy())
                    - (geometric_product_g1.yz() * self.group0().zx()))
                .with_zw(
                    (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]) - (geometric_product_g1[0] * self[e2]) - (geometric_product_g1[2] * self[e321]),
                    (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
                )
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g0.wwwy() * self.group0().xyz().with_w(self[e431])),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx())
                + ((-(Simd32x2::from(self[e321]) * geometric_product_g0.xy()) - (geometric_product_g0.yz() * self.group0().zx()))
                    .with_z(-(geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]))
                    - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()))
                .with_w(geometric_product_g1[3] * self[e321] * -1.0),
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
    //      f32        2        5        0        0
    //    simd3        7       11        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       23       38        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])) + (geometric_product_g1.yzz() * self.group0().zx().with_z(self[e321]))
                - (geometric_product_g1.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321]))
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                + (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e4]))
                - (geometric_product_g0.zxy() * self.group0().yzx())
                - (geometric_product_g1.zxy() * self.group1().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       15       22        0        0
    //    simd2        2        4        0      N/A
    //    simd3        4        3        0      N/A
    //    simd4        1        5        0      N/A
    // Totals...
    // yes simd       22       34        0      N/A
    //  no simd       35       59        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1.yzzw() * self.group0().zx().with_zw(self[e321], self[e4]))
                + ((Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                    + ((Simd32x2::from(self[e321]) * geometric_product_g1.xy()) - (geometric_product_g1.zx() * self.group0().yz()))
                        .with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1])))
                .with_w(geometric_product_g0[3] * self[e321]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_product_g0[1] * self[e3]) + (geometric_product_g1[1] * self[e412]) - (geometric_product_g0[2] * self[e2]) - (geometric_product_g1[2] * self[e431]),
                (geometric_product_g0[2] * self[e1]) + (geometric_product_g1[2] * self[e423]) - (geometric_product_g0[0] * self[e3]) - (geometric_product_g1[0] * self[e412]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g0[2] * self[e321]) + (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4])
                    - (geometric_product_g0[1] * self[e1])
                    - (geometric_product_g1[1] * self[e423]),
            ]) + (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + ((Simd32x2::from(self[e4]) * geometric_product_g1.xy()) + (Simd32x2::from(self[e321]) * geometric_product_g0.xy())).with_z(0.0))
            .with_w(geometric_product_g1[3] * self[e321]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       26       41        0        0
    //    simd2        4        7        0      N/A
    //    simd3       14       16        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       45       67        0      N/A
    //  no simd       80      115        0        0
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
                geometric_product_g4[3] * self[e321] * -1.0,
                (geometric_product_g4[3] * self[e4])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + ((geometric_product_g3.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])) + (geometric_product_g3.yzz() * self.group0().zx().with_z(self[e321]))
                    - (geometric_product_g3.zxy() * self.group0().yzx()))
                .with_w(geometric_product_g0[1] * self[e321]),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g1[1] * self[e412]) + (geometric_product_g4[2] * self[e2]) - (geometric_product_g1[2] * self[e431]) - (geometric_product_g4[1] * self[e3]),
                (geometric_product_g1[2] * self[e423]) + (geometric_product_g4[0] * self[e3]) - (geometric_product_g1[0] * self[e412]) - (geometric_product_g4[2] * self[e1]),
                (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4]) + (geometric_product_g4[1] * self[e1])
                    - (geometric_product_g1[1] * self[e423])
                    - (geometric_product_g4[0] * self[e2])
                    - (geometric_product_g4[2] * self[e321]),
            ]) + (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                + ((Simd32x2::from(self[e4]) * geometric_product_g1.xy()) - (Simd32x2::from(self[e321]) * geometric_product_g4.xy())).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_product_g1[2] * self[e2]) - (geometric_product_g1[1] * self[e3]),
                (geometric_product_g1[0] * self[e3]) - (geometric_product_g1[2] * self[e1]),
                (geometric_product_g1[1] * self[e1]) - (geometric_product_g1[0] * self[e2]) - (geometric_product_g1[2] * self[e321]),
            ]) + -(Simd32x2::from(self[e321]) * geometric_product_g1.xy()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz())
                + (geometric_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_product_g2.yzz() * self.group0().zx().with_z(self[e321]))
                + (geometric_product_g3.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                + (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e4]))
                - (geometric_product_g2.zxy() * self.group0().yzx())
                - (geometric_product_g3.zxy() * self.group1().yzx()))
            .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        2        0
    //    simd3        2        4        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        3       11        2      N/A
    //  no simd       10       25        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(-1.0 / (other[e321] * other[e321])) * other.group0().xyz();
        let geometric_product_g0_w = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0_xyz.zxyy() * self.group0().yzxy())
                + ((Simd32x3::from(geometric_product_g0_w) * self.group1().xyz())
                    - (geometric_product_g0_xyz.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                    - (geometric_product_g0_xyz.yzz() * self.group0().zx().with_z(self[e321])))
                .with_w(geometric_product_g0_xyz[0] * self[e1]),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0_w * -1.0) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       18        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        1        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       13       27        0      N/A
    //  no simd       28       45        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                0.0,
                0.0,
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]) - (geometric_product_g0[1] * self[e423]),
                0.0,
            ]) + (((Simd32x2::from(self[e4]) * geometric_product_g0.xy()) + (geometric_product_g0.yz() * self.group1().zx()) - (geometric_product_g0.zx() * self.group1().yz()))
                .with_z(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from([
                (geometric_product_g0[2] * self[e2]) - (geometric_product_g0[1] * self[e3]),
                (geometric_product_g0[0] * self[e3]) - (geometric_product_g0[2] * self[e1]),
                (geometric_product_g0[1] * self[e1]) - (geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]),
            ]) + -(Simd32x2::from(self[e321]) * geometric_product_g0.xy()).with_z(0.0))
            .with_w(geometric_product_g0[0] * self[e1]),
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
    //      add/sub      mul      div      pow
    // f32        0        4        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321] / (other[scalar] * other[scalar])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] / other[scalar]),
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
    //      f32        0        1        1        0
    //    simd3        0        3        1      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0       10        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e321]) * other.group0().xyz() / (Simd32x4::from(other[e321]).xyz() * Simd32x4::from(other[e321]).xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] / other[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       10        0        0
    //    simd3        0        1        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        8       29        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
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
    //      f32        0        2        2        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        3       11        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0_x) * self.group0()) + (Simd32x3::from(other[e1234] / (other[scalar] * other[scalar])) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0_x) * self.group1(),
        )
    }
}
impl GeometricQuotient<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        7        9        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       14       21        0      N/A
    //  no simd       28       45        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group1().zyz())
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0]]) * self.group1().xxy())
                - (self.group1().yzx() * geometric_product_g0.zxy()))
            .with_w(
                (self[e41] * geometric_product_g0[0]) + (self[e42] * geometric_product_g0[1])
                    - (self[e23] * geometric_product_g1[0])
                    - (self[e31] * geometric_product_g1[1])
                    - (self[e12] * geometric_product_g1[2]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from([geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group1().xxy())
                + (Simd32x3::from([geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]]) * self.group1().zyz())
                + (self.group0().yzx() * geometric_product_g0.zxy())
                - (Simd32x3::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group0().zyz())
                - (Simd32x3::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0]]) * self.group0().xxy())
                - (self.group1().yzx() * geometric_product_g1.zxy()))
            .with_w(0.0),
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
    //      f32        2        5        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       14       29        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_product_g0.yzx() * self.group1().zxy()) + (geometric_product_g1.yzx() * self.group0().zxy())
                - (geometric_product_g0.zxy() * self.group1().yzx())
                - (geometric_product_g1.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_product_g1.yzx() * self.group1().zxy()) - (geometric_product_g1.zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        7        9        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       24       47        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group0().xxy() * geometric_product_g1.wzx())
                + (self.group0().zyz() * geometric_product_g1.yww())
                + (self.group1().xxy() * geometric_product_g0.wzx())
                + (self.group1().zyz() * geometric_product_g0.yww())
                - (self.group0().yzx() * geometric_product_g1.zxy())
                - (self.group1().yzx() * geometric_product_g0.zxy()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            ((self.group1().xxy() * geometric_product_g1.wzx()) + (self.group1().zyz() * geometric_product_g1.yww()) - (self.group1().yzx() * geometric_product_g1.zxy()))
                .with_w(0.0),
        )
    }
}
impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       19        0        0
    //    simd2        3        4        0      N/A
    //    simd3       14       20        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       30       45        0      N/A
    //  no simd       61       95        0        0
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
            ((Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group1().zyz())
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group1().xxy())
                - (self.group1().yzx() * geometric_product_g1.zxy()))
            .with_w(
                (self[e41] * geometric_product_g1[0]) + (self[e42] * geometric_product_g1[1])
                    - (self[e23] * geometric_product_g4[0])
                    - (self[e31] * geometric_product_g4[1])
                    - (self[e12] * geometric_product_g4[2]),
            ),
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
            ((Simd32x3::from([geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]]) * self.group1().xxy())
                + (Simd32x3::from([geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group1().zyz())
                + (self.group0().yzx() * geometric_product_g1.zxy())
                - (Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group0().zyz())
                - (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group0().xxy())
                - (self.group1().yzx() * geometric_product_g4.zxy()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        2        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4       11        2      N/A
    //  no simd        8       21        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(-1.0 / (other[e321] * other[e321])) * other.group0().xyz();
        let geometric_product_g0_w = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_w) * self.group1())
                .with_w(-(geometric_product_g0_xyz[0] * self[e23]) - (geometric_product_g0_xyz[1] * self[e31]) - (geometric_product_g0_xyz[2] * self[e12])),
            // e423, e431, e412, e321
            ((geometric_product_g0_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g0_w) * self.group0())
                - (geometric_product_g0_xyz.zxy() * self.group1().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        8        0        0
    //    simd3        3        5        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        5       17        0      N/A
    //  no simd       17       39        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            ((self.group1().zxy() * geometric_product_g0.yzx()) - (self.group1().yzx() * geometric_product_g0.zxy())).with_w(self[e41] * geometric_product_g0[0]),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[3]) * self.group1()) + (self.group0().yzx() * geometric_product_g0.zxy()) - (self.group0().zxy() * geometric_product_g0.yzx()))
                .with_w(0.0),
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
    //      f32        0        2        2        0
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        1        5        2      N/A
    //  no simd        4       14        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g0_x) * self.group0()) + (Simd32x4::from(other[e1234] / (other[scalar] * other[scalar])) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0_x) * self.group1(),
        )
    }
}
impl GeometricQuotient<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       24        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       24       34        0      N/A
    //  no simd       40       57        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g0.yzzy() * self.group1().zxw().with_w(self[e42]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(
                    (geometric_product_g0[2] * self[e43]) - (geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]) - (geometric_product_g1[3] * self[e1234]),
                )
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_product_g0[2] * self[e42]) + (geometric_product_g1[1] * self[e12]) - (geometric_product_g0[1] * self[e43]) - (geometric_product_g1[2] * self[e31]),
                (geometric_product_g0[0] * self[e43]) + (geometric_product_g1[2] * self[e23]) - (geometric_product_g0[2] * self[e41]) - (geometric_product_g1[0] * self[e12]),
                (geometric_product_g0[1] * self[e41]) + (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[2] * self[scalar])
                    - (geometric_product_g0[0] * self[e42])
                    - (geometric_product_g0[2] * self[e1234])
                    - (geometric_product_g1[1] * self[e23]),
            ]) + (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                + ((Simd32x2::from(self[scalar]) * geometric_product_g1.xy()) - (Simd32x2::from(self[e1234]) * geometric_product_g0.xy())).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()))
            .with_w(geometric_product_g1[3] * self[scalar]),
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
    //      f32        2        5        0        0
    //    simd3        7       11        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd       23       38        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_product_g0.xyx() * self.group1().wwy())
                + (geometric_product_g0.yzz() * self.group1().zxw())
                + (geometric_product_g1.xyx() * self.group0().wwy())
                + (geometric_product_g1.yzz() * self.group0().zxw())
                - (geometric_product_g0.zxy() * self.group1().yzx())
                - (geometric_product_g1.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_product_g1.xyx() * self.group1().wwy()) + (geometric_product_g1.yzz() * self.group1().zxw()) - (geometric_product_g1.zxy() * self.group1().yzx()))
                .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Motor {
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
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0.xyxw() * self.group1().wwyw())
                + ((Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                    + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                    + ((Simd32x2::from(self[e1234]) * geometric_product_g1.xy())
                        + (geometric_product_g0.yz() * self.group1().zx())
                        + (geometric_product_g1.yz() * self.group0().zx())
                        - (geometric_product_g0.zx() * self.group1().yz())
                        - (geometric_product_g1.zx() * self.group0().yz()))
                    .with_z(
                        (geometric_product_g0[2] * self[scalar]) + (geometric_product_g1[0] * self[e42]) + (geometric_product_g1[2] * self[e1234])
                            - (geometric_product_g0[1] * self[e23])
                            - (geometric_product_g1[1] * self[e41]),
                    ))
                .with_w(geometric_product_g1[3] * self[e1234]),
            // e23, e31, e12, scalar
            (Simd32x3::from([
                (geometric_product_g1[1] * self[e12]) - (geometric_product_g1[2] * self[e31]),
                (geometric_product_g1[2] * self[e23]) - (geometric_product_g1[0] * self[e12]),
                (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[2] * self[scalar]) - (geometric_product_g1[1] * self[e23]),
            ]) + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x2::from(self[scalar]) * geometric_product_g1.xy()).with_z(0.0))
            .with_w(geometric_product_g1[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       37        0        0
    //    simd2        4        6        0      N/A
    //    simd3       13       17        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       45       65        0      N/A
    //  no simd       84      120        0        0
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
                geometric_product_g3[0] * self[e23] * -1.0,
                (geometric_product_g0[1] * self[scalar])
                    - (geometric_product_g2[2] * self[e12])
                    - (geometric_product_g3[0] * self[e41])
                    - (geometric_product_g3[1] * self[e42])
                    - (geometric_product_g3[2] * self[e43]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from([geometric_product_g3[1], geometric_product_g2[0]]) * self.group1().yx())
                - (Simd32x2::from([geometric_product_g3[2], geometric_product_g2[1]]) * self.group1().zy()),
            // e1, e2, e3, e4
            (geometric_product_g1.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g1.yzzy() * self.group1().zxw().with_w(self[e42]))
                + (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz()).with_w(
                    (geometric_product_g1[2] * self[e43]) - (geometric_product_g4[1] * self[e31]) - (geometric_product_g4[2] * self[e12]) - (geometric_product_g4[3] * self[e1234]),
                )
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g4[0])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group1().xyz())
                + (geometric_product_g2.xyx() * self.group1().wwy())
                + (geometric_product_g2.yzz() * self.group1().zxw())
                + (geometric_product_g3.xyx() * self.group0().wwy())
                + (geometric_product_g3.yzz() * self.group0().zxw())
                - (geometric_product_g2.zxy() * self.group1().yzx())
                - (geometric_product_g3.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz())
                + (geometric_product_g3.xyx() * self.group1().wwy())
                + (geometric_product_g3.yzz() * self.group1().zxw())
                - (geometric_product_g3.zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_product_g1[2] * self[e42]) + (geometric_product_g4[1] * self[e12]) - (geometric_product_g1[1] * self[e43]) - (geometric_product_g4[2] * self[e31]),
                (geometric_product_g1[0] * self[e43]) + (geometric_product_g4[2] * self[e23]) - (geometric_product_g1[2] * self[e41]) - (geometric_product_g4[0] * self[e12]),
                (geometric_product_g1[1] * self[e41]) + (geometric_product_g4[0] * self[e31]) + (geometric_product_g4[2] * self[scalar])
                    - (geometric_product_g1[0] * self[e42])
                    - (geometric_product_g1[2] * self[e1234])
                    - (geometric_product_g4[1] * self[e23]),
            ]) + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + ((Simd32x2::from(self[scalar]) * geometric_product_g4.xy()) - (Simd32x2::from(self[e1234]) * geometric_product_g1.xy())).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()))
            .with_w(geometric_product_g4[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        2        0
    //    simd3        3        6        0      N/A
    // Totals...
    // yes simd        6       14        2      N/A
    //  no simd       12       26        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(-1.0 / (other[e321] * other[e321])) * other.group0().xyz();
        let geometric_product_g0_w = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_w) * self.group1().xyz()).with_w(
                -(geometric_product_g0_w * self[e1234])
                    - (geometric_product_g0_xyz[0] * self[e23])
                    - (geometric_product_g0_xyz[1] * self[e31])
                    - (geometric_product_g0_xyz[2] * self[e12]),
            ),
            // e423, e431, e412, e321
            ((geometric_product_g0_xyz.xyx() * self.group1().wwy()) + (geometric_product_g0_xyz.yzz() * self.group1().zxw())
                - (Simd32x3::from(geometric_product_g0_w) * self.group0().xyz())
                - (geometric_product_g0_xyz.zxy() * self.group1().yzx()))
            .with_w(geometric_product_g0_w * self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       12        0        0
    //    simd2        2        4        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        6        6        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       33       47        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, geometric_product_g0[1] * self[e23] * -1.0, 0.0])
                + (geometric_product_g0.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g0.yzzy() * self.group1().zxw().with_w(self[e42]))
                + -(geometric_product_g0.zx() * self.group1().yz()).with_zw(0.0, 0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                0.0,
                0.0,
                (geometric_product_g0[1] * self[e41]) - (geometric_product_g0[0] * self[e42]) - (geometric_product_g0[2] * self[e1234]),
                0.0,
            ]) + ((Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                + ((geometric_product_g0.zx() * self.group0().yz())
                    - (Simd32x2::from(self[e1234]) * geometric_product_g0.xy())
                    - (geometric_product_g0.yz() * self.group0().zx()))
                .with_z(0.0))
            .with_w(0.0),
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
    //      f32        2        8        2        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       14        2      N/A
    //  no simd        8       26        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        let geometric_product_g0_y = other[e1234] / (other[scalar] * other[scalar]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product_g0_x * self[scalar], (geometric_product_g0_x * self[e1234]) + (geometric_product_g0_y * self[scalar])]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_x) * self.group1().xyz()).with_w((geometric_product_g0_x * self[e4]) + (geometric_product_g0_y * self[e321])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0_x) * self.group2()) + (Simd32x3::from(geometric_product_g0_y) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0_x) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0_x) * self.group4().xyz()) + (Simd32x3::from(geometric_product_g0_y) * self.group1().xyz()))
                .with_w(geometric_product_g0_x * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       27       40        0        0
    //    simd2        4        6        0      N/A
    //    simd3       14       14        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       46       63        0      N/A
    //  no simd       81      106        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g1[3] * self[e321] * -1.0,
                (geometric_product_g1[3] * self[e4])
                    - (geometric_product_g0[0] * self[e423])
                    - (geometric_product_g0[1] * self[e431])
                    - (geometric_product_g0[2] * self[e412])
                    - (geometric_product_g0[3] * self[e321]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g0[0], geometric_product_g1[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g0[1], geometric_product_g1[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g0[2], geometric_product_g1[2]])),
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + ((Simd32x3::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group3().zyz())
                    + (Simd32x3::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0]]) * self.group3().xxy())
                    - (self.group3().yzx() * geometric_product_g0.zxy()))
                .with_w(
                    (self[e41] * geometric_product_g0[0]) + (self[e42] * geometric_product_g0[1])
                        - (self[e1234] * geometric_product_g1[3])
                        - (self[e23] * geometric_product_g1[0])
                        - (self[e31] * geometric_product_g1[1])
                        - (self[e12] * geometric_product_g1[2]),
                ),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g0[1] * self[e412]) + (geometric_product_g1[2] * self[e2]) - (geometric_product_g0[2] * self[e431]) - (geometric_product_g1[1] * self[e3]),
                (geometric_product_g0[2] * self[e423]) + (geometric_product_g1[0] * self[e3]) - (geometric_product_g0[0] * self[e412]) - (geometric_product_g1[2] * self[e1]),
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]) + (geometric_product_g1[1] * self[e1])
                    - (geometric_product_g0[1] * self[e423])
                    - (geometric_product_g1[0] * self[e2])
                    - (geometric_product_g1[2] * self[e321]),
            ]) + (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                + ((Simd32x2::from(self[e4]) * geometric_product_g0.xy()) - (Simd32x2::from(self[e321]) * geometric_product_g1.xy())).with_z(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_product_g0[2] * self[e2]) - (geometric_product_g0[1] * self[e3]),
                (geometric_product_g0[0] * self[e3]) - (geometric_product_g0[2] * self[e1]),
                (geometric_product_g0[1] * self[e1]) - (geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]),
            ]) + -(Simd32x2::from(self[e321]) * geometric_product_g0.xy()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz())
                + (Simd32x3::from([geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group3().xxy())
                + (Simd32x3::from([geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]]) * self.group3().zyz())
                + (self.group2().yzx() * geometric_product_g0.zxy())
                - (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz())
                - (Simd32x3::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group2().zyz())
                - (Simd32x3::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0]]) * self.group2().xxy())
                - (self.group3().yzx() * geometric_product_g1.zxy()))
            .with_w(self[scalar] * geometric_product_g1[3]),
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
    //      f32        4        8        0        0
    //    simd2        3        3        0      N/A
    //    simd3       14       20        0      N/A
    // Totals...
    // yes simd       21       31        0      N/A
    //  no simd       52       74        0        0
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
            ((geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])) + (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e321]))
                - (geometric_product_g1.zxy() * self.group1().yzx()))
            .with_w(0.0),
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
            ((geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                + (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e321]))
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                + (geometric_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                - (geometric_product_g0.zxy() * self.group1().yzx())
                - (geometric_product_g1.zxy() * self.group4().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       29        0        0
    //    simd2        5        7        0      N/A
    //    simd3       14       15        0      N/A
    //    simd4        1        5        0      N/A
    // Totals...
    // yes simd       39       56        0      N/A
    //  no simd       75      108        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[e23] * geometric_product_g1[0] * -1.0,
                (self[e1234] * geometric_product_g1[3])
                    - (self[e43] * geometric_product_g1[2])
                    - (self[e23] * geometric_product_g0[0])
                    - (self[e31] * geometric_product_g0[1])
                    - (self[e12] * geometric_product_g0[2]),
            ]) + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]]))
                - (Simd32x2::from([self[e31], self[e41]]) * geometric_product_g1.yx())
                - (Simd32x2::from([self[e12], self[e42]]) * geometric_product_g1.zy()),
            // e1, e2, e3, e4
            (geometric_product_g1.yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + ((Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                    + ((Simd32x2::from(self[e321]) * geometric_product_g1.xy()) - (geometric_product_g1.zx() * self.group1().yz()))
                        .with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1])))
                .with_w(geometric_product_g0[3] * self[e321]),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * geometric_product_g0.xyz())
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz())
                + (self.group2().xxy() * geometric_product_g1.wzx())
                + (self.group2().zyz() * geometric_product_g1.yww())
                + (self.group3().xxy() * geometric_product_g0.wzx())
                + (self.group3().zyz() * geometric_product_g0.yww())
                - (self.group2().yzx() * geometric_product_g1.zxy())
                - (self.group3().yzx() * geometric_product_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_product_g1.xyz()) + (self.group3().xxy() * geometric_product_g1.wzx()) + (self.group3().zyz() * geometric_product_g1.yww())
                - (self.group3().yzx() * geometric_product_g1.zxy()),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_product_g0[1] * self[e3]) + (geometric_product_g1[1] * self[e412]) - (geometric_product_g0[2] * self[e2]) - (geometric_product_g1[2] * self[e431]),
                (geometric_product_g0[2] * self[e1]) + (geometric_product_g1[2] * self[e423]) - (geometric_product_g0[0] * self[e3]) - (geometric_product_g1[0] * self[e412]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g0[2] * self[e321]) + (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4])
                    - (geometric_product_g0[1] * self[e1])
                    - (geometric_product_g1[1] * self[e423]),
            ]) + (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                + ((Simd32x2::from(self[e4]) * geometric_product_g1.xy()) + (Simd32x2::from(self[e321]) * geometric_product_g0.xy())).with_z(0.0))
            .with_w(geometric_product_g1[3] * self[e321]),
        )
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
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
                (geometric_product_g1[0] * self[e1]) - (geometric_product_g3[0] * self[e23]) - (geometric_product_g3[1] * self[e31]) - (geometric_product_g4[3] * self[e321]),
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g4[2] * self[e3]) + (geometric_product_g4[3] * self[e4])
                    - (geometric_product_g2[1] * self[e31])
                    - (geometric_product_g2[2] * self[e12])
                    - (geometric_product_g3[0] * self[e41])
                    - (geometric_product_g3[1] * self[e42])
                    - (geometric_product_g3[2] * self[e43])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from([geometric_product_g1[1], geometric_product_g4[0]]) * self.group1().yx())
                + (Simd32x2::from([geometric_product_g1[2], geometric_product_g4[1]]) * self.group1().zy())
                - (Simd32x2::from([geometric_product_g3[2], geometric_product_g2[0]]) * self.group3().zx()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz())
                    + (Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group3().zyz())
                    + (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group3().xxy())
                    + (geometric_product_g3.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                    + (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e321]))
                    - (geometric_product_g3.zxy() * self.group1().yzx())
                    - (self.group3().yzx() * geometric_product_g1.zxy()))
                .with_w(
                    (geometric_product_g0[1] * self[e321])
                        + (self[scalar] * geometric_product_g1[3])
                        + (self[e41] * geometric_product_g1[0])
                        + (self[e42] * geometric_product_g1[1])
                        + (self[e43] * geometric_product_g1[2])
                        - (self[e1234] * geometric_product_g4[3])
                        - (geometric_product_g2[0] * self[e1])
                        - (geometric_product_g2[1] * self[e2])
                        - (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * self[e423])
                        - (geometric_product_g3[1] * self[e431])
                        - (geometric_product_g3[2] * self[e412])
                        - (self[e23] * geometric_product_g4[0])
                        - (self[e31] * geometric_product_g4[1])
                        - (self[e12] * geometric_product_g4[2]),
                ),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g1[1] * self[e412]) + (geometric_product_g4[2] * self[e2]) - (geometric_product_g1[2] * self[e431]) - (geometric_product_g4[1] * self[e3]),
                (geometric_product_g1[2] * self[e423]) + (geometric_product_g4[0] * self[e3]) - (geometric_product_g1[0] * self[e412]) - (geometric_product_g4[2] * self[e1]),
                (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4]) + (geometric_product_g4[1] * self[e1])
                    - (geometric_product_g1[1] * self[e423])
                    - (geometric_product_g4[0] * self[e2])
                    - (geometric_product_g4[2] * self[e321]),
            ]) + (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_product_g4[3]) * self.group4().xyz())
                + (geometric_product_g2.yzx() * self.group3().zxy())
                + (geometric_product_g3.yzx() * self.group2().zxy())
                + ((Simd32x2::from(self[e4]) * geometric_product_g1.xy()) - (Simd32x2::from(self[e321]) * geometric_product_g4.xy())).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (geometric_product_g2.zxy() * self.group3().yzx())
                - (geometric_product_g3.zxy() * self.group2().yzx()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_product_g1[2] * self[e2]) - (geometric_product_g1[1] * self[e3]),
                (geometric_product_g1[0] * self[e3]) - (geometric_product_g1[2] * self[e1]),
                (geometric_product_g1[1] * self[e1]) - (geometric_product_g1[0] * self[e2]) - (geometric_product_g1[2] * self[e321]),
            ]) + (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group3())
                + (geometric_product_g3.yzx() * self.group3().zxy())
                + -(Simd32x2::from(self[e321]) * geometric_product_g1.xy()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (geometric_product_g3.zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_product_g0[0]) * self.group4())
                + ((Simd32x3::from(geometric_product_g0[1]) * self.group1().xyz())
                    + (Simd32x3::from(self[scalar]) * geometric_product_g4.xyz())
                    + (Simd32x3::from([geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]]) * self.group3().xxy())
                    + (Simd32x3::from([geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group3().zyz())
                    + (geometric_product_g2.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                    + (geometric_product_g2.yzz() * self.group1().zx().with_z(self[e321]))
                    + (geometric_product_g3.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                    + (geometric_product_g3.yzz() * self.group4().zx().with_z(self[e4]))
                    + (self.group2().yzx() * geometric_product_g1.zxy())
                    - (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz())
                    - (Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group2().zyz())
                    - (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group2().xxy())
                    - (geometric_product_g2.zxy() * self.group1().yzx())
                    - (geometric_product_g3.zxy() * self.group4().yzx())
                    - (self.group3().yzx() * geometric_product_g4.zxy()))
                .with_w(self[scalar] * geometric_product_g4[3]),
        )
    }
}
impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        2        0
    //    simd3        6       11        0      N/A
    // Totals...
    // yes simd       12       26        2      N/A
    //  no simd       24       48        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(-1.0 / (other[e321] * other[e321])) * other.group0().xyz();
        let geometric_product_g0_w = -1.0 / other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0_w * self[e321] * -1.0,
                (geometric_product_g0_w * self[e4])
                    + (geometric_product_g0_xyz[0] * self[e1])
                    + (geometric_product_g0_xyz[1] * self[e2])
                    + (geometric_product_g0_xyz[2] * self[e3]),
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_w) * self.group3()).with_w(
                -(geometric_product_g0_w * self[e1234])
                    - (geometric_product_g0_xyz[0] * self[e23])
                    - (geometric_product_g0_xyz[1] * self[e31])
                    - (geometric_product_g0_xyz[2] * self[e12]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0_w) * self.group4().xyz()) + (geometric_product_g0_xyz.zxy() * self.group1().yzx())
                - (geometric_product_g0_xyz.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g0_xyz.yzz() * self.group1().zx().with_z(self[e321])),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0_w * -1.0) * self.group1().xyz(),
            // e423, e431, e412, e321
            ((geometric_product_g0_xyz * Simd32x3::from(self[scalar])) + (geometric_product_g0_xyz.yzx() * self.group3().zxy())
                - (Simd32x3::from(geometric_product_g0_w) * self.group2())
                - (geometric_product_g0_xyz.zxy() * self.group3().yzx()))
            .with_w(geometric_product_g0_w * self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       28        0        0
    //    simd2        0        2        0      N/A
    //    simd3        6        6        0      N/A
    //    simd4        4        6        0      N/A
    // Totals...
    // yes simd       23       42        0      N/A
    //  no simd       47       74        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
                -(geometric_product_g0[0] * self[e423]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group3().zxy().with_w(self[e41]))
                + -(self.group3().yzx() * geometric_product_g0.zxy()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g0[1] * self[e412]) - (geometric_product_g0[2] * self[e431]),
                (geometric_product_g0[2] * self[e423]) - (geometric_product_g0[0] * self[e412]),
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]) - (geometric_product_g0[1] * self[e423]),
            ]) + (Simd32x2::from(self[e4]) * geometric_product_g0.xy()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_product_g0[2] * self[e2]) - (geometric_product_g0[1] * self[e3]),
                (geometric_product_g0[0] * self[e3]) - (geometric_product_g0[2] * self[e1]),
                (geometric_product_g0[1] * self[e1]) - (geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]),
            ]) + -(Simd32x2::from(self[e321]) * geometric_product_g0.xy()).with_z(0.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[3]) * self.group3()) + (self.group2().yzx() * geometric_product_g0.zxy())
                - (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz())
                - (self.group2().zxy() * geometric_product_g0.yzx()))
            .with_w(0.0),
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
    //      f32        0       14        0        3
    //    simd3        3        4        0      N/A
    // Totals...
    // yes simd        3       18        0      N/A
    //  no simd        9       26        0        3
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * Simd32x3::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3)]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e2] * other[e2] * other[e1], other[e1] * other[e1] * other[e2], other[e1] * other[e1] * other[e3]]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e3] * other[e3] * other[e1], other[e3] * other[e3] * other[e2], other[e2] * other[e2] * other[e3]]))
                + (Simd32x3::from(other[e321] * other[e321] * self[e4]) * other.group0().xyz()))
            .with_w(0.0),
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
    //      f32        0        0        0        3
    //    simd3        2        7        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       21        0        3
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (-(Simd32x3::powi(other.group1(), 3) * Simd32x3::from(self[e4]))
                - (Simd32x3::powi(other.group1().yxx(), 2) * Simd32x3::from(self[e4]) * other.group1())
                - (Simd32x3::powi(other.group1().zzy(), 2) * Simd32x3::from(self[e4]) * other.group1()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Origin {
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
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x4::from([
            0.0,
            0.0,
            -f32::powi(other[e12], 3) - (other[e23] * other[e23] * other[e12]) - (other[e31] * other[e31] * other[e12]),
            0.0,
        ]) + ((-Simd32x2::powi(other.group1().xy(), 3)
            - (Simd32x2::from(other[e12] * other[e12]) * other.group1().xy())
            - (Simd32x2::powi(other.group1().yx(), 2) * other.group1().xy()))
        .with_z(0.0)
            - (Simd32x3::from(other[scalar] * other[scalar]) * other.group1().xyz()))
        .with_w(0.0);
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
    //      f32        0       12        0        3
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2       15        0      N/A
    //  no simd        6       21        0        3
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3)]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e2] * other[e2] * other[e1], other[e1] * other[e1] * other[e2], other[e1] * other[e1] * other[e3]]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e3] * other[e3] * other[e1], other[e3] * other[e3] * other[e2], other[e2] * other[e2] * other[e3]])),
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
    //      f32        0        3        2        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        7        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321] / (other[scalar] * other[scalar])),
            // e423, e431, e412, e321
            Simd32x4::from(1.0 / other[scalar]) * self.group0(),
        )
    }
}
impl GeometricQuotient<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       16       30        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, (geometric_product_g0[0] * self[e431]) - (geometric_product_g0[1] * self[e423]), 0.0])
                + ((Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                    + ((geometric_product_g0.yz() * self.group0().zx()) - (geometric_product_g0.zx() * self.group0().yz())).with_z(0.0)
                    - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz()))
                .with_w(0.0),
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
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd       10       23        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[e321]).xyz())
                .with_w(-(geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412])),
            // e423, e431, e412, e321
            ((geometric_product_g1.yzx() * self.group0().zxy()) - (Simd32x3::from(other_g0 * self[e321]) * other.group0()) - (geometric_product_g1.zxy() * self.group0().yzx()))
                .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       15        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       11       22        0      N/A
    //  no simd       15       40        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1.xyz() * Simd32x4::from(self[e321]).xyz()).with_w(
                (geometric_product_g0[3] * self[e321]) - (geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]),
            ),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (geometric_product_g1[1] * self[e412]) - (geometric_product_g1[2] * self[e431]),
                (geometric_product_g1[2] * self[e423]) - (geometric_product_g1[0] * self[e412]),
                (geometric_product_g1[0] * self[e431]) - (geometric_product_g1[1] * self[e423]),
            ]) + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                + (Simd32x3::from(self[e321]) * geometric_product_g0.xyz()))
            .with_w(geometric_product_g1[3] * self[e321]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Plane {
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
            (geometric_product_g3 * Simd32x4::from(self[e321]).xyz()).with_w(
                (geometric_product_g0[1] * self[e321]) - (geometric_product_g3[0] * self[e423]) - (geometric_product_g3[1] * self[e431]) - (geometric_product_g3[2] * self[e412]),
            ),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g1[1] * self[e412]) - (geometric_product_g1[2] * self[e431]),
                (geometric_product_g1[2] * self[e423]) - (geometric_product_g1[0] * self[e412]),
                (geometric_product_g1[0] * self[e431]) - (geometric_product_g1[1] * self[e423]),
            ]) + (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz()),
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
    //      f32        0        5        2        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        7        2      N/A
    //  no simd        3       11        2        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_w = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(geometric_product_g0_w) * self.group0().xyz()) + (Simd32x3::from(self[e321] / (other[e321] * other[e321])) * other.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0_w * self[e321] * -1.0),
        )
    }
}
impl GeometricQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd        4       19        0      N/A
    //  no simd       11       37        0        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_product_g0.yz() * self.group0().zx()) - (geometric_product_g0.zx() * self.group0().yz())).with_zw(
                (geometric_product_g0[0] * self[e431]) - (geometric_product_g0[1] * self[e423]),
                geometric_product_g0[3] * self[e321] * -1.0,
            ),
            // e23, e31, e12, scalar
            (geometric_product_g0.xyz() * Simd32x4::from(self[e321]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
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
    //      f32        0        4        3        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        7        3      N/A
    //  no simd        0       15        3        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(1.0 / other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, other[e1234] / (other[scalar] * other[scalar]), 0.0])
                * (self.group0().xyz() * Simd32x2::from(other[e1234] / (other[scalar] * other[scalar])).with_z(1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       24       38        0        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, geometric_product_g1[0] * self[e2] * -1.0, 0.0])
                + (geometric_product_g1.zxyy() * self.group0().yzxy())
                + (self.group0().wwwx() * geometric_product_g0.xyz().with_w(geometric_product_g1[0]))
                + (-(geometric_product_g1.yz() * self.group0().zx()).with_z(0.0) - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x3::from([
                (geometric_product_g0[2] * self[e2]) - (geometric_product_g0[1] * self[e3]),
                (geometric_product_g0[0] * self[e3]) - (geometric_product_g0[2] * self[e1]),
                (geometric_product_g0[1] * self[e1]) - (geometric_product_g0[0] * self[e2]),
            ]) - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()))
            .with_w(geometric_product_g0[0] * self[e1]),
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
    //      f32        2        5        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd       11       26        0        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            ((geometric_product_g1.yzx() * self.group0().zxy()) - (geometric_product_g1.zxy() * self.group0().yzx())).with_w(0.0),
            // e423, e431, e412, e321
            ((geometric_product_g1 * Simd32x3::from(self[e4])) + (geometric_product_g0.yzx() * self.group0().zxy()) - (geometric_product_g0.zxy() * self.group0().yzx()))
                .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        1        4        0      N/A
    // Totals...
    // yes simd       12       22        0      N/A
    //  no simd       22       42        0        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (geometric_product_g1[1] * self[e3]) - (geometric_product_g1[2] * self[e2]),
                (geometric_product_g1[2] * self[e1]) - (geometric_product_g1[0] * self[e3]),
                (geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]),
            ]) + (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()))
            .with_w(geometric_product_g1[3] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, (geometric_product_g0[0] * self[e2]) - (geometric_product_g0[1] * self[e1]), 0.0])
                + ((Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                    + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                    + ((geometric_product_g0.yz() * self.group0().zx()) - (geometric_product_g0.zx() * self.group0().yz())).with_z(0.0))
                .with_w(0.0),
        )
    }
}
impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       25        0        0
    //    simd2        3        4        0      N/A
    //    simd3        8       12        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       24       43        0      N/A
    //  no simd       43       77        0        0
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
            ((Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz()) + (geometric_product_g3.yzx() * self.group0().zxy())
                - (geometric_product_g3.zxy() * self.group0().yzx()))
            .with_w(geometric_product_g0[0] * self[e4]),
            // e41, e42, e43
            Simd32x3::from([
                (geometric_product_g4[2] * self[e2]) - (geometric_product_g4[1] * self[e3]),
                (geometric_product_g4[0] * self[e3]) - (geometric_product_g4[2] * self[e1]),
                (geometric_product_g4[1] * self[e1]) - (geometric_product_g4[0] * self[e2]),
            ]) + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (geometric_product_g1[2] * self[e2]) - (geometric_product_g1[1] * self[e3]),
                (geometric_product_g1[0] * self[e3]) - (geometric_product_g1[2] * self[e1]),
                (geometric_product_g1[1] * self[e1]) - (geometric_product_g1[0] * self[e2]),
            ]) - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()),
            // e423, e431, e412, e321
            ((geometric_product_g3 * Simd32x3::from(self[e4]))
                + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz())
                + (geometric_product_g2.yzx() * self.group0().zxy())
                - (geometric_product_g2.zxy() * self.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        1        0
    //    simd3        1        4        1      N/A
    // Totals...
    // yes simd        1        7        2      N/A
    //  no simd        3       15        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(-1.0 / (other[e321] * other[e321])) * other.group0().xyz();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((geometric_product_g0_xyz.zxy() * self.group0().yzx()) - (geometric_product_g0_xyz.yzx() * self.group0().zxy())).with_w(geometric_product_g0_xyz[0] * self[e1]),
            // e23, e31, e12, scalar
            (self.group0().xyz() / Simd32x4::from(other[e321]).xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Point {
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
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2] * other[e2], other[e1] * other[e1], other[e1] * other[e1], other[e2] * other[e2]]) * other.group0())
            + (other.group0() * Simd32x2::from(other[e3] * other[e3]).with_zw(other[e2] * other[e2], other[e3] * other[e3]))
            + (Simd32x4::powi(other.group0().xyzx(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * geometric_product_g0.xyz()) - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            ((geometric_product_g0.zx() * self.group0().yz()) - (geometric_product_g0.yz() * self.group0().zx()))
                .with_zw((geometric_product_g0[1] * self[e1]) - (geometric_product_g0[0] * self[e2]), geometric_product_g0[0] * self[e1]),
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
    //      f32        0        2        1        0
    //    simd3        0        1        1      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        2      N/A
    //  no simd        0        9        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar] * -1.0 / other[e321]) * (other.group0().xyz() / Simd32x4::from(other[e321]).xyz()).with_w(1.0),
        )
    }
}
impl GeometricQuotient<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       16        0        3
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       19        0      N/A
    //  no simd        8       28        0        3
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e1] * other[e1] * other[e4]]))
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
