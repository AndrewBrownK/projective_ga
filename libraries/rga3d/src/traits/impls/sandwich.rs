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
//  Minimum:         0       2       0     N/A
//   Median:         4      10       0     N/A
//  Average:        14      21       0     N/A
//  Maximum:       146     171       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       2       0       0
//   Median:         8      21       0       0
//  Average:        40      47       0       0
//  Maximum:       396     397       0       0
impl std::ops::Div<SandwichInfix> for DualNum {
    type Output = SandwichInfixPartial<DualNum>;
    fn div(self, _rhs: SandwichInfix) -> Self::Output {
        SandwichInfixPartial(self)
    }
}
impl Sandwich<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar] * self[scalar])
    }
}
impl Sandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        7        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4       14        0      N/A
    //  no simd        8       24        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(self[scalar]) * other.group0().xyz();
        let geometric_product_g1_w = self[scalar] * other[e321];
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0_xyz * Simd32x3::from(self[scalar]))
                .with_w((geometric_product_g1_w * self[e1234]) + (self[scalar] * self[scalar] * other[e4]) - (self[scalar] * self[e1234] * other[e321])),
            // e423, e431, e412, e321
            ((geometric_product_g0_xyz * Simd32x3::from(self[e1234])) + (Simd32x3::from(self[scalar] * self[scalar]) * other.group1().xyz())
                - (Simd32x3::from(self[scalar] * self[e1234]) * other.group0().xyz()))
            .with_w(geometric_product_g1_w * self[scalar]),
        )
    }
}
impl Sandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ self[scalar] * self[scalar] * other[e321])
    }
}
impl Sandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       17        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x3::from(self[scalar]) * other.group1();
        Line::from_groups(
            // e41, e42, e43
            (geometric_product_g1 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[scalar] * self[scalar]) * other.group0())
                + (Simd32x3::from(self[scalar] * self[e1234]) * other.group1()),
            // e23, e31, e12
            geometric_product_g1 * Simd32x3::from(self[scalar]),
        )
    }
}
impl Sandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        2        5        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        8       22        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1 = Simd32x4::from(self[scalar]) * other.group1();
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g1 * Simd32x4::from(self[e1234]))
                + (Simd32x4::from(self[scalar] * self[scalar]) * other.group0())
                + (Simd32x4::from(self[scalar] * self[e1234]) * other.group1()),
            // e23, e31, e12, scalar
            geometric_product_g1 * Simd32x4::from(self[scalar]),
        )
    }
}
impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       18        0        0
    //    simd3        4       10        0      N/A
    // Totals...
    // yes simd        8       28        0      N/A
    //  no simd       16       48        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = self[scalar] * other[scalar];
        let geometric_product_g1_xyz = Simd32x3::from(self[scalar]) * other.group1().xyz();
        let geometric_product_g3 = Simd32x3::from(self[scalar]) * other.group3();
        let geometric_product_g4_w = self[scalar] * other[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g0_x * self[scalar],
                (geometric_product_g0_x * self[e1234]) + (self[scalar] * self[scalar] * other[e1234]) + (self[scalar] * self[e1234] * other[scalar]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g1_xyz * Simd32x3::from(self[scalar]))
                .with_w((geometric_product_g4_w * self[e1234]) + (self[scalar] * self[scalar] * other[e4]) - (self[scalar] * self[e1234] * other[e321])),
            // e41, e42, e43
            (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(self[scalar] * self[scalar]) * other.group2())
                + (Simd32x3::from(self[scalar] * self[e1234]) * other.group3()),
            // e23, e31, e12
            geometric_product_g3 * Simd32x3::from(self[scalar]),
            // e423, e431, e412, e321
            ((geometric_product_g1_xyz * Simd32x3::from(self[e1234])) + (Simd32x3::from(self[scalar] * self[scalar]) * other.group4().xyz())
                - (Simd32x3::from(self[scalar] * self[e1234]) * other.group1().xyz()))
            .with_w(geometric_product_g4_w * self[scalar]),
        )
    }
}
impl Sandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[scalar] * self[scalar] * other[e4])
    }
}
impl Sandwich<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar] * self[scalar]) * other.group0())
    }
}
impl Sandwich<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar] * self[scalar]) * other.group0())
    }
}
impl Sandwich<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        5        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e1234]) * self.group0().xyz();
        AntiScalar::from_groups(
            // e1234
            -(geometric_product_g1_xyz[0] * self[e1])
                - (geometric_product_g1_xyz[1] * self[e2])
                - (geometric_product_g1_xyz[2] * self[e3])
                - (other[e1234] * self[e321] * self[e321]),
        )
    }
}
impl Sandwich<DualNum> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       16        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       14       25        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[scalar]) * self.group0().xyz();
        let geometric_product_g1_xyz = (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group0().xyz());
        let geometric_product_g1_w = other[scalar] * self[e321];
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_product_g1_w * self[e321]) + (geometric_product_g0_xyz[0] * self[e1]) + (geometric_product_g0_xyz[1] * self[e2]) + (geometric_product_g0_xyz[2] * self[e3]),
            -(geometric_product_g1_w * self[e4])
                - (geometric_product_g0_xyz[0] * self[e423])
                - (geometric_product_g0_xyz[1] * self[e431])
                - (geometric_product_g0_xyz[2] * self[e412])
                - (geometric_product_g1_xyz[0] * self[e1])
                - (geometric_product_g1_xyz[1] * self[e2])
                - (geometric_product_g1_xyz[2] * self[e3])
                - (self[e321] * self[e321] * other[e1234])
                - (other[scalar] * self[e4] * self[e321]),
        ]))
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0       10        0      N/A
    //    simd4       23       14        0      N/A
    // Totals...
    // yes simd       30       34        0      N/A
    //  no simd       99       96        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
            + (other.group1().zxyx() * self.group0().yzxx())
            + (other.group1().wwwy() * self.group1().xyz().with_w(self[e2]))
            + Simd32x3::from(0.0).with_w((other[e412] * self[e3]) - (other[e3] * self[e412]))
            + (other.group0().yzx() * self.group1().zxy()).with_w(0.0)
            - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
            - (other.group0().zxyx() * self.group1().yzxx())
            - (self.group1().wwwy() * other.group1().xyz().with_w(other[e2]))
            - (other.group1().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_product_g1 = (other.group0().zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((other[e2] * self[e2]) + (other[e3] * self[e3]))
            - (Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]))
            - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g1[3]) * self.group0())
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g0[0]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e2])
                        + (geometric_product_g0[2] * self[e3])
                        + (geometric_product_g0[3] * self[e321])
                        + (geometric_product_g1[0] * self[e423])
                        + (geometric_product_g1[1] * self[e431])
                        + (geometric_product_g1[2] * self[e412]),
                )
                - (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g1.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz()).with_w(0.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                - (Simd32x4::from(geometric_product_g1[3]) * self.group1())
                - (Simd32x4::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1], geometric_product_g1[0]]) * self.group0().yzxx())
                - (geometric_product_g1.zxyz() * self.group1().yzx().with_w(self[e3]))
                - (self.group0().xyzy() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[1])),
        )
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        4        0      N/A
    //    simd4        7        6        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       28       37        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]);
        let geometric_product_g1 = Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz()).with_w(0.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                - (Simd32x4::from(geometric_product_g1[3]) * self.group1())
                - (Simd32x4::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1], geometric_product_g1[0]]) * self.group0().yzxx())
                - (geometric_product_g1.zxyz() * self.group1().yzx().with_w(self[e3]))
                - (self.group0().xyzy() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[1])),
        )
    }
}
impl Sandwich<Line> for Flector {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd2        0        4        0      N/A
    //    simd3       12       15        0      N/A
    //    simd4        9        2        0      N/A
    // Totals...
    // yes simd       27       34        0      N/A
    //  no simd       78       74        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0)
            .with_w(-(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]))
            + (Simd32x3::from(self[e321]) * other.group1()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = Simd32x3::from(0.0).with_w((self[e3] * other[e12]) * -1.0)
            + (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
            + (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0)
            + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
            + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx())
            - (other.group1().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (geometric_product_g0.zxy() * self.group1().yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) - (geometric_product_g0[0] * self[e431]) - (geometric_product_g1[0] * self[e2]))
                + (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(self[e4]) * geometric_product_g0.xyz())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (geometric_product_g0.yz() * self.group1().zx()).with_z(0.0)
                - (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                + (geometric_product_g0.yzx() * self.group0().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g0[1] * self[e1]) * -1.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g0.zx() * self.group0().yz()).with_z(0.0),
        )
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd3        0       12        0      N/A
    //    simd4       24       12        0      N/A
    // Totals...
    // yes simd       32       37        0      N/A
    //  no simd      104       97        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]))
            + (Simd32x4::from(other[scalar]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]))
            + (self.group0().zxy() * other.group1().yzx()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = (other.group1() * Simd32x3::from(self[e4]).with_w(self[e321]))
            + Simd32x3::from(0.0).with_w((self[e3] * other[e12]) * -1.0)
            + (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            + (self.group0().xxy() * other.group0().wzx()).with_w(0.0)
            + (self.group0().zyz() * other.group0().yww()).with_w(0.0)
            + (self.group1().xxy() * other.group1().wzx()).with_w(0.0)
            + (self.group1().zyz() * other.group1().yww()).with_w(0.0)
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
            - (other.group1().zxyy() * self.group1().yzx().with_w(self[e2]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(
                -(geometric_product_g0[1] * self[e431]) - (geometric_product_g0[2] * self[e412]) - (geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]),
            ) + (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                + (geometric_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                + (geometric_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                - (geometric_product_g0 * Simd32x3::from(self[e4]).with_w(self[e321]))
                - (geometric_product_g1 * Simd32x3::from(self[e321]).with_w(self[e4]))
                - (geometric_product_g0.yzxx() * self.group1().zxyx())
                - (geometric_product_g1.yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * Simd32x4::from([geometric_product_g0[0], geometric_product_g0[1], geometric_product_g0[2], geometric_product_g1[3]]))
                + (geometric_product_g0.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       33        0        0
    //    simd2        8       16        0      N/A
    //    simd3       24       30        0      N/A
    //    simd4       23       11        0      N/A
    // Totals...
    // yes simd       72       90        0      N/A
    //  no simd      197      199        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3])])
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
            - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]));
        let geometric_product_g1 = (Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]))
            + (Simd32x4::from(other[scalar]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]))
            + (other.group3().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (Simd32x3::from(other[e321]) * self.group1().xyz())
            + (self.group0().yzx() * other.group4().zxy())
            + Simd32x2::from(0.0).with_z((self[e431] * other[e1]) - (self[e2] * other[e423]) - (self[e423] * other[e2]))
            + (self.group1().zx() * other.group1().yz()).with_z(0.0)
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (Simd32x3::from(other[e4]) * self.group0().xyz())
            - (self.group0().zx() * other.group4().yz()).with_z(0.0)
            - (self.group1().yz() * other.group1().zx()).with_z(0.0);
        let geometric_product_g3 = (self.group0().yzx() * other.group1().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e1]) * -1.0)
            - (Simd32x3::from(self[e321]) * other.group1().xyz())
            - (Simd32x3::from(other[e321]) * self.group0().xyz())
            - (self.group0().zx() * other.group1().yz()).with_z(0.0);
        let geometric_product_g4 = (Simd32x4::from(other[scalar]) * self.group1())
            + Simd32x3::from(0.0).with_w((self[e3] * other[e12]) * -1.0)
            + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
            + (Simd32x3::from(self[e321]) * other.group2()).with_w(0.0)
            + (Simd32x3::from([other[e1234], other[e43], other[e41]]) * self.group0().xxy()).with_w(0.0)
            + (Simd32x3::from([other[e42], other[e1234], other[e1234]]) * self.group0().zyz()).with_w(0.0)
            + (other.group3().yzx() * self.group1().zxy()).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx())
            - (other.group3().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]);
        let reverse_g1 = self.group1() * Simd32x4::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[3] * reverse_g1[3]) - (geometric_product_g4[0] * self[e1]) - (geometric_product_g4[1] * self[e2]) - (geometric_product_g4[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g1[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g1[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g1[2]]))
                - (Simd32x2::from(geometric_product_g4[3]) * Simd32x2::from([reverse_g1[3], self[e4]])),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g2[2] * self[e3])
                        - (geometric_product_g3[0] * reverse_g1[0])
                        - (geometric_product_g3[1] * reverse_g1[1])
                        - (geometric_product_g3[2] * reverse_g1[2]),
                )
                + (geometric_product_g3 * Simd32x3::from(reverse_g1[3])).with_w(geometric_product_g2[1] * self[e2])
                - (geometric_product_g3.yzx() * self.group0().zxy()).with_w(geometric_product_g0[1] * reverse_g1[3]),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                + (Simd32x3::from(reverse_g1[3]) * geometric_product_g4.xyz())
                + (geometric_product_g1.yzx() * reverse_g1.zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g4[1] * self[e1]) - (geometric_product_g1[1] * reverse_g1[0]) - (geometric_product_g4[0] * self[e2]))
                + (geometric_product_g4.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g1.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g1.zx() * reverse_g1.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g1.yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) * -1.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (Simd32x3::from(reverse_g1[3]) * geometric_product_g1.xyz())
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (reverse_g1 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from([reverse_g1[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g1[2], reverse_g1[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group0().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g1[3])).with_w(geometric_product_g3[2] * self[e3])
                - (geometric_product_g3.yzx() * reverse_g1.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e4] * -1.0) * self.group0().xyz().with_w(self[e321]);
        Origin::from_groups(
            // e4
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g0[3] * self[e321]),
        )
    }
}
impl Sandwich<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        6        0      N/A
    //    simd4       11        7        0      N/A
    // Totals...
    // yes simd       12       16        0      N/A
    //  no simd       45       49        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
            + (self.group0().yzxx() * other.group0().zxyx())
            + Simd32x3::from(0.0).with_w((self[e2] * other[e431]) + (self[e3] * other[e412]))
            - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            - (self.group0().zxy() * other.group0().yzx()).with_w(0.0);
        let geometric_product_g1 = Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from([self[e4], self[e4], self[e431]]) * geometric_product_g1.xyx()).with_w(0.0)
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * geometric_product_g1.yzz()).with_w(0.0)
                + (geometric_product_g0.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0)
                - (Simd32x4::from(geometric_product_g1[3]) * self.group1())
                - (Simd32x4::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1], geometric_product_g1[0]]) * self.group0().yzxx())
                - (geometric_product_g1.zxyz() * self.group1().yzx().with_w(self[e3]))
                - (self.group0().xyzy() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[1])),
        )
    }
}
impl Sandwich<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd3        0        6        0      N/A
    //    simd4       11        5        0      N/A
    // Totals...
    // yes simd       18       21        0      N/A
    //  no simd       51       48        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(self[e431] * other[e2]) - (self[e412] * other[e3]))
            + (Simd32x3::from(self[e4]) * other.group0().xyz()).with_w(0.0)
            + (self.group1().zxy() * other.group0().yzx()).with_w(0.0)
            - (Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]))
            - (self.group1().yzxx() * other.group0().zxyx());
        let geometric_product_g1 = (self.group0().yzxx() * other.group0().zxyx()) + Simd32x3::from(0.0).with_w((self[e2] * other[e2]) + (self[e3] * other[e3]))
            - (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(0.0)
            - (self.group0().zxy() * other.group0().yzx()).with_w(0.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g1[3]) * self.group0())
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g0[0]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e2])
                        + (geometric_product_g0[2] * self[e3])
                        + (geometric_product_g0[3] * self[e321])
                        + (geometric_product_g1[0] * self[e423])
                        + (geometric_product_g1[1] * self[e431])
                        + (geometric_product_g1[2] * self[e412]),
                )
                - (geometric_product_g1.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g1.yzz() * self.group0().zx().with_z(self[e321])).with_w(0.0),
        )
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       10       14        0      N/A
    //  no simd       10       20        0        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]) + (geometric_product_g1[3] * self[e321]),
            -(geometric_product_g0[0] * self[e423])
                - (geometric_product_g0[1] * self[e431])
                - (geometric_product_g0[2] * self[e412])
                - (geometric_product_g0[3] * self[e321])
                - (geometric_product_g1[0] * self[e1])
                - (geometric_product_g1[1] * self[e2])
                - (geometric_product_g1[2] * self[e3])
                - (geometric_product_g1[3] * self[e4]),
        ]))
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e321] * self[e321] * -1.0)
    }
}
impl Sandwich<DualNum> for Horizon {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd2        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        6        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([other[scalar] * self[e321], other[e1234] * self[e321]]) * Simd32x2::from([1.0, -1.0]),
        )
    }
}
impl Sandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        0        9        0      N/A
    //  no simd        0       26        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(self[e321] * -1.0) * other.group1().xyz().with_w(other[e4]);
        let geometric_product_g1 = Simd32x4::from(self[e321] * -1.0) * other.group0().xyz().with_w(other[e321]);
        let reverse_g0 = self[e321] * -1.0;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(reverse_g0) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(reverse_g0) * (geometric_product_g0.xyz() * Simd32x3::from(-1.0)).with_w(geometric_product_g1[3]),
        )
    }
}
impl Sandwich<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[e321] * self[e321])
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       10        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let reverse_g0 = self[e321] * -1.0;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(reverse_g0 * self[e321]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(reverse_g0 * self[e321] * -1.0) * other.group1(),
        )
    }
}
impl Sandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       18        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e1234]);
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[scalar]);
        let reverse_g0 = self[e321] * -1.0;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(reverse_g0) * geometric_product_g1.xyz().with_w(geometric_product_g0[3]),
            // e23, e31, e12, scalar
            Simd32x4::from(reverse_g0 * -1.0) * geometric_product_g0.xyz().with_w(geometric_product_g1[3]),
        )
    }
}
impl Sandwich<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd2        0        3        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0       17        0      N/A
    //  no simd        0       42        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(self[e321] * -1.0) * Simd32x2::from([other[e321], other[e4]]);
        let geometric_product_g1 = Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]);
        let geometric_product_g4 = Simd32x4::from(self[e321]) * other.group2().with_w(other[scalar]);
        let reverse_g0 = self[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(reverse_g0) * Simd32x2::from([geometric_product_g4[3], geometric_product_g1[3]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(reverse_g0 * -1.0) * Simd32x4::from([self[e321], self[e321], self[e321] * other[e3], geometric_product_g0[1]]) * other.group1().xy().with_zw(1.0, 1.0),
            // e41, e42, e43
            Simd32x3::from(reverse_g0) * geometric_product_g4.xyz(),
            // e23, e31, e12
            Simd32x3::from(reverse_g0 * -1.0) * geometric_product_g1.xyz(),
            // e423, e431, e412, e321
            Simd32x4::from(reverse_g0) * Simd32x4::from([self[e321], self[e321], self[e321] * other[e412], geometric_product_g0[0]]) * other.group4().xy().with_zw(1.0, 1.0),
        )
    }
}
impl Sandwich<Origin> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ self[e321] * self[e321] * other[e4] * -1.0)
    }
}
impl Sandwich<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       14        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * (Simd32x3::from(self[e321] * -1.0) * other.group0().xyz()).with_w(self[e321] * other[e321] * -1.0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * (Simd32x3::from(self[e321]) * other.group0().xyz()).with_w(self[e321] * other[e4] * -1.0),
        )
    }
}
impl Sandwich<Scalar> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[e1234]) * self.group1();
        AntiScalar::from_groups(
            // e1234
            (geometric_product_g0[0] * self[e23]) + (geometric_product_g0[1] * self[e31]) + (geometric_product_g0[2] * self[e12]),
        )
    }
}
impl Sandwich<DualNum> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       11       18        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1());
        let geometric_product_g1 = Simd32x3::from(other[scalar]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[0] * self[e41]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]),
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
        )
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd3        0       13        0      N/A
    //    simd4       17        5        0      N/A
    // Totals...
    // yes simd       23       32        0      N/A
    //  no simd       74       73        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
            + (Simd32x3::from(other[e321]) * self.group1()).with_w(other[e2] * self[e42])
            - (self.group1().yzx() * other.group0().zxy()).with_w(other[e423] * self[e23]);
        let geometric_product_g1 = (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            + (self.group1().zxy() * other.group1().yzx()).with_w(0.0)
            - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx())
            - (Simd32x3::from(other[e321]) * self.group0()).with_w(other[e3] * self[e12])
            - (self.group1().yzx() * other.group1().zxy()).with_w(other[e2] * self[e31]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e42])
                        + (geometric_product_g0[2] * self[e43])
                        + (geometric_product_g1[0] * self[e23])
                        + (geometric_product_g1[1] * self[e31])
                        + (geometric_product_g1[2] * self[e12]),
                )
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from([geometric_product_g1[1], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0[1] * self[e31]]) * self.group1().zxy().with_w(1.0))
                + (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * geometric_product_g0.yzxx())
                + Simd32x3::from(0.0).with_w(geometric_product_g0[2] * self[e12])
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1()).with_w(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0()).with_w(0.0)
                - (self.group0().yzx() * geometric_product_g0.zxy()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        7        0      N/A
    //    simd4        4        0        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd       16       25        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1();
        let geometric_product_g1_xyz = Simd32x3::from(other[e321] * -1.0) * self.group0();
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(geometric_product_g0_xyz[2] * self[e12])
                + (geometric_product_g0_xyz.zxy() * reverse_g0.yzx()).with_w(geometric_product_g0_xyz[0] * self[e23])
                + (geometric_product_g1_xyz.yzx() * self.group1().zxy()).with_w(geometric_product_g0_xyz[1] * self[e31])
                - (geometric_product_g0_xyz.yzx() * reverse_g0.zxy()).with_w(0.0)
                - (geometric_product_g1_xyz.zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl Sandwich<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd3        7       17        0      N/A
    //    simd4        6        0        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       49       60        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (other.group0().yzx() * self.group1().zxy()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (other.group0().zxy() * self.group1().yzx()).with_w(other[e41] * self[e23])
            - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]);
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(other[e31] * self[e31]) - (other[e12] * self[e12])) + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
            - (other.group1().zxy() * self.group1().yzx()).with_w(other[e23] * self[e23]);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e41, e42, e43
            (reverse_g0 * Simd32x3::from(geometric_product_g1[3]))
                + (reverse_g1 * Simd32x3::from(geometric_product_g0[3]))
                + (reverse_g0.yzx() * geometric_product_g1.zxy())
                + (reverse_g1.yzx() * geometric_product_g0.zxy())
                - (reverse_g0.zxy() * geometric_product_g1.yzx())
                - (reverse_g1.zxy() * geometric_product_g0.yzx()),
            // e23, e31, e12
            (reverse_g1 * Simd32x3::from(geometric_product_g1[3])) + (reverse_g1.yzx() * geometric_product_g1.zxy()) - (reverse_g1.zxy() * geometric_product_g1.yzx()),
        )
    }
}
impl Sandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0       12        0      N/A
    //    simd4       18        6        0      N/A
    // Totals...
    // yes simd       26       30        0      N/A
    //  no simd       80       72        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e31] * other[e42]) - (self[e12] * other[e43]))
            + (self.group0().xxy() * other.group1().wzx()).with_w(0.0)
            + (self.group0().zyz() * other.group1().yww()).with_w(0.0)
            + (self.group1().xxy() * other.group0().wzx()).with_w(0.0)
            + (self.group1().zyz() * other.group0().yww()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxyx())
            - (self.group1().yzxx() * other.group0().zxyx());
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(self[e31] * other[e31]) - (self[e12] * other[e12]))
            + (self.group1().xxy() * other.group1().wzx()).with_w(0.0)
            + (self.group1().zyz() * other.group1().yww()).with_w(0.0)
            - (self.group1().yzxx() * other.group1().zxyx());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().zxyx() * geometric_product_g1.yzxx())
                + (self.group1().zxyx() * geometric_product_g0.yzxx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e31]) + (geometric_product_g0[2] * self[e12]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]),
                )
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1()).with_w(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0()).with_w(0.0)
                - (self.group0().yzx() * geometric_product_g1.zxy()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(0.0),
            // e23, e31, e12, scalar
            (self.group1().zxyx() * geometric_product_g1.yzxx()) + Simd32x3::from(0.0).with_w((geometric_product_g1[1] * self[e31]) + (geometric_product_g1[2] * self[e12]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       21        0        0
    //    simd2        6        6        0      N/A
    //    simd3       14       33        0      N/A
    //    simd4       17        5        0      N/A
    // Totals...
    // yes simd       47       65        0      N/A
    //  no simd      132      152        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
            - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_product_g1 = (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            + (Simd32x3::from(other[e321]) * self.group1()).with_w(self[e42] * other[e2])
            - (self.group1().yzx() * other.group1().zxy()).with_w(self[e23] * other[e423]);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group0())
            + (Simd32x3::from(other[e1234]) * self.group1())
            + (self.group0().zxy() * other.group3().yzx())
            + (self.group1().zxy() * other.group2().yzx())
            - (self.group0().yzx() * other.group3().zxy())
            - (self.group1().yzx() * other.group2().zxy());
        let geometric_product_g3 = (Simd32x3::from(other[scalar]) * self.group1()) + (self.group1().zxy() * other.group3().yzx()) - (self.group1().yzx() * other.group3().zxy());
        let geometric_product_g4 = (Simd32x3::from([other[e4], other[e412], other[e423]]) * self.group1().xxy()).with_w(0.0)
            + (Simd32x3::from([other[e431], other[e4], other[e4]]) * self.group1().zyz()).with_w(0.0)
            + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
            - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group1().yzxx())
            - (Simd32x3::from(other[e321]) * self.group0()).with_w(self[e31] * other[e2])
            - (self.group1().yzx() * other.group4().zxy()).with_w(self[e12] * other[e3]);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        let reverse_g1 = self.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(geometric_product_g3[0] * reverse_g0[0]) - (geometric_product_g3[1] * reverse_g0[1]) - (geometric_product_g3[2] * reverse_g0[2]),
            ]) - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                -(reverse_g0[1] * geometric_product_g1[1])
                    - (reverse_g0[2] * geometric_product_g1[2])
                    - (reverse_g1[0] * geometric_product_g4[0])
                    - (reverse_g1[1] * geometric_product_g4[1])
                    - (reverse_g1[2] * geometric_product_g4[2]),
            ) + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g1.yzz()).with_w(0.0)
                + (reverse_g1.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                - (geometric_product_g1.yzxx() * reverse_g1.zxy().with_w(reverse_g0[0])),
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
            Simd32x3::from(0.0).with_w((reverse_g1[2] * geometric_product_g1[2]) * -1.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g0.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g1.yzz()).with_w(0.0)
                + (reverse_g0.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g1.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g1[1] * geometric_product_g1[1]])
                    * reverse_g1.zxy().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g0.zxy().with_w(reverse_g1[0])),
        )
    }
}
impl Sandwich<Origin> for Line {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e4]) * self.group1();
        Origin::from_groups(
            // e4
            (geometric_product_g0_xyz[0] * self[e23]) + (geometric_product_g0_xyz[1] * self[e31]) + (geometric_product_g0_xyz[2] * self[e12]),
        )
    }
}
impl Sandwich<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0       10        0      N/A
    //    simd4        7        1        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       30       40        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1();
        let geometric_product_g1 = (self.group1().zxy() * other.group0().yzx()).with_w(0.0)
            - (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
            - (self.group1().yzx() * other.group0().zxy()).with_w(0.0);
        let reverse_g0 = self.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x4::from([geometric_product_g1[1], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0_xyz[2] * self[e12]])
                * self.group1().zxy().with_w(1.0))
                + (reverse_g0 * Simd32x3::from(geometric_product_g1[3])).with_w(geometric_product_g0_xyz[1] * self[e31])
                + (geometric_product_g0_xyz.zxy() * reverse_g0.yzx()).with_w(geometric_product_g0_xyz[0] * self[e23])
                - (Simd32x3::from(-(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])) * self.group1()).with_w(0.0)
                - (geometric_product_g0_xyz.yzx() * reverse_g0.zxy()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g1.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Point> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        5        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       14       17        0      N/A
    //  no simd       38       36        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])) + Simd32x3::from(0.0).with_w((self[e42] * other[e2]) + (self[e43] * other[e3]))
            - (self.group1().yzx() * other.group0().zxy()).with_w(0.0);
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(self[e31] * other[e2]) - (self[e12] * other[e3]))
            + (Simd32x3::from(other[e4]) * self.group1()).with_w(0.0)
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (Simd32x4::from([self[e43], self[e41], self[e42], self[e23]]) * other.group0().yzxx());
        Point::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e42])
                        + (geometric_product_g0[2] * self[e43])
                        + (geometric_product_g1[0] * self[e23])
                        + (geometric_product_g1[1] * self[e31])
                        + (geometric_product_g1[2] * self[e12]),
                )
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1()).with_w(0.0)
                - (self.group1().yzx() * geometric_product_g0.zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Scalar> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd2        3        3        0      N/A
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        8       15        0        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x3::from(other[scalar]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[0] * self[e41]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]),
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]])),
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        8        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e1234]) * self.group1();
        AntiScalar::from_groups(
            // e1234
            (geometric_product_g0[0] * self[e23]) + (geometric_product_g0[1] * self[e31]) + (geometric_product_g0[2] * self[e12]) + (geometric_product_g0[3] * self[scalar]),
        )
    }
}
impl Sandwich<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd2        4        4        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       15       24        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1());
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[0] * self[e41]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]) + (geometric_product_g1[3] * self[e1234]),
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]]))
                + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]])),
        )
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       11        0        0
    //    simd3        0       10        0      N/A
    //    simd4       23       15        0      N/A
    // Totals...
    // yes simd       30       36        0      N/A
    //  no simd       99      101        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[scalar]) * other.group0())
            + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((other[e2] * self[e42]) + (other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234]))
            + (Simd32x3::from(other[e321]) * self.group1().xyz()).with_w(0.0)
            - (self.group1().yzxx() * other.group0().zxy().with_w(other[e423]));
        let geometric_product_g1 = (self.group1() * Simd32x3::from(other[e4]).with_w(other[e321]))
            + (other.group0().zxy() * self.group0().yzx()).with_w(0.0)
            + (other.group1().xyx() * self.group1().wwy()).with_w(0.0)
            + (other.group1().yzz() * self.group1().zxw()).with_w(0.0)
            - (Simd32x4::from([self[e1234], self[e1234], self[e42], self[e23]]) * other.group0().xyxx())
            - (other.group0().yzzy() * self.group0().zxw().with_w(self[e31]))
            - (self.group1().yzxz() * other.group1().zxy().with_w(other[e3]))
            - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(reverse_g1[3]))
                + (Simd32x4::from(geometric_product_g1[3]) * Simd32x4::from([reverse_g1[0], reverse_g1[1], reverse_g1[2], self[e1234]]))
                + (Simd32x4::from([reverse_g1[1], reverse_g1[2], reverse_g1[0], self[e41]]) * geometric_product_g0.zxyx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e42]) + (geometric_product_g0[2] * self[e43])
                        - (geometric_product_g1[1] * reverse_g1[1])
                        - (geometric_product_g1[2] * reverse_g1[2]),
                )
                - (Simd32x4::from([geometric_product_g0[1], geometric_product_g0[2], geometric_product_g0[0], geometric_product_g1[0]]) * reverse_g1.zxyx()),
            // e423, e431, e412, e321
            (reverse_g1 * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[3]))
                + Simd32x3::from(0.0).with_w((geometric_product_g0[2] * reverse_g1[2]) * -1.0)
                + (geometric_product_g0.xyx() * self.group0().wwy()).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zxw()).with_w(0.0)
                + (geometric_product_g1.xxy() * reverse_g1.wzx()).with_w(0.0)
                + (geometric_product_g1.zyz() * reverse_g1.yww()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0[1]]) * reverse_g1.zxyy())
                - (geometric_product_g0.zxyx() * self.group0().yzx().with_w(reverse_g1[0]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        6        0      N/A
    //    simd4        8        7        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       32       48        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let geometric_product_g1 = Simd32x4::from(other[e321]) * (self.group0().xyz() * Simd32x3::from(-1.0)).with_w(self[scalar]);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (reverse_g1 * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[3]))
                + Simd32x3::from(0.0).with_w((geometric_product_g0[2] * reverse_g1[2]) * -1.0)
                + (geometric_product_g0.xyx() * self.group0().wwy()).with_w(0.0)
                + (geometric_product_g0.yzz() * self.group0().zxw()).with_w(0.0)
                + (geometric_product_g1.xxy() * reverse_g1.wzx()).with_w(0.0)
                + (geometric_product_g1.zyz() * reverse_g1.yww()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g0[1]]) * reverse_g1.zxyy())
                - (geometric_product_g0.zxyx() * self.group0().yzx().with_w(reverse_g1[0]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Line> for Motor {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd2        0        4        0      N/A
    //    simd3       12       14        0      N/A
    //    simd4        9        3        0      N/A
    // Totals...
    // yes simd       27       32        0      N/A
    //  no simd       78       73        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (other.group0().xyx() * self.group1().wwy()).with_w(0.0)
            + (other.group0().yzz() * self.group1().zxw()).with_w(0.0)
            + (other.group1().xyx() * self.group0().wwy()).with_w(0.0)
            + (other.group1().yzz() * self.group0().zxw()).with_w(0.0)
            - (other.group0().zxyx() * self.group1().yzxx())
            - (other.group1().zxyx() * self.group0().yzxx());
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(other[e31] * self[e31]) - (other[e12] * self[e12]))
            + (other.group1().xyx() * self.group1().wwy()).with_w(0.0)
            + (other.group1().yzz() * self.group1().zxw()).with_w(0.0)
            - (other.group1().zxyx() * self.group1().yzxx());
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz())
                + (Simd32x3::from(self[scalar]) * geometric_product_g0.xyz())
                + (geometric_product_g0.yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e42]) - (geometric_product_g0[1] * self[e23]) - (geometric_product_g1[1] * self[e41]))
                + (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz())
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g0.zx() * self.group1().yz()).with_z(0.0)
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * geometric_product_g1.xyz())
                + (geometric_product_g1.yzx() * self.group1().zxy())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e23]) * -1.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
        )
    }
}
impl Sandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd3        0       12        0      N/A
    //    simd4       24       12        0      N/A
    // Totals...
    // yes simd       32       36        0      N/A
    //  no simd      104       96        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[e1234]) * other.group1())
            + (Simd32x4::from(self[scalar]) * other.group0())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(other[scalar]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().yzx() * self.group1().zxy()).with_w(0.0)
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (other.group0().zxyx() * self.group1().yzxx())
            - (other.group1().zxyx() * self.group0().yzxx());
        let geometric_product_g1 = (Simd32x4::from(self[scalar]) * other.group1())
            + Simd32x3::from(0.0).with_w(-(other[e31] * self[e31]) - (other[e12] * self[e12]))
            + (Simd32x3::from(other[scalar]) * self.group1().xyz()).with_w(0.0)
            + (other.group1().yzx() * self.group1().zxy()).with_w(0.0)
            - (other.group1().zxyx() * self.group1().yzxx());
        Motor::from_groups(
            // e41, e42, e43, e1234
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g1 * Simd32x4::from(self[e1234]))
                + (geometric_product_g0.yzxx() * self.group1().zxyx())
                + (geometric_product_g1.yzxx() * self.group0().zxyx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e31]) + (geometric_product_g0[2] * self[e12]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]),
                )
                - (Simd32x3::from(geometric_product_g0[3]) * self.group1().xyz()).with_w(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0.zxy() * self.group1().yzx()).with_w(0.0)
                - (geometric_product_g1.zxy() * self.group0().yzx()).with_w(0.0),
            // e23, e31, e12, scalar
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (geometric_product_g1.yzxx() * self.group1().zxyx())
                + Simd32x3::from(0.0).with_w((geometric_product_g1[1] * self[e31]) + (geometric_product_g1[2] * self[e12]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                - (geometric_product_g1.zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       18        0        0
    //    simd2        8        8        0      N/A
    //    simd3       20       33        0      N/A
    //    simd4       23       16        0      N/A
    // Totals...
    // yes simd       63       75        0      N/A
    //  no simd      180      197        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (self[e1234] * other[scalar]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
            + (Simd32x2::from(self[scalar]) * other.group0())
            - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
            - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
            - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]]));
        let geometric_product_g1 = (self.group1() * Simd32x3::from(other[e321]).with_w(other[e4]))
            + (other.group1().xyzy() * Simd32x3::from(self[scalar]).with_w(self[e42]))
            + (other.group1().yzxx() * self.group1().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((self[e43] * other[e3]) - (self[e1234] * other[e321]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e423]));
        let geometric_product_g2 = (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (Simd32x3::from(other[scalar]) * self.group0().xyz())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz())
            + (other.group2().yzx() * self.group1().zxy())
            + (other.group3().yzx() * self.group0().zxy())
            - (other.group2().zxy() * self.group1().yzx())
            - (other.group3().zxy() * self.group0().yzx());
        let geometric_product_g3 =
            (Simd32x3::from(self[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (other.group3().yzx() * self.group1().zxy())
                - (other.group3().zxy() * self.group1().yzx());
        let geometric_product_g4 = (self.group1() * Simd32x3::from(other[e4]).with_w(other[e321]))
            + (Simd32x3::from(self[scalar]) * other.group4().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group1().zxy()).with_w(0.0)
            + (self.group1().zxy() * other.group4().yzx()).with_w(0.0)
            - (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e31]]) * other.group1().xyzy())
            - (self.group1().yzxz() * other.group4().zxy().with_w(other[e3]))
            - (other.group1().yzxx() * self.group0().zxy().with_w(self[e23]))
            - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0);
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0[0] * self[e1234]) + (geometric_product_g3[0] * self[e41]) + (geometric_product_g3[1] * self[e42]) + (geometric_product_g3[2] * self[e43]),
            ]) + (geometric_product_g0 * Simd32x2::from(reverse_g1[3]))
                - (Simd32x2::from(reverse_g1[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g1[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g1[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(reverse_g1[3]))
                + (Simd32x4::from(geometric_product_g4[3]) * Simd32x4::from([reverse_g1[0], reverse_g1[1], reverse_g1[2], self[e1234]]))
                + (Simd32x4::from([reverse_g1[1], reverse_g1[2], reverse_g1[0], self[e41]]) * geometric_product_g1.zxyx())
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43])
                        - (geometric_product_g4[1] * reverse_g1[1])
                        - (geometric_product_g4[2] * reverse_g1[2]),
                )
                - (Simd32x4::from([geometric_product_g1[1], geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[0]]) * reverse_g1.zxyx()),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(reverse_g1[3]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (Simd32x3::from(geometric_product_g0[1]) * reverse_g1.xyz())
                + (geometric_product_g2.zxy() * reverse_g1.yzx())
                + (geometric_product_g3.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_product_g0[0]) * self.group0().xyz())
                - (geometric_product_g2.yzx() * reverse_g1.zxy())
                - (geometric_product_g3.zxy() * self.group0().yzx()),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(reverse_g1[3])) + (Simd32x3::from(geometric_product_g0[0]) * reverse_g1.xyz()) + (geometric_product_g3.zxy() * reverse_g1.yzx())
                - (geometric_product_g3.yzx() * reverse_g1.zxy()),
            // e423, e431, e412, e321
            (reverse_g1 * Simd32x3::from(geometric_product_g1[3]).with_w(geometric_product_g4[3]))
                + Simd32x3::from(0.0).with_w((geometric_product_g1[2] * reverse_g1[2]) * -1.0)
                + (geometric_product_g1.xyx() * self.group0().wwy()).with_w(0.0)
                + (geometric_product_g1.yzz() * self.group0().zxw()).with_w(0.0)
                + (geometric_product_g4.xxy() * reverse_g1.wzx()).with_w(0.0)
                + (geometric_product_g4.zyz() * reverse_g1.yww()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[1]]) * reverse_g1.zxyy())
                - (geometric_product_g1.zxyx() * self.group0().yzx().with_w(reverse_g1[0]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        8        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e4]) * self.group1().xyz();
        Origin::from_groups(
            // e4
            (geometric_product_g1_xyz[0] * self[e23])
                + (geometric_product_g1_xyz[1] * self[e31])
                + (geometric_product_g1_xyz[2] * self[e12])
                + (self[scalar] * self[scalar] * other[e4]),
        )
    }
}
impl Sandwich<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       10        0      N/A
    //    simd4        8        3        0      N/A
    // Totals...
    // yes simd       15       24        0      N/A
    //  no simd       47       54        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(other[e321]) * self.group1().xyz();
        let geometric_product_g1_xyz =
            (Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group1().zxy() * other.group0().yzx()) + Simd32x2::from(0.0).with_z((self[e23] * other[e431]) * -1.0)
                - (Simd32x3::from(other[e321]) * self.group0().xyz())
                - (self.group1().yz() * other.group0().zx()).with_z(0.0);
        let geometric_product_g1_w = self[scalar] * other[e321];
        let reverse_g1 = self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Plane::from_groups(
            // e423, e431, e412, e321
            (reverse_g1
                * Simd32x3::from(-(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])).with_w(geometric_product_g1_w))
                + Simd32x3::from(0.0).with_w((geometric_product_g0_xyz[2] * reverse_g1[2]) * -1.0)
                + (geometric_product_g0_xyz.xyx() * self.group0().wwy()).with_w(0.0)
                + (geometric_product_g0_xyz.yzz() * self.group0().zxw()).with_w(0.0)
                + (geometric_product_g1_xyz.xxy() * reverse_g1.wzx()).with_w(0.0)
                + (geometric_product_g1_xyz.zyz() * reverse_g1.yww()).with_w(0.0)
                - (reverse_g1.zxyx() * geometric_product_g1_xyz.yzx().with_w(geometric_product_g0_xyz[0]))
                - (Simd32x3::from(geometric_product_g1_w) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0_xyz.zxy() * self.group0().yzx()).with_w(geometric_product_g0_xyz[1] * reverse_g1[1]),
        )
    }
}
impl Sandwich<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        0        5        0      N/A
    //    simd4       11        6        0      N/A
    // Totals...
    // yes simd       17       21        0      N/A
    //  no simd       50       49        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[scalar]) * other.group0())
            + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((self[e42] * other[e2]) + (self[e43] * other[e3]))
            - (self.group1().yzx() * other.group0().zxy()).with_w(0.0);
        let geometric_product_g1 = Simd32x3::from(0.0).with_w((self[e12] * other[e3]) * -1.0)
            + (Simd32x3::from(other[e4]) * self.group1().xyz()).with_w(0.0)
            + (self.group0().yzx() * other.group0().zxy()).with_w(0.0)
            - (Simd32x4::from([self[e1234], self[e1234], self[e1234], self[e31]]) * other.group0().xyzy())
            - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23]));
        Point::from_groups(
            // e1, e2, e3, e4
            (geometric_product_g0 * Simd32x4::from(self[scalar]))
                + (geometric_product_g0.yzxx() * self.group1().zxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e42])
                        + (geometric_product_g0[2] * self[e43])
                        + (geometric_product_g1[0] * self[e23])
                        + (geometric_product_g1[1] * self[e31])
                        + (geometric_product_g1[2] * self[e12])
                        + (geometric_product_g1[3] * self[e1234]),
                )
                - (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz()).with_w(0.0)
                - (geometric_product_g0.zxy() * self.group1().yzx()).with_w(0.0),
        )
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd2        4        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       10        0      N/A
    //  no simd       11       20        0        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g1[0] * self[e41]) + (geometric_product_g1[1] * self[e42]) + (geometric_product_g1[2] * self[e43]) + (geometric_product_g1[3] * self[e1234]),
            ]) + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g1[0], geometric_product_g0[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g1[1], geometric_product_g0[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g1[2], geometric_product_g0[2]]))
                + (Simd32x2::from(self[scalar]) * Simd32x2::from([geometric_product_g1[3], geometric_product_g0[3]])),
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
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd       14       20        0      N/A
    //  no simd       14       24        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_y = other[e1234] * self[scalar];
        let geometric_product_g1_w = other[e1234] * self[e321];
        let geometric_product_g2 = Simd32x3::from(other[e1234]) * self.group3();
        let geometric_product_g4_xyz = Simd32x3::from(other[e1234]) * self.group1().xyz();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g2[0] * self[e23]) + (geometric_product_g2[1] * self[e31]) + (geometric_product_g2[2] * self[e12])
                    - (geometric_product_g1_w * self[e321])
                    - (geometric_product_g4_xyz[0] * self[e1])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (geometric_product_g0_y * self[e321])
                    + (geometric_product_g1_w * self[scalar])
                    + (geometric_product_g2[0] * self[e1])
                    + (geometric_product_g2[1] * self[e2])
                    + (geometric_product_g2[2] * self[e3])
                    + (geometric_product_g4_xyz[0] * self[e23])
                    + (geometric_product_g4_xyz[1] * self[e31])
                    + (geometric_product_g4_xyz[2] * self[e12]),
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
    //           add/sub      mul      div      pow
    //      f32       27       34        0        0
    //    simd2        4        4        0      N/A
    //    simd3        2       10        0      N/A
    //    simd4        8        4        0      N/A
    // Totals...
    // yes simd       41       52        0      N/A
    //  no simd       73       88        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[scalar] * self[scalar];
        let geometric_product_g0_y = (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]);
        let geometric_product_g1 = (Simd32x3::from(other[scalar]) * self.group1().xyz()).with_w((other[scalar] * self[e4]) + (other[e1234] * self[e321]));
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3());
        let geometric_product_g3 = Simd32x3::from(other[scalar]) * self.group3();
        let geometric_product_g4_xyz = (Simd32x3::from(other[scalar]) * self.group4().xyz()) + (Simd32x3::from(other[e1234]) * self.group1().xyz());
        let geometric_product_g4_w = other[scalar] * self[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g4_w * self[e321]) + (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]),
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g3[0] * self[e41]) + (geometric_product_g3[1] * self[e42]) + (geometric_product_g3[2] * self[e43])
                    - (geometric_product_g4_w * self[e4])
                    - (geometric_product_g4_xyz[0] * self[e1])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321]),
            ]) + (Simd32x2::from(geometric_product_g0_x) * self.group0())
                + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0_x) * self.group1())
                + (geometric_product_g1.yzxx() * self.group3().zxy().with_w(self[e41]))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0_y * self[e321])
                        + (geometric_product_g4_w * self[e1234])
                        + (geometric_product_g2[1] * self[e2])
                        + (geometric_product_g2[2] * self[e3])
                        + (geometric_product_g3[0] * self[e423])
                        + (geometric_product_g3[1] * self[e431])
                        + (geometric_product_g3[2] * self[e412])
                        + (geometric_product_g4_xyz[0] * self[e23])
                        + (geometric_product_g4_xyz[1] * self[e31])
                        + (geometric_product_g4_xyz[2] * self[e12])
                        + (geometric_product_g1[1] * self[e42])
                        + (geometric_product_g1[2] * self[e43]),
                )
                - (Simd32x3::from(geometric_product_g4_w) * self.group3()).with_w(0.0)
                - (geometric_product_g3.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       26       44        0        0
    //    simd2       12       22        0      N/A
    //    simd3       36       46        0      N/A
    //    simd4       34       19        0      N/A
    // Totals...
    // yes simd      108      131        0      N/A
    //  no simd      294      302        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412])])
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
            - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]));
        let geometric_product_g1 = (Simd32x4::from(self[scalar]) * other.group0())
            + (other.group0().yzxx() * self.group3().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234]))
            + (Simd32x3::from(other[e321]) * self.group3()).with_w(other[e2] * self[e42])
            - (self.group3().yzx() * other.group0().zxy()).with_w(other[e423] * self[e23]);
        let geometric_product_g2 = (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from(self[e4]) * other.group0().xyz())
            + (other.group0().yzx() * self.group4().zxy())
            + Simd32x2::from(0.0).with_z((other[e431] * self[e1]) - (other[e2] * self[e423]) - (other[e423] * self[e2]))
            + (other.group1().zx() * self.group1().yz()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group1().xyz())
            - (other.group0().zx() * self.group4().yz()).with_z(0.0)
            - (other.group1().yz() * self.group1().zx()).with_z(0.0);
        let geometric_product_g3 = (other.group0().zxy() * self.group1().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e2]) * -1.0)
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group0().xyz())
            - (other.group0().yz() * self.group1().zx()).with_z(0.0);
        let geometric_product_g4 = (Simd32x4::from(self[scalar]) * other.group1())
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
            + (self.group3().zxy() * other.group1().yzx()).with_w(0.0)
            - (other.group0().xyxx() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e23]))
            - (other.group0().yzzy() * self.group2().zx().with_zw(self[e1234], self[e31]))
            - (Simd32x3::from(other[e321]) * self.group2()).with_w(0.0)
            - (self.group3().yzx() * other.group1().zxy()).with_w(other[e3] * self[e12]);
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
                    - (geometric_product_g4[0] * self[e1])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from(geometric_product_g4[3]) * Simd32x2::from([reverse_g4[3], self[e4]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from(geometric_product_g4[3]) * reverse_g3.with_w(self[e1234]))
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g2[2] * self[e3]]) * reverse_g3.yzx().with_w(1.0))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[0] * reverse_g4[0])
                        - (geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                + (geometric_product_g3 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g2[1] * self[e2])
                - (geometric_product_g1.yzxx() * reverse_g3.zxy().with_w(reverse_g2[0]))
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * self[e1])
                        - (geometric_product_g1[1] * reverse_g4[0])
                        - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * reverse_g4.zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g1.zx() * reverse_g4.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]))
                + (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(reverse_g4[3]) * geometric_product_g1.xyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g3[2] * geometric_product_g1[2]])
                    * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], reverse_g3[1] * geometric_product_g1[1]]) * geometric_product_g3.yzx().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g2.zxy().with_w(reverse_g3[0]))
                - (self.group1().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g3[2] * self[e3]),
        )
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       10        0        0
    //    simd2        0        8        0      N/A
    //    simd3       24       32        0      N/A
    //    simd4       15       11        0      N/A
    // Totals...
    // yes simd       43       61        0      N/A
    //  no simd      136      166        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(other[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]);
        let geometric_product_g1 = Simd32x4::from(other[e321]) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]);
        let geometric_product_g2 = Simd32x3::from(other[e321]) * self.group4().xyz();
        let geometric_product_g3 = Simd32x3::from(other[e321] * -1.0) * self.group1().xyz();
        let geometric_product_g4 = Simd32x4::from(other[e321]) * (self.group2() * Simd32x3::from(-1.0)).with_w(self[scalar]);
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
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * self[e1])
                        - (geometric_product_g1[1] * reverse_g4[0])
                        - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * reverse_g4.zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g1.zx() * reverse_g4.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]))
                + (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(reverse_g4[3]) * geometric_product_g1.xyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g3[2] * geometric_product_g1[2]])
                    * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], reverse_g3[1] * geometric_product_g1[1]]) * geometric_product_g3.yzx().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g2.zxy().with_w(reverse_g3[0]))
                - (self.group1().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g3[2] * self[e3]),
        )
    }
}
impl Sandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       20        0        0
    //    simd2        3        9        0      N/A
    //    simd3       31       45        0      N/A
    //    simd4       24       10        0      N/A
    // Totals...
    // yes simd       68       84        0      N/A
    //  no simd      205      213        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
            - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_product_g1 = Simd32x3::from(0.0)
            .with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]))
            + (other.group1().xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
            + (other.group1().yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[scalar]) * other.group0())
            + (Simd32x3::from(self[e1234]) * other.group1())
            + (other.group0().yzx() * self.group3().zxy())
            + (other.group1().yzx() * self.group2().zxy())
            - (other.group0().zxy() * self.group3().yzx())
            - (other.group1().zxy() * self.group2().yzx());
        let geometric_product_g3 = (Simd32x3::from(self[scalar]) * other.group1()) + (other.group1().yzx() * self.group3().zxy()) - (other.group1().zxy() * self.group3().yzx());
        let geometric_product_g4 = Simd32x3::from(0.0).with_w((other[e12] * self[e3]) * -1.0)
            + (other.group0().xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
            + (other.group0().yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
            + (other.group1().xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
            + (other.group1().yzz() * self.group4().zx().with_z(self[e4])).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx())
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
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * self[e1])
                        - (geometric_product_g1[1] * reverse_g4[0])
                        - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * reverse_g4.zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g1.zx() * reverse_g4.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]))
                + (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(reverse_g4[3]) * geometric_product_g1.xyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g3[2] * geometric_product_g1[2]])
                    * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], reverse_g3[1] * geometric_product_g1[1]]) * geometric_product_g3.yzx().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g2.zxy().with_w(reverse_g3[0]))
                - (self.group1().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g3[2] * self[e3]),
        )
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       25       40        0        0
    //    simd2       12       18        0      N/A
    //    simd3       34       50        0      N/A
    //    simd4       35       19        0      N/A
    // Totals...
    // yes simd      106      127        0      N/A
    //  no simd      291      302        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, (other[e1234] * self[scalar]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
            + (Simd32x2::from(other[scalar]) * self.group0())
            - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
            - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
            - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]));
        let geometric_product_g1 = (other.group1() * Simd32x3::from(self[e321]).with_w(self[e4]))
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]))
            + (Simd32x3::from(other[scalar]) * self.group1().xyz()).with_w(0.0)
            + (other.group1().yzx() * self.group1().zxy()).with_w(other[e1234] * self[e321])
            - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from(self[scalar]) * other.group0().xyz())
            + (Simd32x3::from(self[e1234]) * other.group1().xyz())
            + (self.group2().zxy() * other.group1().yzx())
            + (self.group3().zxy() * other.group0().yzx())
            - (self.group2().yzx() * other.group1().zxy())
            - (self.group3().yzx() * other.group0().zxy());
        let geometric_product_g3 =
            (Simd32x3::from(other[scalar]) * self.group3()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (self.group3().zxy() * other.group1().yzx())
                - (self.group3().yzx() * other.group1().zxy());
        let geometric_product_g4 = (other.group1() * Simd32x3::from(self[e4]).with_w(self[e321]))
            + Simd32x3::from(0.0).with_w((other[e12] * self[e3]) * -1.0)
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(other[scalar]) * self.group4().xyz()).with_w(0.0)
            + (other.group0().xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
            + (other.group0().yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
            + (other.group1().yzx() * self.group4().zxy()).with_w(0.0)
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
                    - (geometric_product_g4[0] * self[e1])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from(geometric_product_g4[3]) * Simd32x2::from([reverse_g4[3], self[e4]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from(geometric_product_g4[3]) * reverse_g3.with_w(self[e1234]))
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g2[2] * self[e3]]) * reverse_g3.yzx().with_w(1.0))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[0] * reverse_g4[0])
                        - (geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                + (geometric_product_g3 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g2[1] * self[e2])
                - (geometric_product_g1.yzxx() * reverse_g3.zxy().with_w(reverse_g2[0]))
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * self[e1])
                        - (geometric_product_g1[1] * reverse_g4[0])
                        - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * reverse_g4.zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g1.zx() * reverse_g4.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]))
                + (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(reverse_g4[3]) * geometric_product_g1.xyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g3[2] * geometric_product_g1[2]])
                    * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], reverse_g3[1] * geometric_product_g1[1]]) * geometric_product_g3.yzx().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g2.zxy().with_w(reverse_g3[0]))
                - (self.group1().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g3[2] * self[e3]),
        )
    }
}
impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       36       56        0        0
    //    simd2       16       28        0      N/A
    //    simd3       48       63        0      N/A
    //    simd4       46       24        0      N/A
    // Totals...
    // yes simd      146      171        0      N/A
    //  no simd      396      397        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([
            0.0,
            (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                - (other[e1] * self[e423])
                - (other[e2] * self[e431])
                - (other[e3] * self[e412])
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
            - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]));
        let geometric_product_g1 = (Simd32x4::from(other[scalar]) * self.group1())
            + (Simd32x4::from(self[scalar]) * other.group1())
            + (Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]))
            + (other.group1().yzxx() * self.group3().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w(
                -(other[e43] * self[e3])
                    - (other[e23] * self[e423])
                    - (other[e31] * self[e431])
                    - (other[e12] * self[e412])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (other[e321] * self[e1234]),
            )
            + (Simd32x3::from(other[e321]) * self.group3()).with_w(other[e3] * self[e43])
            + (other.group3().yzx() * self.group1().zxy()).with_w(other[e2] * self[e42])
            - (self.group1().yzxx() * other.group3().zxy().with_w(other[e41]))
            - (self.group3().yzx() * other.group1().zxy()).with_w(other[e42] * self[e2]);
        let geometric_product_g2 = (Simd32x3::from(other[scalar]) * self.group2())
            + (Simd32x3::from(other[e1234]) * self.group3())
            + (Simd32x3::from(other[e321]) * self.group4().xyz())
            + (Simd32x3::from(self[scalar]) * other.group2())
            + (Simd32x3::from(self[e1234]) * other.group3())
            + (Simd32x3::from(self[e4]) * other.group1().xyz())
            + (other.group2().yzx() * self.group3().zxy())
            + (other.group3().yzx() * self.group2().zxy())
            + Simd32x2::from(0.0).with_z((other[e1] * self[e431]) + (other[e431] * self[e1]) - (other[e2] * self[e423]) - (other[e423] * self[e2]))
            + (other.group1().yz() * self.group4().zx()).with_z(0.0)
            + (other.group4().zx() * self.group1().yz()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group4().xyz())
            - (other.group2().zxy() * self.group3().yzx())
            - (other.group3().zxy() * self.group2().yzx())
            - (other.group1().zx() * self.group4().yz()).with_z(0.0)
            - (other.group4().yz() * self.group1().zx()).with_z(0.0);
        let geometric_product_g3 = (Simd32x3::from(other[scalar]) * self.group3())
            + (Simd32x3::from(self[scalar]) * other.group3())
            + (other.group3().yzx() * self.group3().zxy())
            + Simd32x2::from(0.0).with_z((other[e2] * self[e1]) - (other[e1] * self[e2]))
            + (other.group1().zx() * self.group1().yz()).with_z(0.0)
            - (Simd32x3::from(other[e321]) * self.group1().xyz())
            - (Simd32x3::from(self[e321]) * other.group1().xyz())
            - (other.group3().zxy() * self.group3().yzx())
            - (other.group1().yz() * self.group1().zx()).with_z(0.0);
        let geometric_product_g4 = (Simd32x4::from(other[scalar]) * self.group4())
            + (Simd32x4::from(self[scalar]) * other.group4())
            + (Simd32x3::from(other[e1234]) * self.group1().xyz()).with_w(0.0)
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            + (other.group2().xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
            + (other.group2().yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
            + (other.group3().xyx() * Simd32x2::from(self[e4]).with_z(self[e431])).with_w(0.0)
            + (other.group3().yzz() * self.group4().zx().with_z(self[e4])).with_w(0.0)
            + (self.group2().yzx() * other.group1().zxy()).with_w(0.0)
            + (self.group3().zxy() * other.group4().yzx()).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group1().yzxx())
            - (other.group1().xyxx() * Simd32x2::from(self[e1234]).with_zw(self[e42], self[e23]))
            - (other.group1().yzzy() * self.group2().zx().with_zw(self[e1234], self[e31]))
            - (Simd32x3::from(other[e321]) * self.group2()).with_w(other[e12] * self[e3])
            - (other.group3().zxy() * self.group4().yzx()).with_w(other[e3] * self[e12])
            - (self.group3().yzx() * other.group4().zxy()).with_w(other[e31] * self[e2]);
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
                    - (geometric_product_g4[0] * self[e1])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(geometric_product_g1[0]) * Simd32x2::from([self[e1], reverse_g4[0]]))
                + (Simd32x2::from(geometric_product_g1[1]) * Simd32x2::from([self[e2], reverse_g4[1]]))
                + (Simd32x2::from(geometric_product_g1[2]) * Simd32x2::from([self[e3], reverse_g4[2]]))
                - (Simd32x2::from(reverse_g3[0]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                - (Simd32x2::from(reverse_g3[1]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                - (Simd32x2::from(reverse_g3[2]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]]))
                - (Simd32x2::from(geometric_product_g4[3]) * Simd32x2::from([reverse_g4[3], self[e4]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (Simd32x4::from(geometric_product_g4[3]) * reverse_g3.with_w(self[e1234]))
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g2[2] * self[e3]]) * reverse_g3.yzx().with_w(1.0))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    -(geometric_product_g3[0] * reverse_g4[0])
                        - (geometric_product_g3[1] * reverse_g4[1])
                        - (geometric_product_g3[2] * reverse_g4[2])
                        - (reverse_g2[1] * geometric_product_g1[1])
                        - (reverse_g2[2] * geometric_product_g1[2])
                        - (reverse_g3[0] * geometric_product_g4[0])
                        - (reverse_g3[1] * geometric_product_g4[1])
                        - (reverse_g3[2] * geometric_product_g4[2]),
                )
                + (geometric_product_g3 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g2[1] * self[e2])
                - (geometric_product_g1.yzxx() * reverse_g3.zxy().with_w(reverse_g2[0]))
                - (geometric_product_g3.yzx() * self.group1().zxy()).with_w(geometric_product_g0[1] * reverse_g4[3]),
            // e41, e42, e43
            (geometric_product_g2 * Simd32x3::from(self[scalar]))
                + (geometric_product_g3 * Simd32x3::from(self[e1234]))
                + (reverse_g2 * Simd32x3::from(geometric_product_g0[0]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[1]))
                + (Simd32x3::from(geometric_product_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z(
                    (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * self[e1])
                        - (geometric_product_g1[1] * reverse_g4[0])
                        - (geometric_product_g4[0] * self[e2]),
                )
                + (geometric_product_g1.yz() * reverse_g4.zx()).with_z(0.0)
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g1.zx() * reverse_g4.yz()).with_z(0.0)
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0[0]))
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e2]) - (geometric_product_g1[1] * self[e1]))
                + (geometric_product_g1.yz() * self.group1().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (Simd32x3::from(reverse_g4[3]) * geometric_product_g1.xyz())
                - (geometric_product_g3.yzx() * reverse_g3.zxy())
                - (geometric_product_g1.zx() * self.group1().yz()).with_z(0.0),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0[0]))
                + (Simd32x3::from(self[e1234]) * geometric_product_g1.xyz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g4[3]]) * reverse_g2.yzz()).with_w(0.0)
                + (Simd32x3::from([geometric_product_g4[2], geometric_product_g4[0], geometric_product_g1[3]]) * reverse_g3.yzz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g2.xyx() * Simd32x2::from(geometric_product_g4[3]).with_z(geometric_product_g1[1])).with_w(0.0)
                + (reverse_g3.xyx() * Simd32x2::from(geometric_product_g1[3]).with_z(geometric_product_g4[1])).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], reverse_g3[2] * geometric_product_g1[2]])
                    * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], reverse_g3[1] * geometric_product_g1[1]]) * geometric_product_g3.yzx().with_w(1.0))
                - (geometric_product_g1.yzxx() * reverse_g2.zxy().with_w(reverse_g3[0]))
                - (self.group1().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g3[2] * self[e3]),
        )
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       20        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd       14       22        0      N/A
    //  no simd       14       26        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_y = self[e321] * other[e4] * -1.0;
        let geometric_product_g1_w = self[scalar] * other[e4];
        let geometric_product_g2 = Simd32x3::from(other[e4] * -1.0) * self.group1().xyz();
        let geometric_product_g4_xyz = Simd32x3::from(other[e4]) * self.group3();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g2[0] * self[e23]) + (geometric_product_g2[1] * self[e31]) + (geometric_product_g2[2] * self[e12])
                    - (geometric_product_g1_w * self[e321])
                    - (geometric_product_g4_xyz[0] * self[e1])
                    - (geometric_product_g4_xyz[1] * self[e2])
                    - (geometric_product_g4_xyz[2] * self[e3]),
            ]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(
                (geometric_product_g0_y * self[e321])
                    + (geometric_product_g1_w * self[scalar])
                    + (geometric_product_g2[0] * self[e1])
                    + (geometric_product_g2[1] * self[e2])
                    + (geometric_product_g2[2] * self[e3])
                    + (geometric_product_g4_xyz[0] * self[e23])
                    + (geometric_product_g4_xyz[1] * self[e31])
                    + (geometric_product_g4_xyz[2] * self[e12]),
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
    //           add/sub      mul      div      pow
    //      f32        7       20        0        0
    //    simd2        0        3        0      N/A
    //    simd3       30       43        0      N/A
    //    simd4       15        7        0      N/A
    // Totals...
    // yes simd       52       73        0      N/A
    //  no simd      157      183        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = self[e321] * other[e321] * -1.0;
        let geometric_product_g0_y = (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]);
        let geometric_product_g1_xyz = Simd32x3::from(other[e321]) * self.group3();
        let geometric_product_g1_w = -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]);
        let geometric_product_g2 =
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().yzx() * other.group0().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e423]) * -1.0)
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().zx() * other.group0().yz()).with_z(0.0);
        let geometric_product_g3 = Simd32x3::from(other[e321] * -1.0) * self.group1().xyz();
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
                + (reverse_g2 * Simd32x3::from(geometric_product_g0_x))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0_y))
                + (Simd32x3::from(geometric_product_g1_w) * self.group1().xyz())
                + (Simd32x3::from(reverse_g4[3]) * geometric_product_g4.xyz())
                + (geometric_product_g1_xyz.yzx() * reverse_g4.zxy())
                + (geometric_product_g2.zxy() * reverse_g3.yzx())
                + (geometric_product_g3.zxy() * reverse_g2.yzx())
                + Simd32x2::from(0.0).with_z((geometric_product_g4[1] * self[e1]) - (geometric_product_g4[0] * self[e2]))
                + (geometric_product_g4.zx() * self.group1().yz()).with_z(0.0)
                - (geometric_product_g1_xyz * Simd32x3::from(self[e4]))
                - (Simd32x3::from(geometric_product_g4[3]) * reverse_g4.xyz())
                - (geometric_product_g1_xyz.zxy() * reverse_g4.yzx())
                - (geometric_product_g2.yzx() * reverse_g3.zxy())
                - (geometric_product_g3.yzx() * reverse_g2.zxy())
                - (geometric_product_g4.yz() * self.group1().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g3 * Simd32x3::from(self[scalar]))
                + (reverse_g3 * Simd32x3::from(geometric_product_g0_x))
                + (geometric_product_g1_xyz.yzx() * self.group1().zxy())
                + (geometric_product_g3.zxy() * reverse_g3.yzx())
                - (geometric_product_g1_xyz * Simd32x3::from(reverse_g4[3]))
                - (Simd32x3::from(geometric_product_g4[3]) * self.group1().xyz())
                - (geometric_product_g1_xyz.zxy() * self.group1().yzx())
                - (geometric_product_g3.yzx() * reverse_g3.zxy()),
            // e423, e431, e412, e321
            (geometric_product_g4 * Simd32x4::from(self[scalar]))
                + (reverse_g4 * Simd32x4::from(geometric_product_g0_x))
                + (reverse_g2 * Simd32x3::from(geometric_product_g4[3])).with_w(0.0)
                + (reverse_g3 * Simd32x3::from(geometric_product_g1_w)).with_w(0.0)
                + (Simd32x3::from([reverse_g2[1], self[e1234], self[e1234]]) * geometric_product_g1_xyz.zyz()).with_w(0.0)
                + (Simd32x3::from([reverse_g4[1], self[e4], self[e4]]) * geometric_product_g3.zyz()).with_w(0.0)
                + (Simd32x3::from([self[e1234], reverse_g2[2], reverse_g2[0]]) * geometric_product_g1_xyz.xxy()).with_w(0.0)
                + (Simd32x3::from([self[e4], reverse_g4[2], reverse_g4[0]]) * geometric_product_g3.xxy()).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group1().zxy()).with_w(0.0)
                + (reverse_g3.yzx() * geometric_product_g4.zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g4[1], geometric_product_g4[2], geometric_product_g4[0], geometric_product_g3[2] * self[e3]]) * reverse_g3.zxy().with_w(1.0))
                - (Simd32x4::from([reverse_g4[2], reverse_g4[0], reverse_g4[1], geometric_product_g1_xyz[2] * reverse_g3[2]]) * geometric_product_g3.yzx().with_w(1.0))
                - (self.group1().xyzx() * Simd32x3::from(geometric_product_g0_y).with_w(geometric_product_g3[0]))
                - (self.group1().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1]))
                - (geometric_product_g2 * Simd32x3::from(reverse_g4[3])).with_w(geometric_product_g1_xyz[1] * reverse_g3[1])
                - (geometric_product_g1_xyz.yzx() * reverse_g2.zxy()).with_w(geometric_product_g1_xyz[0] * reverse_g3[0]),
        )
    }
}
impl Sandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       32       43        0        0
    //    simd2        4        7        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4       15        8        0      N/A
    // Totals...
    // yes simd       58       69        0      N/A
    //  no simd      121      122        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]);
        let geometric_product_g0_y = -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]);
        let geometric_product_g1 = (Simd32x4::from(self[scalar]) * other.group0())
            + (other.group0().yzxx() * self.group3().zxy().with_w(self[e41]))
            + Simd32x3::from(0.0).with_w((self[e42] * other[e2]) + (self[e43] * other[e3]))
            - (self.group3().yzx() * other.group0().zxy()).with_w(0.0);
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group0().xyz())
            + Simd32x2::from(0.0).with_z((self[e431] * other[e1]) - (self[e423] * other[e2]))
            + (self.group4().zx() * other.group0().yz()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group1().xyz())
            - (self.group4().yz() * other.group0().zx()).with_z(0.0);
        let geometric_product_g3 = (self.group1().yzx() * other.group0().zxy()) + Simd32x2::from(0.0).with_z((self[e2] * other[e1]) * -1.0)
            - (Simd32x3::from(self[e321]) * other.group0().xyz())
            - (self.group1().zx() * other.group0().yz()).with_z(0.0);
        let geometric_product_g4 = Simd32x3::from(0.0).with_w((self[e12] * other[e3]) * -1.0)
            + (Simd32x3::from(other[e4]) * self.group3()).with_w(0.0)
            + (self.group2().yzx() * other.group0().zxy()).with_w(0.0)
            - (other.group0().xyzx() * Simd32x3::from(self[e1234]).with_w(self[e23]))
            - (other.group0().yzxy() * self.group2().zxy().with_w(self[e31]));
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]) + (geometric_product_g4[3] * self[e321]),
                (geometric_product_g0_y * self[scalar]) + (geometric_product_g3[0] * self[e41]) + (geometric_product_g3[1] * self[e42]) + (geometric_product_g3[2] * self[e43])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321])
                    - (geometric_product_g4[0] * self[e1])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0_x) * self.group0())
                + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0_x) * self.group1())
                + (geometric_product_g1.yzxx() * self.group3().zxy().with_w(self[e41]))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0_y * self[e321])
                        + (geometric_product_g2[1] * self[e2])
                        + (geometric_product_g2[2] * self[e3])
                        + (geometric_product_g3[0] * self[e423])
                        + (geometric_product_g3[1] * self[e431])
                        + (geometric_product_g3[2] * self[e412])
                        + (geometric_product_g1[1] * self[e42])
                        + (geometric_product_g1[2] * self[e43])
                        + (geometric_product_g4[0] * self[e23])
                        + (geometric_product_g4[1] * self[e31])
                        + (geometric_product_g4[2] * self[e12])
                        + (geometric_product_g4[3] * self[e1234]),
                )
                - (Simd32x3::from(geometric_product_g4[3]) * self.group3()).with_w(0.0)
                - (geometric_product_g3.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(0.0),
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
    //           add/sub      mul      div      pow
    //      f32       25       28        0        0
    //    simd2        4        5        0      N/A
    //    simd3        0        6        0      N/A
    //    simd4        8        6        0      N/A
    // Totals...
    // yes simd       37       45        0      N/A
    //  no simd       65       80        0        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from(other[scalar]) * self.group0();
        let geometric_product_g1 = Simd32x4::from(other[scalar]) * self.group1();
        let geometric_product_g2 = Simd32x3::from(other[scalar]) * self.group2();
        let geometric_product_g3 = Simd32x3::from(other[scalar]) * self.group3();
        let geometric_product_g4 = Simd32x4::from(other[scalar]) * self.group4();
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]) + (geometric_product_g4[3] * self[e321]),
                (geometric_product_g0[1] * self[scalar]) + (geometric_product_g3[0] * self[e41]) + (geometric_product_g3[1] * self[e42]) + (geometric_product_g3[2] * self[e43])
                    - (geometric_product_g1[0] * self[e423])
                    - (geometric_product_g1[1] * self[e431])
                    - (geometric_product_g1[2] * self[e412])
                    - (geometric_product_g1[3] * self[e321])
                    - (geometric_product_g4[0] * self[e1])
                    - (geometric_product_g4[1] * self[e2])
                    - (geometric_product_g4[2] * self[e3])
                    - (geometric_product_g4[3] * self[e4]),
            ]) + (Simd32x2::from(geometric_product_g0[0]) * self.group0())
                + (Simd32x2::from(self[e23]) * Simd32x2::from([geometric_product_g3[0], geometric_product_g2[0]]))
                + (Simd32x2::from(self[e31]) * Simd32x2::from([geometric_product_g3[1], geometric_product_g2[1]]))
                + (Simd32x2::from(self[e12]) * Simd32x2::from([geometric_product_g3[2], geometric_product_g2[2]])),
            // e1, e2, e3, e4
            (geometric_product_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(geometric_product_g0[0]) * self.group1())
                + (geometric_product_g1.yzxx() * self.group3().zxy().with_w(self[e41]))
                + (self.group1().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w(
                    (geometric_product_g0[1] * self[e321])
                        + (geometric_product_g2[1] * self[e2])
                        + (geometric_product_g2[2] * self[e3])
                        + (geometric_product_g3[0] * self[e423])
                        + (geometric_product_g3[1] * self[e431])
                        + (geometric_product_g3[2] * self[e412])
                        + (geometric_product_g1[1] * self[e42])
                        + (geometric_product_g1[2] * self[e43])
                        + (geometric_product_g4[0] * self[e23])
                        + (geometric_product_g4[1] * self[e31])
                        + (geometric_product_g4[2] * self[e12])
                        + (geometric_product_g4[3] * self[e1234]),
                )
                - (Simd32x3::from(geometric_product_g4[3]) * self.group3()).with_w(0.0)
                - (geometric_product_g3.xyx() * Simd32x2::from(self[e321]).with_z(self[e2])).with_w(0.0)
                - (geometric_product_g3.yzz() * self.group1().zx().with_z(self[e321])).with_w(0.0)
                - (self.group3().yzx() * geometric_product_g1.zxy()).with_w(0.0),
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
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e321] * self[e321] * -1.0)
    }
}
impl Sandwich<DualNum> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        5        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * self[e321] * other[scalar], self[e321] * self[e321] * other[e1234] * -1.0]),
        )
    }
}
impl Sandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd2        0        2        0      N/A
    //    simd3        4        5        0      N/A
    //    simd4        4        3        0      N/A
    // Totals...
    // yes simd       13       22        0      N/A
    //  no simd       33       43        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(other[e2] * self[e431]) - (other[e3] * self[e412]))
            + (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
            - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]))
            - (other.group0().zxyx() * self.group0().yzxx());
        let geometric_product_g1 = Simd32x4::from(self[e321] * -1.0) * other.group0().xyz().with_w(other[e321]);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(self[e321] * -1.0) * geometric_product_g1.xyz()).with_w(
                (geometric_product_g0[3] * self[e321]) + (geometric_product_g1[0] * self[e423]) + (geometric_product_g1[1] * self[e431]) + (geometric_product_g1[2] * self[e412]),
            ),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e321]) * geometric_product_g0.xyz())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[0] * self[e431]) - (geometric_product_g1[1] * self[e423]))
                + (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0))
            .with_w(geometric_product_g1[3] * self[e321] * -1.0),
        )
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        3       11        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_w = other[e321] * self[e321] * -1.0;
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(other[e321] * self[e321]) * self.group0().xyz()) - (Simd32x3::from(geometric_product_g1_w) * self.group0().xyz()))
                .with_w(geometric_product_g1_w * self[e321] * -1.0),
        )
    }
}
impl Sandwich<Line> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        3        9        0      N/A
    //    simd4        2        0        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd       17       27        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(self[e321]) * other.group1();
        let geometric_product_g1 = (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0) + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (other.group1().zxy() * self.group0().yzx()).with_w(0.0);
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()) + (geometric_product_g0_xyz.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * geometric_product_g1.xyz())
                - (geometric_product_g0_xyz.yzx() * self.group0().zxy()),
            // e23, e31, e12
            geometric_product_g0_xyz * Simd32x3::from(self[e321]),
        )
    }
}
impl Sandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       12        0        0
    //    simd3        0        1        0      N/A
    //    simd4        6        8        0      N/A
    // Totals...
    // yes simd        6       21        0      N/A
    //  no simd       24       47        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = Simd32x3::from(self[e321]) * other.group1().xyz();
        let geometric_product_g1_w = other[scalar] * self[e321];
        Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e321]) * Simd32x4::from([other[e12] * self[e431], other[e23] * self[e412], other[e31] * self[e423], other[e31] * self[e431]]))
                + (self.group0() * Simd32x3::from(geometric_product_g1_w).with_w(other[e23] * self[e423]))
                + (self.group0().yzxz() * geometric_product_g0_xyz.zxy().with_w(other[e12] * self[e321]))
                - (Simd32x4::from(self[e321] * self[e321]) * other.group0())
                - (Simd32x4::from([other[e31] * self[e321], other[e12] * self[e321], other[e23] * self[e321], geometric_product_g0_xyz[1]]) * self.group0().zxyy())
                - (self.group0().xyzz() * Simd32x3::from(other[scalar] * self[e321]).with_w(geometric_product_g0_xyz[2]))
                - (self.group0().zxyx() * geometric_product_g0_xyz.yzx().with_w(geometric_product_g0_xyz[0])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * geometric_product_g0_xyz.with_w(geometric_product_g1_w),
        )
    }
}
impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       46        0        0
    //    simd3       12       18        0      N/A
    // Totals...
    // yes simd       24       64        0      N/A
    //  no simd       48      100        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_x = other[e321] * self[e321] * -1.0;
        let geometric_product_g1_xyz = Simd32x3::from(self[e321]) * other.group3();
        let geometric_product_g3 = Simd32x3::from(self[e321] * -1.0) * other.group1().xyz();
        let geometric_product_g4_w = other[scalar] * self[e321];
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                geometric_product_g4_w * self[e321],
                (other[e23] * self[e423] * self[e321]) + (other[e31] * self[e431] * self[e321]) + (other[e12] * self[e412] * self[e321])
                    - (geometric_product_g1_xyz[0] * self[e423])
                    - (geometric_product_g1_xyz[1] * self[e431])
                    - (geometric_product_g1_xyz[2] * self[e412])
                    - (self[e321] * self[e321] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            (geometric_product_g3 * Simd32x3::from(self[e321] * -1.0)).with_w(
                (geometric_product_g3[0] * self[e423]) + (geometric_product_g3[1] * self[e431]) + (geometric_product_g3[2] * self[e412])
                    - (self[e321] * self[e321] * other[e4])
                    - (other[e1] * self[e423] * self[e321])
                    - (other[e2] * self[e431] * self[e321])
                    - (other[e3] * self[e412] * self[e321]),
            ),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g4_w) * self.group0().xyz())
                + (Simd32x3::from(self[e321]) * Simd32x3::from([other[e12] * self[e431], other[e23] * self[e412], other[e31] * self[e423]]))
                + (geometric_product_g1_xyz.zxy() * self.group0().yzx())
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e31] * self[e412], other[e12] * self[e423], other[e23] * self[e431]]))
                - (Simd32x3::from(self[e321] * self[e321]) * other.group2())
                - (Simd32x3::from(other[scalar] * self[e321]) * self.group0().xyz())
                - (geometric_product_g1_xyz.yzx() * self.group0().zxy()),
            // e23, e31, e12
            geometric_product_g1_xyz * Simd32x3::from(self[e321]),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[e321]) * Simd32x3::from([other[e2] * self[e412], other[e3] * self[e423], other[e1] * self[e431]]))
                + (Simd32x3::from(other[e321] * self[e321]) * self.group0().xyz())
                + (geometric_product_g3.yzx() * self.group0().zxy())
                - (Simd32x3::from(geometric_product_g0_x) * self.group0().xyz())
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e3] * self[e431], other[e1] * self[e412], other[e2] * self[e423]]))
                - (Simd32x3::from(self[e321] * self[e321]) * other.group4().xyz())
                - (geometric_product_g3.zxy() * self.group0().yzx()))
            .with_w(geometric_product_g0_x * self[e321] * -1.0),
        )
    }
}
impl Sandwich<Origin> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        3        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * self[e321] * self[e321] * -1.0)
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        6       15        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_w = other[e321] * self[e321] * -1.0;
        Plane::from_groups(
            // e423, e431, e412, e321
            ((Simd32x3::from(other[e321] * self[e321]) * self.group0().xyz())
                - (Simd32x3::from(geometric_product_g1_w) * self.group0().xyz())
                - (Simd32x3::from(self[e321] * self[e321]) * other.group0().xyz()))
            .with_w(geometric_product_g1_w * self[e321] * -1.0),
        )
    }
}
impl Sandwich<Point> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd        6       19        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(self[e321] * -1.0) * other.group0().xyz();
        Point::from_groups(/* e1, e2, e3, e4 */ (geometric_product_g1_xyz * Simd32x3::from(self[e321] * -1.0)).with_w(
            (geometric_product_g1_xyz[0] * self[e423]) + (geometric_product_g1_xyz[1] * self[e431]) + (geometric_product_g1_xyz[2] * self[e412])
                - (self[e321] * self[e321] * other[e4])
                - (self[e423] * self[e321] * other[e1])
                - (self[e431] * self[e321] * other[e2])
                - (self[e412] * self[e321] * other[e3]),
        ))
    }
}
impl Sandwich<Scalar> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       13        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x4::from(other[scalar]) * self.group0();
        let geometric_product_g1_xyz = Simd32x3::from(other[e1234]) * self.group0().xyz();
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
            -(geometric_product_g1_xyz[0] * self[e1]) - (geometric_product_g1_xyz[1] * self[e2]) - (geometric_product_g1_xyz[2] * self[e3]),
        ]))
    }
}
impl Sandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        0        7        0      N/A
    //    simd4       14        7        0      N/A
    // Totals...
    // yes simd       17       22        0      N/A
    //  no simd       59       57        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]))
            + (other.group1().zxyx() * self.group0().yzxx())
            + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]))
            - (Simd32x3::from(other[e4]) * self.group0().xyz()).with_w(0.0)
            - (other.group1().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_product_g1 = (other.group0().zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((other[e2] * self[e2]) + (other[e3] * self[e3]))
            - (Simd32x3::from(other[e321]) * self.group0().xyz()).with_w(0.0)
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g1[3]) * self.group0())
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g0[0]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g1[2] * self[e3]) * -1.0)
                + (Simd32x3::from(self[e4]) * geometric_product_g1.xyz()).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1], geometric_product_g1[0]]) * self.group0().yzxx())
                - (self.group0().xyzy() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1[1])),
        )
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       14        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g1_xyz = Simd32x3::from(other[e321] * -1.0) * self.group0().xyz();
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(-(geometric_product_g1_xyz[1] * self[e2]) - (geometric_product_g1_xyz[2] * self[e3]))
                + (geometric_product_g1_xyz * Simd32x3::from(self[e4])).with_w(0.0)
                - (self.group0().xyzx() * Simd32x3::from(other[e321] * self[e4]).with_w(geometric_product_g1_xyz[0])),
        )
    }
}
impl Sandwich<Line> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd2        0        3        0      N/A
    //    simd3        7        7        0      N/A
    //    simd4        5        2        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       44       43        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(0.0).with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3])) + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
            + (Simd32x3::from(self[e4]) * other.group1()).with_w(0.0)
            + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx());
        Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz())
                + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) - (geometric_product_g1[0] * self[e2]))
                + (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(self[e4]) * geometric_product_g0.xyz())
                - (geometric_product_g1.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g0.yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((geometric_product_g0[1] * self[e1]) * -1.0)
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                - (geometric_product_g0.zx() * self.group0().yz()).with_z(0.0),
        )
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd3        0        8        0      N/A
    //    simd4       14        6        0      N/A
    // Totals...
    // yes simd       18       22        0      N/A
    //  no simd       60       56        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (Simd32x4::from(other[scalar]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]))
            + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41]));
        let geometric_product_g1 = Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
            + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
            + (Simd32x3::from(self[e4]) * other.group1().xyz()).with_w(0.0)
            + (other.group0().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]));
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(-(geometric_product_g1[1] * self[e2]) - (geometric_product_g1[2] * self[e3]))
                + (Simd32x3::from(geometric_product_g0[3]) * self.group0().xyz()).with_w(0.0)
                + (geometric_product_g1.zxy() * self.group0().yzx()).with_w(0.0)
                - (Simd32x4::from(self[e4]) * Simd32x4::from([geometric_product_g0[0], geometric_product_g0[1], geometric_product_g0[2], geometric_product_g1[3]]))
                - (geometric_product_g1.yzxx() * self.group0().zxyx()),
            // e23, e31, e12, scalar
            (geometric_product_g0.yzxx() * self.group0().zxyx()) + Simd32x3::from(0.0).with_w((geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]))
                - (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz()).with_w(0.0)
                - (geometric_product_g0.zxy() * self.group0().yzx()).with_w(0.0),
        )
    }
}
impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       24        0        0
    //    simd2        3        9        0      N/A
    //    simd3       14       15        0      N/A
    //    simd4       14        7        0      N/A
    // Totals...
    // yes simd       41       55        0      N/A
    //  no simd      114      115        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x2::from([0.0, other[e321] * self[e4]])
            + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
            + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
            + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]));
        let geometric_product_g1 = (Simd32x4::from(other[scalar]) * self.group0())
            + Simd32x3::from(0.0).with_w(-(other[e42] * self[e2]) - (other[e43] * self[e3]))
            + (other.group3().yzx() * self.group0().zxy()).with_w(0.0)
            - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41]));
        let geometric_product_g2 = (Simd32x3::from(self[e4]) * other.group1().xyz())
            + Simd32x2::from(0.0).with_z((other[e431] * self[e1]) - (other[e423] * self[e2]))
            + (other.group4().zx() * self.group0().yz()).with_z(0.0)
            - (Simd32x3::from(other[e4]) * self.group0().xyz())
            - (other.group4().yz() * self.group0().zx()).with_z(0.0);
        let geometric_product_g3 = (other.group1().zxy() * self.group0().yzx()) + Simd32x2::from(0.0).with_z((other[e1] * self[e2]) * -1.0)
            - (Simd32x3::from(other[e321]) * self.group0().xyz())
            - (other.group1().yz() * self.group0().zx()).with_z(0.0);
        let geometric_product_g4 = Simd32x3::from(0.0).with_w(-(other[e31] * self[e2]) - (other[e12] * self[e3]))
            + (Simd32x3::from(other[e1234]) * self.group0().xyz()).with_w(0.0)
            + (Simd32x3::from(self[e4]) * other.group3()).with_w(0.0)
            + (other.group2().yzx() * self.group0().zxy()).with_w(0.0)
            - (Simd32x4::from([other[e43], other[e41], other[e42], other[e23]]) * self.group0().yzxx());
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (geometric_product_g1[0] * self[e1]) + (geometric_product_g1[1] * self[e2]) + (geometric_product_g1[2] * self[e3]),
                -(geometric_product_g4[0] * self[e1]) - (geometric_product_g4[1] * self[e2]) - (geometric_product_g4[2] * self[e3]) - (geometric_product_g4[3] * self[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g0[0]) * self.group0())
                + (self.group0().yzxx() * geometric_product_g3.zxy().with_w(geometric_product_g2[0]))
                + Simd32x3::from(0.0).with_w((geometric_product_g2[1] * self[e2]) + (geometric_product_g2[2] * self[e3]))
                - (geometric_product_g3.yzx() * self.group0().zxy()).with_w(0.0),
            // e41, e42, e43
            (Simd32x3::from(geometric_product_g1[3]) * self.group0().xyz())
                + Simd32x2::from(0.0).with_z((geometric_product_g4[1] * self[e1]) - (geometric_product_g4[0] * self[e2]))
                + (geometric_product_g4.zx() * self.group0().yz()).with_z(0.0)
                - (Simd32x3::from(self[e4]) * geometric_product_g1.xyz())
                - (geometric_product_g4.yz() * self.group0().zx()).with_z(0.0),
            // e23, e31, e12
            (geometric_product_g1.yzx() * self.group0().zxy()) + Simd32x2::from(0.0).with_z((geometric_product_g1[1] * self[e1]) * -1.0)
                - (Simd32x3::from(geometric_product_g4[3]) * self.group0().xyz())
                - (geometric_product_g1.zx() * self.group0().yz()).with_z(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g3[2] * self[e3]) * -1.0)
                + (geometric_product_g3 * Simd32x3::from(self[e4])).with_w(0.0)
                + (geometric_product_g2.yzx() * self.group0().zxy()).with_w(0.0)
                - (self.group0().xyzx() * Simd32x2::from(geometric_product_g0[1]).with_zw(geometric_product_g0[1], geometric_product_g3[0]))
                - (self.group0().yzxy() * geometric_product_g2.zxy().with_w(geometric_product_g3[1])),
        )
    }
}
impl Sandwich<Origin> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        7        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = Simd32x3::from(other[e4] * -1.0) * self.group0().xyz();
        Origin::from_groups(
            // e4
            (geometric_product_g0[0] * self[e1]) + (geometric_product_g0[1] * self[e2]) + (geometric_product_g0[2] * self[e3]),
        )
    }
}
impl Sandwich<Plane> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        6        3        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       26       30        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0 = (other.group0().zxyx() * self.group0().yzxx())
            + Simd32x3::from(0.0).with_w((other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]))
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        let geometric_product_g1_xyz = Simd32x3::from(other[e321] * -1.0) * self.group0().xyz();
        Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w((geometric_product_g1_xyz[2] * self[e3]) * -1.0)
                + (geometric_product_g1_xyz * Simd32x3::from(self[e4])).with_w(0.0)
                + (geometric_product_g0.yzx() * self.group0().zxy()).with_w(0.0)
                - (Simd32x4::from([geometric_product_g0[2], geometric_product_g0[0], geometric_product_g0[1], geometric_product_g1_xyz[0]]) * self.group0().yzxx())
                - (self.group0().xyzy() * Simd32x3::from(geometric_product_g0[3]).with_w(geometric_product_g1_xyz[1])),
        )
    }
}
impl Sandwich<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        1        4        0      N/A
    //    simd4        5        3        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       25       28        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product_g0_xyz = (Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz());
        let geometric_product_g1 = (other.group0().zxyx() * self.group0().yzxx()) + Simd32x3::from(0.0).with_w((other[e2] * self[e2]) + (other[e3] * self[e3]))
            - (other.group0().yzx() * self.group0().zxy()).with_w(0.0);
        Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(geometric_product_g1[3]) * self.group0())
                + (Simd32x4::from([geometric_product_g1[2], geometric_product_g1[0], geometric_product_g1[1], geometric_product_g0_xyz[0]]) * self.group0().yzxx())
                + Simd32x3::from(0.0).with_w((geometric_product_g0_xyz[1] * self[e2]) + (geometric_product_g0_xyz[2] * self[e3]))
                - (geometric_product_g1.yzx() * self.group0().zxy()).with_w(0.0),
        )
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        7        0        0
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
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar] * self[scalar])
    }
}
impl Sandwich<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar] * self[scalar]) * other.group0())
    }
}
impl Sandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar] * self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar] * self[scalar]) * other.group1(),
        )
    }
}
impl Sandwich<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e321 */ other[e321] * self[scalar] * self[scalar])
    }
}
impl Sandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar] * self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar] * self[scalar]) * other.group1(),
        )
    }
}
impl Sandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar] * self[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar] * self[scalar]) * other.group1(),
        )
    }
}
impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       21        0        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar] * self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar] * self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar] * self[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[scalar] * self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar] * self[scalar]) * other.group4(),
        )
    }
}
impl Sandwich<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        Origin::from_groups(/* e4 */ other[e4] * self[scalar] * self[scalar])
    }
}
impl Sandwich<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar] * self[scalar]) * other.group0())
    }
}
impl Sandwich<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar] * self[scalar]) * other.group0())
    }
}
impl Sandwich<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[scalar] * self[scalar] * self[scalar])
    }
}
