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
//   Median:         3       6       0
//  Average:         8      14       0
//  Maximum:        88     106       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4      15       0
//  Average:        15      27       0
//  Maximum:       188     214       1
impl std::ops::Div<GeometricQuotientInfix> for AntiScalar {
    type Output = GeometricQuotientInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricQuotientInfix) -> Self::Output {
        GeometricQuotientInfixPartial(self)
    }
}
impl GeometricQuotient<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar] * 1.0 / (other[scalar] * other[scalar]))
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * self[e1234] * other[e321]),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0 * self[e1234] * -1.0) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] / other[e321])
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       14        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e321] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other_g0 * self[e1234] * other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * self[e1234] * other[e321]),
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
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * other[e321] * (-1.0 / (other[e321] * other[e321])) * -1.0)
    }
}
impl GeometricQuotient<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        8       15        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            -(Simd32x3::powi(other.group0().xyz(), 3) * Simd32x3::from(self[e1234])).with_w(0.0)
                - (Simd32x3::powi(other.group0().yxx(), 2) * Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0)
                - (Simd32x3::powi(other.group0().zzy(), 2) * Simd32x3::from(self[e1234]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
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
    //      add/sub      mul      div
    // f32        1        3        1
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
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        7       23        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g0[0],
                geometric_product_g0[1],
                geometric_product_g0[2] * self[scalar],
                (geometric_product_g0[3] * self[scalar]) - (geometric_product_g1[3] * self[e1234]),
            ]) * Simd32x2::from(self[scalar]).with_zw(1.0, 1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz()) - (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz()))
                .with_w(geometric_product_g1[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        1
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
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        5       14        0
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
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        7       27        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other_g0 * self[scalar] * other[e41] * -1.0,
                other_g0 * self[scalar] * other[e42] * -1.0,
                other_g0 * self[scalar] * other[e43] * -1.0,
                other_g0 * self[scalar] * other[e1234],
            ]) + (geometric_product_g1 * Simd32x4::from(self[e1234])),
            // e23, e31, e12, scalar
            geometric_product_g1 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       10        0
    //    simd2        0        1        0
    //    simd3        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       15       42        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
            Simd32x4::from([
                geometric_product_g1[0],
                geometric_product_g1[1],
                geometric_product_g1[2] * self[scalar],
                (geometric_product_g1[3] * self[scalar]) - (geometric_product_g4[3] * self[e1234]),
            ]) * Simd32x2::from(self[scalar]).with_zw(1.0, 1.0),
            // e41, e42, e43
            (geometric_product_g3 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other_g0 * self[scalar]) * other.group2()),
            // e23, e31, e12
            geometric_product_g3 * Simd32x3::from(self[scalar]),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g4.xyz()) - (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()))
                .with_w(geometric_product_g4[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            other[e423] / (other[e321] * other[e321]),
            other[e431] / (other[e321] * other[e321]),
            other[e412] / (other[e321] * other[e321]),
            1.0 / other[e321],
        ]) * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e1234] * -1.0),
            // e423, e431, e412, e321
            geometric_product_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8       17        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
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
    //           add/sub      mul      div
    //      f32        1        4        1
    //    simd2        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2        7        1
    //  no simd        4       12        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        let geometric_product_g0_y = other[e1234] / (other[scalar] * other[scalar]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x2::from(geometric_product_g0_x) * self.group0().xy())
                .with_zw(geometric_product_g0_x * self[e3], (geometric_product_g0_x * self[e4]) + (geometric_product_g0_y * self[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0_x) * self.group1().xyz()) + (Simd32x3::from(geometric_product_g0_y) * self.group0().xyz()))
                .with_w(geometric_product_g0_x * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       43       62        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_product_g1.zxyz() * self.group0().yzxz())
                + (self.group0().ww().with_zw(self[e431], geometric_product_g1[0] * self[e1]) * geometric_product_g0.xyx().with_w(1.0))
                + (self.group1().zx().with_zw(self[e4], geometric_product_g1[1] * self[e2]) * geometric_product_g0.yzz().with_w(1.0))
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g0.wwwy() * self.group0().xyz().with_w(self[e431]))
                - (self.group0().zx().with_zw(self[e321], geometric_product_g0[3] * self[e321]) * geometric_product_g1.yzz().with_w(1.0))
                - (self.group1().ww().with_zw(self[e2], geometric_product_g0[2] * self[e412]) * geometric_product_g1.xyx().with_w(1.0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[1] * self[e3]) - (geometric_product_g1[3] * self[e1]),
                -(geometric_product_g0[2] * self[e1]) - (geometric_product_g1[3] * self[e2]),
                -(geometric_product_g0[2] * self[e321]) - (geometric_product_g1[3] * self[e3]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx())
                - (self.group1().ww().with_zw(self[e2], geometric_product_g1[3] * self[e321]) * geometric_product_g0.xyx().with_w(1.0)),
        )
    }
}
impl GeometricQuotient<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
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
    //           add/sub      mul      div
    //      f32       18       28        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       30       45        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (geometric_product_g1[0] * self[e321]) + (geometric_product_g1[1] * self[e3]),
                (geometric_product_g1[1] * self[e321]) + (geometric_product_g1[2] * self[e1]),
                (geometric_product_g1[0] * self[e2]) + (geometric_product_g1[2] * self[e321]),
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ]) - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[0] * self[e321]) + (geometric_product_g0[1] * self[e3]) + (geometric_product_g1[0] * self[e4]) + (geometric_product_g1[1] * self[e412]),
                (geometric_product_g0[1] * self[e321]) + (geometric_product_g0[2] * self[e1]) + (geometric_product_g1[1] * self[e4]) + (geometric_product_g1[2] * self[e423]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g0[2] * self[e321]) + (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4]),
                geometric_product_g1[2] * self[e3] * -1.0,
            ]) - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       43       67        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g1[3] * self[e1],
                geometric_product_g1[3] * self[e2],
                geometric_product_g1[3] * self[e3],
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ]) + (geometric_product_g1.yzzw() * self.group0().zx().with_zw(self[e321], self[e4]))
                + (self.group1().ww().with_zw(self[e2], geometric_product_g0[3] * self[e321]) * geometric_product_g1.xyx().with_w(1.0))
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e3])
                    + (geometric_product_g0[3] * self[e1])
                    + (geometric_product_g1[0] * self[e4])
                    + (geometric_product_g1[1] * self[e412])
                    + (geometric_product_g1[3] * self[e423]),
                (geometric_product_g0[2] * self[e1])
                    + (geometric_product_g0[3] * self[e2])
                    + (geometric_product_g1[1] * self[e4])
                    + (geometric_product_g1[2] * self[e423])
                    + (geometric_product_g1[3] * self[e431]),
                (geometric_product_g0[2] * self[e321])
                    + (geometric_product_g0[3] * self[e3])
                    + (geometric_product_g1[0] * self[e431])
                    + (geometric_product_g1[2] * self[e4])
                    + (geometric_product_g1[3] * self[e412]),
                geometric_product_g1[2] * self[e3] * -1.0,
            ]) + (self.group1().ww().with_zw(self[e2], geometric_product_g1[3] * self[e321]) * geometric_product_g0.xyx().with_w(1.0))
                - (geometric_product_g1.zxyy() * self.group1().yzx().with_w(self[e2]))
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       34        0
    //    simd2        4        5        0
    //    simd3       10       15        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       46       61        0
    //  no simd       88      117        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
                (geometric_product_g4[3] * self[e4]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]) - (geometric_product_g1[3] * self[e321]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]]))
                - (Simd32x2::from([geometric_product_g4[3], geometric_product_g1[0]]) * self.group1().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g3[1] * self[e3],
                geometric_product_g3[2] * self[e1],
                geometric_product_g3[2] * self[e321],
                -(geometric_product_g2[1] * self[e2])
                    - (geometric_product_g2[2] * self[e3])
                    - (geometric_product_g3[0] * self[e423])
                    - (geometric_product_g3[1] * self[e431])
                    - (geometric_product_g3[2] * self[e412]),
            ]) + (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (self.group1().ww().with_zw(self[e2], geometric_product_g0[1] * self[e321]) * geometric_product_g3.xyx().with_w(1.0))
                - (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz())
                + (geometric_product_g4.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g1.zxy() * self.group1().yzx())
                - (geometric_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g4.yzz() * self.group0().zx().with_z(self[e321])),
            // e23, e31, e12
            (geometric_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g1.yzz() * self.group0().zx().with_z(self[e321])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e1])
                    + (geometric_product_g2[0] * self[e321])
                    + (geometric_product_g2[1] * self[e3])
                    + (geometric_product_g3[0] * self[e4])
                    + (geometric_product_g3[1] * self[e412]),
                (geometric_product_g0[1] * self[e2])
                    + (geometric_product_g2[1] * self[e321])
                    + (geometric_product_g2[2] * self[e1])
                    + (geometric_product_g3[1] * self[e4])
                    + (geometric_product_g3[2] * self[e423]),
                (geometric_product_g0[1] * self[e3])
                    + (geometric_product_g2[0] * self[e2])
                    + (geometric_product_g2[2] * self[e321])
                    + (geometric_product_g3[0] * self[e431])
                    + (geometric_product_g3[2] * self[e4]),
                geometric_product_g3[2] * self[e3] * -1.0,
            ]) + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                - (self.group0().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0]))
                - (geometric_product_g3.zxy() * self.group1().yzx()).with_w(geometric_product_g3[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        1
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       15        1
    //  no simd       12       24        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e423] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_y = other[e431] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_z = other[e412] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_w = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0_x * self[e321]) + (geometric_product_g0_y * self[e3]),
                (geometric_product_g0_y * self[e321]) + (geometric_product_g0_z * self[e1]),
                (geometric_product_g0_x * self[e2]) + (geometric_product_g0_z * self[e321]),
                -(geometric_product_g0_y * self[e2]) - (geometric_product_g0_z * self[e3]),
            ]) - (Simd32x4::from(geometric_product_g0_w) * self.group1().xyz().with_w(self[e4]))
                - (Simd32x4::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y, geometric_product_g0_x]) * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0_w) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       28       37        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e412]),
                (geometric_product_g0[1] * self[e4]) + (geometric_product_g0[2] * self[e423]),
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]),
                -(geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]) - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g0.wwwy() * self.group0().xyz().with_w(self[e431])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[0] * self[e321]) - (geometric_product_g0[1] * self[e3]),
                -(geometric_product_g0[1] * self[e321]) - (geometric_product_g0[2] * self[e1]),
                -(geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx()),
        )
    }
}
impl GeometricQuotient<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
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
    //      add/sub      mul      div
    // f32        0        1        1
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
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       19        0
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
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] / other[e321])
    }
}
impl GeometricQuotient<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       10        0
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       24        0
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
    //           add/sub      mul      div
    //      f32        7        6        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       16        0
    //  no simd        7       38        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e321] * -1.0 / (other[e321] * other[e321])) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] * -1.0 / other[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       15        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
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
    //      add/sub      mul      div
    // f32        0        0        1
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        1        3        0
    // Totals...
    // yes simd        1        3        1
    //  no simd        3        9        1
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
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       35       45        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((geometric_product_g0[2] * self[e43]) - (geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(geometric_product_g0[1] * self[e42])
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[0] * self[e23]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[2] * self[e42]) + (geometric_product_g0[3] * self[e23]) + (geometric_product_g1[1] * self[e12]),
                (geometric_product_g0[0] * self[e43]) + (geometric_product_g0[3] * self[e31]) + (geometric_product_g1[2] * self[e23]),
                (geometric_product_g0[1] * self[e41]) + (geometric_product_g0[3] * self[e12]) + (geometric_product_g1[0] * self[e31]),
                0.0,
            ]) - (geometric_product_g0.yzxx() * self.group0().zxy().with_w(self[e23]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0()).with_w(geometric_product_g0[2] * self[e12])
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g0[1] * self[e31]),
        )
    }
}
impl GeometricQuotient<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        8        1
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
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        0        5        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       21       35        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[1] * self[e12]) + (geometric_product_g1[1] * self[e43]),
                (geometric_product_g0[2] * self[e23]) + (geometric_product_g1[2] * self[e41]),
                (geometric_product_g0[0] * self[e31]) + (geometric_product_g1[0] * self[e42]),
                -(geometric_product_g0[2] * self[e12]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) - (geometric_product_g0.zxy() * self.group1().yzx()).with_w(geometric_product_g0[0] * self[e23])
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g0[1] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product_g1[1] * self[e12],
                geometric_product_g1[2] * self[e23],
                geometric_product_g1[0] * self[e31],
                -(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]),
            ]) - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g1[0] * self[e23]),
        )
    }
}
impl GeometricQuotient<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       25        0
    //    simd3        0        1        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       31       52        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[1] * self[e12]) + (geometric_product_g0[3] * self[e23]) + (geometric_product_g1[1] * self[e43]) + (geometric_product_g1[3] * self[e41]),
                (geometric_product_g0[2] * self[e23]) + (geometric_product_g0[3] * self[e31]) + (geometric_product_g1[2] * self[e41]) + (geometric_product_g1[3] * self[e42]),
                (geometric_product_g0[0] * self[e31]) + (geometric_product_g0[3] * self[e12]) + (geometric_product_g1[0] * self[e42]) + (geometric_product_g1[3] * self[e43]),
                -(geometric_product_g0[2] * self[e12]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) - (geometric_product_g0.zxyx() * self.group1().yzx().with_w(self[e23]))
                - (self.group0().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g0[1] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product_g1[1] * self[e12]) + (geometric_product_g1[3] * self[e23]),
                (geometric_product_g1[2] * self[e23]) + (geometric_product_g1[3] * self[e31]),
                (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[3] * self[e12]),
                -(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]),
            ]) - (geometric_product_g1.zxyx() * self.group1().yzx().with_w(self[e23])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       22        0
    //    simd2        3        4        0
    //    simd3        7       15        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       68       91        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e43]) - (geometric_product_g4[1] * self[e31]) - (geometric_product_g4[2] * self[e12]))
                + (Simd32x3::from(geometric_product_g4[3]) * self.group1()).with_w(geometric_product_g1[1] * self[e42])
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g4[0] * self[e23]),
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
            Simd32x4::from([
                (geometric_product_g1[2] * self[e42]) + (geometric_product_g1[3] * self[e23]) + (geometric_product_g4[1] * self[e12]),
                (geometric_product_g1[0] * self[e43]) + (geometric_product_g1[3] * self[e31]) + (geometric_product_g4[2] * self[e23]),
                (geometric_product_g1[1] * self[e41]) + (geometric_product_g1[3] * self[e12]) + (geometric_product_g4[0] * self[e31]),
                0.0,
            ]) - (geometric_product_g1.yzxx() * self.group0().zxy().with_w(self[e23]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0()).with_w(geometric_product_g1[2] * self[e12])
                - (self.group1().yzx() * geometric_product_g4.zxy()).with_w(geometric_product_g1[1] * self[e31]),
        )
    }
}
impl GeometricQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        1
    //    simd3        0        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       12        1
    //  no simd       10       20        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e423] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_y = other[e431] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_z = other[e412] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_w = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_w * -1.0) * self.group1())
                .with_w((geometric_product_g0_x * self[e23]) + (geometric_product_g0_y * self[e31]) + (geometric_product_g0_z * self[e12])),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0_w) * self.group0()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y]) * self.group1().yzx()).with_w(0.0)
                - (Simd32x3::from([geometric_product_g0_y, geometric_product_g0_z, geometric_product_g0_x]) * self.group1().zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       21       33        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g0[2] * self[e31] * -1.0,
                geometric_product_g0[0] * self[e12] * -1.0,
                geometric_product_g0[1] * self[e23] * -1.0,
                (geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43]),
            ]) + (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[2] * self[e42]) + (geometric_product_g0[3] * self[e23]),
                (geometric_product_g0[0] * self[e43]) + (geometric_product_g0[3] * self[e31]),
                (geometric_product_g0[1] * self[e41]) + (geometric_product_g0[3] * self[e12]),
                -(geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]),
            ]) - (geometric_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricQuotient<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        6        1
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        1        3        0
    // Totals...
    // yes simd        1        3        1
    //  no simd        4       12        1
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
    //           add/sub      mul      div
    //      f32       15       19        0
    //    simd3        0        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       23       29        0
    //  no simd       47       57        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g0.yzzy() * self.group1().zxw().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[3] * self[scalar])
                        - (geometric_product_g1[1] * self[e31])
                        - (geometric_product_g1[2] * self[e12])
                        - (geometric_product_g1[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(geometric_product_g0[2] * self[e43])
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[3] * self[e23]) + (geometric_product_g1[0] * self[scalar]) + (geometric_product_g1[1] * self[e12]) - (geometric_product_g1[3] * self[e41]),
                (geometric_product_g0[3] * self[e31]) + (geometric_product_g1[1] * self[scalar]) + (geometric_product_g1[2] * self[e23]) - (geometric_product_g1[3] * self[e42]),
                (geometric_product_g0[3] * self[e12]) + (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[2] * self[scalar]) - (geometric_product_g1[3] * self[e43]),
                0.0,
            ]) + (geometric_product_g0.zxy() * self.group0().yzx()).with_w(geometric_product_g1[3] * self[scalar])
                - (geometric_product_g0.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g0.yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * geometric_product_g1.zxy().with_w(geometric_product_g0[2])),
        )
    }
}
impl GeometricQuotient<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       16        1
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
    //           add/sub      mul      div
    //      f32       18       27        0
    //    simd3        0        3        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       30       44        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[0] * self[scalar]) + (geometric_product_g0[1] * self[e12]) + (geometric_product_g1[0] * self[e1234]) + (geometric_product_g1[1] * self[e43]),
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g0[2] * self[e23]) + (geometric_product_g1[1] * self[e1234]) + (geometric_product_g1[2] * self[e41]),
                (geometric_product_g0[0] * self[e31]) + (geometric_product_g0[2] * self[scalar]) + (geometric_product_g1[0] * self[e42]) + (geometric_product_g1[2] * self[e1234]),
                -(geometric_product_g0[2] * self[e12]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g0[0]))
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g0[1] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product_g1[0] * self[scalar]) + (geometric_product_g1[1] * self[e12]),
                (geometric_product_g1[1] * self[scalar]) + (geometric_product_g1[2] * self[e23]),
                (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[2] * self[scalar]),
                -(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]),
            ]) - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       26        0
    //    simd3        0        2        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       43       64        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[3] * self[e23]) + (geometric_product_g1[0] * self[e1234]) + (geometric_product_g1[1] * self[e43]) + (geometric_product_g1[3] * self[e41]),
                (geometric_product_g0[3] * self[e31]) + (geometric_product_g1[1] * self[e1234]) + (geometric_product_g1[2] * self[e41]) + (geometric_product_g1[3] * self[e42]),
                (geometric_product_g0[3] * self[e12]) + (geometric_product_g1[0] * self[e42]) + (geometric_product_g1[2] * self[e1234]) + (geometric_product_g1[3] * self[e43]),
                -(geometric_product_g0[2] * self[e12]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) + (geometric_product_g0.xyxw() * self.group1().wwyw())
                + (geometric_product_g0.yzz() * self.group1().zxw()).with_w(geometric_product_g1[3] * self[e1234])
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g0[1] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product_g1[1] * self[e12]) + (geometric_product_g1[3] * self[e23]),
                (geometric_product_g1[2] * self[e23]) + (geometric_product_g1[3] * self[e31]),
                (geometric_product_g1[2] * self[scalar]) + (geometric_product_g1[3] * self[e12]),
                -(geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]),
            ]) + (geometric_product_g1.xyxw() * self.group1().wwyw())
                - (geometric_product_g1.zxyx() * self.group1().yzxx()),
        )
    }
}
impl GeometricQuotient<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       25        0
    //    simd2        4        5        0
    //    simd3       10       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       44       54        0
    //  no simd       92      115        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
                (geometric_product_g0[1] * self[scalar]) - (geometric_product_g3[0] * self[e41]) - (geometric_product_g3[1] * self[e42]) - (geometric_product_g3[2] * self[e43]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g1.yzzy() * self.group1().zxw().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[3] * self[scalar])
                        - (geometric_product_g4[1] * self[e31])
                        - (geometric_product_g4[2] * self[e12])
                        - (geometric_product_g4[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz()).with_w(geometric_product_g1[2] * self[e43])
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
            Simd32x4::from([
                (geometric_product_g1[3] * self[e23]) + (geometric_product_g4[0] * self[scalar]) + (geometric_product_g4[1] * self[e12]) - (geometric_product_g4[3] * self[e41]),
                (geometric_product_g1[3] * self[e31]) + (geometric_product_g4[1] * self[scalar]) + (geometric_product_g4[2] * self[e23]) - (geometric_product_g4[3] * self[e42]),
                (geometric_product_g1[3] * self[e12]) + (geometric_product_g4[0] * self[e31]) + (geometric_product_g4[2] * self[scalar]) - (geometric_product_g4[3] * self[e43]),
                0.0,
            ]) + (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g4[3] * self[scalar])
                - (geometric_product_g1.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g1.yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * geometric_product_g4.zxy().with_w(geometric_product_g1[2])),
        )
    }
}
impl GeometricQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       13        1
    //    simd2        0        1        0
    //    simd3        3        4        0
    // Totals...
    // yes simd        6       18        1
    //  no simd       12       27        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e423] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_y = other[e431] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_z = other[e412] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_w = -1.0 / other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x2::from(geometric_product_g0_w * -1.0) * self.group1().xy()).with_zw(
                geometric_product_g0_w * self[e12] * -1.0,
                (geometric_product_g0_w * self[e1234]) + (geometric_product_g0_x * self[e23]) + (geometric_product_g0_y * self[e31]) + (geometric_product_g0_z * self[e12]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0_w) * self.group0().xyz())
                + (Simd32x3::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y]) * self.group1().yzx())
                - (Simd32x3::from([geometric_product_g0_x, geometric_product_g0_y, geometric_product_g0_x]) * self.group1().wwy())
                - (Simd32x3::from([geometric_product_g0_y, geometric_product_g0_z, geometric_product_g0_z]) * self.group1().zxw()))
            .with_w(geometric_product_g0_w * self[scalar] * -1.0),
        )
    }
}
impl GeometricQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       17        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       28       41        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g0[2] * self[e31] * -1.0,
                geometric_product_g0[0] * self[e12] * -1.0,
                geometric_product_g0[1] * self[e23] * -1.0,
                (geometric_product_g0[2] * self[e43]) + (geometric_product_g0[3] * self[scalar]),
            ]) + (geometric_product_g0.xyxx() * self.group1().wwy().with_w(self[e41]))
                + (geometric_product_g0.yzzy() * self.group1().zxw().with_w(self[e42])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[2] * self[e42]) + (geometric_product_g0[3] * self[e23]),
                (geometric_product_g0[0] * self[e43]) + (geometric_product_g0[3] * self[e31]),
                (geometric_product_g0[1] * self[e41]) + (geometric_product_g0[3] * self[e12]),
                geometric_product_g0[2] * self[e12] * -1.0,
            ]) - (geometric_product_g0.xyxx() * self.group0().wwy().with_w(self[e23]))
                - (geometric_product_g0.yzzy() * self.group0().zxw().with_w(self[e31])),
        )
    }
}
impl GeometricQuotient<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        8        1
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
    //           add/sub      mul      div
    //      f32        2        7        1
    //    simd2        0        1        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        4       13        1
    //  no simd        8       24        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = 1.0 / other[scalar];
        let geometric_product_g0_y = other[e1234] / (other[scalar] * other[scalar]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([geometric_product_g0_x * self[scalar], (geometric_product_g0_x * self[e1234]) + (geometric_product_g0_y * self[scalar])]),
            // e1, e2, e3, e4
            (Simd32x2::from(geometric_product_g0_x) * self.group1().xy())
                .with_zw(geometric_product_g0_x * self[e3], (geometric_product_g0_x * self[e4]) + (geometric_product_g0_y * self[e321])),
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
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd2        4        4        0
    //    simd3       10       16        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       40       51        0
    //  no simd       88      105        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * self[e4]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]) + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g0[0], geometric_product_g1[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g0[1], geometric_product_g1[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g0[2], geometric_product_g1[2]]))
                - (Simd32x2::from([geometric_product_g1[3], geometric_product_g0[0]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (geometric_product_g0.xyxx() * Simd32x2::from(self[scalar]).with_zw(self[e31], self[e41]))
                + (geometric_product_g0.yzzy() * self.group3().zx().with_zw(self[scalar], self[e42]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[3] * self[scalar])
                        - (geometric_product_g1[1] * self[e31])
                        - (geometric_product_g1[2] * self[e12])
                        - (geometric_product_g1[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g1[3]) * self.group3()).with_w(geometric_product_g0[2] * self[e43])
                - (self.group3().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[0] * self[e23]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                + (geometric_product_g0.yzz() * self.group4().zx().with_z(self[e4]))
                + (geometric_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (geometric_product_g0.zxy() * self.group4().yzx())
                - (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e321])),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e321])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[3] * self[e23]) + (geometric_product_g1[0] * self[scalar]) + (geometric_product_g1[1] * self[e12]) - (geometric_product_g1[3] * self[e41]),
                (geometric_product_g0[3] * self[e31]) + (geometric_product_g1[1] * self[scalar]) + (geometric_product_g1[2] * self[e23]) - (geometric_product_g1[3] * self[e42]),
                (geometric_product_g0[3] * self[e12]) + (geometric_product_g1[0] * self[e31]) + (geometric_product_g1[2] * self[scalar]) - (geometric_product_g1[3] * self[e43]),
                0.0,
            ]) + (self.group2().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[3] * self[scalar])
                - (geometric_product_g0.xyxx() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e23]))
                - (geometric_product_g0.yzzy() * self.group2().zx().with_zw(self[e1234], self[e31]))
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g0[2] * self[e12]),
        )
    }
}
impl GeometricQuotient<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       10        1
    //  no simd        0       27        1
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
    //           add/sub      mul      div
    //      f32       20       31        0
    //    simd2        3        3        0
    //    simd3        7       12        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       59       81        0
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
            Simd32x4::from([
                (geometric_product_g1[0] * self[e321]) + (geometric_product_g1[1] * self[e3]),
                (geometric_product_g1[1] * self[e321]) + (geometric_product_g1[2] * self[e1]),
                (geometric_product_g1[0] * self[e2]) + (geometric_product_g1[2] * self[e321]),
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ]) - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e41, e42, e43
            (geometric_product_g0.xyx() * Simd32x2::from(self[scalar]).with_z(self[e31]))
                + (geometric_product_g0.yzz() * self.group3().zx().with_z(self[scalar]))
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e1234]).with_z(self[e42]))
                + (geometric_product_g1.yzz() * self.group2().zx().with_z(self[e1234]))
                - (geometric_product_g0.zxy() * self.group3().yzx())
                - (geometric_product_g1.zxy() * self.group2().yzx()),
            // e23, e31, e12
            (geometric_product_g1.xyx() * Simd32x2::from(self[scalar]).with_z(self[e31])) + (geometric_product_g1.yzz() * self.group3().zx().with_z(self[scalar]))
                - (geometric_product_g1.zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[0] * self[e321]) + (geometric_product_g0[1] * self[e3]) + (geometric_product_g1[0] * self[e4]) + (geometric_product_g1[1] * self[e412]),
                (geometric_product_g0[1] * self[e321]) + (geometric_product_g0[2] * self[e1]) + (geometric_product_g1[1] * self[e4]) + (geometric_product_g1[2] * self[e423]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g0[2] * self[e321]) + (geometric_product_g1[0] * self[e431]) + (geometric_product_g1[2] * self[e4]),
                geometric_product_g1[2] * self[e3] * -1.0,
            ]) - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group4().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       34        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       42       60        0
    //  no simd       84      118        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * self[e1234]) - (geometric_product_g1[0] * self[e41]) - (geometric_product_g1[1] * self[e42]) - (geometric_product_g1[2] * self[e43]),
            ]) + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g1[3] * self[e1],
                geometric_product_g1[3] * self[e2],
                geometric_product_g1[3] * self[e3],
                -(geometric_product_g0[1] * self[e2])
                    - (geometric_product_g0[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412]),
            ]) + (geometric_product_g1.yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + (self.group4().ww().with_zw(self[e2], geometric_product_g0[3] * self[e321]) * geometric_product_g1.xyx().with_w(1.0))
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group3())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group2())
                + (Simd32x3::from([geometric_product_g0[0], geometric_product_g0[1], geometric_product_g0[0] * self[e31]]) * Simd32x2::from(self[scalar]).with_z(1.0))
                + (Simd32x3::from([geometric_product_g1[0], geometric_product_g1[1], geometric_product_g1[0] * self[e42]]) * Simd32x2::from(self[e1234]).with_z(1.0))
                + (geometric_product_g0.yzz() * self.group3().zx().with_z(self[scalar]))
                + (geometric_product_g1.yzz() * self.group2().zx().with_z(self[e1234]))
                - (self.group2().yzx() * geometric_product_g1.zxy())
                - (self.group3().yzx() * geometric_product_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g1[3]) * self.group3())
                + (Simd32x3::from([geometric_product_g1[0], geometric_product_g1[1], geometric_product_g1[0] * self[e31]]) * Simd32x2::from(self[scalar]).with_z(1.0))
                + (geometric_product_g1.yzz() * self.group3().zx().with_z(self[scalar]))
                - (self.group3().yzx() * geometric_product_g1.zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e3])
                    + (geometric_product_g0[3] * self[e1])
                    + (geometric_product_g1[0] * self[e4])
                    + (geometric_product_g1[1] * self[e412])
                    + (geometric_product_g1[3] * self[e423]),
                (geometric_product_g0[2] * self[e1])
                    + (geometric_product_g0[3] * self[e2])
                    + (geometric_product_g1[1] * self[e4])
                    + (geometric_product_g1[2] * self[e423])
                    + (geometric_product_g1[3] * self[e431]),
                (geometric_product_g0[2] * self[e321])
                    + (geometric_product_g0[3] * self[e3])
                    + (geometric_product_g1[0] * self[e431])
                    + (geometric_product_g1[2] * self[e4])
                    + (geometric_product_g1[3] * self[e412]),
                geometric_product_g1[2] * self[e3] * -1.0,
            ]) + (self.group4().ww().with_zw(self[e2], geometric_product_g1[3] * self[e321]) * geometric_product_g0.xyx().with_w(1.0))
                - (geometric_product_g1.zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       53        0
    //    simd2        8       10        0
    //    simd3       22       31        0
    //    simd4       16       12        0
    // Totals...
    // yes simd       88      106        0
    //  no simd      188      214        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([geometric_product_g4[3], geometric_product_g1[0]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (geometric_product_g1.xyxy() * Simd32x2::from(self[scalar]).with_zw(self[e31], self[e42]))
                + (geometric_product_g1.yzzz() * self.group3().zx().with_zw(self[scalar], self[e43]))
                + (self.group1().zx().with_zw(self[e321], geometric_product_g1[0] * self[e41]) * geometric_product_g3.yzz().with_w(1.0))
                + (self.group4().ww().with_zw(self[e2], geometric_product_g0[1] * self[e321]) * geometric_product_g3.xyx().with_w(1.0))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * self[e423])
                        - (geometric_product_g3[1] * self[e431])
                        - (geometric_product_g3[2] * self[e412])
                        - (geometric_product_g4[0] * self[e23])
                        - (geometric_product_g4[1] * self[e31])
                        - (geometric_product_g4[2] * self[e12])
                        - (geometric_product_g4[3] * self[e1234]),
                )
                + (Simd32x3::from(geometric_product_g4[3]) * self.group3()).with_w(geometric_product_g1[3] * self[scalar])
                - (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g2[1] * self[e2]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_product_g4[3]) * self.group4().xyz())
                + (geometric_product_g2.xyx() * Simd32x2::from(self[scalar]).with_z(self[e31]))
                + (geometric_product_g2.yzz() * self.group3().zx().with_z(self[scalar]))
                + (geometric_product_g3.xyx() * Simd32x2::from(self[e1234]).with_z(self[e42]))
                + (geometric_product_g3.yzz() * self.group2().zx().with_z(self[e1234]))
                + (geometric_product_g1.xyx() * Simd32x2::from(self[e4]).with_z(self[e431]))
                + (geometric_product_g1.yzz() * self.group4().zx().with_z(self[e4]))
                + (geometric_product_g4.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (geometric_product_g2.zxy() * self.group3().yzx())
                - (geometric_product_g3.zxy() * self.group2().yzx())
                - (geometric_product_g1.zxy() * self.group4().yzx())
                - (geometric_product_g4.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g4.yzz() * self.group1().zx().with_z(self[e321])),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g0[0]) * self.group3())
                + (geometric_product_g3.xyx() * Simd32x2::from(self[scalar]).with_z(self[e31]))
                + (geometric_product_g3.yzz() * self.group3().zx().with_z(self[scalar]))
                + (geometric_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (geometric_product_g3.zxy() * self.group3().yzx())
                - (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g1.yzz() * self.group1().zx().with_z(self[e321])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g2[0] * self[e321])
                    + (geometric_product_g2[1] * self[e3])
                    + (geometric_product_g3[0] * self[e4])
                    + (geometric_product_g3[1] * self[e412])
                    + (geometric_product_g1[2] * self[e42])
                    + (geometric_product_g1[3] * self[e23])
                    + (geometric_product_g4[0] * self[scalar])
                    + (geometric_product_g4[1] * self[e12]),
                (geometric_product_g2[1] * self[e321])
                    + (geometric_product_g2[2] * self[e1])
                    + (geometric_product_g3[1] * self[e4])
                    + (geometric_product_g3[2] * self[e423])
                    + (geometric_product_g1[0] * self[e43])
                    + (geometric_product_g1[3] * self[e31])
                    + (geometric_product_g4[1] * self[scalar])
                    + (geometric_product_g4[2] * self[e23]),
                (geometric_product_g2[0] * self[e2])
                    + (geometric_product_g2[2] * self[e321])
                    + (geometric_product_g3[0] * self[e431])
                    + (geometric_product_g3[2] * self[e4])
                    + (geometric_product_g1[1] * self[e41])
                    + (geometric_product_g1[3] * self[e12])
                    + (geometric_product_g4[0] * self[e31])
                    + (geometric_product_g4[2] * self[scalar]),
                0.0,
            ]) + (Simd32x4::from(geometric_product_g0[0]) * self.group4())
                + (Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g4[3] * self[scalar]) * self.group1().xyz().with_w(1.0))
                - (geometric_product_g1.yzzx() * self.group2().zx().with_zw(self[e1234], self[e23]))
                - (self.group1().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0]))
                - (Simd32x2::from(self[e1234]) * geometric_product_g1.xy()).with_zw(geometric_product_g1[0] * self[e42], geometric_product_g3[2] * self[e3])
                - (Simd32x3::from(geometric_product_g4[3]) * self.group2()).with_w(geometric_product_g1[2] * self[e12])
                - (geometric_product_g3.zxy() * self.group4().yzx()).with_w(geometric_product_g3[1] * self[e2])
                - (self.group3().yzx() * geometric_product_g4.zxy()).with_w(geometric_product_g1[1] * self[e31]),
        )
    }
}
impl GeometricQuotient<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        1
    //    simd3        6       10        0
    // Totals...
    // yes simd       12       28        1
    //  no simd       24       48        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e423] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_y = other[e431] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_z = other[e412] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_w = -1.0 / other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0_w * self[e321],
                -(geometric_product_g0_w * self[e4]) - (geometric_product_g0_x * self[e1]) - (geometric_product_g0_y * self[e2]) - (geometric_product_g0_z * self[e3]),
            ]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0_w * -1.0) * self.group3()).with_w(
                (geometric_product_g0_w * self[e1234]) + (geometric_product_g0_x * self[e23]) + (geometric_product_g0_y * self[e31]) + (geometric_product_g0_z * self[e12]),
            ),
            // e41, e42, e43
            (Simd32x3::from(self[e321]) * Simd32x3::from([geometric_product_g0_x, geometric_product_g0_y, geometric_product_g0_z]))
                + (Simd32x3::from([geometric_product_g0_y, geometric_product_g0_z, geometric_product_g0_x]) * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g0_w) * self.group4().xyz())
                - (Simd32x3::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y]) * self.group1().yzx()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0_w) * self.group1().xyz(),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0_w) * self.group2())
                + (Simd32x3::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y]) * self.group3().yzx())
                - (Simd32x3::from([geometric_product_g0_x, geometric_product_g0_y, geometric_product_g0_x * self[e31]]) * Simd32x2::from(self[scalar]).with_z(1.0))
                - (Simd32x3::from([geometric_product_g0_y, geometric_product_g0_z, geometric_product_g0_z * self[scalar]]) * self.group3().zx().with_z(1.0)))
            .with_w(geometric_product_g0_w * self[scalar] * -1.0),
        )
    }
}
impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd3        5        7        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       20       37        0
    //  no simd       48       69        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
                -(geometric_product_g0[0] * self[e423]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g0[2] * self[e31] * -1.0,
                geometric_product_g0[0] * self[e12] * -1.0,
                geometric_product_g0[1] * self[e23] * -1.0,
                (geometric_product_g0[2] * self[e43]) + (geometric_product_g0[3] * self[scalar]),
            ]) + (geometric_product_g0.xyxx() * Simd32x2::from(self[scalar]).with_zw(self[e31], self[e41]))
                + (geometric_product_g0.yzzy() * self.group3().zx().with_zw(self[scalar], self[e42])),
            // e41, e42, e43
            (geometric_product_g0.xyx() * Simd32x2::from(self[e4]).with_z(self[e431])) + (geometric_product_g0.yzz() * self.group4().zx().with_z(self[e4]))
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (geometric_product_g0.zxy() * self.group4().yzx()),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx())
                - (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2]))
                - (geometric_product_g0.yzz() * self.group1().zx().with_z(self[e321])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[2] * self[e42]) + (geometric_product_g0[3] * self[e23]),
                (geometric_product_g0[0] * self[e43]) + (geometric_product_g0[3] * self[e31]),
                (geometric_product_g0[1] * self[e41]) + (geometric_product_g0[3] * self[e12]),
                geometric_product_g0[2] * self[e12] * -1.0,
            ]) - (geometric_product_g0.xyxx() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e23]))
                - (geometric_product_g0.yzzy() * self.group2().zx().with_zw(self[e1234], self[e31])),
        )
    }
}
impl GeometricQuotient<Scalar> for MultiVector {
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
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * self[e4] * 1.0 / (other[scalar] * other[scalar]))
    }
}
impl GeometricQuotient<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       10        0
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
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * -1.0 / other[e321])
    }
}
impl GeometricQuotient<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
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
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
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
    //           add/sub      mul      div
    //      f32        7        7        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       15        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e321] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other_g0 * other[e321] * self[e4]]) * Simd32x2::from([0.0, -1.0]),
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
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e321] * (-1.0 / (other[e321] * other[e321])))
    }
}
impl GeometricQuotient<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        5        0
    // no simd        6       15        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::powi(other.group0().xyz(), 3) * Simd32x3::from(self[e4]))
                + (Simd32x3::powi(other.group0().yxx(), 2) * Simd32x3::from(self[e4]) * other.group0().xyz())
                + (Simd32x3::powi(other.group0().zzy(), 2) * Simd32x3::from(self[e4]) * other.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
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
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
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
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       30        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[1] * self[e412]) + (geometric_product_g1[3] * self[e423]),
                (geometric_product_g0[2] * self[e423]) + (geometric_product_g1[3] * self[e431]),
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g1[3] * self[e412]),
                -(geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]) - (geometric_product_g0.zxyx() * self.group0().yzxx())
                - (self.group0().wwwy() * geometric_product_g1.xyz().with_w(geometric_product_g0[1])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]),
        )
    }
}
impl GeometricQuotient<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        6        1
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
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd3        0        5        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       20        0
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
    //           add/sub      mul      div
    //      f32        6        5        0
    //    simd3        3        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       36        0
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
                + (geometric_product_g1.yzx() * self.group0().zxy())
                - (geometric_product_g1.zxy() * self.group0().yzx()))
            .with_w(geometric_product_g1[3] * self[e321]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       14        0
    //    simd2        0        2        0
    //    simd3        6       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       31       59        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
            + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x2::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1();
        let geometric_product_g3 = Simd32x3::from(other_g0 * -1.0) * other.group3();
        let geometric_product_g4 = Simd32x4::from(other_g0 * -1.0) * other.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g4[3] * self[e321],
                -(geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412]) - (geometric_product_g1[3] * self[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (geometric_product_g3 * Simd32x3::from(self[e321])).with_w(
                (geometric_product_g0[1] * self[e321]) - (geometric_product_g3[0] * self[e423]) - (geometric_product_g3[1] * self[e431]) - (geometric_product_g3[2] * self[e412]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()) + (geometric_product_g1.yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz())
                - (geometric_product_g1.zxy() * self.group0().yzx()),
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
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        4        1
    //  no simd        3        8        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_w = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x3::from(geometric_product_g0_w) * self.group0().xyz()) - (Simd32x3::from(self[e321] / (other[e321] * other[e321])) * other.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0_w * self[e321]),
        )
    }
}
impl GeometricQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       14       23        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_product_g0[1] * self[e412],
                geometric_product_g0[2] * self[e423],
                geometric_product_g0[0] * self[e431],
                -(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]) - (geometric_product_g0.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321] * -1.0) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(1.0 / other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e1234] / (other[scalar] * other[scalar])) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       23       37        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0 * -1.0) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_product_g0[3] * self[e1]) - (geometric_product_g1[1] * self[e3]),
                -(geometric_product_g0[3] * self[e2]) - (geometric_product_g1[2] * self[e1]),
                -(geometric_product_g0[3] * self[e3]) - (geometric_product_g1[0] * self[e2]),
                (geometric_product_g1[2] * self[e3]) + (geometric_product_g1[3] * self[e4]),
            ]) + (geometric_product_g1.zxyy() * self.group0().yzxy())
                + (self.group0().wwwx() * geometric_product_g0.xyz().with_w(geometric_product_g1[0])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[1] * self[e3]) - (geometric_product_g1[3] * self[e1]),
                -(geometric_product_g0[2] * self[e1]) - (geometric_product_g1[3] * self[e2]),
                -(geometric_product_g0[0] * self[e2]) - (geometric_product_g1[3] * self[e3]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx()),
        )
    }
}
impl GeometricQuotient<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        6        1
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
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       29        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0 * -1.0) * other.group0();
        let geometric_product_g1 = Simd32x3::from(other_g0 * -1.0) * other.group1();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g1[1] * self[e3],
                geometric_product_g1[2] * self[e1],
                geometric_product_g1[0] * self[e2],
                -(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e3]) + (geometric_product_g1[0] * self[e4]),
                (geometric_product_g0[2] * self[e1]) + (geometric_product_g1[1] * self[e4]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g1[2] * self[e4]),
                -(geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       23       44        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                geometric_product_g1[3] * self[e1],
                geometric_product_g1[3] * self[e2],
                geometric_product_g1[3] * self[e3],
                -(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g1.yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e3]) + (geometric_product_g0[3] * self[e1]) + (geometric_product_g1[0] * self[e4]),
                (geometric_product_g0[2] * self[e1]) + (geometric_product_g0[3] * self[e2]) + (geometric_product_g1[1] * self[e4]),
                (geometric_product_g0[0] * self[e2]) + (geometric_product_g0[3] * self[e3]) + (geometric_product_g1[2] * self[e4]),
                -(geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd2        3        4        0
    //    simd3        5        9        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       48       75        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
            Simd32x4::from([
                geometric_product_g3[1] * self[e3],
                geometric_product_g3[2] * self[e1],
                geometric_product_g3[0] * self[e2],
                -(geometric_product_g2[1] * self[e2]) - (geometric_product_g2[2] * self[e3]),
            ]) + (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                - (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product_g1.xyz()) + (geometric_product_g4.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g4.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (geometric_product_g1.yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (geometric_product_g0[1] * self[e1]) + (geometric_product_g2[1] * self[e3]) + (geometric_product_g3[0] * self[e4]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g2[2] * self[e1]) + (geometric_product_g3[1] * self[e4]),
                (geometric_product_g0[1] * self[e3]) + (geometric_product_g2[0] * self[e2]) + (geometric_product_g3[2] * self[e4]),
                -(geometric_product_g3[1] * self[e2]) - (geometric_product_g3[2] * self[e3]),
            ]) - (self.group0().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0])),
        )
    }
}
impl GeometricQuotient<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        1
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       12        1
    //  no simd        6       17        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e423] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_y = other[e431] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_z = other[e412] * -1.0 / (other[e321] * other[e321]);
        let geometric_product_g0_w = -1.0 / other[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                geometric_product_g0_y * self[e3],
                geometric_product_g0_z * self[e1],
                geometric_product_g0_x * self[e2],
                -(geometric_product_g0_x * self[e1]) - (geometric_product_g0_y * self[e2]) - (geometric_product_g0_z * self[e3]),
            ]) - (Simd32x4::from([geometric_product_g0_z, geometric_product_g0_x, geometric_product_g0_y, geometric_product_g0_w]) * self.group0().yzxw()),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_product_g0_w) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       16       27        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e3] * other[e3] * other[e4]])
            + (Simd32x4::powi(other.group0().yxxx(), 2) * other.group0())
            + (Simd32x4::powi(other.group0().zzyy(), 2) * other.group0());
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * geometric_product_g0.xyz()) - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                geometric_product_g0[1] * self[e3] * -1.0,
                geometric_product_g0[2] * self[e1] * -1.0,
                geometric_product_g0[0] * self[e2] * -1.0,
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx()),
        )
    }
}
impl GeometricQuotient<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
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
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * Simd32x2::from([1.0 / other[scalar], other[e1234] / (other[scalar] * other[scalar])]),
        )
    }
}
impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       11        0
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
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * -1.0 / other[e321])
    }
}
impl GeometricQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       10        0
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
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       18        0
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       24        0
    fn geometric_quotient(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[scalar] * other[scalar]
            + other[e1] * other[e1]
            + other[e2] * other[e2]
            + other[e3] * other[e3]
            + other[e23] * other[e23]
            + other[e31] * other[e31]
            + other[e12] * other[e12]
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
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        1
    //  no simd        0        5        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar] * -1.0)
                * Simd32x4::from([
                    other[e423] / (other[e321] * other[e321]),
                    other[e431] / (other[e321] * other[e321]),
                    other[e412] / (other[e321] * other[e321]),
                    1.0 / other[e321],
                ]),
        )
    }
}
impl GeometricQuotient<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8       21        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * Simd32x4::from([f32::powi(other[e1], 3), f32::powi(other[e2], 3), f32::powi(other[e3], 3), other[e1] * other[e1] * other[e4]]))
                + (Simd32x4::powi(other.group0().yxxy(), 2) * Simd32x4::from(self[scalar]) * other.group0())
                + (Simd32x4::powi(other.group0().zzyz(), 2) * Simd32x4::from(self[scalar]) * other.group0()),
        )
    }
}
impl GeometricQuotient<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_quotient(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[scalar] / other[scalar])
    }
}
