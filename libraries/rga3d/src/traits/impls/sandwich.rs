use crate::traits::GeometricProduct;
use crate::traits::Reverse;
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
//   Median:         9      21       0
//  Average:        21      33       0
//  Maximum:       162     187       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        19      39       0
//  Average:        45      64       0
//  Maximum:       362     394       0
impl std::ops::Div<SandwichInfix> for AntiScalar {
    type Output = SandwichInfixPartial<AntiScalar>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e1234]) * other.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0)).geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for AntiScalar {
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
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e1234] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e1234]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for DualNum {
    type Output = SandwichInfixPartial<DualNum>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       26        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])) * other.group0().xyz().with_w(1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz())).with_w(self[scalar] * other[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        4       19        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd3        4       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       16       50        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])]),
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])) * other.group1().xyz().with_w(1.0),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group4().xyz()) - (Simd32x3::from(self[e1234]) * other.group1().xyz())).with_w(self[scalar] * other[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[scalar] * other[e4]).geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        4       22        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group0(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        4       23        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            (other.group0().xyz() * self.group0().yy().with_z(self[e1234]) * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0()).geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Flector {
    type Output = SandwichInfixPartial<Flector>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        9       11        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       40       56        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        1        2        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       44       65        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])) * self.group0().xyz().with_w(1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group0().xyz())).with_w(other[scalar] * self[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       84      100        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
                + (other.group1().zxyz() * self.group0().yzxz())
                + (self.group0().ww().with_zw(self[e431], self[e1]) * other.group0().xyx().with_w(other[e423]))
                + (self.group1().zx().with_zw(self[e4], self[e2]) * other.group0().yzz().with_w(other[e431]))
                - (other.group0().zxyx() * self.group1().yzxx())
                - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431]))
                - (self.group0().zx().with_zw(self[e321], self[e321]) * other.group1().yzz().with_w(other[e4]))
                - (self.group1().ww().with_zw(self[e2], self[e412]) * other.group1().xyx().with_w(other[e3])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other[e2] * self[e3]) - (other[e321] * self[e1]),
                -(other[e3] * self[e1]) - (other[e321] * self[e2]),
                -(other[e3] * self[e321]) - (other[e321] * self[e3]),
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx())
                - (self.group1().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[e321])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       44       64        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        1        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       32       48        0
    //  no simd       68       89        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * other[e31]) + (self[e321] * other[e23]),
                (self[e1] * other[e12]) + (self[e321] * other[e31]),
                (self[e2] * other[e23]) + (self[e321] * other[e12]),
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e1] * other[e43]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e2] * other[e41]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd3        0        2        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       80      101        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e321] * other[e23],
                self[e321] * other[e31],
                self[e321] * other[e12],
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (self.group0().xxyw() * other.group1().wzxw())
                + (self.group0().zyz() * other.group1().yww()).with_w(self[e321] * other[e1234])
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e423] * other[scalar]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e431] * other[scalar]) + (self[e321] * other[e42]),
                (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e412] * other[scalar]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) + (self.group0().xxy() * other.group0().wzx()).with_w(self[e321] * other[scalar])
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxyy() * self.group1().yzx().with_w(self[e2])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       51        0
    //    simd2        8        8        0
    //    simd3       20       26        0
    //    simd4       14       13        0
    // Totals...
    // yes simd       76       98        0
    //  no simd      166      197        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group1().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e3] * other[e31],
                self[e1] * other[e12],
                self[e321] * other[e12],
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group0())
                + (self.group1().ww().with_zw(self[e2], self[e321]) * other.group3().xyx().with_w(other[e1234]))
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz())
                + (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group1().zyz())
                + (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group1().xxy())
                + (self.group0().yzx() * other.group4().zxy())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (Simd32x3::from([other[e4], other[e412], other[e423]]) * self.group0().xxy())
                - (Simd32x3::from([other[e431], other[e4], other[e4]]) * self.group0().zyz())
                - (self.group1().yzx() * other.group1().zxy()),
            // e23, e31, e12
            (self.group0().yzx() * other.group1().zxy())
                - (Simd32x3::from(self[e321]) * other.group1().xyz())
                - (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group0().zyz())
                - (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group0().xxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1] * other[e1234]) + (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e1] * other[e43]) + (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e2] * other[e41]) + (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group3().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       29        0
    //  no simd       44       60        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       56       76        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self[e3] * other[e431]) - (self[e321] * other[e423]),
                -(self[e1] * other[e412]) - (self[e321] * other[e431]),
                -(self[e2] * other[e423]) - (self[e321] * other[e412]),
                (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) + (self.group0().yzxx() * other.group0().zxyx())
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd3        0        2        0
    //    simd4       11       10        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       64       80        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e4] * other[e1]) + (self[e412] * other[e2]),
                (self[e4] * other[e2]) + (self[e423] * other[e3]),
                (self[e4] * other[e3]) + (self[e431] * other[e1]),
                -(self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group1().yzxy() * other.group0().zxyy())
                - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self[e3] * other[e2]) - (self[e321] * other[e1]),
                -(self[e1] * other[e3]) - (self[e321] * other[e2]),
                -(self[e2] * other[e1]) - (self[e321] * other[e3]),
                (self[e2] * other[e2]) + (self[e3] * other[e3]),
            ]) + (self.group0().yzxx() * other.group0().zxyx()),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        9       13        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       40       60        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Horizon {
    type Output = SandwichInfixPartial<Horizon>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e321]).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       33        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0).geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       19        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[scalar]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Horizon {
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
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]),
            // e41, e42, e43
            Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * other.group2().with_w(other[scalar]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e321] * other[e4] * -1.0).geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       28        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       28        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e321] * other[e4]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e321] * other[scalar]).geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Line {
    type Output = SandwichInfixPartial<Line>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        6        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       19       36        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(other[e1234]) * self.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0)).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        8        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       22       42        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd3        0        5        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       33       51        0
    //  no simd       60       79        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                - (self.group1().yzx() * other.group0().zxy()).with_w(other[e423] * self[e23]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]),
                0.0,
            ]) - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(other[e3] * self[e12]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        6        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       28       52        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group1()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e321]) * self.group0() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       43        0
    //    simd3        0        6        0
    //    simd4        6        2        0
    // Totals...
    // yes simd       29       51        0
    //  no simd       47       69        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e42] * self[e12]) + (other[e31] * self[e43]),
                (other[e43] * self[e23]) + (other[e12] * self[e41]),
                (other[e41] * self[e31]) + (other[e23] * self[e42]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (other.group0().zxy() * self.group1().yzx()).with_w(other[e41] * self[e23])
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e31] * self[e12],
                other[e12] * self[e23],
                other[e23] * self[e31],
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) - (other.group1().zxy() * self.group1().yzx()).with_w(other[e23] * self[e23]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       50        0
    //    simd3        0        4        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       56       78        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42]),
                (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234]),
                (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234]),
                -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]) - (other.group1().zxyx() * self.group0().yzx().with_w(self[e41]))
                - (self.group1().yzx() * other.group0().zxy()).with_w(self[e42] * other[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e23] * other[scalar]) + (self[e12] * other[e31]),
                (self[e23] * other[e12]) + (self[e31] * other[scalar]),
                (self[e31] * other[e23]) + (self[e12] * other[scalar]),
                -(self[e31] * other[e31]) - (self[e12] * other[e12]),
            ]) - (other.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       46        0
    //    simd2        6        6        0
    //    simd3       14       23        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       57       81        0
    //  no simd      118      151        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e23] * other[e423]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0())
                + (Simd32x3::from(other[e1234]) * self.group1())
                + (self.group0().zxy() * other.group3().yzx())
                + (self.group1().zxy() * other.group2().yzx())
                - (self.group0().yzx() * other.group3().zxy())
                - (self.group1().yzx() * other.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1()) + (self.group1().zxy() * other.group3().yzx()) - (self.group1().yzx() * other.group3().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e12] * other[e431]),
                (self[e43] * other[e1]) + (self[e23] * other[e412]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e31] * other[e423]) + (self[e12] * other[e4]),
                0.0,
            ]) - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * other.group4().zxy()).with_w(self[e12] * other[e3]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        7        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       10       24        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)).geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        7        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       38       58        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group1()).with_w(-(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e423, e431, e412, e321
            (self.group1().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                - (self.group1().yzx() * other.group0().zxy()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       42        0
    //    simd3        0        3        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       26       49        0
    //  no simd       41       67        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        7        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       19       39        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Motor {
    type Output = SandwichInfixPartial<Motor>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       40       60        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       44       68        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd3        0        4        0
    //    simd4       14       12        0
    // Totals...
    // yes simd       42       61        0
    //  no simd       84      105        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (other.group0().xyxx() * self.group1().wwy().with_w(self[e41]))
                + (other.group0().yzzy() * self.group1().zxw().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w((other[e4] * self[scalar]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234]))
                + (self.group1().xyz() * other.group1().www()).with_w(other[e3] * self[e43])
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e423])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e431] * self[e12]) - (other[e321] * self[e41]),
                (other[e4] * self[e31]) + (other[e431] * self[scalar]) + (other[e412] * self[e23]) - (other[e321] * self[e42]),
                (other[e4] * self[e12]) + (other[e423] * self[e31]) + (other[e412] * self[scalar]) - (other[e321] * self[e43]),
                0.0,
            ]) + (other.group0().zxy() * self.group0().yzx()).with_w(other[e321] * self[scalar])
                - (other.group0().xyxx() * self.group0().wwy().with_w(self[e23]))
                - (other.group0().yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * other.group1().zxy().with_w(other[e3])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        2        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       73        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd3        0        3        0
    //    simd4        9        8        0
    // Totals...
    // yes simd       41       62        0
    //  no simd       68       92        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]),
                (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]),
                (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e41]))
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e23] * self[scalar]) + (other[e31] * self[e12]),
                (other[e31] * self[scalar]) + (other[e12] * self[e23]),
                (other[e23] * self[e31]) + (other[e12] * self[scalar]),
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        0        4        0
    //    simd4       12       10        0
    // Totals...
    // yes simd       44       66        0
    //  no simd       80      104        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]) + (other[scalar] * self[e41]),
                (other[e1234] * self[e31]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]) + (other[scalar] * self[e42]),
                (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (other.group0().xyxw() * self.group1().wwyw())
                + (other.group0().yzz() * self.group1().zxw()).with_w(other[scalar] * self[e1234])
                - (other.group0().zxyx() * self.group1().yzxx())
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e31] * self[e12]) + (other[scalar] * self[e23]),
                (other[e12] * self[e23]) + (other[scalar] * self[e31]),
                (other[e12] * self[scalar]) + (other[scalar] * self[e12]),
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) + (other.group1().xyxw() * self.group1().wwyw())
                - (other.group1().zxyx() * self.group1().yzxx()),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       51        0
    //    simd2        8        8        0
    //    simd3       20       26        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       76       99        0
    //  no simd      166      201        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12])])
                + (Simd32x2::from(other[scalar]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + (other.group1().xyzz() * self.group1().www().with_w(self[e43]))
                + Simd32x3::from(0.0).with_w((self[scalar] * other[e4]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e1234] * other[e321]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0().xyz())
                + (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (other.group2().xyx() * self.group1().wwy())
                + (other.group2().yzz() * self.group1().zxw())
                + (other.group3().xyx() * self.group0().wwy())
                + (other.group3().yzz() * self.group0().zxw())
                - (other.group2().zxy() * self.group1().yzx())
                - (other.group3().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (other.group3().xyx() * self.group1().wwy()) + (other.group3().yzz() * self.group1().zxw())
                - (other.group3().zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e23] * other[e4]) + (self[e12] * other[e431]) + (self[scalar] * other[e423]) - (self[e31] * other[e412]),
                (self[e23] * other[e412]) + (self[e31] * other[e4]) + (self[scalar] * other[e431]) - (self[e12] * other[e423]),
                (self[e31] * other[e423]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]) - (self[e23] * other[e431]),
                0.0,
            ]) + (self.group0().yzx() * other.group1().zxy()).with_w(self[scalar] * other[e321])
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (other.group1().xyzz() * self.group0().www().with_w(self[e12])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd3        0        3        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       37        0
    //  no simd       40       61        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       32        0
    //    simd3        3        7        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       28       45        0
    //  no simd       52       77        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group1().xyz())
                .with_w(-(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group1().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group0().xyz())
                - (self.group1().yzx() * other.group0().zxy()))
            .with_w(self[scalar] * other[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       43        0
    //    simd3        0        2        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       30       55        0
    //  no simd       60       89        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e43] * other[e3]) + (self[scalar] * other[e4]),
            ]) + (other.group0().xyzy() * self.group1().www().with_w(self[e42]))
                + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                self[e12] * other[e3] * -1.0,
            ]) - (other.group0().xyzy() * self.group0().www().with_w(self[e31]))
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       64        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for MultiVector {
    type Output = SandwichInfixPartial<MultiVector>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for MultiVector {
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
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e1234] * self[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for MultiVector {
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
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
            // e1, e2, e3, e4
            other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])) * self.group1().xyz().with_w(1.0),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group4().xyz()) + (Simd32x3::from(other[e1234]) * self.group1().xyz())).with_w(other[scalar] * self[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       65        0
    //    simd2       12       12        0
    //    simd3       32       43        0
    //    simd4       24       20        0
    // Totals...
    // yes simd      118      140        0
    //  no simd      266      298        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group0())
                + (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group3().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group3().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                - (self.group3().yzx() * other.group0().zxy()).with_w(other[e321] * self[e1234]),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * other.group0().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * other.group0().yzz())
                + (other.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
                - (other.group0().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (other.group0().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group0().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group0().xyx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]) - (other[e412] * self[e31]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]) - (other[e423] * self[e12]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]) - (other[e431] * self[e23]),
                0.0,
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e3]]) * self.group2().zyz().with_w(self[e12]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e2]]) * self.group2().xxy().with_w(self[e31]))
                - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       44        0
    //    simd2        8       10        0
    //    simd3       22       33        0
    //    simd4       16       17        0
    // Totals...
    // yes simd       81      104        0
    //  no simd      181      231        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * self.group2().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       73        0
    //    simd2       11       11        0
    //    simd3       29       40        0
    //    simd4       19       15        0
    // Totals...
    // yes simd      112      139        0
    //  no simd      238      275        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e23] * self[e321]) + (other[e31] * self[e3]),
                (other[e31] * self[e321]) + (other[e12] * self[e1]),
                (other[e23] * self[e2]) + (other[e12] * self[e321]),
                -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0())
                + (Simd32x3::from(self[e1234]) * other.group1())
                + (other.group0().yzx() * self.group3().zxy())
                + (other.group1().yzx() * self.group2().zxy())
                - (other.group0().zxy() * self.group3().yzx())
                - (other.group1().zxy() * self.group2().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1()) + (other.group1().yzx() * self.group3().zxy()) - (other.group1().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e12] * self[e423]),
                (other[e41] * self[e2]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e12] * self[e4]),
                other[e12] * self[e3] * -1.0,
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxy() * self.group4().yzx()).with_w(other[e31] * self[e2]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       73        0
    //    simd2       12       12        0
    //    simd3       32       42        0
    //    simd4       22       19        0
    // Totals...
    // yes simd      120      146        0
    //  no simd      262      299        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12])])
                + (Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar], other[e1234]]))
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]) + (other.group1().yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + (self.group4().ww().with_zw(self[e2], self[e321]) * other.group1().xyx().with_w(other[e1234]))
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0().xyz())
                + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (self.group2().xxy() * other.group1().wzx())
                + (self.group2().zyz() * other.group1().yww())
                + (self.group3().xxy() * other.group0().wzx())
                + (self.group3().zyz() * other.group0().yww())
                - (self.group2().yzx() * other.group1().zxy())
                - (self.group3().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (self.group3().xxy() * other.group1().wzx()) + (self.group3().zyz() * other.group1().yww())
                - (self.group3().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) + (other[scalar] * self[e423]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) + (other[scalar] * self[e431]),
                (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) + (other[scalar] * self[e412]),
                other[e12] * self[e3] * -1.0,
            ]) + (self.group4().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[scalar]))
                - (other.group1().zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for MultiVector {
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
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ]) + (Simd32x2::from(other[scalar]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from([other[e2], other[e321], other[e321], other[e3]]) * self.group3().zyz().with_w(self[e43]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e2]]) * self.group3().xxy().with_w(self[e42]))
                + (self.group0().xx().with_zw(self[scalar], other[e1234]) * other.group1().xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], other[e1]) * other.group3().yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], other[e4]) * other.group3().xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e42] * self[e2])
                        - (other[e43] * self[e3])
                        - (other[e23] * self[e423])
                        - (other[e31] * self[e431])
                        - (other[e12] * self[e412])
                        - (other[e423] * self[e23])
                        - (other[e431] * self[e31])
                        - (other[e412] * self[e12]),
                )
                - (other.group3().zxy() * self.group1().yzx()).with_w(other[e321] * self[e1234])
                - (self.group3().yzx() * other.group1().zxy()).with_w(other[e41] * self[e1]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * other.group1().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * other.group1().yzz())
                + (other.group2().yzx() * self.group3().zxy())
                + (other.group3().yzx() * self.group2().zxy())
                + (other.group4().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group4().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group4().xyx())
                - (other.group2().zxy() * self.group3().yzx())
                - (other.group3().zxy() * self.group2().yzx())
                - (other.group1().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group3())
                + (other.group3().yzx() * self.group3().zxy())
                + (other.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
                - (other.group3().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42])
                    + (other[e4] * self[e23])
                    + (other[e41] * self[e321])
                    + (other[e42] * self[e3])
                    + (other[e23] * self[e4])
                    + (other[e31] * self[e412])
                    + (other[e423] * self[scalar])
                    + (other[e431] * self[e12]),
                (other[e1] * self[e43])
                    + (other[e4] * self[e31])
                    + (other[e42] * self[e321])
                    + (other[e43] * self[e1])
                    + (other[e31] * self[e4])
                    + (other[e12] * self[e423])
                    + (other[e431] * self[scalar])
                    + (other[e412] * self[e23]),
                (other[e2] * self[e41])
                    + (other[e4] * self[e12])
                    + (other[e41] * self[e2])
                    + (other[e43] * self[e321])
                    + (other[e23] * self[e431])
                    + (other[e12] * self[e4])
                    + (other[e423] * self[e31])
                    + (other[e412] * self[scalar]),
                0.0,
            ]) + (Simd32x4::from(other[scalar]) * self.group4())
                + (other.group0().yy().with_zw(other[e1234], self[scalar]) * self.group1().xyz().with_w(other[e321]))
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group2().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group2().xxy().with_w(self[e23]))
                - (self.group1().yzxy() * other.group2().zxy().with_w(other[e31]))
                - (self.group0().yy().with_zw(self[e1234], other[e23]) * other.group1().xyz().with_w(self[e1]))
                - (other.group3().zxy() * self.group4().yzx()).with_w(other[e12] * self[e3])
                - (self.group3().yzx() * other.group4().zxy()).with_w(other[e3] * self[e12]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       46        0
    //    simd2        8        9        0
    //    simd3       22       33        0
    //    simd4       16       13        0
    // Totals...
    // yes simd       81      101        0
    //  no simd      181      215        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for MultiVector {
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
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[e321] * other[e321],
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * self.group3()).with_w(-(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().yzx() * other.group0().zxy())
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().zxy() * other.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group3().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group2())
                - (self.group3().yzx() * other.group0().zxy()))
            .with_w(self[scalar] * other[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       67        0
    //    simd2        8        8        0
    //    simd3       27       37        0
    //    simd4       20       17        0
    // Totals...
    // yes simd       99      129        0
    //  no simd      221      262        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0())
                + (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (self.group4().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (self.group1().yzx() * other.group0().zxy()) - (Simd32x3::from(self[e321]) * other.group0().xyz()) - (self.group1().zxy() * other.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                self[e12] * other[e3] * -1.0,
            ]) - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
                - (other.group0().yzxy() * self.group2().zxy().with_w(self[e31])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for MultiVector {
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
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group4(),
        )
        .geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Origin {
    type Output = SandwichInfixPartial<Origin>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)).geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       23        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e4]) * other.group0().xyz(), /* e23, e31, e12 */ Simd32x3::from(0.0)).geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Plane {
    type Output = SandwichInfixPartial<Plane>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e321]).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       33        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group0(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e2] * self[e412]) + (other[e321] * self[e423]),
                (other[e3] * self[e423]) + (other[e321] * self[e431]),
                (other[e1] * self[e431]) + (other[e321] * self[e412]),
                -(other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) - (other.group0().zxyx() * self.group0().yzxx())
                - (self.group0().wwwy() * other.group1().xyz().with_w(other[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       32        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e321] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       22       43        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group1()).with_w(-(other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0) + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group0().yzx()).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd3        3        5        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       12       23        0
    //  no simd       24       48        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group1().xyz())
                .with_w((other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                - (other.group1().zxy() * self.group0().yzx()))
            .with_w(other[scalar] * self[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Plane {
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
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[e321] * self[e321],
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321]) * other.group3()).with_w((other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (other.group1().zxy() * self.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group2()) + (other.group3().yzx() * self.group0().zxy())
                - (other.group3().zxy() * self.group0().yzx()))
            .with_w(other[scalar] * self[e321]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e4] * self[e321] * -1.0).geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        4        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       15       35        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e321] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        3        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       18       40        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                self[e412] * other[e2],
                self[e423] * other[e3],
                self[e431] * other[e1],
                -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group0().yzxx() * other.group0().zxyx()),
            // e23, e31, e12, scalar
            (other.group0().xyz() * self.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       19        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0()).geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Point {
    type Output = SandwichInfixPartial<Point>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       19        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       20       35        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            (self.group0().xyz() * other.group0().yy().with_z(other[e1234])).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       39        0
    //  no simd       40       60        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other[e4] * self[e1]) - (other[e431] * self[e3]),
                -(other[e4] * self[e2]) - (other[e412] * self[e1]),
                -(other[e4] * self[e3]) - (other[e423] * self[e2]),
                (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group1().zxyy() * self.group0().yzxy())
                + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other[e2] * self[e3]) - (other[e321] * self[e1]),
                -(other[e3] * self[e1]) - (other[e321] * self[e2]),
                -(other[e1] * self[e2]) - (other[e321] * self[e3]),
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       17        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       20       39        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e321] * self[e4]),
            // e23, e31, e12, scalar
            (Simd32x3::from(other[e321]) * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       29        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       33       49        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e23] * self[e4]),
                (other[e43] * self[e1]) + (other[e31] * self[e4]),
                (other[e41] * self[e2]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       56        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                -(other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]) + (other.group1().yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]),
                (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Point {
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
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e321] * self[e4]])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
                + (Simd32x4::from(other[scalar]) * self.group0())
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group4().zxy() * self.group0().yzx())
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (other.group4().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (other.group1().zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0().xyz()) - (other.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e1234] * self[e1]) + (other[e42] * self[e3]) + (other[e23] * self[e4]),
                (other[e1234] * self[e2]) + (other[e43] * self[e1]) + (other[e31] * self[e4]),
                (other[e1234] * self[e3]) + (other[e41] * self[e2]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23])),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       20        0
    //  no simd       13       30        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       25        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       11       32        0
    //  no simd       26       51        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[e431] * self[e3] * -1.0,
                other[e412] * self[e1] * -1.0,
                other[e423] * self[e2] * -1.0,
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            (self.group0().xyz() * other.group0().www() * Simd32x3::from(-1.0)).with_w(0.0),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       24        0
    //    simd3        1        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       11       31        0
    //  no simd       28       50        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e2] * self[e3] * -1.0,
                other[e3] * self[e1] * -1.0,
                other[e1] * self[e2] * -1.0,
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        8       22        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0()).geometric_product(self.reverse());
    }
}
impl std::ops::Div<SandwichInfix> for Scalar {
    type Output = SandwichInfixPartial<Scalar>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]).geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar]) * other.group0()).geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * self[scalar]).geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       32        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group4(),
        )
        .geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e4] * self[scalar]).geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0()).geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0()).geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]).geometric_product(self.reverse());
    }
}
