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
//  Maximum:        73      84       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         0       8       0       0
//  Average:        13      17       0       0
//  Maximum:       198     192       0       0
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
            (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w((self[scalar] * other[e4]) + (self[e1234] * other[e321])),
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
            (Simd32x3::from(self[e1234]) * other.group4().xyz()).with_w((self[scalar] * other[e4]) + (self[e1234] * other[e321])),
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
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0),
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
            (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w((other[e1234] * self[e321]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4       12        8        0      N/A
    // Totals...
    // yes simd       14       16        0      N/A
    //  no simd       50       48        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group1().yzxx() * self.group1().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
                - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy()).with_w(0.0)
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
                + (other.group1().yzxx() * self.group0().zxyx())
                + (other.group1().wwwy() * self.group1().xyz().with_w(self[e2]))
                + Simd32x3::from(0.0).with_w((other[e412] * self[e3]) - (other[e3] * self[e412]))
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
                - (other.group0().yzxx() * self.group1().zxyx())
                - (self.group1().wwwy() * other.group1().xyz().with_w(other[e2]))
                - (other.group1().zxy() * self.group0().yzx()).with_w(0.0),
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
    //      f32        2        7        0        0
    //    simd3        0        7        0      N/A
    //    simd4        8        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       34       36        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]))
                - (Simd32x3::from(self[e4]) * other.group1()).with_w(self[e412] * other[e43])
                - (other.group0().yzx() * self.group0().zxy()).with_w(self[e431] * other[e42]),
            // e423, e431, e412, e321
            (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx())
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
                + (Simd32x3::from(self[e4]) * other.group0()).with_w(self[e431] * other[e31])
                - (other.group0().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]),
        )
    }
}
impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4       11        8        0      N/A
    // Totals...
    // yes simd       14       16        0      N/A
    //  no simd       47       48        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group0())
                + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                + (self.group1().zxy() * other.group1().yzx()).with_w(0.0)
                - (self.group1().xyxy() * other.group1().wwy().with_w(other[e42]))
                - (self.group1().yzzz() * other.group1().zxw().with_w(other[e43]))
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
                - (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from([other[e1234], other[e1234], other[e42], other[e23]]) * self.group1().xyxx())
                + (other.group0() * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (self.group1().yzzy() * other.group0().zxw().with_w(other[e31]))
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       16        0        0
    //    simd2        4        8        0      N/A
    //    simd3       12       15        0      N/A
    //    simd4       11        5        0      N/A
    // Totals...
    // yes simd       35       44        0      N/A
    //  no simd       96       97        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group0())
                + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
                + (other.group2().zxy() * self.group0().yzx()).with_w(0.0)
                + (other.group3().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().xyxx() * Simd32x2::from(other[scalar]).with_zw(other[e31], other[e41]))
                - (self.group1().yzzy() * other.group3().zx().with_zw(other[scalar], other[e42]))
                - (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
                - (other.group2().yzx() * self.group0().zxy()).with_w(self[e412] * other[e43]),
            // e41, e42, e43
            (self.group1().zxy() * other.group4().yzx()) + Simd32x2::from(0.0).with_z((self[e423] * other[e431]) * -1.0)
                - (Simd32x3::from(self[e4]) * other.group4().xyz())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (self.group1().yz() * other.group4().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * other.group1().xyz())
                + (Simd32x3::from(other[e321]) * self.group1().xyz())
                + (self.group0().zxy() * other.group4().yzx())
                + Simd32x2::from(0.0).with_z((self[e423] * other[e2]) - (self[e1] * other[e431]) - (self[e431] * other[e1]))
                + (self.group1().yz() * other.group1().zx()).with_z(0.0)
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (self.group0().yz() * other.group4().zx()).with_z(0.0)
                - (self.group1().zx() * other.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx())
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
                + (Simd32x3::from(self[e4]) * other.group2()).with_w(self[e431] * other[e31])
                - (other.group2().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]),
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
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       30       28        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group1().zxyx() * other.group0().yzxx()) + Simd32x3::from(0.0).with_w((self[e431] * other[e431]) + (self[e412] * other[e412]))
                - (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
                - (self.group1().yzx() * other.group0().zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                + (self.group0().zxyx() * other.group0().yzxx())
                + Simd32x3::from(0.0).with_w((self[e2] * other[e431]) + (self[e3] * other[e412]))
                - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                - (self.group0().yzx() * other.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4] * -1.0) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]))
                + (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
                - (self.group1().zxyx() * other.group0().yzxx()),
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
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
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
            Simd32x2::from([self[e321] * other[e4] * -1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321] * -1.0) * other.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
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
        Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0)
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
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        7        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       13       17        0      N/A
    //  no simd       40       37        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((other[e412] * self[e43]) * -1.0)
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                + (self.group1().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(other[e431] * self[e42]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                + (Simd32x3::from(other[e4]) * self.group0()).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group1().yzxx()),
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
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        0        6        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       28       27        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43])) + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group0().zxy()).with_w(other[e41] * self[e41]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e23])
                - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]),
        )
    }
}
impl GeometricAntiProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        6        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       13       15        0      N/A
    //  no simd       40       36        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e42]) - (self[e43] * other[e43]))
                + (self.group0().xyx() * other.group0().wwy()).with_w(0.0)
                + (self.group0().yzz() * other.group0().zxw()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e31] * other[e42]) - (self[e12] * other[e43]))
                + (self.group0().xyx() * other.group1().wwy()).with_w(0.0)
                + (self.group0().yzz() * other.group1().zxw()).with_w(0.0)
                + (self.group1().xyx() * other.group0().wwy()).with_w(0.0)
                + (self.group1().yzz() * other.group0().zxw()).with_w(0.0)
                - (self.group0().zxyx() * other.group1().yzxx())
                - (self.group1().zxyx() * other.group0().yzxx()),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd2        3        3        0      N/A
    //    simd3        7       16        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       25       32        0      N/A
    //  no simd       69       73        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
                + (self.group0().xyx() * Simd32x2::from(other[e321]).with_z(other[e2])).with_w(0.0)
                + (self.group0().yzz() * other.group1().zx().with_z(other[e321])).with_w(0.0)
                + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
                + (self.group1().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
                - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41]))
                - (self.group0().zxy() * other.group1().yzx()).with_w(self[e42] * other[e431]),
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
            Simd32x3::from(0.0).with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                + (self.group0().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
                + (self.group0().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group4().yzxx()),
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
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       22       21        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
                + (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412])) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl GeometricAntiProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       15        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0()).with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
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
    //      f32        4        8        0        0
    //    simd3        0        7        0      N/A
    //    simd4       12        5        0      N/A
    // Totals...
    // yes simd       16       20        0      N/A
    //  no simd       52       49        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w((other[e412] * self[e43]) * -1.0)
                + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                + (other.group1().xxy() * self.group1().wzx()).with_w(0.0)
                + (other.group1().zyz() * self.group1().yww()).with_w(0.0)
                - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e423])),
            // e423, e431, e412, e321
            (self.group0() * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0)
                    .with_w(-(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                + (other.group1().xxy() * self.group0().wzx()).with_w(other[e4] * self[scalar])
                + (other.group1().zyz() * self.group0().yww()).with_w(0.0)
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e1])),
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
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        )
    }
}
impl GeometricAntiProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        6        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       13       15        0      N/A
    //  no simd       40       36        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43]))
                + (other.group0().xxy() * self.group0().wzx()).with_w(0.0)
                + (other.group0().zyz() * self.group0().yww()).with_w(0.0)
                - (other.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
                + (other.group0().xxy() * self.group1().wzx()).with_w(0.0)
                + (other.group0().zyz() * self.group1().yww()).with_w(0.0)
                + (other.group1().xxy() * self.group0().wzx()).with_w(0.0)
                + (other.group1().zyz() * self.group0().yww()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxyx())
                - (other.group1().yzxx() * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        6        0      N/A
    //    simd4       12        6        0      N/A
    // Totals...
    // yes simd       16       18        0      N/A
    //  no simd       52       48        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(other[e42] * self[e42]) - (other[e43] * self[e43]))
                + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e1234]) * other.group1())
                + (Simd32x4::from(self[scalar]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                + (other.group1().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxyx())
                - (other.group1().yzxx() * self.group0().zxyx()),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        4        4        0      N/A
    //    simd3       10       19        0      N/A
    //    simd4       12        5        0      N/A
    // Totals...
    // yes simd       34       40        0      N/A
    //  no simd       94       97        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            (self.group0() * Simd32x3::from(other[e321]).with_w(other[e4]))
                + Simd32x3::from(0.0).with_w((self[e43] * other[e412]) * -1.0)
                + (Simd32x3::from(self[e1234]) * other.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                + (self.group1().xyx() * Simd32x2::from(other[e4]).with_z(other[e431])).with_w(0.0)
                + (self.group1().yzz() * other.group4().zx().with_z(other[e4])).with_w(0.0)
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e423]))
                - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group2()) + (Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().zxy() * self.group0().yzx())
                - (other.group2().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(other[scalar]) * self.group0().xyz())
                + (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (other.group2().zxy() * self.group1().yzx())
                + (other.group3().zxy() * self.group0().yzx())
                - (other.group2().yzx() * self.group1().zxy())
                - (other.group3().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (self.group0() * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    (self[scalar] * other[e4])
                        - (self[e42] * other[e2])
                        - (self[e43] * other[e3])
                        - (self[e23] * other[e423])
                        - (self[e31] * other[e431])
                        - (self[e12] * other[e412]),
                )
                + (Simd32x3::from(self[e1234]) * other.group4().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group4().zxy()).with_w(0.0)
                - (self.group0().zxyx() * other.group4().yzx().with_w(other[e1])),
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
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       30       28        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
                + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                + (self.group1().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        4        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       15       21        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * other.group0().xyz())
                + (Simd32x3::from(other[e4]) * self.group1().xyz())
                + (self.group0().yzx() * other.group0().zxy())
                + Simd32x2::from(0.0).with_z(self[e42] * other[e1] * -1.0)
                - (self.group0().zx() * other.group0().yz()).with_z(0.0))
            .with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
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
            (Simd32x3::from(other[e1234]) * self.group4().xyz()).with_w((other[e1234] * self[e321]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       17        0        0
    //    simd2        4        8        0      N/A
    //    simd3       12       15        0      N/A
    //    simd4       12        5        0      N/A
    // Totals...
    // yes simd       37       45        0      N/A
    //  no simd      101       98        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w((other[e412] * self[e43]) * -1.0)
                + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * other.group1().xxy()).with_w(0.0)
                + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * other.group1().zyz()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group3().zxy().with_w(self[e41]))
                - (self.group2().zxy() * other.group0().yzx()).with_w(other[e431] * self[e42]),
            // e41, e42, e43
            (other.group1().yzx() * self.group4().zxy()) + Simd32x2::from(0.0).with_z((other[e431] * self[e423]) * -1.0)
                - (Simd32x3::from(other[e4]) * self.group4().xyz())
                - (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (other.group1().zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * other.group0().xyz())
                + (other.group0().zxy() * self.group4().yzx())
                + Simd32x2::from(0.0).with_z((other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]))
                + (other.group1().yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from(self[e321]) * other.group1().xyz())
                - (other.group0().yz() * self.group4().zx()).with_z(0.0)
                - (other.group1().zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
                + (Simd32x4::from(self[e1234]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group1().yzxx()),
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
            Simd32x2::from([other[e321] * self[e4], 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        )
    }
}
impl GeometricAntiProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd2        3        3        0      N/A
    //    simd3        7       16        0      N/A
    //    simd4        8        2        0      N/A
    // Totals...
    // yes simd       22       31        0      N/A
    //  no simd       63       72        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group0().zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group0().xxy()).with_w(0.0)
                + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
                - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41]))
                - (Simd32x3::from(self[e4]) * other.group1()).with_w(other[e43] * self[e412])
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e42] * self[e431]),
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
            (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group4().yzxx())
                + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
                + (Simd32x3::from(self[e4]) * other.group0()).with_w(other[e31] * self[e431])
                - (other.group0().yzx() * self.group4().zxy()).with_w(other[e41] * self[e1]),
        )
    }
}
impl GeometricAntiProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd2        4        4        0      N/A
    //    simd3       10       16        0      N/A
    //    simd4       11        8        0      N/A
    // Totals...
    // yes simd       31       36        0      N/A
    //  no simd       88       96        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                + (Simd32x2::from(other[e1234]) * self.group0())
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (other.group0() * Simd32x3::from(self[e321]).with_w(self[e4]))
                + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group1().yzx()).with_w(0.0)
                + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e423]))
                - (self.group4().xyzz() * Simd32x3::from(other[scalar]).with_w(other[e43]))
                - (self.group4().yzxy() * other.group1().zxy().with_w(other[e42]))
                - (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * other.group0().xyz()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(self[scalar]) * other.group0().xyz())
                + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (self.group2().yzx() * other.group1().zxy())
                + (self.group3().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group1().yzx())
                - (self.group3().zxy() * other.group0().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from([other[e1234], other[e1234], other[e1234], other[e31]]) * self.group4().xyzy())
                + (other.group0() * Simd32x3::from(self[e4]).with_w(self[e321]))
                + (self.group4().yzxx() * other.group0().zxy().with_w(other[e23]))
                + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
                - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       18       28        0        0
    //    simd2        8       14        0      N/A
    //    simd3       24       32        0      N/A
    //    simd4       23       10        0      N/A
    // Totals...
    // yes simd       73       84        0      N/A
    //  no simd      198      192        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]))
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1())
                + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
                + (Simd32x3::from([self[scalar], self[e12], self[e23]]) * other.group4().xxy()).with_w(0.0)
                + (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group2().zyz()).with_w(0.0)
                + (Simd32x3::from([self[e31], self[scalar], self[scalar]]) * other.group4().zyz()).with_w(0.0)
                + (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group2().xxy()).with_w(0.0)
                + (other.group3().yzx() * self.group4().zxy()).with_w(0.0)
                + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group4().yzxx() * self.group3().zxy().with_w(self[e41]))
                - (self.group4().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
                - (self.group4().yzxy() * other.group3().zxy().with_w(other[e42]))
                - (Simd32x3::from(self[e4]) * other.group3()).with_w(other[e412] * self[e43])
                - (other.group2().yzx() * self.group1().zxy()).with_w(other[e431] * self[e42])
                - (self.group2().zxy() * other.group1().yzx()).with_w(other[e43] * self[e412]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group2())
                + (other.group2().zxy() * self.group2().yzx())
                + Simd32x2::from(0.0).with_z((other[e423] * self[e431]) - (other[e431] * self[e423]))
                + (other.group4().yz() * self.group4().zx()).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group4().xyz())
                - (Simd32x3::from(self[e4]) * other.group4().xyz())
                - (other.group2().yzx() * self.group2().zxy())
                - (other.group4().zx() * self.group4().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
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
                - (other.group4().zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4())
                + (Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]))
                + (Simd32x4::from(self[e1234]) * other.group4())
                + (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group4().yzxx())
                + Simd32x3::from(0.0).with_w(
                    -(other[e1] * self[e41])
                        - (other[e2] * self[e42])
                        - (other[e3] * self[e43])
                        - (other[e41] * self[e1])
                        - (other[e42] * self[e2])
                        - (other[e43] * self[e3])
                        - (other[e431] * self[e31])
                        - (other[e412] * self[e12]),
                )
                + (Simd32x3::from(self[e4]) * other.group2()).with_w(other[e31] * self[e431])
                + (self.group2().yzx() * other.group4().zxy()).with_w(other[e12] * self[e412])
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group4().yzxx())
                - (other.group2().yzx() * self.group4().zxy()).with_w(other[scalar] * self[e4]),
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
    //      f32        2        9        0        0
    //    simd2        3        5        0      N/A
    //    simd3        7        9        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd       19       26        0      N/A
    //  no simd       57       58        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * other[e321], 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(self[e42] * other[e431]) - (self[e43] * other[e412]))
                + (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
                + (self.group3().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (self.group4().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((self[e423] * other[e431]) * -1.0)
                - (Simd32x3::from(self[e4]) * other.group0().xyz())
                - (self.group4().yz() * other.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((self[e1] * other[e431]) * -1.0)
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().yz() * other.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * other.group0())
                + Simd32x3::from(0.0).with_w(-(self[e31] * other[e431]) - (self[e12] * other[e412]))
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl GeometricAntiProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7        8        0      N/A
    // Totals...
    // yes simd       14       24        0      N/A
    //  no simd       28       42        0        0
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
            (Simd32x3::from(self[e4]) * other.group0().xyz())
                + Simd32x2::from(0.0).with_z((self[e423] * other[e2]) - (self[e431] * other[e1]))
                + (self.group4().yz() * other.group0().zx()).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (self.group4().zx() * other.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group2()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
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
            Simd32x3::from(0.0).with_w(self[e4] * other[e321]),
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
            Simd32x3::from(0.0).with_w(self[e4] * other[e4] * -1.0),
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
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[scalar] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        )
    }
}
impl GeometricAntiProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        3        0      N/A
    // Totals...
    // yes simd        9       11        0      N/A
    //  no simd       30       28        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group1().yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
                - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0)
                - (other.group1().zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]))
                + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]))
                - (other.group0().yzxx() * self.group0().zxyx()),
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
    //      f32        2        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       22       21        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(-(other[e42] * self[e431]) - (other[e43] * self[e412]))
                + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
                - (other.group0().yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        3        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       29       29        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((other[e43] * self[e412]) * -1.0)
                + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
                + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().xyzy() * Simd32x3::from(other[scalar]).with_w(other[e42]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group0())
                + (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
                + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
                - (other.group0().yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       15        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7        8        0      N/A
    //    simd4        7        4        0      N/A
    // Totals...
    // yes simd       20       29        0      N/A
    //  no simd       55       59        0        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w((other[e43] * self[e412]) * -1.0)
                + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
                + (other.group3().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().xyzx() * Simd32x3::from(other[scalar]).with_w(other[e41]))
                - (self.group0().yzxy() * other.group3().zxy().with_w(other[e42])),
            // e41, e42, e43
            (other.group4().yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((other[e431] * self[e423]) * -1.0)
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (other.group4().zx() * self.group0().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e431]) * -1.0)
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (other.group1().yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group0())
                + (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((other[e31] * self[e431]) + (other[e12] * self[e412]))
                - (other.group2().yzx() * self.group0().zxy()).with_w(0.0),
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
            Simd32x3::from(0.0).with_w(other[e4] * self[e321] * -1.0),
        )
    }
}
impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       12       15        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e431]) + (other[e412] * self[e412]))
                - (other.group0().zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, scalar
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       14        0        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group0().zxyx() * other.group0().yzxx()),
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
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       17       21        0        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4] * -1.0) * other.group1().xyz().with_w(other[e4]),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
                + (other.group1().yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]))
                - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0)
                - (other.group1().zxy() * self.group0().yzx()).with_w(0.0),
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
        Scalar::from_groups(/* scalar */ other[e321] * self[e4])
    }
}
impl GeometricAntiProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        4        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       15        0        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
                - (other.group0().yzx() * self.group0().zxy()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0()).with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
    }
}
impl GeometricAntiProduct<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4        4        0      N/A
    // Totals...
    // yes simd        7       12        0      N/A
    //  no simd       15       21        0        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group0().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z(other[e41] * self[e2] * -1.0)
                - (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (other.group0().yz() * self.group0().zx()).with_z(0.0))
            .with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])),
        )
    }
}
impl GeometricAntiProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       14        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7        8        0      N/A
    // Totals...
    // yes simd       14       24        0      N/A
    //  no simd       28       42        0        0
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
            (Simd32x3::from(self[e4]) * other.group1().xyz())
                + Simd32x2::from(0.0).with_z((other[e423] * self[e2]) - (other[e431] * self[e1]))
                + (other.group4().yz() * self.group0().zx()).with_z(0.0)
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (other.group4().zx() * self.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group2()).with_w(-(other[scalar] * self[e4]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
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
            Simd32x3::from(0.0).with_w(other[e4] * self[e4] * -1.0),
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e4] * -1.0) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl GeometricAntiProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       14        0        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4] * -1.0) * other.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group0().yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]))
                - (other.group0().zxy() * self.group0().yzx()).with_w(0.0),
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
