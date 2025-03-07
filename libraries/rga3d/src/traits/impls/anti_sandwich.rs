use crate::traits::AntiReverse;
use crate::traits::GeometricAntiProduct;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 109
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        10      22       0
//  Average:        21      32       0
//  Maximum:       162     187       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        19      39       0
//  Average:        45      64       0
//  Maximum:       362     394       0
impl std::ops::Div<AntiSandwichInfix> for AntiScalar {
    type Output = AntiSandwichInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * other.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       32        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e4]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]).geometric_anti_product(self.anti_reverse());
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
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       26        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) + (Simd32x3::from(self[e1234]) * other.group0().xyz())).with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321])) * other.group1().xyz().with_w(1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd3        4       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       16       50        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[scalar]) * other.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group1().xyz())).with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321])) * other.group4().xyz().with_w(1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       15        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       20        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (other.group0().xyz() * self.group0().xx().with_z(self[scalar])).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        4       18        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]).geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Flector {
    type Output = AntiSandwichInfixPartial<Flector>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        9       13        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        2        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       44       65        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(other[scalar]) * self.group1().xyz())).with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            other.group0().yy().with_zw(other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4])) * self.group1().xyz().with_w(1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4       15       17        0
    // Totals...
    // yes simd       35       50        0
    //  no simd       80      101        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other[e423] * self[e4]) - (other[e412] * self[e431]),
                -(other[e423] * self[e412]) - (other[e431] * self[e4]),
                -(other[e431] * self[e423]) - (other[e412] * self[e4]),
                (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]) + (other.group1().yzxx() * self.group1().zxyx())
                - (Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4])),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e1]]) * other.group0().xxy().with_w(other[e423]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e2]]) * other.group0().zyz().with_w(other[e431]))
                + (other.group1().yzxz() * self.group0().zxyz())
                - (Simd32x4::from([self[e2], self[e321], self[e321], self[e321]]) * other.group1().zyz().with_w(other[e4]))
                - (Simd32x4::from([self[e321], self[e3], self[e1], self[e412]]) * other.group1().xxy().with_w(other[e3]))
                - (other.group0().yzxx() * self.group1().zxyx())
                - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       33        0
    //  no simd       40       57        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       22        0
    //    simd3        0        2        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       72       88        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
                0.0,
            ]) - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
                - (other.group0().yzx() * self.group0().zxy()).with_w(self[e423] * other[e41]),
            // e423, e431, e412, e321
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group0().xxy().with_w(other[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group0().zyz().with_w(other[e31]))
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
                - (other.group0().yzx() * self.group1().zxy()).with_w(self[e1] * other[e41]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       84      100        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
                (self[e3] * other[e1234]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e412] * other[scalar]),
                0.0,
            ]) + (self.group0().xyxw() * other.group0().wwyw())
                - (self.group1().xyxz() * other.group1().wwy().with_w(other[e43]))
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e423]))
                - (other.group1().xyz() * self.group0().www()).with_w(self[e431] * other[e42]),
            // e423, e431, e412, e321
            (self.group1().xyxy() * other.group0().wwy().with_w(other[e31]))
                + (self.group1().yzzz() * other.group0().zxw().with_w(other[e12]))
                + Simd32x3::from(0.0).with_w((self[e321] * other[e1234]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar]))
                + (other.group0().xyz() * self.group0().www()).with_w(self[e423] * other[e23])
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e1])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       53        0
    //    simd2        8        8        0
    //    simd3       20       28        0
    //    simd4       14       11        0
    // Totals...
    // yes simd       76      100        0
    //  no simd      166      197        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * other.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e4] * other[e31]),
                (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e4] * other[e12]),
                0.0,
            ]) + (Simd32x4::from(other[e1234]) * self.group0())
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * other.group3().xxy().with_w(other[e43]))
                - (self.group1().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
                - (other.group2().yzx() * self.group0().zxy()).with_w(self[e431] * other[e42]),
            // e41, e42, e43
            (self.group1().zxy() * other.group4().yzx())
                - (Simd32x3::from(self[e4]) * other.group4().xyz())
                - (Simd32x3::from([other[e4], other[e4], other[e431]]) * self.group1().xyx())
                - (Simd32x3::from([other[e412], other[e423], other[e4]]) * self.group1().yzz()),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * other.group1().xyz())
                + (Simd32x3::from([other[e3], other[e1], other[e321]]) * self.group1().yzz())
                + (Simd32x3::from([other[e321], other[e321], other[e2]]) * self.group1().xyx())
                + (self.group0().zxy() * other.group4().yzx())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (Simd32x3::from([other[e4], other[e4], other[e431]]) * self.group0().xyx())
                - (Simd32x3::from([other[e412], other[e423], other[e4]]) * self.group0().yzz())
                - (self.group1().zxy() * other.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group2().xxy().with_w(other[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group2().zyz().with_w(other[e31]))
                + Simd32x3::from(0.0).with_w((self[e412] * other[e12]) - (self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]))
                - (other.group2().yzx() * self.group1().zxy()).with_w(self[e4] * other[scalar]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        6       11        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       69        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       33       51        0
    //  no simd       60       81        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self[e4] * other[e423]) - (self[e431] * other[e412]),
                -(self[e4] * other[e431]) - (self[e412] * other[e423]),
                -(self[e4] * other[e412]) - (self[e423] * other[e431]),
                (self[e431] * other[e431]) + (self[e412] * other[e412]),
            ]) + (self.group1().zxyx() * other.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self[e2] * other[e412]) - (self[e321] * other[e423]),
                -(self[e3] * other[e423]) - (self[e321] * other[e431]),
                -(self[e1] * other[e431]) - (self[e321] * other[e412]),
                (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) + (self.group0().zxyx() * other.group0().yzxx())
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4        8       11        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       52       77        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e4] * other[e1]) + (self[e431] * other[e3]),
                (self[e4] * other[e2]) + (self[e412] * other[e1]),
                (self[e4] * other[e3]) + (self[e423] * other[e2]),
                -(self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group1().zxyy() * other.group0().yzxy())
                - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       40       63        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[scalar]) * self.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e4] * other[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Horizon {
    type Output = AntiSandwichInfixPartial<Horizon>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       23        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Line {
    type Output = AntiSandwichInfixPartial<Line>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        7        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       19       39        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        8        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       22       42        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd3        0        6        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       33       52        0
    //  no simd       60       79        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]),
                other[e412] * self[e43] * -1.0,
            ]) - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * other.group0().yzx()).with_w(other[e423] * self[e41]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * self[e41]) + (other[e412] * self[e42]),
                (other[e4] * self[e42]) + (other[e423] * self[e43]),
                (other[e4] * self[e43]) + (other[e431] * self[e41]),
                -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]) - (self.group0().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        7        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       10       24        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       43        0
    //    simd3        0        6        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       29       51        0
    //  no simd       47       69        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[e43] * self[e42],
                other[e41] * self[e43],
                other[e42] * self[e41],
                -(other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]) - (other.group0().yzx() * self.group0().zxy()).with_w(other[e41] * self[e41]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e43] * self[e31]) + (other[e12] * self[e42]),
                (other[e41] * self[e12]) + (other[e23] * self[e43]),
                (other[e42] * self[e23]) + (other[e31] * self[e41]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e23])
                - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       56       78        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * other[e1234]) + (self[e42] * other[e43]),
                (self[e42] * other[e1234]) + (self[e43] * other[e41]),
                (self[e41] * other[e42]) + (self[e43] * other[e1234]),
                -(self[e42] * other[e42]) - (self[e43] * other[e43]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e41] * other[scalar]) + (self[e42] * other[e12]) + (self[e23] * other[e1234]) + (self[e31] * other[e43]),
                (self[e42] * other[scalar]) + (self[e43] * other[e23]) + (self[e31] * other[e1234]) + (self[e12] * other[e41]),
                (self[e41] * other[e31]) + (self[e43] * other[scalar]) + (self[e23] * other[e42]) + (self[e12] * other[e1234]),
                -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]) - (other.group1().yzxx() * self.group0().zxy().with_w(self[e41]))
                - (self.group1().zxy() * other.group0().yzx()).with_w(self[e42] * other[e31]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       47        0
    //    simd2        6        6        0
    //    simd3       14       24        0
    //    simd4        9        5        0
    // Totals...
    // yes simd       57       82        0
    //  no simd      118      151        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e31] * other[e412]),
                (self[e42] * other[e321]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[e12] * other[e423]),
                (self[e41] * other[e2]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]),
                self[e43] * other[e412] * -1.0,
            ]) - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxy() * other.group1().yzx()).with_w(self[e41] * other[e423]),
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
            Simd32x4::from([
                (self[e41] * other[e4]) + (self[e42] * other[e412]),
                (self[e42] * other[e4]) + (self[e43] * other[e423]),
                (self[e41] * other[e431]) + (self[e43] * other[e4]),
                -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) - (self.group0().zxy() * other.group4().yzx()).with_w(self[e41] * other[e1]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd3        0        6        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       32       48        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0()).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       27        0
    //    simd3        0        4        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       45       63        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e42] * other[e412],
                self[e43] * other[e423],
                self[e41] * other[e431],
                -(self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd3        0        8        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       42       57        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0) + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (self.group0().zxy() * other.group0().yzx()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0()).with_w(-(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       19       36        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[scalar]) * self.group0())
            .geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Motor {
    type Output = AntiSandwichInfixPartial<Motor>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       64        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       44       68        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        2        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       42       59        0
    //  no simd       84      105        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42]),
                (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43]),
                other[e412] * self[e43] * -1.0,
            ]) + (other.group0().xxyw() * self.group0().wzxw())
                - (other.group1().yzxy() * self.group1().zxy().with_w(self[e42]))
                - (self.group0().zxyx() * other.group0().yzx().with_w(other[e423])),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e412] * self[e42],
                other[e431] * self[e1234],
                other[e412] * self[e1234],
                -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]) + (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]))
                + (other.group1().xxyw() * self.group0().wzxw())
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e1])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       19        0
    //    simd3        0        3        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       44       60        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        3        0
    //    simd4        9        8        0
    // Totals...
    // yes simd       41       62        0
    //  no simd       68       92        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[e1234]) + (other[e43] * self[e42]),
                (other[e41] * self[e43]) + (other[e42] * self[e1234]),
                (other[e42] * self[e41]) + (other[e43] * self[e1234]),
                -(other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]) - (self.group0().zxyx() * other.group0().yzx().with_w(other[e41])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e41] * self[scalar]) + (other[e43] * self[e31]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]),
                (other[e41] * self[e12]) + (other[e42] * self[scalar]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]),
                (other[e42] * self[e23]) + (other[e43] * self[scalar]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (self.group1().zxyx() * other.group0().yzx().with_w(other[e41]))
                - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        4        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       44       66        0
    //  no simd       80      104        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e43] * self[e42]) + (other[e1234] * self[e41]),
                (other[e42] * self[e1234]) + (other[e1234] * self[e42]),
                (other[e43] * self[e1234]) + (other[e1234] * self[e43]),
                -(other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]) + (other.group0().xxyw() * self.group0().wzxw())
                - (other.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]) + (other[scalar] * self[e41]),
                (other[e1234] * self[e31]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]) + (other[scalar] * self[e42]),
                (other[e1234] * self[e12]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (other.group0().xxyw() * self.group1().wzxw())
                + (other.group0().zyz() * self.group1().yww()).with_w(other[scalar] * self[e1234])
                - (other.group0().yzxx() * self.group1().zxyx())
                - (other.group1().yzx() * self.group0().zxy()).with_w(other[e42] * self[e31]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       49        0
    //    simd2        8        8        0
    //    simd3       20       24        0
    //    simd4       14       16        0
    // Totals...
    // yes simd       76       97        0
    //  no simd      166      201        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]), 0.0])
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e1234] * other[e1]) + (self[e23] * other[e4]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]),
                (self[e43] * other[e1]) + (self[e1234] * other[e2]) + (self[e31] * other[e4]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]),
                (self[e43] * other[e321]) + (self[e1234] * other[e3]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]),
                self[e43] * other[e412] * -1.0,
            ]) + (self.group0().xyxw() * other.group4().ww().with_zw(other[e2], other[e4]))
                - (self.group0().zxyx() * other.group1().yzx().with_w(other[e423]))
                - (other.group4().yzxy() * self.group1().zxy().with_w(self[e42])),
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
            Simd32x4::from([
                self[e1234] * other[e423],
                self[e1234] * other[e431],
                self[e1234] * other[e412],
                -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (self.group0().xyxw() * other.group1().ww().with_zw(other[e431], other[e321]))
                + (other.group4().zx().with_zw(other[e4], other[e4]) * self.group0().yzz().with_w(self[scalar]))
                - (self.group0().zxyx() * other.group4().yzx().with_w(other[e1])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       44       64        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group1().xyz().with_w(self[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[scalar]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        2        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       64       84        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[scalar] * other[e412]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e1234] * other[e423],
                self[e1234] * other[e431],
                self[e1234] * other[e412],
                -(self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (self.group0().yzxw() * other.group0().zxyw())
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        3        7        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       56       76        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group1().xyz()) + (self.group0().yzx() * other.group0().zxy())
                - (self.group0().zxy() * other.group0().yzx()))
            .with_w(self[e1234] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
        )
        .geometric_anti_product(self.anti_reverse());
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
    //           add/sub      mul      div
    //      f32       35       44        0
    //    simd2        8        9        0
    //    simd3       22       32        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       81      100        0
    //  no simd      181      218        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
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
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       50        0
    //    simd2        8        8        0
    //    simd3       24       35        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       85      107        0
    //  no simd      189      227        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(other[scalar]) * self.group4().xyz())).with_w(other[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e423, e431, e412, e321
            other.group0().yy().with_zw(other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4])) * self.group4().xyz().with_w(1.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       75        0
    //    simd2       12       12        0
    //    simd3       32       44        0
    //    simd4       22       17        0
    // Totals...
    // yes simd      120      148        0
    //  no simd      262      299        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from([self[e423], self[e4]]) * other.group0().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1] * self[e1234]) + (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e1] * self[e43]) + (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e2] * self[e41]) + (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]),
                other[e412] * self[e43] * -1.0,
            ]) + (self.group0().xx().with_zw(self[scalar], self[e1234]) * other.group1().xyz().with_w(other[e4]))
                - (other.group1().yzxy() * self.group3().zxy().with_w(self[e42]))
                - (self.group2().zxy() * other.group0().yzx()).with_w(other[e423] * self[e41]),
            // e41, e42, e43
            (other.group1().yzx() * self.group4().zxy())
                - (Simd32x3::from(other[e4]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group0().xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group0().zyz())
                + (other.group1().yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group1().zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group1().xxy())
                - (other.group0().yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e412] * self[e42],
                other[e423] * self[e43],
                other[e4] * self[e43],
                -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]) + (self.group0().yy().with_zw(self[e1234], self[scalar]) * other.group1().xyz().with_w(other[e4]))
                + (other.group0().ww().with_zw(other[e431], other[e321]) * self.group2().xyx().with_w(self[e1234]))
                - (self.group2().zxy() * other.group1().yzx()).with_w(other[e1] * self[e41]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       46        0
    //    simd2        8        9        0
    //    simd3       22       32        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       81      100        0
    //  no simd      181      212        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd2       11       11        0
    //    simd3       29       41        0
    //    simd4       22       17        0
    // Totals...
    // yes simd      107      130        0
    //  no simd      242      274        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412]),
                (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431]),
                0.0,
            ]) - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
                - (other.group0().yzx() * self.group1().zxy()).with_w(other[e41] * self[e423]),
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
            (Simd32x4::from([self[e4], self[e412], self[e423], self[e423]]) * other.group0().xxy().with_w(other[e23]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e431]]) * other.group0().zyz().with_w(other[e31]))
                + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
                - (other.group0().yzx() * self.group4().zxy()).with_w(other[e41] * self[e1]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       64        0
    //    simd2       12       12        0
    //    simd3       32       42        0
    //    simd4       24       21        0
    // Totals...
    // yes simd      118      139        0
    //  no simd      266      298        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]), 0.0])
                + (Simd32x2::from(other[e1234]) * self.group0())
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e43] * self[e2]) + (other[e1234] * self[e1]) + (other[e31] * self[e412]) - (other[scalar] * self[e423]),
                (other[e42] * self[e321]) + (other[e1234] * self[e2]) + (other[e12] * self[e423]) - (other[scalar] * self[e431]),
                (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) - (other[scalar] * self[e412]),
                0.0,
            ]) + (Simd32x4::from([self[e321], self[e3], self[e1], self[e4]]) * other.group0().xxyw())
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e431]]) * other.group1().xxy().with_w(other[e42]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], self[e412]]) * other.group1().zyz().with_w(other[e43]))
                - (other.group0().yzxx() * self.group1().zxy().with_w(self[e423])),
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
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * other.group0().zyz().with_w(other[e23]))
                + (self.group4().xyzy() * other.group0().www().with_w(other[e31]))
                + Simd32x3::from(0.0).with_w((other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
                - (other.group0().yzxx() * self.group4().zxy().with_w(self[e1])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70       88        0
    //    simd2       16       16        0
    //    simd3       44       58        0
    //    simd4       32       25        0
    // Totals...
    // yes simd      162      187        0
    //  no simd      362      394        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]]))
                - (Simd32x2::from([self[e423], self[e4]]) * other.group1().xw()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1] * self[e1234])
                    + (other[e3] * self[e42])
                    + (other[e4] * self[e23])
                    + (other[e41] * self[e321])
                    + (other[e43] * self[e2])
                    + (other[e31] * self[e412])
                    + (other[e412] * self[e31])
                    + (other[e321] * self[e41]),
                (other[e1] * self[e43])
                    + (other[e2] * self[e1234])
                    + (other[e4] * self[e31])
                    + (other[e41] * self[e3])
                    + (other[e42] * self[e321])
                    + (other[e12] * self[e423])
                    + (other[e423] * self[e12])
                    + (other[e321] * self[e42]),
                (other[e2] * self[e41])
                    + (other[e3] * self[e1234])
                    + (other[e4] * self[e12])
                    + (other[e42] * self[e1])
                    + (other[e43] * self[e321])
                    + (other[e23] * self[e431])
                    + (other[e431] * self[e23])
                    + (other[e321] * self[e43]),
                0.0,
            ]) + (Simd32x4::from(other[e1234]) * self.group1())
                + (self.group0().xx().with_zw(self[scalar], self[e1234]) * other.group4().xyz().with_w(other[e4]))
                - (Simd32x4::from([self[e4], self[e412], self[e423], self[e412]]) * other.group3().xxy().with_w(other[e43]))
                - (Simd32x4::from([self[e431], self[e4], self[e4], other[e423]]) * other.group3().zyz().with_w(self[e41]))
                - (other.group4().yzxz() * self.group3().zxy().with_w(self[e43]))
                - (self.group4().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
                - (other.group2().yzx() * self.group1().zxy()).with_w(other[e42] * self[e431])
                - (self.group2().zxy() * other.group1().yzx()).with_w(other[e431] * self[e42]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group2())
                + (other.group2().zxy() * self.group2().yzx())
                + (other.group4().yzx() * self.group4().zxy())
                - (Simd32x3::from(other[e4]) * self.group4().xyz())
                - (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group4().xxy())
                - (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group4().zyz())
                - (other.group2().yzx() * self.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from([self[e4], self[e412], self[e423]]) * other.group1().xxy())
                + (Simd32x3::from([self[e431], self[e4], self[e4]]) * other.group1().zyz())
                + (other.group2().zxy() * self.group3().yzx())
                + (other.group3().zxy() * self.group2().yzx())
                + (other.group4().yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e2], self[e321], self[e321]]) * other.group4().zyz())
                - (Simd32x3::from([self[e321], self[e3], self[e1]]) * other.group4().xxy())
                - (other.group2().yzx() * self.group3().zxy())
                - (other.group3().yzx() * self.group2().zxy())
                - (other.group1().yzx() * self.group4().zxy()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4())
                + (Simd32x4::from([self[e4], self[e412], self[e423], other[e321]]) * other.group2().xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e431], self[e4], self[e4], self[e423]]) * other.group2().zyz().with_w(other[e23]))
                + (self.group0().yy().with_zw(self[e1234], self[scalar]) * other.group4().xyz().with_w(other[e4]))
                + (other.group1().ww().with_zw(other[e431], self[e431]) * self.group2().xyx().with_w(other[e31]))
                + (other.group4().zx().with_zw(other[e4], self[e412]) * self.group2().yzz().with_w(other[e12]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e1] * self[e41])
                        - (other[e2] * self[e42])
                        - (other[e3] * self[e43])
                        - (other[e42] * self[e2])
                        - (other[e43] * self[e3])
                        - (other[e423] * self[e23])
                        - (other[e431] * self[e31])
                        - (other[e412] * self[e12]),
                )
                - (other.group2().yzx() * self.group4().zxy()).with_w(other[scalar] * self[e4])
                - (self.group2().zxy() * other.group4().yzx()).with_w(other[e41] * self[e1]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       44        0
    //    simd2        8       10        0
    //    simd3       22       34        0
    //    simd4       16       15        0
    // Totals...
    // yes simd       81      103        0
    //  no simd      181      226        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * self.group3().with_w(self[e1234]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * self.group2().with_w(self[scalar]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       61        0
    //    simd2       11       11        0
    //    simd3       27       37        0
    //    simd4       19       16        0
    // Totals...
    // yes simd      100      125        0
    //  no simd      222      258        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * other[e321], 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[scalar] * other[e423]) + (self[e41] * other[e321]) + (self[e31] * other[e412]),
                (self[scalar] * other[e431]) + (self[e42] * other[e321]) + (self[e12] * other[e423]),
                (self[scalar] * other[e412]) + (self[e43] * other[e321]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (self.group4().zxy() * other.group0().yzx()) - (Simd32x3::from(self[e4]) * other.group0().xyz()) - (self.group4().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().zxy() * other.group0().yzx())
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e42] * other[e412],
                self[e43] * other[e423],
                self[e41] * other[e431],
                -(self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0())
                - (other.group0().yzxx() * self.group2().zxy().with_w(self[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       54        0
    //    simd2        8        9        0
    //    simd3       28       41        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       93      117        0
    //  no simd      205      247        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
                self[e4] * other[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(self[e1234]) * other.group0().xyz()) + (Simd32x3::from(other[e4]) * self.group3()) + (self.group2().yzx() * other.group0().zxy())
                - (self.group2().zxy() * other.group0().yzx()))
            .with_w(self[e1234] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().yzx() * other.group0().zxy())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (self.group4().zxy() * other.group0().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group2()).with_w((self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       46        0
    //    simd2        8        9        0
    //    simd3       22       33        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       81      102        0
    //  no simd      181      219        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e1234] * other[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[scalar]) * self.group4().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e4] * other[scalar]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
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
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       23        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       26        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e4]) * other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0()).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       33        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        4        0
    //    simd3        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       54        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * other.group3().with_w(other[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * other.group2().with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4] * -1.0).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e4]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e4] * other[e321]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e4] * other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0).geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Plane {
    type Output = AntiSandwichInfixPartial<Plane>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        8       22        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd3        0        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       20       38        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyz() * other.group0().xx().with_z(other[scalar]) * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other[e4] * self[e423]) - (other[e412] * self[e431]),
                -(other[e4] * self[e431]) - (other[e423] * self[e412]),
                -(other[e4] * self[e412]) - (other[e431] * self[e423]),
                (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]) + (other.group1().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e431]) + (other[e321] * self[e423]),
                (other[e1] * self[e412]) + (other[e321] * self[e431]),
                (other[e2] * self[e423]) + (other[e321] * self[e412]),
                -(other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) - (other.group0().yzxx() * self.group0().zxyx())
                - (self.group0().wwwy() * other.group1().xyz().with_w(other[e2])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       13       24        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[e321]) * self.group0().xyz())
            .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       18       37        0
    //  no simd       33       52        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e42] * self[e412] * -1.0,
                other[e43] * self[e423] * -1.0,
                other[e41] * self[e431] * -1.0,
                (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]) + (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       39        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                other[e43] * self[e412] * -1.0,
            ]) - (self.group0().xyzy() * other.group1().www().with_w(other[e42]))
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e42] * self[e412] * -1.0,
                other[e43] * self[e423] * -1.0,
                other[e41] * self[e431] * -1.0,
                (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]) + (other.group0().zxyw() * self.group0().yzxw())
                + (self.group0().xyzx() * other.group0().www().with_w(other[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       40        0
    //    simd2        3        3        0
    //    simd3       10       14        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       37       64        0
    //  no simd       81      116        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                other[e43] * self[e412] * -1.0,
            ]) - (self.group0().xyzx() * other.group0().xx().with_zw(other[scalar], other[e41]))
                - (self.group0().yzxy() * other.group3().zxy().with_w(other[e42])),
            // e41, e42, e43
            (other.group4().yzx() * self.group0().zxy()) - (Simd32x3::from(other[e4]) * self.group0().xyz()) - (other.group4().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (other.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e42] * self[e412] * -1.0,
                other[e43] * self[e423] * -1.0,
                other[e41] * self[e431] * -1.0,
                (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0())
                + (self.group0().yzxx() * other.group2().zxy().with_w(other[e23])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       20       39        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e4] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd3        1        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       30        0
    //  no simd       28       46        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[e412] * self[e431] * -1.0,
                other[e423] * self[e412] * -1.0,
                other[e431] * self[e423] * -1.0,
                (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]) + (other.group0().yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       26       44        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().xyz() * other.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e431] * other[e3],
                self[e412] * other[e1],
                self[e423] * other[e2],
                -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group0().zxyx() * other.group0().yzxx()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        0        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       14        0
    //  no simd        6       25        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(other[scalar]) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0))
            .geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Point {
    type Output = AntiSandwichInfixPartial<Point>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0()).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       37        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other[e4] * self[e1]) - (other[e412] * self[e2]),
                -(other[e4] * self[e2]) - (other[e423] * self[e3]),
                -(other[e4] * self[e3]) - (other[e431] * self[e1]),
                (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group1().yzxy() * self.group0().zxyy())
                + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]).geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       22       43        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
                - (other.group0().yzx() * self.group0().zxy()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0()).with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group0().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (other.group0().yzx() * self.group0().zxy()))
            .with_w(other[e1234] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(-(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd2        0        2        0
    //    simd3       12       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       24       45        0
    //  no simd       48       94        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
                other[e4] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (other.group2().zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * other.group3())
                - (other.group2().yzx() * self.group0().zxy()))
            .with_w(other[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group4().yzx() * self.group0().zxy())
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (other.group4().zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group2()).with_w(-(other[scalar] * self[e4]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3])),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       35        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e4] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       14        0
    //    simd3        3        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       18       43        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().xyz() * self.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e412] * self[e2] * -1.0,
                other[e423] * self[e3] * -1.0,
                other[e431] * self[e1] * -1.0,
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group0().yzxx() * self.group0().zxyx()),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        4        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       15       35        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e4] * self[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0).geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<AntiSandwichInfix> for Scalar {
    type Output = AntiSandwichInfixPartial<Scalar>;
    fn div(self, _rhs: AntiSandwichInfix) -> Self::Output {
        AntiSandwichInfixPartial(self)
    }
}
impl AntiSandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e4] * self[scalar]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(self[scalar]) * other.group0())
            .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       27        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e1234] * self[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group2(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e4] * self[scalar]),
        )
        .geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0)).geometric_anti_product(self.anti_reverse());
    }
}
