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
//   Median:         0       2       0     N/A
//  Average:         2       4       0     N/A
//  Maximum:        35      41       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         0       4       0       0
//  Average:         5       9       0       0
//  Maximum:        76      81       0       0
impl std::ops::Div<WedgeInfix> for AntiScalar {
    type Output = WedgeInfixPartial<AntiScalar>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar])
    }
}
impl Wedge<Motor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar])
    }
}
impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar])
    }
}
impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar])
    }
}
impl std::ops::Div<WedgeInfix> for DualNum {
    type Output = WedgeInfixPartial<DualNum>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar])
    }
}
impl Wedge<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        3        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
        )
    }
}
impl Wedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * other[e321])
    }
}
impl Wedge<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w((self[scalar] * other[e1234]) + (self[e1234] * other[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1       17        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group4(),
        )
    }
}
impl Wedge<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[scalar] * other[e4])
    }
}
impl Wedge<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<WedgeInfix> for Flector {
    type Output = WedgeInfixPartial<Flector>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       20        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(
                    (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
                )
                - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321])),
            // e23, e31, e12, scalar
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Horizon> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e321])
    }
}
impl Wedge<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e2] * other[e31]) - (self[e3] * other[e12]))
                + (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
                + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx()),
        )
    }
}
impl Wedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            (other.group1() * Simd32x3::from(self[e4]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w(-(self[e2] * other[e31]) - (self[e3] * other[e12]))
                + (Simd32x3::from(other[scalar]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * other.group0().yzx()).with_w(0.0)
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       15       21        0      N/A
    //  no simd       33       41        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
                    - (self[e423] * other[e1])
                    - (self[e431] * other[e2])
                    - (self[e412] * other[e3])
                    - (self[e321] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (self.group0().yzx() * other.group1().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e1]) * -1.0) - (self.group0().zx() * other.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group1())
                + Simd32x3::from(0.0).with_w(-(self[e2] * other[e31]) - (self[e3] * other[e12]))
                + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx()),
        )
    }
}
impl Wedge<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4] * -1.0) * self.group0().xyz().with_w(self[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl Wedge<Plane> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
        )
    }
}
impl Wedge<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]))
                + (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
                - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321])),
            // e23, e31, e12, scalar
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl Wedge<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Horizon {
    type Output = WedgeInfixPartial<Horizon>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[scalar] * self[e321])
    }
}
impl Wedge<Flector> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e321] * -1.0)
    }
}
impl Wedge<Motor> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar])
    }
}
impl Wedge<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[e321] * other[e4] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[scalar]),
        )
    }
}
impl Wedge<Origin> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e321] * other[e4] * -1.0)
    }
}
impl Wedge<Point> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e321] * other[e4] * -1.0)
    }
}
impl Wedge<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[e321] * other[scalar])
    }
}
impl std::ops::Div<WedgeInfix> for Line {
    type Output = WedgeInfixPartial<Line>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Flector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12]))
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl Wedge<Line> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        )
    }
}
impl Wedge<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[scalar]) * self.group0()).with_w(
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ),
            // e23, e31, e12, scalar
            (Simd32x3::from(other[scalar]) * self.group1()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       18       24        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group1().yzxx()),
        )
    }
}
impl Wedge<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0))
    }
}
impl Wedge<Point> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
                + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl Wedge<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for Motor {
    type Output = WedgeInfixPartial<Motor>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiScalar> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar])
    }
}
impl Wedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w((other[scalar] * self[e1234]) + (other[e1234] * self[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl Wedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       17       20        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            (self.group1() * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12]))
                + (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[scalar])
    }
}
impl Wedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[scalar]) * other.group0()).with_w(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ),
            // e23, e31, e12, scalar
            (Simd32x3::from(self[scalar]) * other.group1()).with_w(0.0),
        )
    }
}
impl Wedge<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       21        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0())
                + (Simd32x4::from(self[scalar]) * other.group0())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[scalar] * self[scalar]),
        )
    }
}
impl Wedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        2        6        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       30       41        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[scalar] * other[scalar],
                (self[e1234] * other[scalar]) + (self[scalar] * other[e1234])
                    - (self[e41] * other[e23])
                    - (self[e42] * other[e31])
                    - (self[e43] * other[e12])
                    - (self[e23] * other[e41])
                    - (self[e31] * other[e42])
                    - (self[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * self.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * self.group1().xyz()),
            // e423, e431, e412, e321
            (self.group1() * Simd32x3::from(other[e4]).with_w(other[e321]))
                + Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
                + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
                - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl Wedge<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
                + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        )
    }
}
impl Wedge<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        )
    }
}
impl std::ops::Div<WedgeInfix> for MultiVector {
    type Output = WedgeInfixPartial<MultiVector>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar])
    }
}
impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1       17        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group4(),
        )
    }
}
impl Wedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       15       21        0      N/A
    //  no simd       33       41        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (other.group0().zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e2]) * -1.0) - (other.group0().yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group1())
                + Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12]))
                + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl Wedge<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e321] * self[e4]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[scalar]),
        )
    }
}
impl Wedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        4        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        9       13        0      N/A
    //  no simd       18       24        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
                + (other.group0().yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx()),
        )
    }
}
impl Wedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       11        0        0
    //    simd3        2        6        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       30       41        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[scalar] * self[scalar],
                (other[e1234] * self[scalar]) + (other[scalar] * self[e1234])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * other.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()),
            // e423, e431, e412, e321
            (other.group1() * Simd32x3::from(self[e4]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(other[scalar]) * self.group4().xyz()).with_w(0.0)
                + (other.group0().yzx() * self.group1().zxy()).with_w(0.0)
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       23        0        0
    //    simd2        0        2        0      N/A
    //    simd3        7       10        0      N/A
    //    simd4        9        6        0      N/A
    // Totals...
    // yes simd       35       41        0      N/A
    //  no simd       76       81        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[scalar] * self[scalar],
                (other[scalar] * self[e1234])
                    + (other[e1234] * self[scalar])
                    + (other[e423] * self[e1])
                    + (other[e431] * self[e2])
                    + (other[e412] * self[e3])
                    + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e4]) * other.group1().xyz())
                - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group3())
                + Simd32x2::from(0.0).with_z((other[e2] * self[e1]) - (other[e1] * self[e2]))
                + (other.group1().zx() * self.group1().yz()).with_z(0.0)
                - (other.group1().yz() * self.group1().zx()).with_z(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group4())
                + (Simd32x4::from(self[scalar]) * other.group4())
                + Simd32x3::from(0.0).with_w(-(other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
                + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
                + (other.group2().yzx() * self.group1().zxy()).with_w(0.0)
                + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx())
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group1().yzxx()),
        )
    }
}
impl Wedge<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, self[e321] * other[e4] * -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4] * -1.0) * self.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0),
        )
    }
}
impl Wedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group0(),
        )
    }
}
impl Wedge<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       25       33        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (self.group1().yzx() * other.group0().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e1]) * -1.0) - (self.group1().zx() * other.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
                + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
                + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx()),
        )
    }
}
impl Wedge<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
    }
}
impl std::ops::Div<WedgeInfix> for Origin {
    type Output = WedgeInfixPartial<Origin>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[scalar] * self[e4])
    }
}
impl Wedge<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl Wedge<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e321] * self[e4])
    }
}
impl Wedge<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0))
    }
}
impl Wedge<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0),
        )
    }
}
impl Wedge<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e321] * self[e4]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0),
        )
    }
}
impl Wedge<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e321])
    }
}
impl Wedge<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        1        0      N/A
    // no simd        0        3        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e4]) * other.group0().xyz(), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl Wedge<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e4] * other[scalar])
    }
}
impl std::ops::Div<WedgeInfix> for Plane {
    type Output = WedgeInfixPartial<Plane>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<Flector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
        )
    }
}
impl Wedge<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group0(),
        )
    }
}
impl Wedge<Origin> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e321] * -1.0)
    }
}
impl Wedge<Point> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        )
    }
}
impl Wedge<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<WedgeInfix> for Point {
    type Output = WedgeInfixPartial<Point>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl Wedge<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        5        7        0      N/A
    //  no simd       13       16        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
                + Simd32x3::from(0.0).with_w((other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]))
                - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0),
            // e23, e31, e12, scalar
            ((other.group0().zxy() * self.group0().yzx()) - (other.group0().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl Wedge<Horizon> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e321] * self[e4])
    }
}
impl Wedge<Line> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        1        0      N/A
    // Totals...
    // yes simd        4        5        0      N/A
    //  no simd       13       12        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
                + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx()),
        )
    }
}
impl Wedge<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       13       16        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0)
                + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        )
    }
}
impl Wedge<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       25       33        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e2]) * -1.0) - (other.group1().yz() * self.group0().zx()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
                + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
                + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx()),
        )
    }
}
impl Wedge<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        4        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4] * -1.0) * self.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        )
    }
}
impl Wedge<Plane> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        )
    }
}
impl Wedge<Point> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        9       13        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (other.group0().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e2]) * -1.0) - (other.group0().yz() * self.group0().zx()).with_z(0.0),
        )
    }
}
impl Wedge<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0())
    }
}
impl std::ops::Div<WedgeInfix> for Scalar {
    type Output = WedgeInfixPartial<Scalar>;
    fn div(self, _rhs: WedgeInfix) -> Self::Output {
        WedgeInfixPartial(self)
    }
}
impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar])
    }
}
impl Wedge<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[scalar])
    }
}
impl Wedge<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        )
    }
}
impl Wedge<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
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
    }
}
impl Wedge<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * self[scalar])
    }
}
impl Wedge<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0())
    }
}
impl Wedge<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar])
    }
}
