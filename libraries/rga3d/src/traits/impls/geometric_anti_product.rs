// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 117
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         0       3       0     N/A
//  Average:         4       7       0     N/A
//  Maximum:        87      99       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         0       8       0       0
//  Average:        10      16       0       0
//  Maximum:       172     186       0       0
impl std::ops::Div<GeometricAntiProductInfix> for AntiScalar {
    type Output = GeometricAntiProductInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl GeometricAntiProduct<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * other.group0())
    }
}
impl GeometricAntiProduct<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        )
    }
}
impl GeometricAntiProduct<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * other[e321])
    }
}
impl GeometricAntiProduct<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        )
    }
}
impl GeometricAntiProduct<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        )
    }
}
impl GeometricAntiProduct<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e1234] * other[e4])
    }
}
impl GeometricAntiProduct<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0())
    }
}
impl GeometricAntiProduct<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0())
    }
}
impl GeometricAntiProduct<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234] * other[scalar])
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for DualNum {
    type Output = GeometricAntiProductInfixPartial<DualNum>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0())
    }
}
impl GeometricAntiProduct<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        3        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
        )
    }
}
impl GeometricAntiProduct<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        4       12        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(self[e1234]) * other.group0().xyz())).with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            (other.group1().xyz() * Simd32x2::from(self[e1234]).with_z(self[e1234])).with_w((self[scalar] * other[e4]) + (self[e1234] * other[e321])),
        )
    }
}
impl GeometricAntiProduct<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e1234] * other[e321])
    }
}
impl GeometricAntiProduct<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        3        0      N/A
    // no simd        3        9        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
        )
    }
}
impl GeometricAntiProduct<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        3        0      N/A
    // no simd        4       12        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group1().xyz())).with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            (other.group4().xyz() * Simd32x2::from(self[e1234]).with_z(self[e1234])).with_w((self[scalar] * other[e4]) + (self[e1234] * other[e321])),
        )
    }
}
impl GeometricAntiProduct<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
        )
    }
}
impl GeometricAntiProduct<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        )
    }
}
impl GeometricAntiProduct<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
        )
    }
}
impl GeometricAntiProduct<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e1234] * other[scalar])
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Flector {
    type Output = GeometricAntiProductInfixPartial<Flector>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        )
    }
}
impl GeometricAntiProduct<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        4       12        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(other[scalar]) * self.group1().xyz())).with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            (self.group1().xyz() * Simd32x2::from(other[e1234]).with_z(other[e1234])).with_w((other[e1234] * self[e321]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd2        4        6        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       43       46        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, -(other[e431] * self[e423]) - (other[e412] * self[e4]), 0.0])
                + (other.group1().yzxx() * self.group1().zxyx())
                + (-(Simd32x2::from(other[e423]) * Simd32x2::from([self[e4], self[e412]])) - (Simd32x2::from([self[e431], self[e4]]) * other.group1().zy())).with_zw(0.0, 0.0)
                - (Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
                + (other.group1().yzxz() * self.group0().zxyz())
                + ((Simd32x2::from(other[e1]) * Simd32x2::from([self[e4], self[e412]])) + (Simd32x2::from([self[e431], self[e4]]) * other.group0().zy())
                    - (Simd32x2::from(other[e423]) * Simd32x2::from([self[e321], self[e3]]))
                    - (Simd32x2::from([self[e2], self[e321]]) * other.group1().zy()))
                .with_zw(
                    (other[e2] * self[e423]) + (other[e3] * self[e4]) - (other[e431] * self[e1]) - (other[e412] * self[e321]),
                    (other[e423] * self[e1]) + (other[e431] * self[e2]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                )
                - (other.group0().yzxx() * self.group1().zxyx())
                - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431])),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]),
        )
    }
}
impl GeometricAntiProduct<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        7        9        0      N/A
    // Totals...
    // yes simd       11       14        0      N/A
    //  no simd       25       32        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group0().zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group0().xxy())
                + (other.group1().yzx() * self.group1().zxy())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz())
                - (other.group0().yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group0().xxy()) + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group0().zyz())
                - (other.group0().yzx() * self.group1().zxy()))
            .with_w((other[e23] * self[e423]) + (other[e31] * self[e431]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
    }
}
impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd2        4        5        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       18       21        0      N/A
    //  no simd       41       44        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                (self[e3] * other[e1234]) + (self[e431] * other[e23]) - (self[e2] * other[e41]) - (self[e423] * other[e31]) - (self[e412] * other[scalar]),
                0.0,
            ]) + (self.group0().xyxw() * other.group0().wwyw())
                + ((Simd32x3::from(self[e321]) * other.group0().xyz())
                    + ((self.group0().yz() * other.group0().zx()) + (self.group1().zx() * other.group1().yz())
                        - (Simd32x2::from(other[scalar]) * self.group1().xy())
                        - (self.group0().zx() * other.group0().yz())
                        - (self.group1().yz() * other.group1().zx()))
                    .with_z(0.0)
                    - (Simd32x3::from(self[e4]) * other.group1().xyz()))
                .with_w(0.0),
            // e423, e431, e412, e321
            (self.group1().xyxy() * other.group0().wwy().with_w(other[e31]))
                + (self.group1().yzzz() * other.group0().zxw().with_w(other[e12]))
                + (Simd32x3::from(self[e4]) * other.group0().xyz())
                    .with_w((self[e423] * other[e23]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       24       34        0        0
    //    simd2        4        6        0      N/A
    //    simd3       13       13        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       43       55        0      N/A
    //  no simd       79       93        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
                self[e4] * other[e4] * -1.0,
            ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group0())
                + ((Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group2().zyz())
                    + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group2().xxy())
                    + (other.group3().yzx() * self.group1().zxy())
                    - (Simd32x3::from(other[scalar]) * self.group1().xyz())
                    - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group3().xxy())
                    - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group3().zyz())
                    - (other.group2().yzx() * self.group0().zxy()))
                .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]) - (self[e412] * other[e4]),
            ]) + -(Simd32x2::from(other[e4]) * self.group1().xy()).with_z(0.0)
                - (Simd32x3::from(self[e4]) * other.group4().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e431]) + (self[e431] * other[e3]) - (self[e2] * other[e412]) - (self[e412] * other[e2]),
                (self[e1] * other[e412]) + (self[e412] * other[e1]) - (self[e3] * other[e423]) - (self[e423] * other[e3]),
                (self[e2] * other[e423]) + (self[e423] * other[e2]) + (self[e412] * other[e321]) - (self[e1] * other[e431]) - (self[e3] * other[e4]) - (self[e431] * other[e1]),
            ]) + (Simd32x3::from(self[e4]) * other.group1().xyz())
                + ((Simd32x2::from(other[e321]) * self.group1().xy()) - (Simd32x2::from(other[e4]) * self.group0().xy())).with_z(0.0)
                - (Simd32x3::from(self[e321]) * other.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group1())
                + ((Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group2().xxy()) + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group2().zyz())
                    - (other.group2().yzx() * self.group1().zxy()))
                .with_w(
                    (other[e23] * self[e423]) + (other[e31] * self[e431])
                        - (other[scalar] * self[e4])
                        - (other[e41] * self[e1])
                        - (other[e42] * self[e2])
                        - (other[e43] * self[e3]),
                ),
        )
    }
}
impl GeometricAntiProduct<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e4] * -1.0) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl GeometricAntiProduct<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd2        0        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       26       26        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, self[e423] * other[e431] * -1.0, 0.0])
                + (self.group1().zxyx() * other.group0().yzxx())
                + (-(self.group1().yz() * other.group0().zx()).with_z(0.0) - (Simd32x3::from(self[e4]) * other.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, self[e1] * other[e431] * -1.0, 0.0])
                + (self.group0().zxyx() * other.group0().yzxx())
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2]))
                + (-(self.group0().yz() * other.group0().zx()).with_z(0.0) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       13       17        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (self[e423] * other[e2]) - (self[e431] * other[e1]), 0.0])
                + ((Simd32x3::from(self[e4]) * other.group0().xyz()) + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0)
                    - (Simd32x3::from(other[e4]) * self.group0().xyz()))
                .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[scalar] * -1.0) * self.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e4] * other[scalar] * -1.0),
        )
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Horizon {
    type Output = GeometricAntiProductInfixPartial<Horizon>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e1234] * self[e321])
    }
}
impl GeometricAntiProduct<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e1234] * self[e321])
    }
}
impl GeometricAntiProduct<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321] * -1.0) * other.group1().xyz().with_w(other[e4]),
        )
    }
}
impl GeometricAntiProduct<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0))
    }
}
impl GeometricAntiProduct<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e4] * self[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * other.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
        )
    }
}
impl GeometricAntiProduct<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0)
    }
}
impl GeometricAntiProduct<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * other.group0().xyz(),
        )
    }
}
impl GeometricAntiProduct<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e4] * self[e321] * -1.0)
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Line {
    type Output = GeometricAntiProductInfixPartial<Line>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        )
    }
}
impl GeometricAntiProduct<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        3        0      N/A
    // no simd        3        9        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
        )
    }
}
impl GeometricAntiProduct<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        7        9        0      N/A
    // no simd       21       27        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((self.group0().xyx() * Simd32x2::from(other[e321]).with_z(other[e2]))
                + (self.group0().yzz() * other.group0().zx().with_z(other[e321]))
                + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                + (self.group1().yzz() * other.group1().zx().with_z(other[e4]))
                - (self.group0().zxy() * other.group0().yzx())
                - (self.group1().zxy() * other.group1().yzx()))
            .with_w(0.0),
            // e423, e431, e412, e321
            ((self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])) + (self.group0().yzz() * other.group1().zx().with_z(other[e4]))
                - (self.group0().zxy() * other.group1().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0))
    }
}
impl GeometricAntiProduct<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        6        0      N/A
    // no simd       12       18        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            ((other.group0().zxy() * self.group1().yzx()) + (other.group1().zxy() * self.group0().yzx())
                - (other.group0().yzx() * self.group1().zxy())
                - (other.group1().yzx() * self.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        7        9        0      N/A
    // no simd       21       27        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group0().xyx() * other.group0().wwy()) + (self.group0().yzz() * other.group0().zxw()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            ((self.group0().xyx() * other.group1().wwy())
                + (self.group0().yzz() * other.group1().zxw())
                + (self.group1().xyx() * other.group0().wwy())
                + (self.group1().yzz() * other.group0().zxw())
                - (self.group0().zxy() * other.group1().yzx())
                - (self.group1().zxy() * other.group0().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3       14       18        0      N/A
    // Totals...
    // yes simd       19       24        0      N/A
    //  no simd       50       63        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            ((self.group0().xyx() * Simd32x2::from(other[e321]).with_z(other[e2]))
                + (self.group0().yzz() * other.group1().zx().with_z(other[e321]))
                + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                + (self.group1().yzz() * other.group4().zx().with_z(other[e4]))
                - (self.group0().zxy() * other.group1().yzx())
                - (self.group1().zxy() * other.group4().yzx()))
            .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group0()) + (self.group0().yzx() * other.group2().zxy()) - (self.group0().zxy() * other.group2().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0())
                + (Simd32x3::from(other[e1234]) * self.group1())
                + (self.group0().yzx() * other.group3().zxy())
                + (self.group1().yzx() * other.group2().zxy())
                - (self.group0().zxy() * other.group3().yzx())
                - (self.group1().zxy() * other.group2().yzx()),
            // e423, e431, e412, e321
            ((self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])) + (self.group0().yzz() * other.group4().zx().with_z(other[e4]))
                - (self.group0().zxy() * other.group4().yzx()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        3        5        0      N/A
    // no simd        9       15        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e321]) * self.group0()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(0.0),
            // e423, e431, e412, e321
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        8       15        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e4]) * self.group1()) + (self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(other[e4]).xyz()).with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Motor {
    type Output = GeometricAntiProductInfixPartial<Motor>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        )
    }
}
impl GeometricAntiProduct<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        1        3        0      N/A
    // no simd        4       12        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
        )
    }
}
impl GeometricAntiProduct<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd2        4        6        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       39       42        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                (other[e3] * self[e1234]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) - (other[e1] * self[e42]) - (other[e423] * self[e31]),
                0.0,
            ]) + (other.group0().xxyw() * self.group0().wzxw())
                + ((Simd32x3::from(other[e4]) * self.group1().xyz())
                    + (Simd32x3::from(other[e321]) * self.group0().xyz())
                    + ((Simd32x2::from(other[e423]) * self.group1().wz()) + (other.group0().zy() * self.group0().yw()) + (other.group1().zy() * self.group1().yw())
                        - (other.group0().yz() * self.group0().zx())
                        - (other.group1().yz() * self.group1().zx()))
                    .with_z(0.0))
                .with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]))
                + (other.group1().xxyw() * self.group0().wzxw())
                + (other.group1().zy() * self.group0().yw()).with_zw(other[e412] * self[e1234], -(other[e431] * self[e31]) - (other[e412] * self[e12]))
                - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]),
        )
    }
}
impl GeometricAntiProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        7        9        0      N/A
    // no simd       21       27        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group0().xxy() * self.group0().wzx()) + (other.group0().zyz() * self.group0().yww()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            ((other.group0().xxy() * self.group1().wzx())
                + (other.group0().zyz() * self.group1().yww())
                + (other.group1().xxy() * self.group0().wzx())
                + (other.group1().zyz() * self.group0().yww())
                - (other.group0().yzx() * self.group1().zxy())
                - (other.group1().yzx() * self.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd2        5        7        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       36       39        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, (other[e43] * self[e1234]) - (other[e41] * self[e42]), 0.0])
                + (other.group0().xxyw() * self.group0().wzxw())
                + ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + ((other.group0().zy() * self.group0().yw()) - (other.group0().yz() * self.group0().zx())).with_z(0.0))
                    .with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().xxyw() * self.group1().wzxw())
                + ((Simd32x3::from(other[e1234]) * self.group1().xyz())
                    + (Simd32x3::from(other[scalar]) * self.group0().xyz())
                    + ((Simd32x2::from(other[e23]) * self.group0().wz()) + (other.group0().zy() * self.group1().yw()) + (other.group1().zy() * self.group0().yw())
                        - (other.group0().yz() * self.group1().zx())
                        - (other.group1().yz() * self.group0().zx()))
                    .with_z((other[e43] * self[scalar]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]) - (other[e41] * self[e31]) - (other[e23] * self[e42])))
                .with_w(other[scalar] * self[e1234]),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd2        7        9        0      N/A
    //    simd3       13       15        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       34       43        0      N/A
    //  no simd       79       91        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
                other[e41] * self[e41] * -1.0,
            ]) + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from([self[e23], self[e42]]) * other.group2().xy())
                - (Simd32x2::from([self[e31], self[e43]]) * other.group2().yz()),
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) - (self[e42] * other[e1]) - (self[e31] * other[e423]),
                0.0,
            ]) + (self.group0().xyxw() * Simd32x2::from(other[e321]).with_zw(other[e2], other[e4]))
                + ((Simd32x3::from(self[e1234]) * other.group1().xyz())
                    + (Simd32x3::from(self[scalar]) * other.group4().xyz())
                    + ((Simd32x2::from(other[e4]) * self.group1().xy()) + (self.group0().yz() * other.group1().zx()) + (self.group1().yz() * other.group4().zx())
                        - (self.group0().zx() * other.group1().yz())
                        - (self.group1().zx() * other.group4().yz()))
                    .with_z(0.0))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().xxy() * self.group0().wzx()) + (other.group2().zyz() * self.group0().yww())
                - (other.group2().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0().xyz())
                + (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (other.group2().xxy() * self.group1().wzx())
                + (other.group2().zyz() * self.group1().yww())
                + (other.group3().xxy() * self.group0().wzx())
                + (other.group3().zyz() * self.group0().yww())
                - (other.group2().yzx() * self.group1().zxy())
                - (other.group3().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (self.group0().xyxw() * Simd32x2::from(other[e4]).with_zw(other[e431], other[e321]))
                + ((Simd32x3::from(self[e1234]) * other.group4().xyz()) + (self.group0().yz() * other.group4().zx()).with_z(self[e43] * other[e4]))
                    .with_w((self[scalar] * other[e4]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (other.group4().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiProduct<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]),
        )
    }
}
impl GeometricAntiProduct<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        3        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       24       23        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (self[e23] * other[e431]) - (self[e31] * other[e423]), 0.0])
                + ((Simd32x3::from(self[scalar]) * other.group0().xyz())
                    + (Simd32x3::from(other[e321]) * self.group0().xyz())
                    + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0))
                .with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, self[e42] * other[e423] * -1.0, 0.0])
                + (self.group0().yzxw() * other.group0().zxyw())
                + ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + -(self.group0().zx() * other.group0().yz()).with_z(0.0)).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        8       14        0      N/A
    //  no simd       12       20        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (self[e42] * other[e3]) - (self[e43] * other[e2]),
                (self[e43] * other[e1]) - (self[e41] * other[e3]),
                (self[e41] * other[e2]) - (self[e42] * other[e1]),
            ]) + (Simd32x3::from(self[e1234]) * other.group0().xyz())
                + (Simd32x3::from(other[e4]) * self.group1().xyz()))
            .with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e4]).xyz() * self.group0().xyz())
                .with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for MultiVector {
    type Output = GeometricAntiProductInfixPartial<MultiVector>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        )
    }
}
impl GeometricAntiProduct<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd        8       24        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(other[scalar]) * self.group4().xyz())).with_w(other[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e423, e431, e412, e321
            (self.group4().xyz() * Simd32x2::from(other[e1234]).with_z(other[e1234])).with_w((other[e1234] * self[e321]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       33        0        0
    //    simd2        4        6        0      N/A
    //    simd3       14       14        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       41       54        0      N/A
    //  no simd       76       91        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                other[e4] * self[e4] * -1.0,
            ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group1().xyz())
                + (Simd32x3::from(self[e1234]) * other.group0().xyz())
                + (self.group2().xyx() * Simd32x2::from(other[e321]).with_z(other[e2]))
                + (self.group2().yzz() * other.group0().zx().with_z(other[e321]))
                + (self.group3().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                + (self.group3().yzz() * other.group1().zx().with_z(other[e4]))
                - (self.group2().zxy() * other.group0().yzx())
                - (self.group3().zxy() * other.group1().yzx()))
            .with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e431] * self[e4]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]) - (other[e412] * self[e4]),
            ]) + -(Simd32x2::from(other[e423]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group4().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) + (other[e431] * self[e3]) - (other[e2] * self[e412]) - (other[e412] * self[e2]),
                (other[e2] * self[e4]) + (other[e412] * self[e1]) - (other[e3] * self[e423]) - (other[e431] * self[e321]),
                (other[e2] * self[e423]) + (other[e3] * self[e4]) + (other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]) - (other[e412] * self[e321]),
            ]) + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + ((Simd32x2::from(other[e1]) * Simd32x2::from([self[e4], self[e412]])) - (Simd32x2::from(other[e423]) * Simd32x2::from([self[e321], self[e3]]))).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (self.group2().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                + (self.group2().yzz() * other.group1().zx().with_z(other[e4])))
            .with_w((self[scalar] * other[e4]) + (self[e1234] * other[e321]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (other.group1().yzxx() * self.group2().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiProduct<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * other[e321], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]),
        )
    }
}
impl GeometricAntiProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd2        3        3        0      N/A
    //    simd3       14       18        0      N/A
    // Totals...
    // yes simd       23       29        0      N/A
    //  no simd       54       68        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            ((Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group0().zyz())
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group0().xxy())
                + (other.group1().yzx() * self.group4().zxy())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz())
                - (other.group0().yzx() * self.group1().zxy()))
            .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group0()) + (other.group0().zxy() * self.group2().yzx()) - (other.group0().yzx() * self.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0())
                + (Simd32x3::from(self[e1234]) * other.group1())
                + (other.group0().zxy() * self.group3().yzx())
                + (other.group1().zxy() * self.group2().yzx())
                - (other.group0().yzx() * self.group3().zxy())
                - (other.group1().yzx() * self.group2().zxy()),
            // e423, e431, e412, e321
            ((Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group0().xxy()) + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group0().zyz())
                - (other.group0().yzx() * self.group4().zxy()))
            .with_w((other[e23] * self[e423]) + (other[e31] * self[e431]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
    }
}
impl GeometricAntiProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd2        7        9        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        5        4        0      N/A
    // Totals...
    // yes simd       35       44        0      N/A
    //  no simd       81       93        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1234] * other[scalar]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
                self[e41] * other[e41] * -1.0,
            ]) + (Simd32x2::from(other[e1234]) * self.group0())
                - (Simd32x2::from([other[e23], other[e42]]) * self.group2().xy())
                - (Simd32x2::from([other[e31], other[e43]]) * self.group2().yz()),
            // e1, e2, e3, e4
            Simd32x4::from([
                0.0,
                0.0,
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e41] * self[e2]) - (other[e31] * self[e423]) - (other[e12] * self[e4]),
                0.0,
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * other.group0().xxyw())
                + ((Simd32x3::from(other[e1234]) * self.group1().xyz())
                    + ((Simd32x2::from([self[e2], self[e321]]) * other.group0().zy()) + (other.group1().yz() * self.group4().zx())
                        - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e4], self[e412]]))
                        - (Simd32x2::from([self[e431], self[e4]]) * other.group1().zy())
                        - (other.group0().yz() * self.group1().zx()))
                    .with_z(0.0)
                    - (Simd32x3::from(other[scalar]) * self.group4().xyz()))
                .with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (self.group2().xyx() * other.group0().wwy()) + (self.group2().yzz() * other.group0().zxw())
                - (self.group2().zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0().xyz())
                + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (self.group2().xyx() * other.group1().wwy())
                + (self.group2().yzz() * other.group1().zxw())
                + (self.group3().xyx() * other.group0().wwy())
                + (self.group3().yzz() * other.group0().zxw())
                - (self.group2().zxy() * other.group1().yzx())
                - (self.group3().zxy() * other.group0().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e321]]) * other.group0().xxyw())
                + (self.group4().xyzy() * Simd32x3::from(other[e1234]).with_w(other[e31]))
                + (Simd32x2::from([self[e431], self[e4]]) * other.group0().zy()).with_zw(
                    other[e43] * self[e4],
                    (other[e23] * self[e423]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]),
                )
                - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       43       53        0        0
    //    simd2        5        7        0      N/A
    //    simd3       37       37        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       87       99        0      N/A
    //  no simd      172      186        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321]),
                (other[e423] * self[e423]) - (other[e41] * self[e41]) - (other[e42] * self[e42]) - (other[e4] * self[e4]),
            ]) + (Simd32x2::from(self[e1234]) * other.group0())
                + (Simd32x2::from([self[e1], self[e431]]) * other.group4().xy())
                + (Simd32x2::from([self[e2], self[e412]]) * other.group4().yz())
                - (Simd32x2::from([self[e23], self[e43]]) * other.group2().xz()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group1())
                + ((Simd32x3::from(self[scalar]) * other.group4().xyz())
                    + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                    + (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group2().zyz())
                    + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group2().xxy())
                    + (other.group3().yzx() * self.group4().zxy())
                    + (self.group2().xyx() * Simd32x2::from(other[e321]).with_z(other[e2]))
                    + (self.group2().yzz() * other.group1().zx().with_z(other[e321]))
                    + (self.group3().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                    + (self.group3().yzz() * other.group4().zx().with_z(other[e4]))
                    - (Simd32x3::from(other[scalar]) * self.group4().xyz())
                    - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group3().xxy())
                    - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group3().zyz())
                    - (other.group2().yzx() * self.group1().zxy())
                    - (self.group2().zxy() * other.group1().yzx())
                    - (self.group3().zxy() * other.group4().yzx()))
                .with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e431] * self[e4]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]) - (other[e412] * self[e4]),
            ]) + (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group2())
                + (other.group2().zxy() * self.group2().yzx())
                + -(Simd32x2::from(other[e423]) * Simd32x2::from([self[e4], self[e412]])).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group4().xyz())
                - (other.group2().yzx() * self.group2().zxy()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) + (other[e431] * self[e3]) - (other[e2] * self[e412]) - (other[e412] * self[e2]),
                (other[e2] * self[e4]) + (other[e412] * self[e1]) - (other[e3] * self[e423]) - (other[e431] * self[e321]),
                (other[e2] * self[e423]) + (other[e3] * self[e4]) + (other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]) - (other[e412] * self[e321]),
            ]) + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (other.group2().zxy() * self.group3().yzx())
                + (other.group3().zxy() * self.group2().yzx())
                + ((Simd32x2::from(other[e1]) * Simd32x2::from([self[e4], self[e412]])) - (Simd32x2::from(other[e423]) * Simd32x2::from([self[e321], self[e3]]))).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (other.group2().yzx() * self.group3().zxy())
                - (other.group3().yzx() * self.group2().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4())
                + ((Simd32x3::from(self[e1234]) * other.group4().xyz())
                    + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group2().xxy())
                    + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group2().zyz())
                    + (self.group2().xyx() * Simd32x2::from(other[e4]).with_z(other[e431]))
                    + (self.group2().yzz() * other.group4().zx().with_z(other[e4]))
                    - (other.group2().yzx() * self.group4().zxy())
                    - (self.group2().zxy() * other.group4().yzx()))
                .with_w(
                    (self[scalar] * other[e4]) + (self[e1234] * other[e321]) + (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412])
                        - (other[scalar] * self[e4])
                        - (other[e41] * self[e1])
                        - (other[e42] * self[e2])
                        - (other[e43] * self[e3])
                        - (self[e41] * other[e1])
                        - (self[e42] * other[e2])
                        - (self[e43] * other[e3])
                        - (self[e23] * other[e423])
                        - (self[e31] * other[e431])
                        - (self[e12] * other[e412]),
                ),
        )
    }
}
impl GeometricAntiProduct<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       19        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e4] * -1.0) * Simd32x2::from([self[e321], self[e4]]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group3().with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(other[e4] * -1.0) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(other[e4] * -1.0) * self.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]),
        )
    }
}
impl GeometricAntiProduct<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd2        3        3        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       37       50        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * other[e321], 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (Simd32x3::from(other[e321]) * self.group2()) + (self.group3().yzx() * other.group0().zxy())
                - (self.group3().zxy() * other.group0().yzx()))
            .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
            ]) - (Simd32x3::from(self[e4]) * other.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e431]) - (self[e2] * other[e412]),
                (self[e1] * other[e412]) - (self[e3] * other[e423]),
                (self[e2] * other[e423]) - (self[e1] * other[e431]),
            ]) + (Simd32x3::from(other[e321]) * self.group4().xyz())
                - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * other.group0()) + ((self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       18        0        0
    //    simd3        5        8        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       24       42        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
                self[e4] * other[e4] * -1.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx()))
            .with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4] * -1.0) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from([
                (self[e431] * other[e3]) - (self[e412] * other[e2]),
                (self[e412] * other[e1]) - (self[e423] * other[e3]),
                (self[e423] * other[e2]) - (self[e431] * other[e1]),
            ]) + (Simd32x3::from(self[e4]) * other.group0().xyz())
                - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e423, e431, e412, e321
            (self.group2() * Simd32x4::from(other[e4]).xyz()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
    }
}
impl GeometricAntiProduct<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e1234] * other[scalar], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[scalar] * -1.0) * self.group4().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e4] * other[scalar] * -1.0),
        )
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Origin {
    type Output = GeometricAntiProductInfixPartial<Origin>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e1234] * self[e4])
    }
}
impl GeometricAntiProduct<DualNum> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4] * -1.0),
        )
    }
}
impl GeometricAntiProduct<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e321] * self[e4])
    }
}
impl GeometricAntiProduct<Line> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        7        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e4] * -1.0) * other.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        3        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       26        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * (other.group3() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * other.group2().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl GeometricAntiProduct<Origin> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4] * -1.0)
    }
}
impl GeometricAntiProduct<Plane> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4] * -1.0) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e321] * self[e4]),
        )
    }
}
impl GeometricAntiProduct<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e4] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0)
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Plane {
    type Output = GeometricAntiProductInfixPartial<Plane>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0())
    }
}
impl GeometricAntiProduct<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       15        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, other[scalar], 0.0])
                * (self.group0().xyz() * Simd32x2::from(other[scalar] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        )
    }
}
impl GeometricAntiProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        3        0      N/A
    //    simd3        3        3        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       24       23        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, other[e431] * self[e423] * -1.0, 0.0])
                + (other.group1().yzxx() * self.group0().zxyx())
                + (-(other.group1().zx() * self.group0().yz()).with_z(0.0) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (other[e2] * self[e423]) - (other[e1] * self[e431]), 0.0])
                + ((Simd32x3::from(other[e321]) * self.group0().xyz()) + ((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_z(0.0)
                    - (Simd32x3::from(self[e321]) * other.group1().xyz()))
                .with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[e321]) * self.group0().xyz())
    }
}
impl GeometricAntiProduct<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        2        4        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd       10       16        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e321]) * other.group0()) + (other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e423, e431, e412, e321
            (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])) + -(other.group0().yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        4        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       24        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, (other[e23] * self[e431]) - (other[e31] * self[e423]), 0.0])
                + ((Simd32x3::from(self[e321]) * other.group0().xyz()) + ((other.group1().yz() * self.group0().zx()) - (other.group1().zx() * self.group0().yz())).with_z(0.0)
                    - (Simd32x3::from(other[scalar]) * self.group0().xyz()))
                .with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, other[e41] * self[e431] * -1.0, 0.0])
                + (other.group0().zxyw() * self.group0().yzxw())
                + (self.group0().xyzx() * Simd32x3::from(other[e1234]).with_w(other[e23]))
                + -(other.group0().yz() * self.group0().zx()).with_zw(0.0, 0.0),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       19        0        0
    //    simd3        6        8        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       19       29        0      N/A
    //  no simd       37       51        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e321]) * other.group2()) + (other.group3().yzx() * self.group0().zxy())
                - (Simd32x3::from(other[scalar]) * self.group0().xyz())
                - (other.group3().zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) - (other[e2] * self[e412]),
                (other[e1] * self[e412]) - (other[e3] * self[e423]),
                (other[e2] * self[e423]) - (other[e1] * self[e431]),
            ]) + (Simd32x3::from(other[e321]) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * other.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group0())
                + (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]))
                + -(other.group2().yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] * other[e4] * -1.0),
        )
    }
}
impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd       11       14        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, other[e431] * self[e423] * -1.0, 0.0])
                + (other.group0().yzxx() * self.group0().zxyx())
                + -(other.group0().zx() * self.group0().yz()).with_zw(0.0, 0.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        8       13        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e4]).xyz() * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, self[e423] * other[e2], 0.0]) + (self.group0().yz() * other.group0().zx()).with_zw(0.0, 0.0) - (self.group0().zxyw() * other.group0().yzxw()),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(other[scalar] * -1.0) * self.group0().xyz()).with_w(0.0))
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Point {
    type Output = GeometricAntiProductInfixPartial<Point>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0())
    }
}
impl GeometricAntiProduct<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4] * -1.0),
        )
    }
}
impl GeometricAntiProduct<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       15       20        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, other[e431] * self[e1] * -1.0, 0.0])
                + (other.group1().yzxy() * self.group0().zxyy())
                + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]))
                + (-(other.group1().zx() * self.group0().yz()).with_z(0.0) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Horizon> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e4] * other[e321])
    }
}
impl GeometricAntiProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd        8       15        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((other.group0().zxy() * self.group0().yzx()) - (Simd32x3::from(self[e4]) * other.group1()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
    }
}
impl GeometricAntiProduct<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        8       14        0      N/A
    //  no simd       12       20        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from([
                (other[e43] * self[e2]) - (other[e42] * self[e3]),
                (other[e41] * self[e3]) - (other[e43] * self[e1]),
                (other[e42] * self[e1]) - (other[e41] * self[e2]),
            ]) + (Simd32x3::from(other[e1234]) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * other.group1().xyz()))
            .with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz())
                .with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       18        0        0
    //    simd3        5        8        0      N/A
    // Totals...
    // yes simd       14       26        0      N/A
    //  no simd       24       42        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
                other[e4] * self[e4] * -1.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * other.group3())
                - (other.group2().yzx() * self.group0().zxy()))
            .with_w(other[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from([
                (other[e431] * self[e3]) - (other[e412] * self[e2]),
                (other[e412] * self[e1]) - (other[e423] * self[e3]),
                (other[e423] * self[e2]) - (other[e431] * self[e1]),
            ]) + (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e423, e431, e412, e321
            (other.group2() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[scalar] * self[e4]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
    }
}
impl GeometricAntiProduct<Origin> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e4] * other[e4] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        8       14        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, other[e431] * self[e1] * -1.0, 0.0])
                + (other.group0().yzxx() * self.group0().zxyx())
                + -(other.group0().zx() * self.group0().yz()).with_zw(0.0, 0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3        8        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e4] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Scalar> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0)
    }
}
impl std::ops::Div<GeometricAntiProductInfix> for Scalar {
    type Output = GeometricAntiProductInfixPartial<Scalar>;
    fn div(self, _rhs: GeometricAntiProductInfix) -> Self::Output {
        GeometricAntiProductInfixPartial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e1234] * self[scalar])
    }
}
impl GeometricAntiProduct<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e1234] * self[scalar])
    }
}
impl GeometricAntiProduct<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e4] * self[scalar]),
        )
    }
}
impl GeometricAntiProduct<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(self[scalar]) * other.group0())
    }
}
impl GeometricAntiProduct<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e1234] * self[scalar], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e4] * self[scalar]),
        )
    }
}
impl GeometricAntiProduct<Origin> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e4] * self[scalar])
    }
}
impl GeometricAntiProduct<Plane> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0))
    }
}
impl GeometricAntiProduct<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e4] * self[scalar])
    }
}
