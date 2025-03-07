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
//   Median:         5      11       0
//  Average:        14      22       0
//  Maximum:       140     163       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         9      26       0
//  Average:        35      51       0
//  Maximum:       373     394       0
impl std::ops::Div<SandwichInfix> for DualNum {
    type Output = SandwichInfixPartial<DualNum>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for DualNum {
    type Output = AntiScalar;
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[scalar], 2))
    }
}
impl Sandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[scalar] * self[scalar];
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product_g0_x * self[scalar],
            (geometric_product_g0_x * self[e1234]) + (self[scalar] * self[scalar] * other[e1234]) + (other[scalar] * self[scalar] * self[e1234]),
        ]))
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
        let geometric_product_g0 = self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])) * other.group0().xyz().with_w(1.0);
        let geometric_product_g1_w = self[scalar] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            self.group0()
                .xx()
                .with_zw(self[scalar], (geometric_product_g1_w * self[e1234]) + (geometric_product_g0[3] * self[scalar]))
                * geometric_product_g0.xyz().with_w(1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * ((Simd32x3::from(self[scalar]) * other.group1().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz())))
                + (Simd32x3::from(self[e1234]) * geometric_product_g0.xyz()))
            .with_w(geometric_product_g1_w * self[scalar]),
        )
    }
}
impl Sandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * self[scalar] * other[e321])
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
        let geometric_product_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Line::from_groups(
            // e41, e42, e43
            (geometric_product_g1 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[scalar]) * ((Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()))),
            // e23, e31, e12
            geometric_product_g1 * Simd32x3::from(self[scalar]),
        )
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
        let geometric_product_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(self[scalar]) * ((Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()))),
            // e23, e31, e12, scalar
            geometric_product_g1 * Simd32x4::from(self[scalar]),
        )
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
        let geometric_product_g0_x = self[scalar] * other[scalar];
        let geometric_product_g1 = self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])) * other.group1().xyz().with_w(1.0);
        let geometric_product_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let geometric_product_g4_w = self[scalar] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0_x * self[scalar],
                (geometric_product_g0_x * self[e1234]) + (self[scalar] * self[scalar] * other[e1234]) + (self[scalar] * self[e1234] * other[scalar]),
            ]),
            // e1, e2, e3, e4
            self.group0()
                .xx()
                .with_zw(self[scalar], (geometric_product_g4_w * self[e1234]) + (geometric_product_g1[3] * self[scalar]))
                * geometric_product_g1.xyz().with_w(1.0),
            // e41, e42, e43
            (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[scalar]) * ((Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()))),
            // e23, e31, e12
            geometric_product_g3 * Simd32x3::from(self[scalar]),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * ((Simd32x3::from(self[scalar]) * other.group4().xyz()) - (Simd32x3::from(self[e1234]) * other.group1().xyz())))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()))
            .with_w(geometric_product_g4_w * self[scalar]),
        )
    }
}
impl Sandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[scalar] * self[scalar] * other[e4])
    }
}
impl Sandwich<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       16        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x4::from(self[scalar]) * other.group0();
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * geometric_product_g1.xyz())
                + (Simd32x3::from(self[e1234]) * (Simd32x3::from(0.0).with_w(self[e1234] * other[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])).xyz()))
            .with_w(geometric_product_g1[3] * self[scalar]),
        )
    }
}
impl Sandwich<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(self[scalar]) * other.group0();
        Point::from_groups(
            // e1, e2, e3, e4
            self.group0().xx().with_zw(self[scalar], geometric_product_g0[3] * self[scalar]) * geometric_product_g0.xyz().with_w(1.0),
        )
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
        let geometric_product_g0 = Simd32x2::from(other[scalar]) * self.group0();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            geometric_product_g0[0] * self[scalar],
            (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar]),
        ]))
    }
}
impl std::ops::Div<SandwichInfix> for Flector {
    type Output = SandwichInfixPartial<Flector>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        6        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e1234]) * self.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            -(geometric_product_g1_xyz[0] * self[e1])
                - (geometric_product_g1_xyz[1] * self[e2])
                - (geometric_product_g1_xyz[2] * self[e3])
                - (other[e1234] * f32::powi(self[e321], 2)),
        )
    }
}
impl Sandwich<DualNum> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        7        0
    //    simd2        4        4        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       15       29        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])) * self.group0().xyz().with_w(1.0);
        let geometric_product_g1_xyz = (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let geometric_product_g1_w = other[scalar] * self[e321];
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[3] * reverse_g1[3])
                    - (geometric_product_g1_w * self[e4])
                    - (geometric_product_g1_xyz[1] * self[e2])
                    - (geometric_product_g1_xyz[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * Simd32x2::from([self[e1], reverse_g1[0]]))
                + (Simd32x2::from(geometric_product_g0[1]) * Simd32x2::from([self[e2], reverse_g1[1]]))
                + (Simd32x2::from(geometric_product_g0[2]) * Simd32x2::from([self[e3], reverse_g1[2]]))
                - (Simd32x2::from([reverse_g1[3], self[e1]]) * geometric_product_g1_xyz.with_w(geometric_product_g1_w).wx()),
        )
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        0        6        0
    //    simd4       20       17        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       87      100        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
            + (other.group1().zxyz() * self.group0().yzxz())
            + (self.group0().ww().with_zw(self[e431], self[e1]) * other.group0().xyx().with_w(other[e423]))
            + (self.group1().zx().with_zw(self[e4], self[e2]) * other.group0().yzz().with_w(other[e431]))
            - (other.group0().zxyx() * self.group1().yzxx())
            - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431]))
            - (self.group0().zx().with_zw(self[e321], self[e321]) * other.group1().yzz().with_w(other[e4]))
            - (self.group1().ww().with_zw(self[e2], self[e412]) * other.group1().xyx().with_w(other[e3]));
        let geometric_product_g1 = Simd32x4::from([
            -(other[e2] * self[e3]) - (other[e321] * self[e1]),
            -(other[e3] * self[e1]) - (other[e321] * self[e2]),
            -(other[e3] * self[e321]) - (other[e321] * self[e3]),
            (other[e2] * self[e2]) + (other[e3] * self[e3]),
        ]) + (other.group0().zxyx() * self.group0().yzxx())
            - (self.group1().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[e321]));
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyxx() * geometric_product_g1.wwy().with_w(geometric_product_g0[0]))
                + (self.group0().yzzy() * geometric_product_g1.zxw().with_w(geometric_product_g0[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[3] * self[e4])
                        - (geometric_product_g0[3] * reverse_g1[3])
                        - (geometric_product_g1[1] * reverse_g1[1])
                        - (geometric_product_g1[2] * reverse_g1[2]),
                )
                + (geometric_product_g1.xyz() * reverse_g1.www()).with_w(geometric_product_g0[2] * self[e3])
                - (geometric_product_g1.yzxx() * self.group0().zxy().with_w(reverse_g1[0])),
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g1[3]) * reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g1.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g1.xxy()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(geometric_product_g1[3] * reverse_g1[3])
                - (geometric_product_g1.yzxz() * reverse_g1.zxy().with_w(self[e3]))
                - (self.group0().xyxx() * geometric_product_g0.wwy().with_w(geometric_product_g1[0]))
                - (self.group0().yzzy() * geometric_product_g0.zxw().with_w(geometric_product_g1[1]))
                - (Simd32x3::from(reverse_g1[3]) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        5        0
    //    simd4        7        7        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       28       44        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]);
        let geometric_product_g1 = Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g1[3]) * reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g1.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g1.xxy()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(geometric_product_g1[3] * reverse_g1[3])
                - (geometric_product_g1.yzxz() * reverse_g1.zxy().with_w(self[e3]))
                - (self.group0().xyxx() * geometric_product_g0.wwy().with_w(geometric_product_g1[0]))
                - (self.group0().yzzy() * geometric_product_g0.zxw().with_w(geometric_product_g1[1]))
                - (Simd32x3::from(reverse_g1[3]) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Line> for Flector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       20        0
    //    simd3       12       15        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       58       77        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            (self[e3] * other[e31]) + (self[e321] * other[e23]),
            (self[e1] * other[e12]) + (self[e321] * other[e31]),
            (self[e2] * other[e23]) + (self[e321] * other[e12]),
            -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
        ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = (Simd32x3::from([
            (self[e3] * other[e42]) + (self[e412] * other[e31]),
            (self[e1] * other[e43]) + (self[e423] * other[e12]),
            (self[e2] * other[e41]) + (self[e431] * other[e23]),
        ]) + (Simd32x3::from(self[e4]) * other.group1())
            + (Simd32x3::from(self[e321]) * other.group0()))
        .with_w(self[e3] * other[e12] * -1.0)
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
            - (other.group1().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                + (Simd32x3::from([reverse_g1[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                + (Simd32x3::from([self[e2], reverse_g1[3], reverse_g1[3]]) * geometric_product_g1.zyz())
                + (geometric_product_g0.yzx() * reverse_g1.zxy())
                - (Simd32x3::from(geometric_product_g1[3]) * reverse_g1.xyz())
                - (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g0.zyz())
                - (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g0.xxy())
                - (geometric_product_g1.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product_g0.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (Simd32x3::from([reverse_g1[3], self[e3], self[e1]]) * geometric_product_g0.xxy())
                - (Simd32x3::from([self[e2], reverse_g1[3], reverse_g1[3]]) * geometric_product_g0.zyz()),
        )
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd3        3        6        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       29       44        0
    //  no simd       80      101        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (self.group0().xxyw() * other.group1().wzxw())
            + (Simd32x3::from(self[e321]) * other.group1().xyz())
                .with_w(-(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]))
            + (self.group0().zyz() * other.group1().yww()).with_w(self[e321] * other[e1234])
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = (self.group0().xxy() * other.group0().wzx()).with_w(self[e321] * other[scalar])
            + (Simd32x3::from([
                (self[e3] * other[e42]) + (self[e412] * other[e31]),
                (self[e2] * other[e1234]) + (self[e423] * other[e12]),
                (self[e3] * other[e1234]) + (self[e431] * other[e23]),
            ]) + (Simd32x3::from(self[e4]) * other.group1().xyz())
                + (Simd32x3::from(self[e321]) * other.group0().xyz())
                + (Simd32x3::from(other[scalar]) * self.group1().xyz()))
            .with_w(self[e3] * other[e12] * -1.0)
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
            - (other.group1().zxyy() * self.group1().yzx().with_w(self[e2]));
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([reverse_g1[3], self[e3], self[e1], reverse_g1[2]]) * geometric_product_g1.xxy().with_w(geometric_product_g0[2]))
                + (Simd32x4::from([self[e2], reverse_g1[3], reverse_g1[3], reverse_g1[3]]) * geometric_product_g1.zyz().with_w(geometric_product_g0[3]))
                + (geometric_product_g0.yzxx() * reverse_g1.zxyx())
                + (geometric_product_g0.wwwy() * self.group0().xyz().with_w(reverse_g1[1]))
                - (Simd32x4::from(geometric_product_g1[3]) * reverse_g1.xyz().with_w(self[e4]))
                - (Simd32x4::from([reverse_g1[1], self[e4], self[e4], self[e2]]) * geometric_product_g0.zyz().with_w(geometric_product_g1[1]))
                - (Simd32x4::from([self[e4], reverse_g1[2], reverse_g1[0], self[e1]]) * geometric_product_g0.xxy().with_w(geometric_product_g1[0]))
                - (geometric_product_g1.yzxz() * self.group0().zxyz()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[2] * self[e2]) - (geometric_product_g1[3] * self[e1]),
                -(geometric_product_g0[1] * reverse_g1[3]) - (geometric_product_g1[3] * self[e2]),
                -(geometric_product_g0[2] * reverse_g1[3]) - (geometric_product_g1[3] * self[e3]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.yzxx() * self.group0().zxyx())
                - (Simd32x4::from([reverse_g1[3], self[e3], self[e1], reverse_g1[3]]) * geometric_product_g0.xxy().with_w(geometric_product_g1[3])),
        )
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd2        8        8        0
    //    simd3       23       33        0
    //    simd4       17       13        0
    // Totals...
    // yes simd       64       84        0
    //  no simd      169      197        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])])
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
            - (Simd32x2::from([other[e321], other[e1]]) * self.group1().wx());
        let geometric_product_g1 = Simd32x4::from([
            self[e3] * other[e31],
            self[e1] * other[e12],
            self[e321] * other[e12],
            -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
        ]) + (Simd32x4::from(other[scalar]) * self.group0())
            + (self.group1().ww().with_zw(self[e2], self[e321]) * other.group3().xyx().with_w(other[e1234]))
            - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group1().zyz())
            + (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group1().xxy())
            + (self.group0().yzx() * other.group4().zxy())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (Simd32x3::from([other[e4], other[e412], other[e423]]) * self.group0().xxy())
            - (Simd32x3::from([other[e431], other[e4], other[e4]]) * self.group0().zyz())
            - (self.group1().yzx() * other.group1().zxy());
        let geometric_product_g3 = (self.group0().yzx() * other.group1().zxy())
            - (Simd32x3::from(self[e321]) * other.group1().xyz())
            - (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group0().zyz())
            - (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group0().xxy());
        let geometric_product_g4 = (Simd32x4::from(other[scalar]) * self.group1())
            + (Simd32x3::from([
                (self[e3] * other[e42]) + (self[e412] * other[e31]),
                (self[e1] * other[e43]) + (self[e423] * other[e12]),
                (self[e2] * other[e41]) + (self[e431] * other[e23]),
            ]) + (Simd32x3::from(self[e4]) * other.group3())
                + (Simd32x3::from(self[e321]) * other.group2())
                + (Simd32x3::from(other[e1234]) * self.group0().xyz()))
            .with_w(self[e3] * other[e12] * -1.0)
            - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]))
            - (other.group3().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * reverse_g1[3]) - (geometric_product_g4[1] * self[e2]) - (geometric_product_g4[2] * self[e3]) - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g1[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g1[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g1[2]]))
                - (Simd32x2::from([reverse_g1[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x4::from([reverse_g1[3], self[e3], self[e1], self[e1]]) * geometric_product_g3.xxy().with_w(geometric_product_g2[0]))
                + (Simd32x4::from([self[e2], reverse_g1[3], reverse_g1[3], self[e2]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * reverse_g1[0])
                        - (geometric_product_g3[1] * reverse_g1[1])
                        - (geometric_product_g3[2] * reverse_g1[2]),
                )
                - (geometric_product_g3.yzx() * self.group0().zxy()).with_w(geometric_product_g0[1] * reverse_g1[3]),
            // e41, e42, e43
            (Simd32x3::from(reverse_g1[3]) * geometric_product_g4.xyz())
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * self.group0().xyx())
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * self.group0().yzz())
                + (geometric_product_g1.yzx() * reverse_g1.zxy())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g1.yzz())
                - (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g1.xyx())
                - (geometric_product_g4.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product_g1.yzx() * self.group0().zxy())
                - (Simd32x3::from(reverse_g1[3]) * geometric_product_g1.xyz())
                - (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * self.group0().yzz())
                - (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * self.group0().xyx()),
            // e423, e431, e412, e321
            (reverse_g1 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g1[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g1[3], reverse_g1[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (self.group0().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g1.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        Origin::from_groups(
            // e4
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e321]),
        )
    }
}
impl Sandwich<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        5        0
    //    simd4        9        8        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       40       56        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            -(self[e3] * other[e431]) - (self[e321] * other[e423]),
            -(self[e1] * other[e412]) - (self[e321] * other[e431]),
            -(self[e2] * other[e423]) - (self[e321] * other[e412]),
            (self[e3] * other[e412]) + (self[e4] * other[e321]),
        ]) + (self.group0().yzxx() * other.group0().zxyx())
            + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2]));
        let geometric_product_g1 = Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(geometric_product_g1[3]) * reverse_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g1.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g1.xxy()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(geometric_product_g1[3] * reverse_g1[3])
                - (geometric_product_g1.yzxz() * reverse_g1.zxy().with_w(self[e3]))
                - (self.group0().xyxx() * geometric_product_g0.wwy().with_w(geometric_product_g1[0]))
                - (self.group0().yzzy() * geometric_product_g0.zxw().with_w(geometric_product_g1[1]))
                - (Simd32x3::from(reverse_g1[3]) * geometric_product_g0.xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        0        1        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       39       52        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            (self[e4] * other[e1]) + (self[e412] * other[e2]),
            (self[e4] * other[e2]) + (self[e423] * other[e3]),
            (self[e4] * other[e3]) + (self[e431] * other[e1]),
            -(self[e412] * other[e3]) - (self[e321] * other[e4]),
        ]) - (self.group1().yzxy() * other.group0().zxyy())
            - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423]));
        let geometric_product_g1 = Simd32x4::from([
            -(self[e3] * other[e2]) - (self[e321] * other[e1]),
            -(self[e1] * other[e3]) - (self[e321] * other[e2]),
            -(self[e2] * other[e1]) - (self[e321] * other[e3]),
            (self[e2] * other[e2]) + (self[e3] * other[e3]),
        ]) + (self.group0().yzxx() * other.group0().zxyx());
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyxx() * geometric_product_g1.wwy().with_w(geometric_product_g0[0]))
                + (self.group0().yzzy() * geometric_product_g1.zxw().with_w(geometric_product_g0[1]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[3] * self[e4])
                        - (geometric_product_g0[3] * reverse_g1[3])
                        - (geometric_product_g1[1] * reverse_g1[1])
                        - (geometric_product_g1[2] * reverse_g1[2]),
                )
                + (geometric_product_g1.xyz() * reverse_g1.www()).with_w(geometric_product_g0[2] * self[e3])
                - (geometric_product_g1.yzxx() * self.group0().zxy().with_w(reverse_g1[0])),
        )
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       11        0
    //  no simd       11       24        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[3] * reverse_g1[3]) - (geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]) - (geometric_product_g1[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * Simd32x2::from([self[e1], reverse_g1[0]]))
                + (Simd32x2::from(geometric_product_g0[1]) * Simd32x2::from([self[e2], reverse_g1[1]]))
                + (Simd32x2::from(geometric_product_g0[2]) * Simd32x2::from([self[e3], reverse_g1[2]]))
                - (Simd32x2::from([reverse_g1[3], self[e1]]) * geometric_product_g1.wx()),
        )
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
    // f32        0        1        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[e321], 2) * -1.0)
    }
}
impl Sandwich<DualNum> for Horizon {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321] * -1.0) * Simd32x2::from([other[scalar] * self[e321], other[e1234] * self[e321]]) * Simd32x2::from([-1.0, 1.0]),
        )
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
        let geometric_product_g0 = Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0);
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0);
        let reverse_g0 = self[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(reverse_g0) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(reverse_g0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Sandwich<Horizon> for Horizon {
    type Output = Horizon;
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * f32::powi(self[e321], 2))
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self[e321] * -1.0;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(reverse_g0) * Simd32x3::from(self[e321]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(reverse_g0) * Simd32x3::from(self[e321]) * other.group1() * Simd32x3::from(-1.0),
        )
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
        let geometric_product_g0 = Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e1234]);
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[scalar]);
        let reverse_g0 = self[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(reverse_g0) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from(reverse_g0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]) * Simd32x4::from(-1.0),
        )
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
        let geometric_product_g0 = Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from(-1.0);
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]);
        let geometric_product_g4 = Simd32x4::from(self[e321]) * other.group2().with_w(other[scalar]);
        let reverse_g0 = self[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(reverse_g0) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(reverse_g0)
                * (Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(geometric_product_g0[1])
                * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(reverse_g0) * geometric_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(reverse_g0) * geometric_product_g1.xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(reverse_g0)
                * (Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0)).with_w(geometric_product_g0[0])
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Sandwich<Origin> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e321] * self[e321] * other[e4] * -1.0)
    }
}
impl Sandwich<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       17        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e321] * -1.0)
                * (Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] * other[e321] * -1.0)
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       17        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321] * -1.0)
                * (Simd32x3::from(self[e321]) * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[e321] * other[e4] * -1.0)
                * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl Sandwich<Scalar> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] * other[scalar])
    }
}
impl std::ops::Div<SandwichInfix> for Line {
    type Output = SandwichInfixPartial<Line>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[e1234]) * self.group1();
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        AntiScalar::from_groups(
            // e1234
            -(geometric_product_g0[0] * reverse_g1[0]) - (geometric_product_g0[1] * reverse_g1[1]) - (geometric_product_g0[2] * reverse_g1[2]),
        )
    }
}
impl Sandwich<DualNum> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd2        3        3        0
    //    simd3        1        5        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       11       24        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1());
        let geometric_product_g1 = Simd32x3::from(other[scalar]) * self.group1();
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g1[0] * reverse_g0[0]) - (geometric_product_g1[1] * reverse_g0[1]) - (geometric_product_g1[2] * reverse_g0[2]),
            ]) - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
        )
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       25        0
    //    simd3        3       10        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       61       79        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
            + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
            - (self.group1().yzx() * other.group0().zxy()).with_w(other[e423] * self[e23]);
        let geometric_product_g1 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e431] * self[e12]),
            (other[e1] * self[e43]) + (other[e412] * self[e23]),
            (other[e2] * self[e41]) + (other[e423] * self[e31]),
            0.0,
        ]) + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
            - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
            - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
            - (self.group1().yzx() * other.group1().zxy()).with_w(other[e3] * self[e12]);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse_g1[0] * geometric_product_g1[3]) + (reverse_g1[1] * geometric_product_g0[2]),
                (reverse_g1[1] * geometric_product_g1[3]) + (reverse_g1[2] * geometric_product_g0[0]),
                (reverse_g1[0] * geometric_product_g0[1]) + (reverse_g1[2] * geometric_product_g1[3]),
                -(reverse_g0[1] * geometric_product_g0[1])
                    - (reverse_g0[2] * geometric_product_g0[2])
                    - (reverse_g1[0] * geometric_product_g1[0])
                    - (reverse_g1[1] * geometric_product_g1[1])
                    - (reverse_g1[2] * geometric_product_g1[2]),
            ]) - (geometric_product_g0.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
            // e423, e431, e412, e321
            ((Simd32x3::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g1[3]]) * reverse_g0.yzz())
                + (Simd32x3::from([geometric_product_g0[3], geometric_product_g0[3], geometric_product_g1[1]]) * reverse_g1.xyx())
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0[3]]) * reverse_g1.yzz())
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g0[1]]) * reverse_g0.xyx()))
            .with_w(reverse_g1[2] * geometric_product_g0[2] * -1.0)
                - (geometric_product_g0.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (reverse_g1.zxy() * geometric_product_g1.yzx()).with_w(reverse_g1[1] * geometric_product_g0[1]),
        )
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        3       11        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       17       36        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1();
        let geometric_product_g1_xyz = Simd32x3::from(other[e321]) * self.group0() * Simd32x3::from(-1.0);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from([geometric_product_g0_xyz[2], geometric_product_g0_xyz[0], 0.0]) * reverse_g0.yzz())
                + (Simd32x3::from([geometric_product_g1_xyz[2], geometric_product_g1_xyz[0], 0.0]) * reverse_g1.yzz())
                + (reverse_g0.xyx() * Simd32x2::from(0.0).with_z(geometric_product_g0_xyz[1]))
                + (reverse_g1.xyx() * Simd32x2::from(0.0).with_z(geometric_product_g1_xyz[1])))
            .with_w(geometric_product_g0_xyz[2] * reverse_g1[2] * -1.0)
                - (geometric_product_g0_xyz.yzx() * reverse_g0.zxy()).with_w(0.0)
                - (geometric_product_g1_xyz.yzx() * reverse_g1.zxy()).with_w(geometric_product_g0_xyz[1] * reverse_g1[1]),
        )
    }
}
impl Sandwich<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        7       14        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       40       60        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            (other[e42] * self[e12]) + (other[e31] * self[e43]),
            (other[e43] * self[e23]) + (other[e12] * self[e41]),
            (other[e41] * self[e31]) + (other[e23] * self[e42]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) - (other.group0().zxy() * self.group1().yzx()).with_w(other[e41] * self[e23])
            - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]);
        let geometric_product_g1 = Simd32x4::from([
            other[e31] * self[e12],
            other[e12] * self[e23],
            other[e23] * self[e31],
            -(other[e31] * self[e31]) - (other[e12] * self[e12]),
        ]) - (other.group1().zxy() * self.group1().yzx()).with_w(other[e23] * self[e23]);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (reverse_g0.xyx() * geometric_product_g1.wwy())
                + (reverse_g0.yzz() * geometric_product_g1.zxw())
                + (reverse_g1.xyx() * geometric_product_g0.wwy())
                + (reverse_g1.yzz() * geometric_product_g0.zxw())
                - (reverse_g0.zxy() * geometric_product_g1.yzx())
                - (reverse_g1.zxy() * geometric_product_g0.yzx()),
            // e23, e31, e12
            (reverse_g1.xyx() * geometric_product_g1.wwy()) + (reverse_g1.yzz() * geometric_product_g1.zxw()) - (reverse_g1.zxy() * geometric_product_g1.yzx()),
        )
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
        let geometric_product_g0 = Simd32x4::from([
            (self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42]),
            (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234]),
            (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234]),
            -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
        ]) - (other.group1().zxyx() * self.group0().yzx().with_w(self[e41]))
            - (self.group1().yzx() * other.group0().zxy()).with_w(self[e42] * other[e31]);
        let geometric_product_g1 = Simd32x4::from([
            (self[e23] * other[scalar]) + (self[e12] * other[e31]),
            (self[e23] * other[e12]) + (self[e31] * other[scalar]),
            (self[e31] * other[e23]) + (self[e12] * other[scalar]),
            -(self[e31] * other[e31]) - (self[e12] * other[e12]),
        ]) - (other.group1().zxyx() * self.group1().yzx().with_w(self[e23]));
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (reverse_g0[0] * geometric_product_g1[3])
                    + (reverse_g0[1] * geometric_product_g1[2])
                    + (reverse_g1[0] * geometric_product_g0[3])
                    + (reverse_g1[1] * geometric_product_g0[2]),
                (reverse_g0[1] * geometric_product_g1[3])
                    + (reverse_g0[2] * geometric_product_g1[0])
                    + (reverse_g1[1] * geometric_product_g0[3])
                    + (reverse_g1[2] * geometric_product_g0[0]),
                (reverse_g0[0] * geometric_product_g1[1])
                    + (reverse_g0[2] * geometric_product_g1[3])
                    + (reverse_g1[0] * geometric_product_g0[1])
                    + (reverse_g1[2] * geometric_product_g0[3]),
                -(reverse_g0[2] * geometric_product_g1[2])
                    - (reverse_g1[0] * geometric_product_g0[0])
                    - (reverse_g1[1] * geometric_product_g0[1])
                    - (reverse_g1[2] * geometric_product_g0[2]),
            ]) - (geometric_product_g1.yzxx() * reverse_g0.zxy().with_w(reverse_g0[0]))
                - (reverse_g1.zxy() * geometric_product_g0.yzx()).with_w(reverse_g0[1] * geometric_product_g1[1]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (reverse_g1[0] * geometric_product_g1[3]) + (reverse_g1[1] * geometric_product_g1[2]),
                (reverse_g1[1] * geometric_product_g1[3]) + (reverse_g1[2] * geometric_product_g1[0]),
                (reverse_g1[0] * geometric_product_g1[1]) + (reverse_g1[2] * geometric_product_g1[3]),
                -(reverse_g1[1] * geometric_product_g1[1]) - (reverse_g1[2] * geometric_product_g1[2]),
            ]) - (geometric_product_g1.yzxx() * reverse_g1.zxy().with_w(reverse_g1[0])),
        )
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd2        6        6        0
    //    simd3       17       28        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       49       71        0
    //  no simd      119      151        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
            - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_product_g1 = (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
            + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            - (self.group1().yzx() * other.group1().zxy()).with_w(self[e23] * other[e423]);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group0())
            + (Simd32x3::from(other[e1234]) * self.group1())
            + (self.group0().zxy() * other.group3().yzx())
            + (self.group1().zxy() * other.group2().yzx())
            - (self.group0().yzx() * other.group3().zxy())
            - (self.group1().yzx() * other.group2().zxy());
        let geometric_product_g3 = (Simd32x3::from(other[scalar]) * self.group1()) + (self.group1().zxy() * other.group3().yzx()) - (self.group1().yzx() * other.group3().zxy());
        let geometric_product_g4 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e12] * other[e431]),
            (self[e43] * other[e1]) + (self[e23] * other[e412]),
            (self[e41] * other[e2]) + (self[e31] * other[e423]),
            0.0,
        ]) + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
            - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
            - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
            - (self.group1().yzx() * other.group4().zxy()).with_w(self[e12] * other[e3]);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g2[0] * reverse_g1[0]) - (geometric_product_g2[1] * reverse_g1[1]) - (geometric_product_g2[2] * reverse_g1[2]),
            ]) - (Simd32x2::from(geometric_product_g3[0]) * Simd32x2::from([reverse_g1[0], reverse_g0[0]]))
                - (Simd32x2::from(geometric_product_g3[1]) * Simd32x2::from([reverse_g1[1], reverse_g0[1]]))
                - (Simd32x2::from(geometric_product_g3[2]) * Simd32x2::from([reverse_g1[2], reverse_g0[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse_g1[0] * geometric_product_g4[3]) + (reverse_g1[1] * geometric_product_g1[2]),
                (reverse_g1[1] * geometric_product_g4[3]) + (reverse_g1[2] * geometric_product_g1[0]),
                (reverse_g1[0] * geometric_product_g1[1]) + (reverse_g1[2] * geometric_product_g4[3]),
                -(reverse_g0[1] * geometric_product_g1[1])
                    - (reverse_g0[2] * geometric_product_g1[2])
                    - (reverse_g1[0] * geometric_product_g4[0])
                    - (reverse_g1[1] * geometric_product_g4[1])
                    - (reverse_g1[2] * geometric_product_g4[2]),
            ]) - (geometric_product_g1.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
            // e41, e42, e43
            (reverse_g0 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g1 * Simd32x3::from(geometric_product_g0[1]))
                + (geometric_product_g2.zxy() * reverse_g1.yzx())
                + (geometric_product_g3.zxy() * reverse_g0.yzx())
                - (geometric_product_g2.yzx() * reverse_g1.zxy())
                - (geometric_product_g3.yzx() * reverse_g0.zxy()),
            // e23, e31, e12
            (reverse_g1 * Simd32x3::from(geometric_product_g0[0])) + (geometric_product_g3.zxy() * reverse_g1.yzx()) - (geometric_product_g3.yzx() * reverse_g1.zxy()),
            // e423, e431, e412, e321
            ((Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g0.yzz())
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g1.xyx())
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g1.yzz())
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g0.xyx()))
            .with_w(reverse_g1[2] * geometric_product_g1[2] * -1.0)
                - (geometric_product_g1.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (reverse_g1.zxy() * geometric_product_g4.yzx()).with_w(reverse_g1[1] * geometric_product_g1[1]),
        )
    }
}
impl Sandwich<Origin> for Line {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e4]) * self.group1();
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Origin::from_groups(
            // e4
            -(geometric_product_g0_xyz[0] * reverse_g1[0]) - (geometric_product_g0_xyz[1] * reverse_g1[1]) - (geometric_product_g0_xyz[2] * reverse_g1[2]),
        )
    }
}
impl Sandwich<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        3       12        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       27       43        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1();
        let geometric_product_g0_w = -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        let geometric_product_g1 = (self.group1().zxy() * other.group0().yzx()).with_w(0.0)
            - (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
            - (self.group1().yzx() * other.group0().zxy()).with_w(0.0);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from([geometric_product_g0_xyz[2], geometric_product_g0_xyz[0], geometric_product_g1[3]]) * reverse_g0.yzz())
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0_w]) * reverse_g1.yzz())
                + (reverse_g0.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g0_xyz[1]))
                + (reverse_g1.xyx() * Simd32x2::from(geometric_product_g0_w).with_z(geometric_product_g1[1])))
            .with_w(geometric_product_g0_xyz[2] * reverse_g1[2] * -1.0)
                - (geometric_product_g0_xyz.yzx() * reverse_g0.zxy()).with_w(geometric_product_g0_w * reverse_g1[0])
                - (reverse_g1.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g0_xyz[1] * reverse_g1[1]),
        )
    }
}
impl Sandwich<Point> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd3        0        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       24       46        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], (self[e42] * other[e2]) + (self[e43] * other[e3])])
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_product_g1 = Simd32x4::from([
            (self[e42] * other[e3]) + (self[e23] * other[e4]),
            (self[e43] * other[e1]) + (self[e31] * other[e4]),
            (self[e41] * other[e2]) + (self[e12] * other[e4]),
            -(self[e31] * other[e2]) - (self[e12] * other[e3]),
        ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (reverse_g1[0] * geometric_product_g1[3]) + (reverse_g1[1] * geometric_product_g0[2]),
                (reverse_g1[1] * geometric_product_g1[3]) + (reverse_g1[2] * geometric_product_g0[0]),
                (reverse_g1[0] * geometric_product_g0[1]) + (reverse_g1[2] * geometric_product_g1[3]),
                -(reverse_g0[1] * geometric_product_g0[1])
                    - (reverse_g0[2] * geometric_product_g0[2])
                    - (reverse_g1[0] * geometric_product_g1[0])
                    - (reverse_g1[1] * geometric_product_g1[1])
                    - (reverse_g1[2] * geometric_product_g1[2]),
            ]) - (geometric_product_g0.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
        )
    }
}
impl Sandwich<Scalar> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd2        3        3        0
    //    simd3        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       21        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x3::from(other[scalar]) * self.group1();
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g1[0] * reverse_g0[0]) - (geometric_product_g1[1] * reverse_g0[1]) - (geometric_product_g1[2] * reverse_g0[2]),
            ]) - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
        )
    }
}
impl std::ops::Div<SandwichInfix> for Motor {
    type Output = SandwichInfixPartial<Motor>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1234]) * self.group1();
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiScalar::from_groups(
            // e1234
            (geometric_product_g0[3] * reverse_g1[3])
                - (geometric_product_g0[0] * reverse_g1[0])
                - (geometric_product_g0[1] * reverse_g1[1])
                - (geometric_product_g0[2] * reverse_g1[2]),
        )
    }
}
impl Sandwich<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       15       32        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1());
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * reverse_g0[3])
                    - (geometric_product_g1[0] * reverse_g0[0])
                    - (geometric_product_g1[1] * reverse_g0[1])
                    - (geometric_product_g1[2] * reverse_g0[2]),
            ]) + (Simd32x2::from(reverse_g1[3]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]]))
                - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
        )
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        4       12        0
    //    simd4       17       13        0
    // Totals...
    // yes simd       28       43        0
    //  no simd       87      106        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (other.group0().xyxx() * self.group1().wwy().with_w(self[e41]))
            + (other.group0().yzzy() * self.group1().zxw().with_w(self[e42]))
            + Simd32x3::from(0.0).with_w((other[e4] * self[scalar]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234]))
            + (self.group1().xyz() * other.group1().www()).with_w(other[e3] * self[e43])
            - (self.group1().yzxx() * other.group0().zxy().with_w(other[e423]));
        let geometric_product_g1 = Simd32x4::from([other[e431] * self[e12], other[e412] * self[e23], other[e423] * self[e31], 0.0])
            + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(0.0)
            + (other.group0().zxy() * self.group0().yzx()).with_w(other[e321] * self[scalar])
            - (other.group0().xyxx() * self.group0().wwy().with_w(self[e23]))
            - (other.group0().yzzy() * self.group0().zxw().with_w(self[e31]))
            - (self.group1().yzxz() * other.group1().zxy().with_w(other[e3]))
            - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0);
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([reverse_g1[0], reverse_g1[1], reverse_g1[2], 1.0])
                * geometric_product_g1.www().with_w(
                    -(geometric_product_g0[1] * reverse_g0[1])
                        - (geometric_product_g0[2] * reverse_g0[2])
                        - (geometric_product_g1[0] * reverse_g1[0])
                        - (geometric_product_g1[1] * reverse_g1[1])
                        - (geometric_product_g1[2] * reverse_g1[2]),
                ))
                + (geometric_product_g0.xxyw() * reverse_g1.wzxw())
                + (geometric_product_g0.zyz() * reverse_g1.yww()).with_w(geometric_product_g1[3] * reverse_g0[3])
                - (geometric_product_g0.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
            // e423, e431, e412, e321
            (geometric_product_g0.xxy() * reverse_g0.wzx()).with_w(geometric_product_g1[3] * reverse_g1[3])
                + ((Simd32x3::from(geometric_product_g0[3]) * reverse_g1.xyz())
                    + (Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz())
                    + (geometric_product_g0.zyz() * reverse_g0.yww())
                    + (geometric_product_g1.xxy() * reverse_g1.wzx())
                    + (geometric_product_g1.zyz() * reverse_g1.yww()))
                .with_w(geometric_product_g0[2] * reverse_g1[2] * -1.0)
                - (geometric_product_g0.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (reverse_g1.zxyy() * geometric_product_g1.yzx().with_w(geometric_product_g0[1])),
        )
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        4        6        0
    //    simd4        3        8        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       24       53        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let geometric_product_g1 = Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_product_g0.xxy() * reverse_g0.wzx()).with_w(geometric_product_g1[3] * reverse_g1[3])
                + ((Simd32x3::from(geometric_product_g0[3]) * reverse_g1.xyz())
                    + (Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz())
                    + (geometric_product_g0.zyz() * reverse_g0.yww())
                    + (geometric_product_g1.xxy() * reverse_g1.wzx())
                    + (geometric_product_g1.zyz() * reverse_g1.yww()))
                .with_w(geometric_product_g0[2] * reverse_g1[2] * -1.0)
                - (geometric_product_g0.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (reverse_g1.zxyy() * geometric_product_g1.yzx().with_w(geometric_product_g0[1])),
        )
    }
}
impl Sandwich<Line> for Motor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3       10       13        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       58       80        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            (other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]),
            (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]),
            (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e41]))
            - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]);
        let geometric_product_g1 = Simd32x4::from([
            (other[e23] * self[scalar]) + (other[e31] * self[e12]),
            (other[e31] * self[scalar]) + (other[e12] * self[e23]),
            (other[e23] * self[e31]) + (other[e12] * self[scalar]),
            -(other[e31] * self[e31]) - (other[e12] * self[e12]),
        ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e23]));
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * reverse_g1.xyz())
                + (Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz())
                + (geometric_product_g0.xxy() * reverse_g1.wzx())
                + (geometric_product_g0.zyz() * reverse_g1.yww())
                + (geometric_product_g1.xxy() * reverse_g0.wzx())
                + (geometric_product_g1.zyz() * reverse_g0.yww())
                - (geometric_product_g0.yzx() * reverse_g1.zxy())
                - (geometric_product_g1.yzx() * reverse_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g1[3]) * reverse_g1.xyz()) + (geometric_product_g1.xxy() * reverse_g1.wzx()) + (geometric_product_g1.zyz() * reverse_g1.yww())
                - (geometric_product_g1.yzx() * reverse_g1.zxy()),
        )
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
        let geometric_product_g0 = Simd32x4::from([
            (other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]) + (other[scalar] * self[e41]),
            (other[e1234] * self[e31]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]) + (other[scalar] * self[e42]),
            (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]),
            -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        ]) + (other.group0().xyxw() * self.group1().wwyw())
            + (other.group0().yzz() * self.group1().zxw()).with_w(other[scalar] * self[e1234])
            - (other.group0().zxyx() * self.group1().yzxx())
            - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]);
        let geometric_product_g1 = Simd32x4::from([
            (other[e31] * self[e12]) + (other[scalar] * self[e23]),
            (other[e12] * self[e23]) + (other[scalar] * self[e31]),
            (other[e12] * self[scalar]) + (other[scalar] * self[e12]),
            -(other[e31] * self[e31]) - (other[e12] * self[e12]),
        ]) + (other.group1().xyxw() * self.group1().wwyw())
            - (other.group1().zxyx() * self.group1().yzxx());
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[3] * reverse_g1[0])
                    + (geometric_product_g1[0] * reverse_g0[3])
                    + (geometric_product_g1[2] * reverse_g0[1])
                    + (geometric_product_g1[3] * reverse_g0[0]),
                (geometric_product_g0[3] * reverse_g1[1])
                    + (geometric_product_g1[0] * reverse_g0[2])
                    + (geometric_product_g1[1] * reverse_g0[3])
                    + (geometric_product_g1[3] * reverse_g0[1]),
                (geometric_product_g0[3] * reverse_g1[2])
                    + (geometric_product_g1[1] * reverse_g0[0])
                    + (geometric_product_g1[2] * reverse_g0[3])
                    + (geometric_product_g1[3] * reverse_g0[2]),
                -(geometric_product_g0[2] * reverse_g1[2])
                    - (geometric_product_g1[0] * reverse_g0[0])
                    - (geometric_product_g1[1] * reverse_g0[1])
                    - (geometric_product_g1[2] * reverse_g0[2]),
            ]) + (geometric_product_g0.xxyw() * reverse_g1.wzxw())
                + (geometric_product_g0.zyz() * reverse_g1.yww()).with_w(geometric_product_g1[3] * reverse_g0[3])
                - (geometric_product_g0.yzxx() * reverse_g1.zxyx())
                - (geometric_product_g1.yzx() * reverse_g0.zxy()).with_w(geometric_product_g0[1] * reverse_g1[1]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (geometric_product_g1[2] * reverse_g1[1]) + (geometric_product_g1[3] * reverse_g1[0]),
                (geometric_product_g1[1] * reverse_g1[3]) + (geometric_product_g1[3] * reverse_g1[1]),
                (geometric_product_g1[2] * reverse_g1[3]) + (geometric_product_g1[3] * reverse_g1[2]),
                -(geometric_product_g1[1] * reverse_g1[1]) - (geometric_product_g1[2] * reverse_g1[2]),
            ]) + (geometric_product_g1.xxyw() * reverse_g1.wzxw())
                - (geometric_product_g1.yzxx() * reverse_g1.zxyx()),
        )
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd2        8        8        0
    //    simd3       24       34        0
    //    simd4       16       14        0
    // Totals...
    // yes simd       64       83        0
    //  no simd      168      201        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12])])
            + (Simd32x2::from(other[scalar]) * Simd32x2::from([self[scalar], self[e1234]]))
            - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_product_g1 = (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
            + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
            + (other.group1().xyzz() * self.group1().www().with_w(self[e43]))
            + Simd32x3::from(0.0).with_w((self[scalar] * other[e4]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            - (self.group1().yzx() * other.group1().zxy()).with_w(self[e1234] * other[e321]);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group0().xyz())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz())
            + (other.group2().xyx() * self.group1().wwy())
            + (other.group2().yzz() * self.group1().zxw())
            + (other.group3().xyx() * self.group0().wwy())
            + (other.group3().yzz() * self.group0().zxw())
            - (other.group2().zxy() * self.group1().yzx())
            - (other.group3().zxy() * self.group0().yzx());
        let geometric_product_g3 =
            (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (other.group3().xyx() * self.group1().wwy()) + (other.group3().yzz() * self.group1().zxw())
                - (other.group3().zxy() * self.group1().yzx());
        let geometric_product_g4 = Simd32x4::from([
            (self[e12] * other[e431]) - (self[e31] * other[e412]),
            (self[e23] * other[e412]) - (self[e12] * other[e423]),
            (self[e31] * other[e423]) - (self[e23] * other[e431]),
            0.0,
        ]) + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
            + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group1().zxy()).with_w(self[scalar] * other[e321])
            - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
            - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
            - (other.group1().xyzz() * self.group0().www().with_w(self[e12]));
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * reverse_g1[3])
                    - (geometric_product_g3[0] * reverse_g0[0])
                    - (geometric_product_g3[1] * reverse_g0[1])
                    - (geometric_product_g3[2] * reverse_g0[2]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * Simd32x2::from([reverse_g1[3], reverse_g0[3]]))
                - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (reverse_g1.yzzw() * geometric_product_g1.zx().with_zw(geometric_product_g4[3], geometric_product_g1[3]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], geometric_product_g4[3]) * reverse_g1.xyx().with_w(reverse_g0[3]))
                + (Simd32x3::from(reverse_g1[3]) * geometric_product_g1.xyz()).with_w(
                    -(geometric_product_g1[1] * reverse_g0[1])
                        - (geometric_product_g1[2] * reverse_g0[2])
                        - (geometric_product_g4[0] * reverse_g1[0])
                        - (geometric_product_g4[1] * reverse_g1[1])
                        - (geometric_product_g4[2] * reverse_g1[2]),
                )
                - (geometric_product_g1.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[0]) * reverse_g0.xyz())
                + (Simd32x3::from(geometric_product_g0[1]) * reverse_g1.xyz())
                + (geometric_product_g2.xxy() * reverse_g1.wzx())
                + (geometric_product_g2.zyz() * reverse_g1.yww())
                + (geometric_product_g3.xxy() * reverse_g0.wzx())
                + (geometric_product_g3.zyz() * reverse_g0.yww())
                - (geometric_product_g2.yzx() * reverse_g1.zxy())
                - (geometric_product_g3.yzx() * reverse_g0.zxy()),
            // e23, e31, e12
            (Simd32x3::from(geometric_product_g0[0]) * reverse_g1.xyz()) + (geometric_product_g3.xxy() * reverse_g1.wzx()) + (geometric_product_g3.zyz() * reverse_g1.yww())
                - (geometric_product_g3.yzx() * reverse_g1.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4.ww().with_zw(geometric_product_g1[1], geometric_product_g4[3]) * reverse_g0.xyx().with_w(reverse_g1[3]))
                + ((Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g1.xyx())
                    + (Simd32x3::from([reverse_g0[1], reverse_g0[3], reverse_g1[2]]) * geometric_product_g1.zyw())
                    + (Simd32x3::from([reverse_g1[1], reverse_g1[3], reverse_g0[2]]) * geometric_product_g4.zyw())
                    + (geometric_product_g1.xxz() * reverse_g0.wzw())
                    + (geometric_product_g4.xxz() * reverse_g1.wzw()))
                .with_w(geometric_product_g1[2] * reverse_g1[2] * -1.0)
                - (geometric_product_g1.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0]))
                - (reverse_g1.zxyy() * geometric_product_g4.yzx().with_w(geometric_product_g1[1])),
        )
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       12        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e4]) * self.group1().xyz();
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Origin::from_groups(
            // e4
            (reverse_g1[3] * self[scalar] * other[e4])
                - (geometric_product_g1_xyz[0] * reverse_g1[0])
                - (geometric_product_g1_xyz[1] * reverse_g1[1])
                - (geometric_product_g1_xyz[2] * reverse_g1[2]),
        )
    }
}
impl Sandwich<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        7       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       36       57        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1().xyz();
        let geometric_product_g0_w = -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        let geometric_product_g1_xyz = (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group1().zxy() * other.group0().yzx())
            - (Simd32x3::from(other[e321]) * self.group0().xyz())
            - (self.group1().yzx() * other.group0().zxy());
        let geometric_product_g1_w = self[scalar] * other[e321];
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_product_g0_xyz.xxy() * reverse_g0.wzx()).with_w(geometric_product_g1_w * reverse_g1[3])
                + ((Simd32x3::from(geometric_product_g0_w) * reverse_g1.xyz())
                    + (Simd32x3::from(geometric_product_g1_w) * reverse_g0.xyz())
                    + (geometric_product_g0_xyz.zyz() * reverse_g0.yww())
                    + (geometric_product_g1_xyz.xxy() * reverse_g1.wzx())
                    + (geometric_product_g1_xyz.zyz() * reverse_g1.yww()))
                .with_w(geometric_product_g0_xyz[2] * reverse_g1[2] * -1.0)
                - (reverse_g1.zxyy() * geometric_product_g1_xyz.yzx().with_w(geometric_product_g0_xyz[1]))
                - (geometric_product_g0_xyz.yzx() * reverse_g0.zxy()).with_w(geometric_product_g0_w * reverse_g1[0]),
        )
    }
}
impl Sandwich<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        1        2        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       36       62        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([
            self[e31] * other[e3],
            self[e12] * other[e1],
            self[e23] * other[e2],
            (self[e43] * other[e3]) + (self[scalar] * other[e4]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().xyzy() * self.group1().www().with_w(self[e42]))
            + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]));
        let geometric_product_g1 = (Simd32x3::from([self[e42] * other[e3], self[e43] * other[e1], self[e41] * other[e2]]) + (Simd32x3::from(other[e4]) * self.group1().xyz()))
            .with_w(self[e12] * other[e3] * -1.0)
            - (other.group0().xyzy() * self.group0().www().with_w(self[e31]))
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([reverse_g1[0], reverse_g1[1], reverse_g1[2], 1.0])
                * geometric_product_g1.www().with_w(
                    -(geometric_product_g0[1] * reverse_g0[1])
                        - (geometric_product_g0[2] * reverse_g0[2])
                        - (geometric_product_g1[0] * reverse_g1[0])
                        - (geometric_product_g1[1] * reverse_g1[1])
                        - (geometric_product_g1[2] * reverse_g1[2]),
                ))
                + (geometric_product_g0.xxyw() * reverse_g1.wzxw())
                + (geometric_product_g0.zyz() * reverse_g1.yww()).with_w(geometric_product_g1[3] * reverse_g0[3])
                - (geometric_product_g0.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
        )
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd2        4        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       11       28        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let reverse_g0 = self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * reverse_g0[3])
                    - (geometric_product_g1[0] * reverse_g0[0])
                    - (geometric_product_g1[1] * reverse_g0[1])
                    - (geometric_product_g1[2] * reverse_g0[2]),
            ]) + (Simd32x2::from(reverse_g1[3]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]]))
                - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
        )
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
    //      f32       16       20        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       16       35        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([1.0, other[e1234] * self[scalar]]) * Simd32x2::from([0.0, 1.0]);
        let geometric_product_g1_w = other[e1234] * self[e321];
        let geometric_product_g2 = Simd32x3::from(other[e1234]) * self.group3();
        let geometric_product_g4_xyz = Simd32x3::from(other[e1234]) * self.group1().xyz();
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1_w * reverse_g4[3]) + (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar])
                    - (geometric_product_g2[0] * reverse_g3[0])
                    - (geometric_product_g2[1] * reverse_g3[1])
                    - (geometric_product_g2[2] * reverse_g3[2])
                    - (geometric_product_g4_xyz[0] * self[e1])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (geometric_product_g1_w * self[scalar])
                    + (geometric_product_g0[0] * self[e4])
                    + (geometric_product_g2[0] * self[e1])
                    + (geometric_product_g2[1] * self[e2])
                    + (geometric_product_g2[2] * self[e3])
                    - (geometric_product_g0[1] * reverse_g4[3])
                    - (geometric_product_g4_xyz[0] * reverse_g3[0])
                    - (geometric_product_g4_xyz[1] * reverse_g3[1])
                    - (geometric_product_g4_xyz[2] * reverse_g3[2]),
            ),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl Sandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd2        8        8        0
    //    simd3        2        9        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       34       49        0
    //  no simd       70       99        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[scalar] * self[scalar];
        let geometric_product_g0_y = (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]);
        let geometric_product_g1 = other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])) * self.group1().xyz().with_w(1.0);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3());
        let geometric_product_g3 = Simd32x3::from(other[scalar]) * self.group3();
        let geometric_product_g4_xyz = (Simd32x3::from(other[scalar]) * self.group4().xyz()) + (Simd32x3::from(other[e1234]) * self.group1().xyz());
        let geometric_product_g4_w = other[scalar] * self[e321];
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g4_w * self[e4])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g0_x) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4_xyz.with_w(geometric_product_g4_w).wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0_x) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4_w]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4_w, self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4_xyz.with_w(geometric_product_g4_w).ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (geometric_product_g4_xyz[0] * reverse_g3[0])
                        - (geometric_product_g4_xyz[1] * reverse_g3[1])
                        - (geometric_product_g4_xyz[2] * reverse_g3[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0_y * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl Sandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd2       12       12        0
    //    simd3       32       52        0
    //    simd4       32       20        0
    // Totals...
    // yes simd      102      122        0
    //  no simd      274      298        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])])
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
            - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx());
        let geometric_product_g1 = (Simd32x4::from(self[scalar]) * other.group0())
            + (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group3().zyz().with_w(self[e42]))
            + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group3().xxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
            - (self.group3().yzx() * other.group0().zxy()).with_w(other[e321] * self[e1234]);
        let geometric_product_g2 = (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from([self[e4], self[e4], self[e431]]) * other.group0().xyx())
            + (Simd32x3::from([self[e412], self[e423], self[e4]]) * other.group0().yzz())
            + (other.group1().zxy() * self.group1().yzx())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
            - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
            - (other.group0().zxy() * self.group4().yzx());
        let geometric_product_g3 = (other.group0().zxy() * self.group1().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group0().yzz())
            - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group0().xyx());
        let geometric_product_g4 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e431] * self[e12]) - (other[e412] * self[e31]),
            (other[e1] * self[e43]) + (other[e412] * self[e23]) - (other[e423] * self[e12]),
            (other[e2] * self[e41]) + (other[e423] * self[e31]) - (other[e431] * self[e23]),
            0.0,
        ]) + (Simd32x4::from(self[scalar]) * other.group1())
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            - (Simd32x4::from([other[e2], other[e321], other[e321], other[e3]]) * self.group2().zyz().with_w(self[e12]))
            - (Simd32x4::from([other[e321], other[e3], other[e1], other[e2]]) * self.group2().xxy().with_w(self[e31]))
            - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]));
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4[3]]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4[3], self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + (geometric_product_g1.yzx() * reverse_g4.zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g2.xyx()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (geometric_product_g1.yzxy() * reverse_g2.zxy().with_w(reverse_g3[1]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(reverse_g3[0] * geometric_product_g1[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(reverse_g3[2] * geometric_product_g1[2]),
        )
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        2        0
    //    simd3       22       39        0
    //    simd4       15       11        0
    // Totals...
    // yes simd       37       54        0
    //  no simd      126      167        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(other[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other[e321]) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let geometric_product_g2 = Simd32x3::from(other[e321]) * self.group4().xyz();
        let geometric_product_g3 = Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_product_g4 = Simd32x4::from(other[e321]) * self.group2().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + (geometric_product_g1.yzx() * reverse_g4.zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g2.xyx()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (geometric_product_g1.yzxy() * reverse_g2.zxy().with_w(reverse_g3[1]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(reverse_g3[0] * geometric_product_g1[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(reverse_g3[2] * geometric_product_g1[2]),
        )
    }
}
impl Sandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       25        0
    //    simd2        3        3        0
    //    simd3       31       48        0
    //    simd4       18        9        0
    // Totals...
    // yes simd       64       85        0
    //  no simd      183      211        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
            - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_product_g1 = Simd32x4::from([
            (other[e23] * self[e321]) + (other[e31] * self[e3]),
            (other[e31] * self[e321]) + (other[e12] * self[e1]),
            (other[e23] * self[e2]) + (other[e12] * self[e321]),
            -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
        ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[scalar]) * other.group0())
            + (Simd32x3::from(self[e1234]) * other.group1())
            + (other.group0().yzx() * self.group3().zxy())
            + (other.group1().yzx() * self.group2().zxy())
            - (other.group0().zxy() * self.group3().yzx())
            - (other.group1().zxy() * self.group2().yzx());
        let geometric_product_g3 = (Simd32x3::from(self[scalar]) * other.group1()) + (other.group1().yzx() * self.group3().zxy()) - (other.group1().zxy() * self.group3().yzx());
        let geometric_product_g4 = (Simd32x3::from([
            (other[e42] * self[e3]) + (other[e31] * self[e412]),
            (other[e43] * self[e1]) + (other[e12] * self[e423]),
            (other[e41] * self[e2]) + (other[e23] * self[e431]),
        ]) + (Simd32x3::from(self[e4]) * other.group1())
            + (Simd32x3::from(self[e321]) * other.group0()))
        .with_w(other[e12] * self[e3] * -1.0)
            - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]))
            - (other.group1().zxy() * self.group4().yzx()).with_w(other[e31] * self[e2]);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + (geometric_product_g1.yzx() * reverse_g4.zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g2.xyx()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (geometric_product_g1.yzxy() * reverse_g2.zxy().with_w(reverse_g3[1]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(reverse_g3[0] * geometric_product_g1[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(reverse_g3[2] * geometric_product_g1[2]),
        )
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       37        0
    //    simd2       12       12        0
    //    simd3       35       54        0
    //    simd4       29       19        0
    // Totals...
    // yes simd      100      122        0
    //  no simd      269      299        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12])])
            + (Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar], other[e1234]]))
            - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_product_g1 = (other.group1().yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
            + (self.group4().ww().with_zw(self[e2], self[e321]) * other.group1().xyx().with_w(other[e1234]))
            + (Simd32x3::from(other[scalar]) * self.group1().xyz())
                .with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]))
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[scalar]) * other.group0().xyz())
            + (Simd32x3::from(self[e1234]) * other.group1().xyz())
            + (self.group2().xxy() * other.group1().wzx())
            + (self.group2().zyz() * other.group1().yww())
            + (self.group3().xxy() * other.group0().wzx())
            + (self.group3().zyz() * other.group0().yww())
            - (self.group2().yzx() * other.group1().zxy())
            - (self.group3().yzx() * other.group0().zxy());
        let geometric_product_g3 =
            (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (self.group3().xxy() * other.group1().wzx()) + (self.group3().zyz() * other.group1().yww())
                - (self.group3().yzx() * other.group1().zxy());
        let geometric_product_g4 = (self.group4().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[scalar]))
            + (Simd32x3::from([
                (other[e42] * self[e3]) + (other[e31] * self[e412]),
                (other[e43] * self[e1]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
            ]) + (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (Simd32x3::from(other[scalar]) * self.group4().xyz())
                + (Simd32x3::from(self[e4]) * other.group1().xyz()))
            .with_w(other[e12] * self[e3] * -1.0)
            - (other.group1().zxyy() * self.group4().yzx().with_w(self[e2]))
            - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]));
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4[3]]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4[3], self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + (geometric_product_g1.yzx() * reverse_g4.zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g2.xyx()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (geometric_product_g1.yzxy() * reverse_g2.zxy().with_w(reverse_g3[1]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(reverse_g3[0] * geometric_product_g1[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(reverse_g3[2] * geometric_product_g1[2]),
        )
    }
}
impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       52        0
    //    simd2       16       16        0
    //    simd3       44       70        0
    //    simd4       43       25        0
    // Totals...
    // yes simd      140      163        0
    //  no simd      373      394        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([
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
            - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx());
        let geometric_product_g1 = (Simd32x4::from(other[scalar]) * self.group1())
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
            - (self.group3().yzx() * other.group1().zxy()).with_w(other[e41] * self[e1]);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group2())
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
            - (other.group1().zxy() * self.group4().yzx());
        let geometric_product_g3 = (Simd32x3::from(other[scalar]) * self.group3())
            + (Simd32x3::from(self[scalar]) * other.group3())
            + (other.group3().yzx() * self.group3().zxy())
            + (other.group1().zxy() * self.group1().yzx())
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
            - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
            - (other.group3().zxy() * self.group3().yzx());
        let geometric_product_g4 = Simd32x4::from([
            (other[e3] * self[e42]) + (other[e42] * self[e3]) + (other[e31] * self[e412]) + (other[e431] * self[e12]),
            (other[e1] * self[e43]) + (other[e43] * self[e1]) + (other[e12] * self[e423]) + (other[e412] * self[e23]),
            (other[e2] * self[e41]) + (other[e41] * self[e2]) + (other[e23] * self[e431]) + (other[e423] * self[e31]),
            0.0,
        ]) + (Simd32x4::from(other[scalar]) * self.group4())
            + (other.group0().yy().with_zw(other[e1234], self[scalar]) * self.group1().xyz().with_w(other[e321]))
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
            + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group2().zyz().with_w(self[e31]))
            - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group2().xxy().with_w(self[e23]))
            - (self.group1().yzxy() * other.group2().zxy().with_w(other[e31]))
            - (self.group0().yy().with_zw(self[e1234], other[e23]) * other.group1().xyz().with_w(self[e1]))
            - (other.group3().zxy() * self.group4().yzx()).with_w(other[e12] * self[e3])
            - (self.group3().yzx() * other.group4().zxy()).with_w(other[e3] * self[e12]);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4[3]]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4[3], self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + (geometric_product_g1.yzx() * reverse_g4.zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[3], geometric_product_g1[3], geometric_product_g4[1]]) * reverse_g3.xyx()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[3], geometric_product_g4[3], geometric_product_g1[1]]) * reverse_g2.xyx()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (geometric_product_g1.yzxy() * reverse_g2.zxy().with_w(reverse_g3[1]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(reverse_g3[0] * geometric_product_g1[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(reverse_g3[2] * geometric_product_g1[2]),
        )
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       20        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       16       38        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([1.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]);
        let geometric_product_g1_w = self[scalar] * other[e4];
        let geometric_product_g2 = Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_product_g4_xyz = Simd32x3::from(other[e4]) * self.group3();
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1_w * reverse_g4[3]) + (geometric_product_g0[0] * self[e1234]) + (geometric_product_g0[1] * self[scalar])
                    - (geometric_product_g2[0] * reverse_g3[0])
                    - (geometric_product_g2[1] * reverse_g3[1])
                    - (geometric_product_g2[2] * reverse_g3[2])
                    - (geometric_product_g4_xyz[0] * self[e1])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (geometric_product_g1_w * self[scalar])
                    + (geometric_product_g0[0] * self[e4])
                    + (geometric_product_g2[0] * self[e1])
                    + (geometric_product_g2[1] * self[e2])
                    + (geometric_product_g2[2] * self[e3])
                    - (geometric_product_g0[1] * reverse_g4[3])
                    - (geometric_product_g4_xyz[0] * reverse_g3[0])
                    - (geometric_product_g4_xyz[1] * reverse_g3[1])
                    - (geometric_product_g4_xyz[2] * reverse_g3[2]),
            ),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl Sandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd2        0        1        0
    //    simd3       28       48        0
    //    simd4       15        6        0
    // Totals...
    // yes simd       49       68        0
    //  no simd      150      183        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([
            self[e321] * other[e321],
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
        ]) * Simd32x2::from([-1.0, 1.0]);
        let geometric_product_g1_xyz = Simd32x3::from(other[e321]) * self.group3();
        let geometric_product_g1_w = -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        let geometric_product_g2 = (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().yzx() * other.group0().zxy())
            - (Simd32x3::from(self[e321]) * other.group0().xyz())
            - (self.group1().zxy() * other.group0().yzx());
        let geometric_product_g3 = Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_product_g4 = ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group3().zxy() * other.group0().yzx())
            - (Simd32x3::from(other[e321]) * self.group2())
            - (self.group3().yzx() * other.group0().zxy()))
        .with_w(self[scalar] * other[e321]);
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1_w) * self.group1().xyz())
                + (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g4.xxy())
                + (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g4.zyz())
                + (geometric_product_g1_xyz.yzx() * reverse_g4.zxy())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g1_xyz.zyz())
                - (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g1_xyz.xxy())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yzx() * self.group1().zxy()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g1_xyz.yzx() * self.group1().zxy())
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from([reverse_g4[3], self[e3], self[e1]]) * geometric_product_g1_xyz.xxy())
                - (Simd32x3::from([self[e2], reverse_g4[3], reverse_g4[3]]) * geometric_product_g1_xyz.zyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (geometric_product_g1_xyz * Simd32x3::from(self[e1234])).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1_xyz[2], geometric_product_g1_xyz[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1_w]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1_xyz[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1_w).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([reverse_g4[3], self[e3], self[e1], self[e2]]) * geometric_product_g2.xxy().with_w(geometric_product_g3[1]))
                - (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e3]]) * geometric_product_g2.zyz().with_w(geometric_product_g3[2]))
                - (self.group1().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (geometric_product_g1_xyz.yzx() * reverse_g2.zxy()).with_w(geometric_product_g1_w * reverse_g3[1])
                - (geometric_product_g3.yzx() * reverse_g4.zxy()).with_w(geometric_product_g1_xyz[0] * reverse_g3[0])
                - (reverse_g3.zxy() * geometric_product_g4.yzx()).with_w(geometric_product_g1_xyz[2] * reverse_g3[2]),
        )
    }
}
impl Sandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd2        8        8        0
    //    simd3        6       12        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       46       67        0
    //  no simd      102      135        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]);
        let geometric_product_g0_y = -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]);
        let geometric_product_g1 = (Simd32x4::from([self[e31] * other[e3], self[e12] * other[e1], self[e23] * other[e2], (self[e42] * other[e2]) + (self[e43] * other[e3])])
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (Simd32x4::from(self[scalar]) * other.group0())
            + (other.group0().yzxx() * self.group3().zxy().with_w(self[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().zxy() * other.group0().yzx())
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group4().yzx() * other.group0().zxy());
        let geometric_product_g3 =
            (self.group1().yzx() * other.group0().zxy()) - (Simd32x3::from(self[e321]) * other.group0().xyz()) - (self.group1().zxy() * other.group0().yzx());
        let geometric_product_g4 = (Simd32x3::from([self[e42] * other[e3], self[e43] * other[e1], self[e41] * other[e2]]) + (Simd32x3::from(other[e4]) * self.group3()))
            .with_w(self[e12] * other[e3] * -1.0)
            - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
            - (other.group0().yzxy() * self.group2().zxy().with_w(self[e31]));
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0_x) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0_x) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4[3]]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4[3], self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0_y * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl Sandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       18        0
    //    simd2        8        9        0
    //    simd3        0        6        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       62       90        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let geometric_product_g2 = Simd32x3::from(other[scalar]) * self.group2();
        let geometric_product_g3 = Simd32x3::from(other[scalar]) * self.group3();
        let geometric_product_g4 = Simd32x4::from(other[scalar]) * self.group4();
        let reverse_g2 = self.group2() * Simd32x3::from(-1.0);
        let reverse_g3 = self.group3() * Simd32x3::from(-1.0);
        let reverse_g4 = self.group4() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g1[3] * reverse_g4[3])
                    - (geometric_product_g3[0] * reverse_g2[0])
                    - (geometric_product_g3[1] * reverse_g2[1])
                    - (geometric_product_g3[2] * reverse_g2[2])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from([reverse_g4[3], self[e1]]) * geometric_product_g4.wx()),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from([reverse_g4[3], self[e3], self[e1], geometric_product_g4[3]]) * geometric_product_g3.xxy().with_w(self[e1234]))
                + (Simd32x4::from([self[e2], reverse_g4[3], reverse_g4[3], self[e1]]) * geometric_product_g3.zyz().with_w(geometric_product_g2[0]))
                + (geometric_product_g1.zx().with_zw(geometric_product_g4[3], self[e3]) * reverse_g3.yzz().with_w(geometric_product_g2[2]))
                + (geometric_product_g4.ww().with_zw(geometric_product_g1[1], self[e2]) * reverse_g3.xyx().with_w(geometric_product_g2[1]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[0] * geometric_product_g1[0])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3])
                - (reverse_g3.zxy() * geometric_product_g1.yzx()).with_w(geometric_product_g3[0] * reverse_g4[0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
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
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[e321], 2) * -1.0)
    }
}
impl Sandwich<DualNum> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from(self[e321] * -1.0) * other.group0() * Simd32x2::from([-1.0, 1.0]),
        )
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
        let geometric_product_g0 = Simd32x4::from([
            (other[e2] * self[e412]) + (other[e321] * self[e423]),
            (other[e3] * self[e423]) + (other[e321] * self[e431]),
            (other[e1] * self[e431]) + (other[e321] * self[e412]),
            -(other[e3] * self[e412]) - (other[e4] * self[e321]),
        ]) - (other.group0().zxyx() * self.group0().yzxx())
            - (self.group0().wwwy() * other.group1().xyz().with_w(other[e2]));
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0);
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(reverse_g0[3]) * geometric_product_g1.xyz()).with_w(
                -(geometric_product_g0[3] * reverse_g0[3])
                    - (geometric_product_g1[0] * reverse_g0[0])
                    - (geometric_product_g1[1] * reverse_g0[1])
                    - (geometric_product_g1[2] * reverse_g0[2]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz()) + (geometric_product_g1.zxy() * reverse_g0.yzx())
                - (Simd32x3::from(reverse_g0[3]) * geometric_product_g0.xyz())
                - (geometric_product_g1.yzx() * reverse_g0.zxy()))
            .with_w(geometric_product_g1[3] * reverse_g0[3]),
        )
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        9       25        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(other[e321] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz()) + (geometric_product_g1.zxy() * reverse_g0.yzx())
                - (geometric_product_g1.yzx() * reverse_g0.zxy())
                - (Simd32x3::from(reverse_g0[3]) * Simd32x3::from(other[e321]) * self.group0().xyz()))
            .with_w(geometric_product_g1[3] * reverse_g0[3]),
        )
    }
}
impl Sandwich<Line> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        3       10        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       17       34        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(self[e321]) * other.group1();
        let geometric_product_g1 = (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0) + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (other.group1().zxy() * self.group0().yzx()).with_w(0.0);
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(reverse_g0[3]) * geometric_product_g1.xyz()) + (geometric_product_g0_xyz.yzx() * reverse_g0.zxy())
                - (Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz())
                - (geometric_product_g0_xyz.zxy() * reverse_g0.yzx()),
            // e23, e31, e12
            geometric_product_g0_xyz * Simd32x3::from(reverse_g0[3]) * Simd32x3::from(-1.0),
        )
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
        let geometric_product_g0_xyz = Simd32x3::from(self[e321]) * other.group1().xyz();
        let geometric_product_g0_w = (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]);
        let geometric_product_g1_w = other[scalar] * self[e321];
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(geometric_product_g1_w * reverse_g0[0]) - (geometric_product_g0_xyz[2] * reverse_g0[1]),
                -(geometric_product_g1_w * reverse_g0[1]) - (geometric_product_g0_xyz[0] * reverse_g0[2]),
                -(geometric_product_g1_w * reverse_g0[2]) - (geometric_product_g0_xyz[1] * reverse_g0[0]),
                (geometric_product_g0_w * reverse_g0[3]) + (geometric_product_g0_xyz[2] * reverse_g0[2]),
            ]) + (reverse_g0.zxyx() * geometric_product_g0_xyz.yzx().with_w(geometric_product_g0_w))
                + (reverse_g0.wwwy()
                    * ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                        - (other.group1().zxy() * self.group0().yzx()))
                    .with_w(geometric_product_g0_xyz[1])),
            // e23, e31, e12, scalar
            Simd32x4::from(reverse_g0[3]) * geometric_product_g0_xyz.with_w(geometric_product_g1_w) * Simd32x4::from(-1.0),
        )
    }
}
impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd2        0        2        0
    //    simd3       12       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       24       48        0
    //  no simd       48       97        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([
            other[e321] * self[e321],
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
        ]) * Simd32x2::from([-1.0, 1.0]);
        let geometric_product_g1_xyz = Simd32x3::from(self[e321]) * other.group3();
        let geometric_product_g3 = Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0);
        let geometric_product_g4_w = other[scalar] * self[e321];
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g4_w * reverse_g0[3],
                (geometric_product_g1_xyz[0] * reverse_g0[0])
                    + (geometric_product_g1_xyz[1] * reverse_g0[1])
                    + (geometric_product_g1_xyz[2] * reverse_g0[2])
                    + (reverse_g0[3] * other[e1234] * self[e321])
                    - (reverse_g0[3] * other[e23] * self[e423])
                    - (reverse_g0[3] * other[e31] * self[e431])
                    - (reverse_g0[3] * other[e12] * self[e412]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (geometric_product_g3 * Simd32x3::from(reverse_g0[3])).with_w(
                -(geometric_product_g0[1] * reverse_g0[3])
                    - (geometric_product_g3[0] * reverse_g0[0])
                    - (geometric_product_g3[1] * reverse_g0[1])
                    - (geometric_product_g3[2] * reverse_g0[2]),
            ),
            // e41, e42, e43
            (Simd32x3::from(reverse_g0[3])
                * ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group2()) + (other.group3().yzx() * self.group0().zxy())
                    - (other.group3().zxy() * self.group0().yzx())))
                + (geometric_product_g1_xyz.yzx() * reverse_g0.zxy())
                - (Simd32x3::from(geometric_product_g4_w) * reverse_g0.xyz())
                - (geometric_product_g1_xyz.zxy() * reverse_g0.yzx()),
            // e23, e31, e12
            geometric_product_g1_xyz * Simd32x3::from(reverse_g0[3]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g0[0]) * reverse_g0.xyz()) + (geometric_product_g3.zxy() * reverse_g0.yzx())
                - (Simd32x3::from(reverse_g0[3])
                    * ((Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                        - (Simd32x3::from(self[e321]) * other.group4().xyz())
                        - (other.group1().zxy() * self.group0().yzx())))
                - (geometric_product_g3.yzx() * reverse_g0.zxy()))
            .with_w(geometric_product_g0[0] * reverse_g0[3]),
        )
    }
}
impl Sandwich<Origin> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * f32::powi(self[e321], 2) * -1.0)
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        4        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       10        0
    //  no simd       12       28        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(other[e321] * self[e321]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]);
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(geometric_product_g1[3]) * reverse_g0.xyz()) + (geometric_product_g1.zxy() * reverse_g0.yzx())
                - (Simd32x3::from(reverse_g0[3]) * ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())))
                - (geometric_product_g1.yzx() * reverse_g0.zxy()))
            .with_w(geometric_product_g1[3] * reverse_g0[3]),
        )
    }
}
impl Sandwich<Point> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        6       24        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = other.group0().xyz() * self.group0().www() * Simd32x3::from(-1.0);
        let reverse_g0 = self.group0() * Simd32x4::from(-1.0);
        Point::from_groups(/* e1, e2, e3, e4 */ (geometric_product_g1_xyz * Simd32x3::from(reverse_g0[3])).with_w(
            (reverse_g0[3] * self.group0().yzxx()[3] * other.group0().zxyx()[3])
                + (reverse_g0[3] * self[e431] * other[e2])
                + (reverse_g0[3] * self[e412] * other[e3])
                + (reverse_g0[3] * self[e321] * other[e4])
                - (geometric_product_g1_xyz[0] * reverse_g0[0])
                - (geometric_product_g1_xyz[1] * reverse_g0[1])
                - (geometric_product_g1_xyz[2] * reverse_g0[2]),
        ))
    }
}
impl Sandwich<Scalar> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ self[e321] * self[e321] * other[scalar])
    }
}
impl std::ops::Div<SandwichInfix> for Point {
    type Output = SandwichInfixPartial<Point>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        6        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e1234]) * self.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            -(geometric_product_g0_xyz[0] * self[e1]) - (geometric_product_g0_xyz[1] * self[e2]) - (geometric_product_g0_xyz[2] * self[e3]),
        )
    }
}
impl Sandwich<DualNum> for Point {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        4       13        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1_xyz = self.group0().xyz() * other.group0().yy().with_z(other[e1234]);
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            -(geometric_product_g1_xyz[0] * self[e1]) - (geometric_product_g1_xyz[1] * self[e2]) - (geometric_product_g1_xyz[2] * self[e3]),
        ]))
    }
}
impl Sandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        1        4        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       40       60        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([
            -(other[e4] * self[e1]) - (other[e431] * self[e3]),
            -(other[e4] * self[e2]) - (other[e412] * self[e1]),
            -(other[e4] * self[e3]) - (other[e423] * self[e2]),
            (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) + (other.group1().zxyy() * self.group0().yzxy())
            + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423]));
        let geometric_product_g1 = Simd32x4::from([
            -(other[e2] * self[e3]) - (other[e321] * self[e1]),
            -(other[e3] * self[e1]) - (other[e321] * self[e2]),
            -(other[e1] * self[e2]) - (other[e321] * self[e3]),
            (other[e2] * self[e2]) + (other[e3] * self[e3]),
        ]) + (other.group0().zxyx() * self.group0().yzxx());
        Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyzy() * geometric_product_g1.www().with_w(geometric_product_g0[1]))
                + (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0[0]))
                + (geometric_product_g1.yzx() * self.group0().zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g0[2] * self[e3]) + (geometric_product_g1[3] * self[e4])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e4]) * geometric_product_g1.xyz()) + (geometric_product_g0.yzx() * self.group0().zxy())).with_w(geometric_product_g1[2] * self[e3] * -1.0)
                - (self.group0().xyzy() * geometric_product_g0.www().with_w(geometric_product_g1[1]))
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1[0])),
        )
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        8       20        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e321]) * self.group0().xyz() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            (geometric_product_g1_xyz * Simd32x3::from(self[e4])).with_w(geometric_product_g1_xyz[2] * self[e3] * -1.0)
                - (self.group0().xyzy() * Simd32x3::from(other[e321] * self[e4]).with_w(geometric_product_g1_xyz[1]))
                - (self.group0().yzxx() * Simd32x3::from(0.0).with_w(geometric_product_g1_xyz[0])),
        )
    }
}
impl Sandwich<Line> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd3        5        7        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       28       42        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()) + (geometric_product_g1.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_product_g0.xyz())
                - (geometric_product_g1.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product_g0.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g0.zxy() * self.group0().yzx()),
        )
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       29        0
    //    simd3        0        1        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       40       56        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (other.group1().yzxw() * self.group0().zxyw())
            + (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]))
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = Simd32x4::from([
            (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (geometric_product_g0[3] * self[e1]) + (geometric_product_g1[2] * self[e2]),
                (geometric_product_g0[3] * self[e2]) + (geometric_product_g1[0] * self[e3]),
                (geometric_product_g0[3] * self[e3]) + (geometric_product_g1[1] * self[e1]),
                -(geometric_product_g1[2] * self[e3]) - (geometric_product_g1[3] * self[e4]),
            ]) - (geometric_product_g1.yzxy() * self.group0().zxyy())
                - (self.group0().wwwx() * geometric_product_g0.xyz().with_w(geometric_product_g1[0])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(geometric_product_g0[2] * self[e2]) - (geometric_product_g1[3] * self[e1]),
                -(geometric_product_g0[0] * self[e3]) - (geometric_product_g1[3] * self[e2]),
                -(geometric_product_g0[1] * self[e1]) - (geometric_product_g1[3] * self[e3]),
                (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            ]) + (geometric_product_g0.yzxx() * self.group0().zxyx()),
        )
    }
}
impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd2        3        4        0
    //    simd3       11       18        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       35       57        0
    //  no simd       81      118        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x2::from([1.0, other[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]))
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]));
        let geometric_product_g1 = Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
            + (Simd32x4::from(other[scalar]) * self.group0())
            - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group4().zxy() * self.group0().yzx())
            - (Simd32x3::from(other[e4]) * self.group0().xyz())
            - (other.group4().yzx() * self.group0().zxy());
        let geometric_product_g3 =
            (other.group1().zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0().xyz()) - (other.group1().yzx() * self.group0().zxy());
        let geometric_product_g4 = Simd32x4::from([
            (other[e1234] * self[e1]) + (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e1234] * self[e2]) + (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e1234] * self[e3]) + (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e31] * self[e2]) - (other[e12] * self[e3]),
        ]) - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]),
                -(geometric_product_g4[0] * self[e1]) - (geometric_product_g4[1] * self[e2]) - (geometric_product_g4[2] * self[e3]) - (geometric_product_g4[3] * self[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + (geometric_product_g3.yzx() * self.group0().zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g2[1] * self[e2]) + (geometric_product_g2[2] * self[e3])),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()) + (geometric_product_g4.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g4.yzx() * self.group0().zxy()),
            // e23, e31, e12
            (geometric_product_g1.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (geometric_product_g1.zxy() * self.group0().yzx()),
            // e423, e431, e412, e321
            ((geometric_product_g3 * Simd32x3::from(self[e4])) + (geometric_product_g2.yzx() * self.group0().zxy())).with_w(geometric_product_g3[2] * self[e3] * -1.0)
                - (self.group0().xyzx() * geometric_product_g0.yy().with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group0().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1])),
        )
    }
}
impl Sandwich<Origin> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2        9        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0);
        Origin::from_groups(
            // e4
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
        )
    }
}
impl Sandwich<Plane> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        1        4        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       17       36        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from([
            other[e431] * self[e3],
            other[e412] * self[e1],
            other[e423] * self[e2],
            (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().zxyx() * self.group0().yzxx());
        let geometric_product_g1_xyz = self.group0().xyz() * other.group0().www() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            ((geometric_product_g1_xyz * Simd32x3::from(self[e4])) + (geometric_product_g0.yzx() * self.group0().zxy())).with_w(geometric_product_g1_xyz[2] * self[e3] * -1.0)
                - (self.group0().xyzy() * geometric_product_g0.www().with_w(geometric_product_g1_xyz[1]))
                - (self.group0().yzxx() * geometric_product_g0.zxy().with_w(geometric_product_g1_xyz[0])),
        )
    }
}
impl Sandwich<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        1        4        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       17       35        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let geometric_product_g1 = (Simd32x4::from([other[e2] * self[e3], other[e3] * self[e1], other[e1] * self[e2], (other[e2] * self[e2]) + (other[e3] * self[e3])])
            * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
            + (other.group0().zxyx() * self.group0().yzxx());
        Point::from_groups(
            // e1, e2, e3, e4
            (self.group0().xyzy() * geometric_product_g1.www().with_w(geometric_product_g0_xyz[1]))
                + (self.group0().yzxx() * geometric_product_g1.zxy().with_w(geometric_product_g0_xyz[0]))
                + (geometric_product_g1.yzx() * self.group0().zxy() * Simd32x3::from(-1.0)).with_w((geometric_product_g0_xyz[2] * self[e3]) + (geometric_product_g1[3] * self[e4])),
        )
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        Scalar::from_groups(
            // scalar
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
        )
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
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * f32::powi(self[scalar], 2))
    }
}
impl Sandwich<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar] * self[scalar], other[e1234] * self[scalar]]),
        )
    }
}
impl Sandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group1(),
        )
    }
}
impl Sandwich<Horizon> for Scalar {
    type Output = Horizon;
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * f32::powi(self[scalar], 2))
    }
}
impl Sandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(self[scalar]), 2) * other.group0(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(self[scalar]), 2) * other.group1(),
        )
    }
}
impl Sandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group1(),
        )
    }
}
impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::powi(Simd32x2::from(self[scalar]), 2) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group1(),
            // e41, e42, e43
            Simd32x3::powi(Simd32x3::from(self[scalar]), 2) * other.group2(),
            // e23, e31, e12
            Simd32x3::powi(Simd32x3::from(self[scalar]), 2) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::powi(Simd32x4::from(self[scalar]), 2) * other.group4(),
        )
    }
}
impl Sandwich<Origin> for Scalar {
    type Output = Origin;
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * f32::powi(self[scalar], 2))
    }
}
impl Sandwich<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], other[e321] * self[scalar]]),
        )
    }
}
impl Sandwich<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * Simd32x4::from([other[e1] * self[scalar], other[e2] * self[scalar], other[e3] * self[scalar], other[e4] * self[scalar]]),
        )
    }
}
impl Sandwich<Scalar> for Scalar {
    type Output = Scalar;
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * f32::powi(self[scalar], 2))
    }
}
