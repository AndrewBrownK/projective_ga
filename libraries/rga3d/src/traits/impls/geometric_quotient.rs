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
//   Median:         3      19       0
//  Average:        15      29       0
//  Maximum:       195     218       1
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
    // f32        0        0        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] / (other[scalar]))
    }
}
impl GeometricQuotient<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       12        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * self[e1234] * other[e321]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * (Simd32x4::from(other_g0) * other.group0()).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
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
        Origin::from_groups(/* e4 */ self[e1234] / (other[e321]))
    }
}
impl GeometricQuotient<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * Simd32x3::from(other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12]) * other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl GeometricQuotient<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234])
                * Simd32x4::from(other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar])
                * other.group1()
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
    //      f32        7        4        0
    //    simd2        0        1        0
    //    simd3        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       11        0
    //  no simd        7       25        0
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
            Simd32x2::from([1.0, other_g0 * self[e1234] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * self[e1234] * other[e321]),
            // e41, e42, e43
            Simd32x3::from(other_g0) * Simd32x3::from(self[e1234]) * other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * (Simd32x4::from(other_g0) * other.group1()).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] / (other[e321]))
    }
}
impl GeometricQuotient<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * Simd32x3::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w(0.0),
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
        AntiScalar::from_groups(/* e1234 */ self[e1234] / (other[scalar]))
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product_g0[0] * self[scalar],
            (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
        ]))
    }
}
impl GeometricQuotient<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        7       25        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0()
                .xx()
                .with_zw(self[scalar], (geometric_product_g0[3] * self[scalar]) - (geometric_product_g1[3] * self[e1234]))
                * geometric_product_g0.xyz().with_w(1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz()) - (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz()))
                .with_w(geometric_product_g1[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        7        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e1234]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[scalar]),
        )
    }
}
impl GeometricQuotient<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        6        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        5       18        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (geometric_product_g1 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            geometric_product_g1 * Simd32x3::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        7        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        7       28        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
                + (geometric_product_g1 * Simd32x4::from(self[e1234])),
            // e23, e31, e12, scalar
            geometric_product_g1 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<MultiVector> for DualNum {
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
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[0] * self[scalar],
                (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            self.group0()
                .xx()
                .with_zw(self[scalar], (geometric_product_g1[3] * self[scalar]) - (geometric_product_g4[3] * self[e1234]))
                * geometric_product_g1.xyz().with_w(1.0),
            // e41, e42, e43
            (geometric_product_g3 * Simd32x3::from(self[e1234])) - (Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group2()),
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
    //      f32        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       17        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e1234]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            geometric_product_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2       14        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_product_g0 * Simd32x4::from(self[scalar]),
            // e423, e431, e412, e321
            (geometric_product_g0.xyz() * self.group0().yy().with_z(self[e1234]) * Simd32x3::from(-1.0)).with_w(0.0),
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
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        4       15        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            geometric_product_g0
                .xx()
                .with_zw(geometric_product_g0[0], (geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e321]))
                * self.group0().xyz().with_w(1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group1().xyz()) + (Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz()))
                .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        9       13        0
    // Totals...
    // yes simd       16       21        0
    //  no simd       43       60        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(geometric_product_g1[3]) * self.group1().xyz().with_w(self[e4]))
                + (geometric_product_g1.zxyz() * self.group0().yzxz())
                + (self.group0().ww().with_zw(self[e431], self[e1]) * geometric_product_g0.xyx().with_w(geometric_product_g1[0]))
                + (self.group1().zx().with_zw(self[e4], self[e2]) * geometric_product_g0.yzz().with_w(geometric_product_g1[1]))
                - (geometric_product_g0.zxyx() * self.group1().yzxx())
                - (geometric_product_g0.wwwy() * self.group0().xyz().with_w(self[e431]))
                - (self.group0().zx().with_zw(self[e321], self[e321]) * geometric_product_g1.yzz().with_w(geometric_product_g0[3]))
                - (self.group1().ww().with_zw(self[e2], self[e412]) * geometric_product_g1.xyx().with_w(geometric_product_g0[2])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[1] * self[e3]) - (geometric_product_g1[3] * self[e1]),
                -(geometric_product_g0[2] * self[e1]) - (geometric_product_g1[3] * self[e2]),
                -(geometric_product_g0[2] * self[e321]) - (geometric_product_g1[3] * self[e3]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx())
                - (self.group1().ww().with_zw(self[e2], self[e321]) * geometric_product_g0.xyx().with_w(geometric_product_g1[3])),
        )
    }
}
impl GeometricQuotient<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       13        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(geometric_product_g0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricQuotient<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        3        9        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       30       49        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
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
            ((Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g0.yzz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz())
                + (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g0.xyx()))
            .with_w(geometric_product_g1[2] * self[e3] * -1.0)
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd3        4        7        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       43       66        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * geometric_product_g1.xyz().with_w(
                    -(geometric_product_g0[1] * self[e2])
                        - (geometric_product_g0[2] * self[e3])
                        - (geometric_product_g1[0] * self[e423])
                        - (geometric_product_g1[1] * self[e431])
                        - (geometric_product_g1[2] * self[e412]),
                ))
                + (geometric_product_g1.wzxw() * self.group0().xxyw())
                + (self.group0().zyz() * geometric_product_g1.yww()).with_w(geometric_product_g0[3] * self[e321])
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
            // e423, e431, e412, e321
            (self.group0().xxy() * geometric_product_g0.wzx()).with_w(geometric_product_g1[3] * self[e321])
                + ((Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                    + (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                    + (Simd32x3::from([self[e3], self[e2], self[e3]]) * geometric_product_g0.yww())
                    + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                    + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz()))
                .with_w(geometric_product_g1[2] * self[e3] * -1.0)
                - (geometric_product_g1.zxyy() * self.group1().yzx().with_w(self[e2]))
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       12        0
    //    simd2        4        5        0
    //    simd3       14       22        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       38       48        0
    //  no simd       88      124        0
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
        let geometric_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
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
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x4::from([self[e3], self[e1], self[e321], 1.0])
                    * geometric_product_g3.yzz().with_w(
                        -(geometric_product_g2[1] * self[e2])
                            - (geometric_product_g2[2] * self[e3])
                            - (geometric_product_g3[0] * self[e423])
                            - (geometric_product_g3[1] * self[e431])
                            - (geometric_product_g3[2] * self[e412]),
                    ))
                + (self.group1().ww().with_zw(self[e2], self[e321]) * geometric_product_g3.xyx().with_w(geometric_product_g0[1]))
                - (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                + (Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group1().zyz())
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group1().xxy())
                + (geometric_product_g4.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_product_g4.xyz())
                - (Simd32x3::from([geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]]) * self.group0().xxy())
                - (Simd32x3::from([geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]]) * self.group0().zyz())
                - (geometric_product_g1.zxy() * self.group1().yzx()),
            // e23, e31, e12
            (geometric_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (Simd32x3::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3]]) * self.group0().zyz())
                - (Simd32x3::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0]]) * self.group0().xxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + ((Simd32x3::from(geometric_product_g0[1]) * self.group0().xyz())
                    + (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g2.yzz())
                    + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g3.xyx())
                    + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g3.yzz())
                    + (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g2.xyx()))
                .with_w(geometric_product_g3[2] * self[e3] * -1.0)
                - (self.group0().yzxx() * geometric_product_g2.zxy().with_w(geometric_product_g3[0]))
                - (geometric_product_g3.zxy() * self.group1().yzx()).with_w(geometric_product_g3[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       32        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_product_g0[0] * self[e321]) - (geometric_product_g0[1] * self[e3]),
                -(geometric_product_g0[1] * self[e321]) - (geometric_product_g0[2] * self[e1]),
                -(geometric_product_g0[0] * self[e2]) - (geometric_product_g0[2] * self[e321]),
                (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e4]),
            ]) + (geometric_product_g0.zxyx() * self.group0().yzxx())
                + (geometric_product_g0.wwwy() * self.group1().xyz().with_w(self[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(geometric_product_g0[3]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricQuotient<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       22       32        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e412]),
                (geometric_product_g0[1] * self[e4]) + (geometric_product_g0[2] * self[e423]),
                (geometric_product_g0[0] * self[e431]) + (geometric_product_g0[2] * self[e4]),
                -(geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]) - (geometric_product_g0.zxyy() * self.group1().yzxy())
                - (geometric_product_g0.wwwx() * self.group0().xyz().with_w(self[e423])),
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
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       28        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321]) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]) * Simd32x4::from(-1.0),
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
        Scalar::from_groups(/* scalar */ self[e321] / (other[e321]))
    }
}
impl GeometricQuotient<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       18        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e321]) * other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e321]) * other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
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
    //      f32        7        0        0
    //    simd2        0        3        0
    //    simd3        0        8        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        7       16        0
    //  no simd        7       50        0
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
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * (Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0)).with_w(geometric_product_g0[1]),
            // e41, e42, e43
            Simd32x3::from(self[e321]) * geometric_product_g4.xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * geometric_product_g1.xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * (Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0)).with_w(geometric_product_g0[0]),
        )
    }
}
impl GeometricQuotient<Plane> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       19        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e321]) * geometric_product_g0.xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricQuotient<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       15        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321]) * geometric_product_g0.xyz() * Simd32x3::from(-1.0)).with_w(0.0),
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
        Horizon::from_groups(/* e321 */ self[e321] / (other[scalar]))
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
    //    simd2        0        1        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       11        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32        5        5        0
    //    simd3        0        5        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       13       17        0
    //  no simd       37       48        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3], geometric_product_g0[1]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[0]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((geometric_product_g0[2] * self[e43]) - (geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]))
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[0] * self[e23]),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0[3]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                + (self.group1().zxy() * geometric_product_g1.yzx()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3], geometric_product_g0[1]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[0]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g0[2] * self[e12]),
        )
    }
}
impl GeometricQuotient<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0       10        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0) * self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd3        1        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       21       39        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
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
            ((geometric_product_g1.yzx() * self.group1().zxy()) - (geometric_product_g1.zxy() * self.group1().yzx()))
                .with_w(-(geometric_product_g1[0] * self[e23]) - (geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12])),
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
                -(geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g1[2] * self[e43]),
            ]) - (geometric_product_g1.zxyx() * self.group0().yzx().with_w(self[e41]))
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[1] * self[e42]),
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
    //      f32       11        8        0
    //    simd2        3        4        0
    //    simd3        7       18        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       70       98        0
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
        let geometric_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g2[0] * self[e23]) - (geometric_product_g2[1] * self[e31]) - (geometric_product_g2[2] * self[e12]),
            ]) - (Simd32x2::from(geometric_product_g3[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_product_g3[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_product_g3[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[0]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e43]) - (geometric_product_g4[1] * self[e31]) - (geometric_product_g4[2] * self[e12]))
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
            (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * geometric_product_g1.zxy()).with_w(0.0)
                + (self.group1().zxy() * geometric_product_g4.yzx()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[0]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * geometric_product_g4.zxy()).with_w(geometric_product_g1[2] * self[e12]),
        )
    }
}
impl GeometricQuotient<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        4        9        0
    //  no simd       10       23        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group1())
                .with_w(-(geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12])),
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
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       28        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + (self.group1().yzx() * geometric_product_g0.zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43])),
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
    //    simd2        0        1        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4       14        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        0        6        0
    //    simd4       11        9        0
    // Totals...
    // yes simd       17       21        0
    //  no simd       50       60        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
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
                + (self.group1().xyz() * geometric_product_g1.www()).with_w(geometric_product_g0[2] * self[e43])
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from([self[e12], self[e23], self[scalar]]) * geometric_product_g1.yzz()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[scalar], self[e31]]) * geometric_product_g1.xyx()).with_w(0.0)
                + (geometric_product_g0.zxy() * self.group0().yzx()).with_w(geometric_product_g1[3] * self[scalar])
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
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        1
    //  no simd        0       17        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * self.group0().xyz().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricQuotient<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        0        5        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       30       48        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
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
    //      f32       13       10        0
    //    simd2        4        5        0
    //    simd3       10       22        0
    //    simd4       11        9        0
    // Totals...
    // yes simd       38       46        0
    //  no simd       95      122        0
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
        let geometric_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
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
            (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[0]]) * self.group1().xxy().with_w(self[e41]))
                + (geometric_product_g1.xyzz() * self.group1().www().with_w(self[e43]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[3] * self[scalar])
                        - (geometric_product_g4[0] * self[e23])
                        - (geometric_product_g4[1] * self[e31])
                        - (geometric_product_g4[2] * self[e12]),
                )
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(geometric_product_g4[3] * self[e1234]),
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
            (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from([self[e12], self[e23], self[scalar]]) * geometric_product_g4.yzz()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[scalar], self[e31]]) * geometric_product_g4.xyx()).with_w(0.0)
                + (geometric_product_g1.zxy() * self.group0().yzx()).with_w(geometric_product_g4[3] * self[scalar])
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[0]]) * self.group0().xxy().with_w(self[e23]))
                - (geometric_product_g1.xyzz() * self.group0().www().with_w(self[e12]))
                - (geometric_product_g4.zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       28        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(
                -(geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g0[3] * self[e1234]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g0.xyz()) + (geometric_product_g0.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                - (geometric_product_g0.zxy() * self.group1().yzx()))
            .with_w(geometric_product_g0[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       22       36        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.xyzy() * self.group1().www().with_w(self[e42]))
                + (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + (geometric_product_g0.zxy() * self.group1().yzx() * Simd32x3::from(-1.0))
                    .with_w((geometric_product_g0[2] * self[e43]) + (geometric_product_g0[3] * self[scalar])),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()) + (geometric_product_g0.zxy() * self.group0().yzx()))
                .with_w(geometric_product_g0[2] * self[e12] * -1.0)
                - (geometric_product_g0.xyzy() * self.group0().www().with_w(self[e31]))
                - (geometric_product_g0.yzxx() * self.group0().zxy().with_w(self[e23])),
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
    //      f32        2        6        0
    //    simd2        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        8       27        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[0] * self[scalar],
                (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
            ]),
            // e1, e2, e3, e4
            geometric_product_g0
                .xx()
                .with_zw(geometric_product_g0[0], (geometric_product_g0[0] * self[e4]) + (geometric_product_g0[1] * self[e321]))
                * self.group1().xyz().with_w(1.0),
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
    //           add/sub      mul      div
    //      f32        9        9        0
    //    simd2        4        4        0
    //    simd3       10       17        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       34       40        0
    //  no simd       91      108        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
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
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3], geometric_product_g0[1]]) * self.group3().zyz().with_w(self[e42]))
                + (Simd32x4::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[0]]) * self.group3().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[2] * self[e43]) - (geometric_product_g1[0] * self[e23]) - (geometric_product_g1[1] * self[e31]) - (geometric_product_g1[2] * self[e12]),
                )
                - (self.group3().yzx() * geometric_product_g0.zxy()).with_w(geometric_product_g1[3] * self[e1234]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g0.xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g0.yzz())
                + (geometric_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g1.yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g1.xyx())
                - (geometric_product_g0.zxy() * self.group4().yzx()),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g0.yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g0.xyx()),
            // e423, e431, e412, e321
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x3::from(geometric_product_g0[3]) * self.group3()).with_w(0.0)
                + (self.group2().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                + (self.group3().zxy() * geometric_product_g1.yzx()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g0[1], geometric_product_g1[3], geometric_product_g1[3], geometric_product_g0[2]]) * self.group2().zyz().with_w(self[e12]))
                - (Simd32x4::from([geometric_product_g1[3], geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1]]) * self.group2().xxy().with_w(self[e31]))
                - (geometric_product_g0.xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        1
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        1
    //  no simd        0       30        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(geometric_product_g0) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(geometric_product_g0) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(geometric_product_g0) * self.group2().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricQuotient<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd2        3        3        0
    //    simd3       10       18        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       27       40        0
    //  no simd       59       85        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
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
            (geometric_product_g0 * Simd32x3::from(self[scalar]))
                + (geometric_product_g1 * Simd32x3::from(self[e1234]))
                + (geometric_product_g0.yzx() * self.group3().zxy())
                + (geometric_product_g1.yzx() * self.group2().zxy())
                - (geometric_product_g0.zxy() * self.group3().yzx())
                - (geometric_product_g1.zxy() * self.group2().yzx()),
            // e23, e31, e12
            (geometric_product_g1 * Simd32x3::from(self[scalar])) + (geometric_product_g1.yzx() * self.group3().zxy()) - (geometric_product_g1.zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            ((Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g0.yzz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz())
                + (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g0.xyx()))
            .with_w(geometric_product_g1[2] * self[e3] * -1.0)
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.zxy() * self.group4().yzx()).with_w(geometric_product_g1[1] * self[e2]),
        )
    }
}
impl GeometricQuotient<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       11        0
    //    simd2        4        4        0
    //    simd3       14       18        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       34       43        0
    //  no simd       84      113        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * self[e1234]) - (geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]),
            ]) + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]]))
                - (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (geometric_product_g1.yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + (self.group4().ww().with_zw(self[e2], self[e321]) * geometric_product_g1.xyx().with_w(geometric_product_g0[3]))
                + (self.group1().xyz() * geometric_product_g1.www()).with_w(
                    -(geometric_product_g0[1] * self[e2])
                        - (geometric_product_g0[2] * self[e3])
                        - (geometric_product_g1[0] * self[e423])
                        - (geometric_product_g1[1] * self[e431])
                        - (geometric_product_g1[2] * self[e412]),
                )
                - (self.group1().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
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
            (self.group4().ww().with_zw(self[e2], self[e321]) * geometric_product_g0.xyx().with_w(geometric_product_g1[3]))
                + ((Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                    + (Simd32x3::from(geometric_product_g1[3]) * self.group4().xyz())
                    + (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g0.yzz())
                    + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                    + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz()))
                .with_w(geometric_product_g1[2] * self[e3] * -1.0)
                - (geometric_product_g1.zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl GeometricQuotient<MultiVector> for MultiVector {
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
        let geometric_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
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
                + (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[2]]) * self.group3().zyz().with_w(self[e43]))
                + (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1]]) * self.group3().xxy().with_w(self[e42]))
                + (self.group0().xx().with_zw(self[scalar], geometric_product_g0[1]) * geometric_product_g1.xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], geometric_product_g1[0]) * geometric_product_g3.yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], geometric_product_g1[3]) * geometric_product_g3.xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g2[1] * self[e2])
                        - (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * self[e423])
                        - (geometric_product_g3[1] * self[e431])
                        - (geometric_product_g3[2] * self[e412])
                        - (geometric_product_g4[0] * self[e23])
                        - (geometric_product_g4[1] * self[e31])
                        - (geometric_product_g4[2] * self[e12]),
                )
                - (geometric_product_g3.zxy() * self.group1().yzx()).with_w(geometric_product_g4[3] * self[e1234])
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(geometric_product_g2[0] * self[e1]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group2())
                + (Simd32x3::from(geometric_product_g0[1]) * self.group3())
                + (Simd32x3::from(geometric_product_g4[3]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz())
                + (geometric_product_g2.yzx() * self.group3().zxy())
                + (geometric_product_g3.yzx() * self.group2().zxy())
                + (geometric_product_g4.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g4.yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g4.xyx())
                - (geometric_product_g2.zxy() * self.group3().yzx())
                - (geometric_product_g3.zxy() * self.group2().yzx())
                - (geometric_product_g1.zxy() * self.group4().yzx()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(geometric_product_g0[0]) * self.group3())
                + (geometric_product_g3.yzx() * self.group3().zxy())
                + (geometric_product_g1.zxy() * self.group1().yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g1.yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g1.xyx())
                - (geometric_product_g3.zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from(geometric_product_g0[0]) * self.group4())
                + (geometric_product_g0.yy().with_zw(geometric_product_g0[1], self[scalar]) * self.group1().xyz().with_w(geometric_product_g4[3]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group3()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[scalar], self[e31]]) * geometric_product_g4.xyx()).with_w(0.0)
                + (Simd32x3::from([self[e3], self[e1], self[e321]]) * geometric_product_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([self[e12], self[e23], self[scalar]]) * geometric_product_g4.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e321], self[e2]]) * geometric_product_g2.xyx()).with_w(0.0)
                + (self.group2().yzx() * geometric_product_g1.zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group2().zyz().with_w(self[e31]))
                - (Simd32x4::from([geometric_product_g4[3], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[0]]) * self.group2().xxy().with_w(self[e23]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (self.group0().yy().with_zw(self[e1234], geometric_product_g3[0]) * geometric_product_g1.xyz().with_w(self[e1]))
                - (geometric_product_g3.zxy() * self.group4().yzx()).with_w(geometric_product_g3[2] * self[e3])
                - (self.group3().yzx() * geometric_product_g4.zxy()).with_w(geometric_product_g1[2] * self[e12]),
        )
    }
}
impl GeometricQuotient<Plane> for MultiVector {
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
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0[3] * self[e321],
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e4]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(geometric_product_g0[3]) * self.group3()).with_w(
                -(geometric_product_g0[0] * self[e23]) - (geometric_product_g0[1] * self[e31]) - (geometric_product_g0[2] * self[e12]) - (geometric_product_g0[3] * self[e1234]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group4().xyz()) + (geometric_product_g0.zxy() * self.group1().yzx())
                - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                - (geometric_product_g0.yzx() * self.group1().zxy()),
            // e23, e31, e12
            Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g0.xyz()) + (self.group3().zxy() * geometric_product_g0.yzx())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group2())
                - (self.group3().yzx() * geometric_product_g0.zxy()))
            .with_w(geometric_product_g0[3] * self[scalar]),
        )
    }
}
impl GeometricQuotient<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       11        0
    //    simd3        6       11        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       42       64        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
                -(geometric_product_g0[0] * self[e423]) - (geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group3().zxy().with_w(self[e41]))
                + (self.group3().yzx() * geometric_product_g0.zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * geometric_product_g0.xyz()) + (geometric_product_g0.yzx() * self.group4().zxy())
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (geometric_product_g0.zxy() * self.group4().yzx()),
            // e23, e31, e12
            (geometric_product_g0.zxy() * self.group1().yzx()) - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz()) - (geometric_product_g0.yzx() * self.group1().zxy()),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[3]) * self.group3()) + (self.group2().yzx() * geometric_product_g0.zxy())).with_w(geometric_product_g0[2] * self[e12] * -1.0)
                - (geometric_product_g0.xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
                - (geometric_product_g0.yzxy() * self.group2().zxy().with_w(self[e31])),
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
    // f32        0        0        1
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] / (other[scalar]))
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
        AntiScalar::from_groups(/* e1234 */ self[e4] / (other[e321]) * -1.0)
    }
}
impl GeometricQuotient<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        2        9        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * Simd32x3::from(other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12]) * other.group1() * Simd32x3::from(-1.0))
                .with_w(0.0),
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
    //      f32        7        4        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       22        0
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
            Simd32x2::from([1.0, other_g0 * other[e321] * self[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other_g0 * other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * (Simd32x4::from(other_g0) * other.group1()).xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e4]) * other.group3() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] / (other[e321]) * -1.0)
    }
}
impl GeometricQuotient<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        6        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * Simd32x3::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0().xyz(),
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
        Origin::from_groups(/* e4 */ self[e4] / (other[scalar]))
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
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
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
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       36        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
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
            Simd32x4::from(self[e321]) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]) * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricQuotient<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        1
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        9        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(geometric_product_g0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricQuotient<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        8        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       27        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x3::from(self[e321]))
                .with_w(-(geometric_product_g1[0] * self[e423]) - (geometric_product_g1[1] * self[e431]) - (geometric_product_g1[2] * self[e412])),
            // e423, e431, e412, e321
            (Simd32x3::from(other_g0) * Simd32x3::from(self[e321]) * other.group0() * Simd32x3::from(-1.0)).with_w(0.0)
                + (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0)
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
    //      f32       13       10        0
    //    simd2        0        2        0
    //    simd3        6       14        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       31       68        0
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
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
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
            Simd32x3::from(self[e321]) * geometric_product_g1.xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz()) + (geometric_product_g3.yzx() * self.group0().zxy())
                - (geometric_product_g3.zxy() * self.group0().yzx())
                - (Simd32x3::from(other_g0) * Simd32x3::from(self[e321]) * other.group2()))
            .with_w(geometric_product_g0[0] * self[e321]),
        )
    }
}
impl GeometricQuotient<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(geometric_product_g0[3] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
    }
}
impl GeometricQuotient<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        0        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        8       20        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0.yzx() * self.group0().zxy())
                .with_w(-(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g0[3] * self[e321]))
                - (geometric_product_g0.zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            (geometric_product_g0.xyz() * self.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
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
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0();
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(geometric_product_g0[0]) * self.group0(),
            // e423, e431, e412, e321
            (self.group0().xyz() * geometric_product_g0.yy().with_z(geometric_product_g0[1])).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       23       40        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0();
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from(-1.0);
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
    //      f32        0        2        1
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        1
    //  no simd        0        8        1
    fn geometric_quotient(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = 1.0 / other[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(geometric_product_g0 * self[e4]),
            // e23, e31, e12, scalar
            (Simd32x3::from(geometric_product_g0) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       10        0
    //    simd3        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       33        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        let geometric_product_g0 = Simd32x3::from(other_g0) * other.group0() * Simd32x3::from(-1.0);
        let geometric_product_g1 = Simd32x3::from(other_g0) * other.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1.yzx() * self.group0().zxy()).with_w(-(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]))
                - (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0])),
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
    //      f32       11       13        0
    //    simd3        0        1        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       23       44        0
    fn geometric_quotient(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12] + other[scalar] * other[scalar];
        let geometric_product_g0 = Simd32x4::from(other_g0) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other_g0) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g1.yzxw() * self.group0().zxyw())
                + (self.group0().xyz() * geometric_product_g1.www()).with_w(-(geometric_product_g0[1] * self[e2]) - (geometric_product_g0[2] * self[e3]))
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
    //      f32       15       14        0
    //    simd2        3        5        0
    //    simd3        5       12        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       48       84        0
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
        let geometric_product_g2 = Simd32x3::from(other_g0) * other.group2() * Simd32x3::from(-1.0);
        let geometric_product_g3 = Simd32x3::from(other_g0) * other.group3() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other_g0) * other.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([1.0, geometric_product_g4[3] * self[e4]]) * Simd32x2::from([0.0, 1.0]))
                + (Simd32x2::from(self[e1]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g4[0]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g4[1]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g4[2]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (geometric_product_g3.yzx() * self.group0().zxy()).with_w(-(geometric_product_g2[1] * self[e2]) - (geometric_product_g2[2] * self[e3]))
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
    //      f32        2        3        0
    //    simd3        0        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       27        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0.zxyx() * self.group0().yzxx())
                + (geometric_product_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0))
                    .with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e4])),
            // e23, e31, e12, scalar
            (self.group0().xyz() * geometric_product_g0.www() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl GeometricQuotient<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        1        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd       10       22        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0();
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * geometric_product_g0.xyz()) - (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_product_g0.zxyx() * self.group0().yzxx())
                + (geometric_product_g0.yzx() * self.group0().zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3])),
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
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn geometric_quotient(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar]) * Simd32x2::from(f32::powi(other[scalar], -2)) * other.group0())
    }
}
impl GeometricQuotient<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3       20        0
    fn geometric_quotient(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3] + other[e321] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from(-1.0),
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
        Horizon::from_groups(/* e321 */ self[scalar] / (other[e321]) * -1.0)
    }
}
impl GeometricQuotient<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       18        0
    fn geometric_quotient(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let other_g0 = other[e23] * other[e23] + other[e31] * other[e31] + other[e12] * other[e12];
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group1() * Simd32x3::from(-1.0),
        )
    }
}
impl GeometricQuotient<Motor> for Scalar {
    type Output = Motor;
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
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl GeometricQuotient<MultiVector> for Scalar {
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
            Simd32x2::from(other_g0) * Simd32x2::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other_g0) * Simd32x3::from(self[scalar]) * other.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other_g0) * Simd32x4::from(self[scalar]) * other.group4() * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricQuotient<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn geometric_quotient(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * Simd32x4::from(f32::powi(other[e321], -2)) * other.group0() * Simd32x4::from(-1.0),
        )
    }
}
impl GeometricQuotient<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        2        8        0
    fn geometric_quotient(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * Simd32x4::from(other[e1] * other[e1] + other[e2] * other[e2] + other[e3] * other[e3]) * other.group0(),
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
        Scalar::from_groups(/* scalar */ self[scalar] / (other[scalar]))
    }
}
