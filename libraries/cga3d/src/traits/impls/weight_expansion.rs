// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 502
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         3       7       0     N/A
//  Average:         5      10       0     N/A
//  Maximum:       107     133       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         5      14       0       0
//  Average:         9      20       0       0
//  Maximum:       196     244       0       0
impl std::ops::Div<WeightExpansionInfix> for AntiCircleRotor {
    type Output = WeightExpansionInfixPartial<AntiCircleRotor>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       10       31        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group2().xyz() * Simd32x3::from(-1.0)).with_w(
                (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[scalar] * self[scalar])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
            ),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       24       41        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (right_anti_dual_g2_xyz * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(self[e41] * right_anti_dual_g1[0])
                    - (self[e42] * right_anti_dual_g1[1])
                    - (self[e43] * right_anti_dual_g1[2])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (self[scalar] * other[e4]),
            ),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (Simd32x3::from(self[scalar]) * other.group3().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx())
                + (other.group0().yzx() * self.group2().zxy())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(self[scalar] * other[e5] * -1.0),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x2::from(other[e3215]).with_z(other[e3215])).with_w(other[scalar] * self[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       14        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()) + (self.group0().yzx() * right_anti_dual_g0.zxy()) - (self.group0().zxy() * right_anti_dual_g0.yzx()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiCircleRotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        9       19        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group1().xyz())
                + (self.group0().yzx() * right_anti_dual_g0.zxy())
                - (self.group0().zxy() * right_anti_dual_g0.yzx()))
            .with_w(self[scalar] * other[e5] * -1.0),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       18        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(right_anti_dual_g0[0] * self[e23])
                    - (right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ),
            // e235, e315, e125, e5
            (right_anti_dual_g1 * Simd32x4::from(self[scalar]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       14        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e3215]) * self.group0()) - (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(self[scalar] * other[scalar]),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(self[scalar] * other[e3215]),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       12        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
        )
    }
}
impl WeightExpansion<Circle> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       20       35        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(self[e41] * right_anti_dual_g1[0])
                    - (self[e42] * right_anti_dual_g1[1])
                    - (self[e43] * right_anti_dual_g1[2])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            ),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (self.group0().yzx() * other.group2().zxy())
                + (other.group0().yzx() * self.group2().zxy())
                - (self.group0().zxy() * other.group2().yzx())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        7       10        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        8       15        0      N/A
    //  no simd       25       41        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group0()) + (Simd32x3::from(self[scalar]) * other.group0())).with_w(right_anti_dual_g2_w * self[scalar]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g2_w) * self.group1()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx())
                + (other.group0().yzx() * self.group2().zxy())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(0.0),
        )
    }
}
impl WeightExpansion<Dipole> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd        9       30        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (right_anti_dual_g2 * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g2[0] * self[e41])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
            ),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        8       11        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       11       22        0      N/A
    //  no simd       27       47        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx())
                - (Simd32x3::from(self[scalar]) * other.group0())
                - (self.group0().yzx() * other.group3().zxy()))
            .with_w(self[e45] * other[e45] * -1.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group3().xyz()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(self[scalar] * other[e45]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
                (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
                (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       23        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e12345] * -1.0) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, other[e12345], 0.0])
                * (self.group2().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e5] * self[scalar] * -1.0),
        )
    }
}
impl WeightExpansion<FlatPoint> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e45]),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45])),
        )
    }
}
impl WeightExpansion<Flector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        3        7        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       18       37        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy())).with_w(self[e45] * other[e45] * -1.0),
            // e415, e425, e435, e321
            (self.group1().wwwy() * other.group1().xyzy())
                + (Simd32x3::from(other[e3215]) * self.group0()).with_w((self[e23] * other[e4235]) + (self[e12] * other[e4125]) + (self[scalar] * other[e45])),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
                (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
                (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<Line> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (other.group0() * Simd32x4::from(self[scalar]).xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(self[scalar]).xyz()).with_w(-(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        4        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd       12       29        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_anti_dual_g0_w) * self.group0().with_w(self[scalar]),
            // e23, e31, e12, e45
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())).with_w(right_anti_dual_g0_w * self[e45]),
            // e15, e25, e35, e1234
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e45])) + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()))
                .with_w(self[scalar] * other[e5] * -1.0),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       19       30        0        0
    //    simd2        0        1        0      N/A
    //    simd3       16       25        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       36       59        0      N/A
    //  no simd       71      119        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g0[0] * self[scalar],
                (right_anti_dual_g0[1] * self[scalar])
                    - (right_anti_dual_g6_xyz[0] * self[e23])
                    - (right_anti_dual_g6_xyz[1] * self[e31])
                    - (right_anti_dual_g6_xyz[2] * self[e12])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (self[e45] * other[e45]),
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0[0]) * self.group2().xyz().with_w(self[e45])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g0[0]) * self.group0()) + (Simd32x3::from(self[scalar]) * other.group7()),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g6_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz()))
            .with_w(self[scalar] * other[e45]),
            // e423, e431, e412
            (right_anti_dual_g7 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()) + (self.group0().yzx() * right_anti_dual_g1.zxy())
                - (self.group0().zxy() * right_anti_dual_g1.yzx()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g1[1] * self[e35]) - (right_anti_dual_g1[2] * self[e25]),
                (right_anti_dual_g1[2] * self[e15]) - (right_anti_dual_g1[0] * self[e35]),
                (right_anti_dual_g1[0] * self[e25]) - (right_anti_dual_g1[1] * self[e15]),
            ]) + (right_anti_dual_g8 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g5 * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group1().xyz())
                + (self.group0().yzx() * right_anti_dual_g3.zxy())
                + (other.group7().yzx() * self.group2().zxy())
                - (self.group0().zxy() * right_anti_dual_g3.yzx())
                - (other.group7().zxy() * self.group2().yzx()))
            .with_w(self[scalar] * other[e5] * -1.0),
            // e1234
            -(right_anti_dual_g5[0] * self[e41])
                - (right_anti_dual_g5[1] * self[e42])
                - (right_anti_dual_g5[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12])
                - (self[scalar] * other[e4]),
        )
    }
}
impl WeightExpansion<Plane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       16       29        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0().yzx() * right_anti_dual_g0.zxy()) - (self.group0().zxy() * right_anti_dual_g0.yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0()) - (Simd32x3::from(self[e45]) * right_anti_dual_g0.xyz())).with_w(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]), 0.0])
                + ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                    + ((right_anti_dual_g0.yz() * self.group2().zx()) - (right_anti_dual_g0.zx() * self.group2().yz())).with_z(0.0))
                .with_w(0.0),
            // e1, e2, e3, e5
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<RoundPoint> for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[scalar] * other[e4] * -1.0,
        )
    }
}
impl WeightExpansion<Scalar> for AntiCircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar])
    }
}
impl WeightExpansion<Sphere> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        6       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       18       35        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e45])))
                .with_w(0.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx()))
                .with_w(self[scalar] * other[e1234]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * right_anti_dual_g0_xyz.with_w(other[e3215]),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        7        0        0
    //    simd3        8       11        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        9       20        0      N/A
    //  no simd       28       48        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group0())).with_w(right_anti_dual_g0_w * self[scalar]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0_w) * self.group1()),
            // e15, e25, e35, e1234
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())).with_w(self[scalar] * other[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (Simd32x3::from(self[scalar]) * other.group3().xyz())
                + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy())
                + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx())
                - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
            .with_w(self[scalar] * other[e5] * -1.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        8       11        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       27       46        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz()) + (self.group0().yzx() * right_anti_dual_g3.zxy())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz())
                - (self.group0().zxy() * right_anti_dual_g3.yzx()))
            .with_w(self[scalar] * other[scalar]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g3.xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(self[scalar] * other[e45]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (right_anti_dual_g3[1] * self[e35]) - (right_anti_dual_g3[2] * self[e25]),
                (right_anti_dual_g3[2] * self[e15]) - (right_anti_dual_g3[0] * self[e35]),
                (right_anti_dual_g3[0] * self[e25]) - (right_anti_dual_g3[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            right_anti_dual_g3 * Simd32x4::from(self[scalar]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiDipoleInversion {
    type Output = WeightExpansionInfixPartial<AntiDipoleInversion>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        6        9        0      N/A
    // Totals...
    // yes simd       11       15        0      N/A
    //  no simd       23       33        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy()))
                .with_w(
                    (other[e1] * self[e1])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125])
                        - (other[e5] * self[e4]),
                ),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       15        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())).with_w(right_anti_dual_g0_w * self[e321] * -1.0),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiDipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group3().xyz())).with_w(0.0),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiDipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0.yzx() * self.group3().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_anti_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiDipoleInversion {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        9       17        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e4]),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (self[e2] * other[e12]) - (self[e3] * other[e31]),
                (self[e3] * other[e23]) - (self[e1] * other[e12]),
                (self[e1] * other[e31]) - (self[e2] * other[e23]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0())
                - (Simd32x3::from(self[e4]) * other.group1().xyz()))
            .with_w(self[e321] * other[e3215]),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (self[e4] * other[e5]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(right_anti_dual_g0) * self.group3(),
        )
    }
}
impl WeightExpansion<Circle> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6        9        0      N/A
    // no simd       18       27        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group3().xyz())).with_w(0.0),
            // e235, e315, e125, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group3().yzx()) - (other.group2().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        9       12        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       19        0      N/A
    //  no simd       27       46        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group0()) + (other.group0().yzx() * self.group3().zxy())
                - (other.group0().zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g1_w * self[e321] * -1.0),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group0())
                - (Simd32x3::from(right_anti_dual_g1_w) * self.group3().xyz()))
            .with_w(right_anti_dual_g2_w * self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g2_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g2_w) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl WeightExpansion<Dipole> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e1] * other[e31]) - (self[e2] * other[e23]), 0.0])
                + (((self.group3().yz() * other.group1().zx()) - (self.group3().zx() * other.group1().yz())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (self[e4] * other[e45]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7       10        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        5        6        0      N/A
    // Totals...
    // yes simd       17       25        0      N/A
    //  no simd       41       59        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            -(Simd32x3::from(self[e4]) * other.group3().xyz()) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (other.group3().yzxw() * self.group3().zxy().with_w(self[e4])) - (self.group3().yzxw() * other.group3().zxy().with_w(other[e1234])),
            // e15, e25, e35, e1234
            (self.group3().xyzx() * Simd32x3::from(other[e3215]).with_w(right_anti_dual_g0[0]))
                + (Simd32x3::from(self[e5]) * other.group3().xyz())
                    .with_w((self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234])),
            // e4235, e4315, e4125, e3215
            (self.group3().yzxy() * other.group1().zxy().with_w(other[e25]))
                + ((Simd32x3::from(other[e3215]) * self.group0())
                    + ((self.group1().yz() * other.group3().zx()) - (self.group3().zx() * other.group1().yz())).with_z((self[e415] * other[e4315]) - (self[e2] * other[e23]))
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w((self[e321] * other[e3215]) + (self[e1] * other[e15]) - (self[e235] * other[e4235]) - (self[e5] * other[e45]))
                - (self.group2().xyzz() * Simd32x3::from(other[e1234]).with_w(other[e4125]))
                - (other.group3().yzxy() * self.group1().zxy().with_w(self[e315])),
        )
    }
}
impl WeightExpansion<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0       11        0      N/A
    //  no simd        0       24        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(other[e5] * self[e4] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group2().xyz().with_w(self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * -1.0) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl WeightExpansion<FlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e1] * other[e15]) + (self[e2] * other[e25]) + (self[e3] * other[e35]) - (self[e5] * other[e45])),
            // e1234
            self[e4] * other[e45],
        )
    }
}
impl WeightExpansion<Flector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       22       38        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((self.group3().zx() * other.group1().yz()) - (self.group3().yz() * other.group1().zx()))
                .with_zw((self[e2] * other[e4235]) - (self[e1] * other[e4315]), self[e4] * other[e3215]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group3().xyz()))
                .with_w((self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) + (self[e4] * other[e45])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yz() * other.group1().zx()).with_z(self[e415] * other[e4315]))
                .with_w((self[e321] * other[e3215]) + (self[e1] * other[e15]) - (self[e5] * other[e45]))
                - (self.group2().wwwy() * other.group0().xyz().with_w(other[e4315]))
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e235])),
        )
    }
}
impl WeightExpansion<Line> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[e415] * self[e1]) - (other[e425] * self[e2]) - (other[e435] * self[e3])),
            // e235, e315, e125, e12345
            ((Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group3().yzx()) - (other.group1().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       14        0      N/A
    //  no simd       15       33        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group0())).with_w(self[e4] * other[e5] * -1.0),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())).with_w(right_anti_dual_g0_w * self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       28       38        0        0
    //    simd2        1        3        0      N/A
    //    simd3       15       21        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       46       66        0      N/A
    //  no simd       83      123        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3_w = other[e321] * -1.0;
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3])
                    - (right_anti_dual_g3_w * self[e321])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (self[e4] * other[e5])
                    - (self[e5] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[0]) * self.group3().xyz().with_w(self[e4]),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]) * self.group3().xyz().with_w(self[e4])) - (right_anti_dual_g1 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_anti_dual_g1.xyz()) - (Simd32x3::from(right_anti_dual_g1[3]) * self.group3().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g1[2] * self[e2]) - (right_anti_dual_g1[1] * self[e3]),
                (right_anti_dual_g1[0] * self[e3]) - (right_anti_dual_g1[2] * self[e1]),
                (right_anti_dual_g1[1] * self[e1]) - (right_anti_dual_g1[0] * self[e2]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e4]) * other.group8()) + (Simd32x3::from(self[e5]) * other.group7())
                - (Simd32x3::from(right_anti_dual_g3_w) * self.group3().xyz()))
            .with_w(right_anti_dual_g0[0] * self[e321]),
            // e423, e431, e412
            (right_anti_dual_g5 * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group0()) + (other.group7().yzx() * self.group3().zxy())
                - (other.group7().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (right_anti_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group2().xyz()) + (other.group8().zxy() * self.group3().yzx())
                - (other.group8().yzx() * self.group3().zxy()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e5]) * other.group4())
                + (Simd32x3::from(other[e3215]) * self.group0())
                + (right_anti_dual_g6_xyz.yzx() * self.group3().zxy())
                + ((right_anti_dual_g1.yz() * self.group1().zx()) - (right_anti_dual_g1.zx() * self.group1().yz()))
                    .with_z((right_anti_dual_g1[0] * self[e425]) - (right_anti_dual_g1[1] * self[e415]))
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group2().xyz())
                - (Simd32x3::from(self[e4]) * other.group3().xyz()))
            .with_w((right_anti_dual_g1[0] * self[e235]) + (right_anti_dual_g1[1] * self[e315]) + (right_anti_dual_g1[2] * self[e125]) + (self[e321] * other[e3215]))
                - (self.group3().yzxw() * right_anti_dual_g6_xyz.zxy().with_w(other[e45])),
            // e1234
            (self[e4] * other[e45])
                - (self[e423] * right_anti_dual_g1[0])
                - (self[e431] * right_anti_dual_g1[1])
                - (self[e412] * right_anti_dual_g1[2])
                - (other[e41] * self[e1])
                - (other[e42] * self[e2])
                - (other[e43] * self[e3])
                - (right_anti_dual_g1[3] * self[e321]),
        )
    }
}
impl WeightExpansion<Plane> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd       12       27        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((self.group3().zx() * other.group0().yz()) - (self.group3().yz() * other.group0().zx()))
                .with_zw((self[e2] * other[e4235]) - (self[e1] * other[e4315]), self[e4] * other[e3215]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group3().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0())
                + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z((self[e415] * other[e4315]) - (self[e425] * other[e4235])))
            .with_w(self[e321] * other[e3215]),
        )
    }
}
impl WeightExpansion<RoundPoint> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3])
                - (self[e4] * other[e5])
                - (self[e5] * other[e4]),
        )
    }
}
impl WeightExpansion<Sphere> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        5        0        0
    //    simd3        5       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        7       16        0      N/A
    //  no simd       20       39        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()).with_w(self[e4] * other[e3215]) - (self.group3().zxyw() * right_anti_dual_g0_xyz.yzx().with_w(other[e1234])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))).with_w(self[e321] * other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g0_xyz[0] * self[e235]) + (self[e321] * other[e3215])),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       16        0        0
    //    simd3        8       11        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       20       29        0      N/A
    //  no simd       39       57        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group3().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g3_xyz[0]))
                + ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group0())
                    - (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()))
                .with_w(
                    (right_anti_dual_g3_xyz[1] * self[e2])
                        - (right_anti_dual_g1_w * self[e321])
                        - (right_anti_dual_g0_xyz[0] * self[e235])
                        - (right_anti_dual_g0_xyz[1] * self[e315])
                        - (right_anti_dual_g0_xyz[2] * self[e125])
                        - (right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (self[e4] * other[e5]),
                ),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e4]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())
                - (Simd32x3::from(right_anti_dual_g1_w) * self.group3().xyz()))
            .with_w(right_anti_dual_g0_w * self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group3().xyz().with_w(self[e4]),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       12        0        0
    //    simd2        1        2        0      N/A
    //    simd3        8       12        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       18       27        0      N/A
    //  no simd       38       56        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (right_anti_dual_g3_xyz.zxy() * self.group3().yzx()).with_w(self[e4] * other[e3215]) - (self.group3().zxyw() * right_anti_dual_g3_xyz.yzx().with_w(other[e1234])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e5]))).with_w(
                (self[e4] * other[e45])
                    - (right_anti_dual_g3_xyz[0] * self[e423])
                    - (right_anti_dual_g3_xyz[1] * self[e431])
                    - (right_anti_dual_g3_xyz[2] * self[e412])
                    - (self[e321] * other[e1234]),
            ),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e5]) * other.group0().xyz())
                + (Simd32x3::from(other[e3215]) * self.group0())
                + (right_anti_dual_g3_xyz.yzx() * self.group1().zxy())
                + ((self.group3().yz() * other.group1().zx()) - (self.group3().zx() * other.group1().yz())).with_z((self[e1] * other[e31]) - (self[e2] * other[e23]))
                - (Simd32x3::from(self[e4]) * other.group2().xyz())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g3_xyz[0] * self[e235]) + (right_anti_dual_g3_xyz[1] * self[e315]) + (right_anti_dual_g3_xyz[2] * self[e125]) + (self[e321] * other[e3215])),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiDualNum {
    type Output = WeightExpansionInfixPartial<AntiDualNum>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiDualNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       20        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiDualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       27        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl WeightExpansion<AntiFlector> for AntiDualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiDualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group1(),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiDualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiDualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Circle> for AntiDualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       21        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0().with_w(right_anti_dual_g2_w),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group2().xyz() * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(right_anti_dual_g2_w * self[e3215]),
        )
    }
}
impl WeightExpansion<Dipole> for AntiDualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       16        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group2(),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       10        0      N/A
    //  no simd        0       30        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x2::from(self[scalar] * -1.0).with_z(self[scalar]) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(self[e3215] * other[e1234]),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        1        4        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(
            // e3215, scalar
            Simd32x2::from([-(self[e3215] * other[e12345]) - (self[scalar] * other[e5]), self[scalar] * other[e12345] * -1.0]),
        )
    }
}
impl WeightExpansion<FlatPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<Flector> for AntiDualNum {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<Line> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl WeightExpansion<Motor> for AntiDualNum {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        1       10        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e3215
            (other.group1().xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar])).with_w((self[e3215] * right_anti_dual_g0[3]) - (self[scalar] * other[e5])),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       10        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        7        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        2       22        0      N/A
    //  no simd        2       49        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([right_anti_dual_g0[0] * self[scalar], (right_anti_dual_g0[1] * self[scalar]) + (self[e3215] * right_anti_dual_g1[3])]),
            // e1, e2, e3, e4
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]),
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group4(),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group3().xyz(),
            // e4235, e4315, e4125, e3215
            (other.group1().xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar])).with_w((right_anti_dual_g0[0] * self[e3215]) - (self[scalar] * other[e5])),
            // e1234
            self[scalar] * other[e4] * -1.0,
        )
    }
}
impl WeightExpansion<Plane> for AntiDualNum {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightExpansion<RoundPoint> for AntiDualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[scalar] * other[e4] * -1.0,
        )
    }
}
impl WeightExpansion<Scalar> for AntiDualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar])
    }
}
impl WeightExpansion<Sphere> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(self[e3215] * right_anti_dual_g0[3]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        1        9        0      N/A
    //  no simd        1       26        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            (other.group3().xyz() * Simd32x2::from(self[scalar]).with_z(self[scalar])).with_w((self[e3215] * right_anti_dual_g0[3]) - (self[scalar] * other[e5])),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        1       11        0      N/A
    //  no simd        1       31        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0().xyz() * Simd32x2::from(self[scalar] * -1.0).with_z(self[scalar]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w((self[e3215] * right_anti_dual_g3[3]) + (self[scalar] * other[scalar])),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]),
            // e1, e2, e3, e4
            right_anti_dual_g3 * Simd32x4::from(self[scalar]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiFlatPoint {
    type Output = WeightExpansionInfixPartial<AntiFlatPoint>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([other[e3215] * self[e321], 0.0]))
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e321] * self[e321])
    }
}
impl WeightExpansion<AntiFlector> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e321] * other[e321])
    }
}
impl WeightExpansion<AntiMotor> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e321] * other[e3215], 0.0]))
    }
}
impl WeightExpansion<AntiScalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Circle> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g2_w * self[e321]),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz())
                .with_w((self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125])),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       12        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e1234]).xyz() * self.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((self[e321] * other[e3215]) - (self[e235] * other[e4235]) - (self[e315] * other[e4315]) - (self[e125] * other[e4125])),
            // e1234
            self[e321] * other[e1234] * -1.0,
        )
    }
}
impl WeightExpansion<DualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Flector> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e321] * other[e3215]) - (self[e235] * other[e4235]) - (self[e315] * other[e4315]) - (self[e125] * other[e4125]),
            0.0,
        ]))
    }
}
impl WeightExpansion<Motor> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       23        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self[e321] * other[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0[0] * self[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz()).with_w(
                (right_anti_dual_g1_xyz[0] * self[e235]) + (right_anti_dual_g1_xyz[1] * self[e315]) + (right_anti_dual_g1_xyz[2] * self[e125]) + (self[e321] * other[e3215]),
            ),
            // e1234
            self[e321] * other[e1234] * -1.0,
        )
    }
}
impl WeightExpansion<Plane> for AntiFlatPoint {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e321] * other[e3215]) - (self[e235] * other[e4235]) - (self[e315] * other[e4315]) - (self[e125] * other[e4125]),
            0.0,
        ]))
    }
}
impl WeightExpansion<Sphere> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       13        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz()).with_w(
                (right_anti_dual_g0_xyz[0] * self[e235]) + (right_anti_dual_g0_xyz[1] * self[e315]) + (right_anti_dual_g0_xyz[2] * self[e125]) + (self[e321] * other[e3215]),
            ),
            // e1234
            self[e321] * other[e1234] * -1.0,
        )
    }
}
impl WeightExpansion<VersorEven> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(right_anti_dual_g0_w * self[e321]),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz()).with_w(
                (self[e321] * other[e321]) - (right_anti_dual_g0_xyz[0] * self[e235]) - (right_anti_dual_g0_xyz[1] * self[e315]) - (right_anti_dual_g0_xyz[2] * self[e125]),
            ),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        3       15        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e1234]).xyz() * self.group0().xyz() * Simd32x3::from(-1.0)).with_w(
                (right_anti_dual_g3_xyz[0] * self[e235]) + (right_anti_dual_g3_xyz[1] * self[e315]) + (right_anti_dual_g3_xyz[2] * self[e125]) + (self[e321] * other[e3215]),
            ),
            // e1234
            self[e321] * other[e1234] * -1.0,
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiFlector {
    type Output = WeightExpansionInfixPartial<AntiFlector>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd        8       17        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (other[e12] * self[e2]) - (other[e31] * self[e3]),
                (other[e23] * self[e3]) - (other[e12] * self[e1]),
                (other[e31] * self[e1]) - (other[e23] * self[e2]),
            ]) - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(other[e45] * self[e5] * -1.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       17       28        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz())).with_w(0.0),
            // e235, e315, e125, e12345
            (self.group1().yzxx() * right_anti_dual_g2_xyz.zxy().with_w(right_anti_dual_g3_xyz[0]))
                + ((Simd32x3::from(self[e5]) * other.group1().xyz()) - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy())).with_w(
                    (right_anti_dual_g3_xyz[1] * self[e2])
                        - (right_anti_dual_g1_w * self[e321])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                ),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, other[e3215], 0.0]) * (self.group1().xyz() * Simd32x2::from(other[e3215]).with_z(1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * self[e321]),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        3        0      N/A
    //  no simd        3       10        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        6       15        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(right_anti_dual_g0_w * -1.0) * self.group1().xyz()).with_w(
                (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3]) - (right_anti_dual_g0_w * self[e321]),
            ),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        3        0      N/A
    // no simd        3        9        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0.yzx() * self.group1().zxy()) - (right_anti_dual_g0.zxy() * self.group1().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        3       10        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]).xyz() * self.group1().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz()))
                .with_zw((self[e1] * other[e31]) - (self[e2] * other[e23]), self[e321] * other[e3215]),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        7        0      N/A
    // no simd       12       21        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group1().xyz())).with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        4        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       18       36        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx())).with_w(right_anti_dual_g1_w * self[e321] * -1.0),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e5]) * other.group0())
                .with_w((right_anti_dual_g2_w * self[e321]) - (right_anti_dual_g1_xyz[1] * self[e2]) - (right_anti_dual_g1_xyz[2] * self[e3]))
                - (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g1_w).with_w(right_anti_dual_g1_xyz[0])),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy()))
            .with_w(right_anti_dual_g2_w * self[e5]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<Dipole> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd        8       17        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (self[e2] * other[e12]) - (self[e3] * other[e31]),
                (self[e3] * other[e23]) - (self[e1] * other[e12]),
                (self[e1] * other[e31]) - (self[e2] * other[e23]),
            ]) - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(self[e5] * other[e45] * -1.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        9        0        0
    //    simd2        1        3        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        7       19        0      N/A
    //  no simd       18       38        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e23, e31, e12, e45
            ((self.group1().zx() * other.group3().yz()) - (self.group1().yz() * other.group3().zx()))
                .with_zw((self[e2] * other[e4235]) - (self[e1] * other[e4315]), self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(right_anti_dual_g0[0])) + (Simd32x3::from(self[e5]) * other.group3().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (self.group1().yzxx() * other.group1().zxy().with_w(other[e15]))
                + (-(self.group1().zx() * other.group1().yz()).with_z(self[e2] * other[e23] * -1.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(other[e1234]) * self.group0().xyz()))
                .with_w(-(self[e235] * other[e4235]) - (self[e5] * other[e45])),
        )
    }
}
impl WeightExpansion<DualNum> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl WeightExpansion<FlatPoint> for AntiFlector {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e1] * other[e15]) + (self[e2] * other[e25]) + (self[e3] * other[e35]) - (self[e5] * other[e45]),
            0.0,
        ]))
    }
}
impl WeightExpansion<Flector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd4        3        2        0      N/A
    // Totals...
    // yes simd        7        9        0      N/A
    //  no simd       17       17        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
                + ((self.group1().zx() * other.group1().yz()) - (self.group1().yz() * other.group1().zx())).with_zw(0.0, 0.0),
            // e15, e25, e35, e3215
            (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e15]))
                + (self.group1().wwwy() * other.group1().xyz().with_w(other[e25]))
                + Simd32x3::from(0.0).with_w((self[e3] * other[e35]) - (self[e235] * other[e4235]) - (self[e5] * other[e45])),
        )
    }
}
impl WeightExpansion<Line> for AntiFlector {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       18        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz())
                + (Simd32x3::from(self[e5]) * other.group0().xyz())
                + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()))
            .with_w(right_anti_dual_g0_w * self[e321]),
            // e1, e2, e3, e5
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       21        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       18        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       22       43        0      N/A
    //  no simd       44       89        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3_w = other[e321] * -1.0;
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3])
                    - (right_anti_dual_g3_w * self[e321])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (self[e5] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, right_anti_dual_g0[0], 0.0]) * (self.group1().xyz() * Simd32x2::from(right_anti_dual_g0[0]).with_z(1.0)).with_w(0.0),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))).with_w(self[e5] * other[e1234] * -1.0),
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e23, e31, e12
            (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()),
            // e415, e425, e435, e321
            (Simd32x3::from(self[e5]) * other.group7()).with_w((right_anti_dual_g0[0] * self[e321]) - (right_anti_dual_g5[1] * self[e2]) - (right_anti_dual_g5[2] * self[e3]))
                - (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g3_w).with_w(right_anti_dual_g5[0])),
            // e423, e431, e412
            (other.group7().yzx() * self.group1().zxy()) - (other.group7().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (right_anti_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz()) + (other.group8().zxy() * self.group1().yzx())
                - (other.group8().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e5]) * other.group4()) + (right_anti_dual_g6_xyz.yzx() * self.group1().zxy()) - (Simd32x3::from(other[e1234]) * self.group0().xyz()))
                .with_w(right_anti_dual_g1_xyz[0] * self[e235])
                - (self.group1().yzxw() * right_anti_dual_g6_xyz.zxy().with_w(other[e45])),
            // e1234
            -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<Plane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       14        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
                + ((self.group1().zx() * other.group0().yz()) - (self.group1().yz() * other.group0().zx())).with_zw(0.0, 0.0),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group1().xyz())).with_w(self[e235] * other[e4235] * -1.0),
        )
    }
}
impl WeightExpansion<RoundPoint> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl WeightExpansion<Sphere> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        2        7        0      N/A
    // Totals...
    // yes simd        5       17        0      N/A
    //  no simd        9       31        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual_g0_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))).with_w(self[e321] * other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz()).with_w(
                (right_anti_dual_g0_xyz[0] * self[e235]) + (right_anti_dual_g0_xyz[1] * self[e315]) + (right_anti_dual_g0_xyz[2] * self[e125]) + (self[e321] * other[e3215]),
            ),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       12       21        0      N/A
    //  no simd       25       39        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g0.yz() * self.group1().zx()) - (right_anti_dual_g0.zx() * self.group1().yz())).with_zw(
                (right_anti_dual_g0[0] * self[e2]) - (right_anti_dual_g0[1] * self[e1]),
                (self[e1] * other[e1])
                    - (right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g0[0] * self[e235])
                    - (right_anti_dual_g0[1] * self[e315])
                    - (right_anti_dual_g0[2] * self[e125]),
            ),
            // e415, e425, e435, e321
            (right_anti_dual_g0 * Simd32x3::from(self[e5]).with_w(self[e321]))
                + Simd32x3::from(0.0).with_w(-(right_anti_dual_g1_xyz[1] * self[e2]) - (right_anti_dual_g1_xyz[2] * self[e3]))
                - (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g1_w).with_w(right_anti_dual_g1_xyz[0])),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy()))
            .with_w(right_anti_dual_g0[3] * self[e5]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_anti_dual_g0[3]).xyz() * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd       15       34        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group1().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual_g3_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group1().zxy())).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e5]))).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e5]) * other.group0().xyz())
                + ((self.group1().yz() * other.group1().zx()) - (self.group1().zx() * other.group1().yz())).with_z((self[e1] * other[e31]) - (self[e2] * other[e23]))
                - (Simd32x3::from(other[e1234]) * self.group0().xyz()))
            .with_w(right_anti_dual_g3_xyz[0] * self[e235]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiLine {
    type Output = WeightExpansionInfixPartial<AntiLine>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((other.group0().yzx() * self.group1().zxy()) - (Simd32x3::from(other[e321]) * self.group0()) - (other.group0().zxy() * self.group1().yzx())).with_w(0.0),
            // e1234
            -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, other[e3215], 0.0]) * (self.group0() * Simd32x2::from(other[e3215]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e23]) - (right_anti_dual_g0_xyz[1] * self[e31]) - (right_anti_dual_g0_xyz[2] * self[e12])),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        2        9        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other[e321]).xyz() * Simd32x3::from(-1.0))
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e23]) - (right_anti_dual_g0_xyz[1] * self[e31]) - (right_anti_dual_g0_xyz[2] * self[e12])),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]))
    }
}
impl WeightExpansion<AntiMotor> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w((self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])),
            // e235, e315, e125, e5
            (self.group0() * Simd32x4::from(other[e3215]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((self.group1().zxy() * other.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0),
            // e1234
            -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        8       19        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (Simd32x3::from(right_anti_dual_g2_w) * self.group0()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1()).with_w(-(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e4235, e4315, e4125, e3215
            ((self.group1().zxy() * other.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Dipole> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0(),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[e1234]).xyz()).with_w((self[e23] * other[e4235]) + (self[e31] * other[e4315]) + (self[e12] * other[e4125])),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group3().zxy()) - (self.group1().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<DualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl WeightExpansion<Flector> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group1().zxy()) - (self.group1().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Line> for AntiLine {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            -(self[e23] * other[e235]) - (self[e31] * other[e315]) - (self[e12] * other[e125]) - (self[e15] * other[e415]) - (self[e25] * other[e425]) - (self[e35] * other[e435]),
            0.0,
        ]))
    }
}
impl WeightExpansion<Motor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       13        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0),
            // e15, e25, e35, e3215
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1()).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e15])
                    - (right_anti_dual_g0_xyz[1] * self[e25])
                    - (right_anti_dual_g0_xyz[2] * self[e35])
                    - (right_anti_dual_g1_xyz[0] * self[e23])
                    - (right_anti_dual_g1_xyz[1] * self[e31])
                    - (right_anti_dual_g1_xyz[2] * self[e12]),
            ),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd2        0        1        0      N/A
    //    simd3        4       11        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       25        0      N/A
    //  no simd       21       51        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, right_anti_dual_g0[0], 0.0]) * (self.group1() * Simd32x2::from(right_anti_dual_g0[0]).with_z(1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e1234]) * self.group1())
                .with_w(-(right_anti_dual_g1_xyz[0] * self[e23]) - (right_anti_dual_g1_xyz[1] * self[e31]) - (right_anti_dual_g1_xyz[2] * self[e12])),
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0(),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()),
            // e4235, e4315, e4125, e3215
            ((self.group1().zxy() * other.group7().yzx()) - (Simd32x3::from(other[e321]) * self.group0()) - (self.group1().yzx() * other.group7().zxy())).with_w(0.0),
            // e1234
            -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
        )
    }
}
impl WeightExpansion<Plane> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (self.group1().yzx() * other.group0().zxy()) - (self.group1().zxy() * other.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Sphere> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        8       21        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0(),
            // e415, e425, e435, e321
            (Simd32x3::from(other[e1234]) * self.group1())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e23]) - (right_anti_dual_g0_xyz[1] * self[e31]) - (right_anti_dual_g0_xyz[2] * self[e12])),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        8       19        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(0.0),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e23]) - (right_anti_dual_g0_xyz[1] * self[e31]) - (right_anti_dual_g0_xyz[2] * self[e12])),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.yzx() * self.group1().zxy()) - (Simd32x3::from(other[e321]) * self.group0()) - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        8       21        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e1234]) * self.group0(),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[e1234]).xyz())
                .with_w(-(right_anti_dual_g3_xyz[0] * self[e23]) - (right_anti_dual_g3_xyz[1] * self[e31]) - (right_anti_dual_g3_xyz[2] * self[e12])),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g3_xyz.yzx() * self.group1().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()))
                .with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiMotor {
    type Output = WeightExpansionInfixPartial<AntiMotor>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd        6       27        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group2().xyz() * Simd32x3::from(-1.0)).with_w(
                (other[scalar] * self[scalar])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12]),
            ),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        3        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       12       29        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (Simd32x4::from(self[scalar]).xyz() * other.group2().xyz())
                .with_w(-(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e4] * self[scalar])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group3().xyz()) + (other.group0().yzx() * self.group1().zxy())
                - (other.group0().zxy() * self.group1().yzx()))
            .with_w(other[e5] * self[scalar] * -1.0),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(other[scalar] * self[scalar]),
            // e235, e315, e125, e5
            Simd32x4::from(other[e3215]) * self.group0(),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       11        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_anti_dual_g0[3]).xyz() * self.group0().xyz())
                .with_w(-(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12])),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        6        0      N/A
    //  no simd        3       13        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[e5] * self[scalar] * -1.0),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       15        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]).xyz())
                .with_w(-(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12])),
            // e235, e315, e125, e5
            (other.group1() * Simd32x4::from(self[scalar]).xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        1        4        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        6       17        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[scalar] * self[scalar])),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[e3215] * self[scalar]),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiMotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        2        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       10        0      N/A
    //  no simd        8       23        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(self[scalar]).xyz()).with_w(-(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        4        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       12        0      N/A
    //  no simd       12       29        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0().with_w(right_anti_dual_g2_w),
            // e23, e31, e12, e45
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(right_anti_dual_g1_w * self[scalar]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1_w) * self.group0().xyz()) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()))
                .with_w(right_anti_dual_g2_w * self[e3215]),
        )
    }
}
impl WeightExpansion<Dipole> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd        5       26        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (other.group2() * Simd32x4::from(self[scalar]).xyz() * Simd32x3::from(-1.0)).with_w(
                -(right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12]),
            ),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        4        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd       15       34        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[scalar]) * other.group0())).with_w(self[e3215] * other[e1234]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(self[scalar] * other[e45]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
                (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
                (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        7        0      N/A
    //  no simd        1       14        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e15, e25, e35, e3215
            (self.group1().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e5] * self[scalar]) - (other[e12345] * self[e3215])),
        )
    }
}
impl WeightExpansion<FlatPoint> for AntiMotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<Flector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       11        0      N/A
    //  no simd        9       21        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            (Simd32x3::from([
                (right_anti_dual_g1[1] * self[e35]) - (right_anti_dual_g1[2] * self[e25]),
                (right_anti_dual_g1[2] * self[e15]) - (right_anti_dual_g1[0] * self[e35]),
                (right_anti_dual_g1[0] * self[e25]) - (right_anti_dual_g1[1] * self[e15]),
            ]) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz()))
            .with_w(self[scalar] * other[e45]),
            // e1, e2, e3, e5
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<Line> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (other.group0() * Simd32x4::from(self[scalar]).xyz()).with_w(0.0),
            // e15, e25, e35, e3215
            (other.group1() * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(other[e415] * self[e15])
                    - (other[e425] * self[e25])
                    - (other[e435] * self[e35])
                    - (other[e235] * self[e23])
                    - (other[e315] * self[e31])
                    - (other[e125] * self[e12]),
            ),
        )
    }
}
impl WeightExpansion<Motor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        8       13        0      N/A
    //  no simd       16       23        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz())).with_w(right_anti_dual_g0_w * self[scalar]),
            // e15, e25, e35, e3215
            (right_anti_dual_g1 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(right_anti_dual_g0_w) * self.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0_xyz[0] * self[e15])
                        - (right_anti_dual_g0_xyz[1] * self[e25])
                        - (right_anti_dual_g0_xyz[2] * self[e35])
                        - (right_anti_dual_g1[0] * self[e23])
                        - (right_anti_dual_g1[1] * self[e31])
                        - (right_anti_dual_g1[2] * self[e12]),
                ),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       24        0        0
    //    simd2        0        1        0      N/A
    //    simd3        9       18        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       22       44        0      N/A
    //  no simd       40       84        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3_w = other[e321] * -1.0;
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g0[0] * self[scalar],
                (right_anti_dual_g0[1] * self[scalar]) + (right_anti_dual_g1[3] * self[e3215])
                    - (right_anti_dual_g6_xyz[0] * self[e23])
                    - (right_anti_dual_g6_xyz[1] * self[e31])
                    - (right_anti_dual_g6_xyz[2] * self[e12])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35]),
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            ((Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group8())).with_w(right_anti_dual_g3_w * self[scalar]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group7(),
            // e23, e31, e12
            (Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group6().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g6_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())).with_w(self[scalar] * other[e45]),
            // e423, e431, e412
            (right_anti_dual_g7 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g1[1] * self[e35]) - (right_anti_dual_g1[2] * self[e25]),
                (right_anti_dual_g1[2] * self[e15]) - (right_anti_dual_g1[0] * self[e35]),
                (right_anti_dual_g1[0] * self[e25]) - (right_anti_dual_g1[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[scalar]) * other.group3().xyz()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g3_w) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (other.group7().yzx() * self.group1().zxy())
                - (other.group7().zxy() * self.group1().yzx()))
            .with_w(right_anti_dual_g0[0] * self[e3215]),
            // e1234
            -(other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (self[scalar] * other[e4]),
        )
    }
}
impl WeightExpansion<Plane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       17        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([0.0, 0.0, (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]), 0.0])
                + ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz())
                    + ((right_anti_dual_g0.yz() * self.group1().zx()) - (right_anti_dual_g0.zx() * self.group1().yz())).with_z(0.0))
                .with_w(0.0),
            // e1, e2, e3, e5
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<RoundPoint> for AntiMotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[scalar] * other[e4] * -1.0,
        )
    }
}
impl WeightExpansion<Scalar> for AntiMotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar])
    }
}
impl WeightExpansion<Sphere> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       10        0        0
    //    simd3        1        3        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       15        0      N/A
    //  no simd        8       27        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_anti_dual_g0[3]) * self.group0().xyz().with_w(self[e3215]),
            // e415, e425, e435, e321
            (Simd32x4::from(right_anti_dual_g0[3]).xyz() * self.group1().xyz())
                .with_w(-(right_anti_dual_g0[0] * self[e23]) - (right_anti_dual_g0[1] * self[e31]) - (right_anti_dual_g0[2] * self[e12])),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (right_anti_dual_g0[1] * self[e35]) - (right_anti_dual_g0[2] * self[e25]),
                (right_anti_dual_g0[2] * self[e15]) - (right_anti_dual_g0[0] * self[e35]),
                (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       12        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       19        0      N/A
    //  no simd       15       34        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_w = other[e321] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e23, e31, e12, e45
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(right_anti_dual_g1_w * self[scalar]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[scalar]) * other.group2().xyz())).with_w(self[scalar] * other[e4] * -1.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g0[1] * self[e35]) - (right_anti_dual_g0[2] * self[e25]),
                (right_anti_dual_g0[2] * self[e15]) - (right_anti_dual_g0[0] * self[e35]),
                (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]),
            ]) + (Simd32x3::from(right_anti_dual_g1_w) * self.group0().xyz())
                + (Simd32x3::from(self[scalar]) * other.group3().xyz()))
            .with_w(right_anti_dual_g0[3] * self[e3215]),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       14        0        0
    //    simd3        3        5        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       13       23        0      N/A
    //  no simd       25       45        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(right_anti_dual_g3[3]) * self.group0().xyz().with_w(self[e3215]))
                + Simd32x3::from(0.0).with_w(
                    (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])
                        - (right_anti_dual_g0[0] * self[e15])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35]),
                ),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(self[scalar] * other[e45]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (right_anti_dual_g3[1] * self[e35]) - (right_anti_dual_g3[2] * self[e25]),
                (right_anti_dual_g3[2] * self[e15]) - (right_anti_dual_g3[0] * self[e35]),
                (right_anti_dual_g3[0] * self[e25]) - (right_anti_dual_g3[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()))
            .with_w(self[scalar] * other[e3215]),
            // e1, e2, e3, e4
            right_anti_dual_g3 * Simd32x4::from(self[scalar]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiPlane {
    type Output = WeightExpansionInfixPartial<AntiPlane>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd        8       17        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (other[e12] * self[e2]) - (other[e31] * self[e3]),
                (other[e23] * self[e3]) - (other[e12] * self[e1]),
                (other[e31] * self[e1]) - (other[e23] * self[e2]),
            ]) - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(other[e45] * self[e5] * -1.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for AntiPlane {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        3        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       13       23        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            (self.group0().yzxx() * right_anti_dual_g2_xyz.zxy().with_w(right_anti_dual_g3_xyz[0]))
                + ((Simd32x3::from(self[e5]) * other.group1().xyz()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())).with_w(right_anti_dual_g3_xyz[1] * self[e2]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for AntiPlane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, other[e3215], 0.0]) * (self.group0().xyz() * Simd32x2::from(other[e3215]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for AntiPlane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        3        0      N/A
    // no simd        3        9        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e321]) * self.group0().xyz(),
            // e235, e315, e125
            (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
        )
    }
}
impl WeightExpansion<AntiFlector> for AntiPlane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        5       12        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(other[e321]).xyz() * self.group0().xyz())
                .with_w((right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3])),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        1        3        0      N/A
    // no simd        3        9        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0.yzx() * self.group0().zxy()) - (right_anti_dual_g0.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for AntiPlane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        7        9        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]).xyz() * self.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + ((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<AntiPlane> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]),
        )
    }
}
impl WeightExpansion<AntiScalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Circle> for AntiPlane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        7        0      N/A
    // no simd       12       21        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group1().xyz()) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy()),
        )
    }
}
impl WeightExpansion<CircleRotor> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        4        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       12       26        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0),
            // e235, e315, e125, e4
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
        )
    }
}
impl WeightExpansion<Dipole> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       11        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        6       13        0      N/A
    //  no simd        8       17        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (self[e2] * other[e12]) - (self[e3] * other[e31]),
                (self[e3] * other[e23]) - (self[e1] * other[e12]),
                (self[e1] * other[e31]) - (self[e2] * other[e23]),
            ]) - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(self[e5] * other[e45] * -1.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       13        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        7       20        0      N/A
    //  no simd       12       32        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e23, e31, e12, e45
            ((self.group0().zx() * other.group3().yz()) - (self.group0().yz() * other.group3().zx()))
                .with_zw((self[e2] * other[e4235]) - (self[e1] * other[e4315]), self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(self[e5]) * other.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())).with_w(right_anti_dual_g0[0] * self[e1]),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (self[e2] * other[e12]) - (self[e3] * other[e31]),
                (self[e3] * other[e23]) - (self[e1] * other[e12]),
                (self[e1] * other[e31]) - (self[e2] * other[e23]),
            ]) - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
            .with_w(self[e3] * other[e35]),
        )
    }
}
impl WeightExpansion<DualNum> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<FlatPoint> for AntiPlane {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([
            (self[e1] * other[e15]) + (self[e2] * other[e25]) + (self[e3] * other[e35]) - (self[e5] * other[e45]),
            0.0,
        ]))
    }
}
impl WeightExpansion<Flector> for AntiPlane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       14        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, (self[e2] * other[e4235]) - (self[e1] * other[e4315]), 0.0])
                + ((self.group0().zx() * other.group1().yz()) - (self.group0().yz() * other.group1().zx())).with_zw(0.0, 0.0),
            // e15, e25, e35, e3215
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())).with_w(self[e5] * other[e45] * -1.0),
        )
    }
}
impl WeightExpansion<Line> for AntiPlane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       14        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            ((Simd32x3::from(self[e5]) * other.group0().xyz()) + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
        )
    }
}
impl WeightExpansion<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       13        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       19        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       34        0      N/A
    //  no simd       29       76        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3]) - (self[e5] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, right_anti_dual_g0[0], 0.0]) * (self.group0().xyz() * Simd32x2::from(right_anti_dual_g0[0]).with_z(1.0)).with_w(0.0),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))).with_w(self[e5] * other[e1234] * -1.0),
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e23, e31, e12
            (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e5]) * other.group7()) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group6().xyz()) + (other.group8().zxy() * self.group0().yzx()) - (other.group8().yzx() * self.group0().zxy()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g6_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g7 * Simd32x3::from(self[e5])) - (right_anti_dual_g6_xyz.zxy() * self.group0().yzx()))
                .with_w(self[e5] * other[e45] * -1.0),
            // e1234
            (right_anti_dual_g7[0] * self[e1]) + (right_anti_dual_g7[1] * self[e2]) + (right_anti_dual_g7[2] * self[e3]),
        )
    }
}
impl WeightExpansion<Plane> for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        6       12        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e4315]) - (self[e2] * other[e4125]),
                (self[e1] * other[e4125]) - (self[e3] * other[e4235]),
                (self[e2] * other[e4235]) - (self[e1] * other[e4315]),
            ]),
            // e15, e25, e35
            (Simd32x3::from(self[e5]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz()),
        )
    }
}
impl WeightExpansion<RoundPoint> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (self[e5] * other[e4]),
        )
    }
}
impl WeightExpansion<Sphere> for AntiPlane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        2        6        0      N/A
    // Totals...
    // yes simd        2        9        0      N/A
    //  no simd        6       21        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e5])),
        )
    }
}
impl WeightExpansion<VersorEven> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        4        8        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd       12       27        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx())).with_w(self[e1] * other[e1]),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(0.0),
            // e235, e315, e125, e5
            ((Simd32x3::from(self[e5]) * other.group1().xyz()) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
                .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       13        0        0
    //    simd3        3        7        0      N/A
    // Totals...
    // yes simd        6       20        0      N/A
    //  no simd       12       34        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234] * -1.0) * self.group0().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy())).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e5]))).with_w(self[e1] * other[e41] * -1.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (self[e2] * other[e12]) - (self[e3] * other[e31]),
                (self[e3] * other[e23]) - (self[e1] * other[e12]),
                (self[e1] * other[e31]) - (self[e2] * other[e23]),
            ]) + (Simd32x3::from(self[e5]) * other.group0().xyz()))
            .with_w(self[e5] * other[e45] * -1.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for AntiScalar {
    type Output = WeightExpansionInfixPartial<AntiScalar>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightExpansion<CircleRotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightExpansion<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightExpansion<Motor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightExpansion<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl WeightExpansion<VersorEven> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345] * -1.0)
    }
}
impl std::ops::Div<WeightExpansionInfix> for Circle {
    type Output = WeightExpansionInfixPartial<Circle>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiDipoleInversion> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (right_anti_dual_g2_xyz[0] * self[e423])
                - (right_anti_dual_g2_xyz[1] * self[e431])
                - (right_anti_dual_g2_xyz[2] * self[e412])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]))
    }
}
impl WeightExpansion<AntiFlatPoint> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightExpansion<AntiFlector> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightExpansion<AntiMotor> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]))
    }
}
impl WeightExpansion<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       11        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0) * self.group2(),
        )
    }
}
impl WeightExpansion<Circle> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        )
    }
}
impl WeightExpansion<CircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       21        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g2_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g2_w) * self.group2()).with_w(
                (self[e321] * other[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
            ),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       16        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group3().zx()) - (self.group1().zx() * other.group3().yz())).with_z(0.0)
                    - (Simd32x3::from(other[e1234]) * self.group2()))
                .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       13        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * -1.0) * self.group2(),
        )
    }
}
impl WeightExpansion<Flector> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       12       12        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group1().zx()) - (self.group1().zx() * other.group1().yz())).with_z(0.0))
                    .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]),
        )
    }
}
impl WeightExpansion<Line> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        )
    }
}
impl WeightExpansion<Motor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       17        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2()).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightExpansion<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       16        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       16       25        0      N/A
    //  no simd       22       43        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e321] * other[e321])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0[0]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[0]) * self.group2(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g1_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2())
                - (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g1_xyz[0] * self[e235]) + (right_anti_dual_g1_xyz[1] * self[e315])),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e423]) - (right_anti_dual_g1_xyz[1] * self[e431]) - (right_anti_dual_g1_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<Plane> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       12       12        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0))
                    .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]),
        )
    }
}
impl WeightExpansion<Sphere> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2())
                - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g0_xyz[0] * self[e235]) + (right_anti_dual_g0_xyz[1] * self[e315])),
            // e1234
            -(right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<VersorEven> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd        9       21        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2()).with_w(
                (self[e321] * other[e321])
                    - (right_anti_dual_g0_xyz[0] * self[e235])
                    - (right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightExpansion<VersorOdd> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g3_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2())
                - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g3_xyz[0] * self[e235]) + (right_anti_dual_g3_xyz[1] * self[e315])),
            // e1234
            -(right_anti_dual_g3_xyz[0] * self[e423]) - (right_anti_dual_g3_xyz[1] * self[e431]) - (right_anti_dual_g3_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for CircleRotor {
    type Output = WeightExpansionInfixPartial<CircleRotor>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiDipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (right_anti_dual_g2_xyz[0] * self[e423])
                - (right_anti_dual_g2_xyz[1] * self[e431])
                - (right_anti_dual_g2_xyz[2] * self[e412])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]))
    }
}
impl WeightExpansion<AntiFlatPoint> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightExpansion<AntiFlector> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
        )
    }
}
impl WeightExpansion<AntiMotor> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e3215]) * self.group0().with_w(self[e321]))
    }
}
impl WeightExpansion<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       12        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
        )
    }
}
impl WeightExpansion<Circle> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        )
    }
}
impl WeightExpansion<CircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       10       22        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g2_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g2_w * self[e12345]) + (other[e321] * self[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
        )
    }
}
impl WeightExpansion<DipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       10        0      N/A
    //  no simd       16       16        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group3().zx()) - (self.group1().zx() * other.group3().yz())).with_z(0.0)
                    - (Simd32x3::from(other[e1234]) * self.group2().xyz()))
                .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       14        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[e12345] * -1.0) * self.group2(),
        )
    }
}
impl WeightExpansion<Flector> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       12       12        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group1().zx()) - (self.group1().zx() * other.group1().yz())).with_z(0.0))
                    .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]),
        )
    }
}
impl WeightExpansion<Line> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        )
    }
}
impl WeightExpansion<Motor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd        6       18        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g0_w * self[e12345])
                    - (right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightExpansion<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       23       44        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g0[0] * self[e12345]) + (self[e321] * other[e321])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e423] * other[e235])
                    - (self[e431] * other[e315])
                    - (self[e412] * other[e125])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0[0]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[0]) * self.group2().xyz(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g1_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g1_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g1_xyz[0] * self[e235]) + (self[e321] * other[e3215])),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e423]) - (right_anti_dual_g1_xyz[1] * self[e431]) - (right_anti_dual_g1_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<Plane> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        8        0      N/A
    //  no simd       12       12        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0()) + ((self.group1().yz() * other.group0().zx()) - (self.group1().zx() * other.group0().yz())).with_z(0.0))
                    .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]),
        )
    }
}
impl WeightExpansion<Sphere> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g0_xyz[0] * self[e235]) + (self[e321] * other[e3215])),
            // e1234
            -(right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl WeightExpansion<VersorEven> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       15        0      N/A
    //  no simd       10       22        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e235, e315, e125, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g0_w * self[e12345]) + (self[e321] * other[e321])
                    - (right_anti_dual_g0_xyz[0] * self[e235])
                    - (right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412]),
            ),
        )
    }
}
impl WeightExpansion<VersorOdd> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd       13       21        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0()) + (right_anti_dual_g3_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g3_xyz[0] * self[e235]) + (self[e321] * other[e3215])),
            // e1234
            -(right_anti_dual_g3_xyz[0] * self[e423]) - (right_anti_dual_g3_xyz[1] * self[e431]) - (right_anti_dual_g3_xyz[2] * self[e412]) - (self[e321] * other[e1234]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Dipole {
    type Output = WeightExpansionInfixPartial<Dipole>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       13        0      N/A
    //  no simd       20       25        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (self.group0().yzx() * right_anti_dual_g2.zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (other.group0().zxy() * self.group2().yzx())
                - (self.group0().zxy() * right_anti_dual_g2.yzx()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for Dipole {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e3215]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e3215]) * self.group1().xyz(),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        )
    }
}
impl WeightExpansion<AntiMotor> for Dipole {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(other[e3215]).xyz()).with_w(
                (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]),
            ),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e3215]).xyz() * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       11        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(right_anti_dual_g0) * self.group2(),
        )
    }
}
impl WeightExpansion<Circle> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       12        0      N/A
    //  no simd       20       24        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (other.group2().zxy() * self.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (other.group0().zxy() * self.group2().yzx())
                - (other.group2().yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<CircleRotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       20       35        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g2_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g2_w) * self.group2()).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e41])
                    - (right_anti_dual_g1_xyz[1] * self[e42])
                    - (right_anti_dual_g1_xyz[2] * self[e43])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            ),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) + (other.group0().yzx() * self.group2().zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(0.0),
        )
    }
}
impl WeightExpansion<Dipole> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6        9        0      N/A
    // no simd       18       27        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group3().xyz()) + (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(other[e3215]) * self.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group3().zxy()) - (self.group2().zxy() * other.group3().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       13        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[e12345] * -1.0) * self.group2(),
        )
    }
}
impl WeightExpansion<FlatPoint> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightExpansion<Flector> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        7        0      N/A
    // no simd       12       21        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group1().zxy()) - (self.group2().zxy() * other.group1().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Line> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
            // e1234
            -(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435]),
        )
    }
}
impl WeightExpansion<Motor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        2        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd        8       24        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e41]) - (right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43])),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e45])) + (self.group0().yzx() * right_anti_dual_g1.zxy()) - (self.group0().zxy() * right_anti_dual_g1.yzx()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       16        0        0
    //    simd2        0        1        0      N/A
    //    simd3       11       19        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       25       37        0      N/A
    //  no simd       47       79        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (self[e45] * other[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0[0]) * self.group2().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0),
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_anti_dual_g1_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group2().yzx()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g5 * Simd32x3::from(self[e45])) + (self.group0().yzx() * other.group8().zxy()) + (self.group2().zxy() * other.group7().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (self.group0().zxy() * other.group8().yzx())
                - (self.group2().yzx() * other.group7().zxy()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g5[0] * self[e41])
                - (right_anti_dual_g5[1] * self[e42])
                - (right_anti_dual_g5[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<Plane> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        4        7        0      N/A
    // no simd       12       21        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())).with_w(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (self.group2().yzx() * other.group0().zxy()) - (self.group2().zxy() * other.group0().yzx()),
        )
    }
}
impl WeightExpansion<Sphere> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6       10        0      N/A
    // no simd       18       30        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e45]))).with_w(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx()),
        )
    }
}
impl WeightExpansion<VersorEven> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        8        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       20       36        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2()).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e23])
                    - (right_anti_dual_g0_xyz[1] * self[e31])
                    - (right_anti_dual_g0_xyz[2] * self[e12])
                    - (right_anti_dual_g1_xyz[0] * self[e41])
                    - (right_anti_dual_g1_xyz[1] * self[e42])
                    - (right_anti_dual_g1_xyz[2] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy()) + (self.group0().yzx() * right_anti_dual_g2.zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())
                - (self.group0().zxy() * right_anti_dual_g2.yzx()))
            .with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6       10        0      N/A
    // no simd       18       30        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(other[e3215]) * self.group0()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e45]))).with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_anti_dual_g3_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group2().yzx()))
                .with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for DipoleInversion {
    type Output = WeightExpansionInfixPartial<DipoleInversion>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       13        0      N/A
    //  no simd       20       25        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (self.group0().yzx() * right_anti_dual_g2.zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (other.group0().zxy() * self.group2().yzx())
                - (self.group0().zxy() * right_anti_dual_g2.yzx()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0       11        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e3215]) * self.group0().with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, other[e3215], 0.0]) * (self.group1().xyz() * Simd32x2::from(other[e3215]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group1().xyz()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        )
    }
}
impl WeightExpansion<AntiMotor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(other[e3215]).xyz()).with_w(
                (self[e41] * other[e15])
                    + (self[e42] * other[e25])
                    + (self[e43] * other[e35])
                    + (other[e23] * self[e23])
                    + (other[e31] * self[e31])
                    + (other[e12] * self[e12])
                    + (other[e3215] * self[e1234]),
            ),
            // e235, e315, e125, e5
            (Simd32x4::from(other[e3215]).xyz() * self.group1().xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       16        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group3(),
        )
    }
}
impl WeightExpansion<Circle> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        5        6        0      N/A
    // Totals...
    // yes simd       10       12        0      N/A
    //  no simd       20       24        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45])) + (other.group0().yzx() * self.group2().zxy()) + (other.group2().zxy() * self.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (other.group0().zxy() * self.group2().yzx())
                - (other.group2().yzx() * self.group0().zxy()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<CircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       19        0      N/A
    //  no simd       24       40        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g2_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g2_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g2_w * self[e1234])
                    - (right_anti_dual_g1_xyz[0] * self[e41])
                    - (right_anti_dual_g1_xyz[1] * self[e42])
                    - (right_anti_dual_g1_xyz[2] * self[e43])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            ),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx())
                + (other.group0().yzx() * self.group2().zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(right_anti_dual_g2_w * self[e3215]),
        )
    }
}
impl WeightExpansion<Dipole> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        9       10        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5        7        0      N/A
    // Totals...
    // yes simd       15       20        0      N/A
    //  no simd       26       36        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (self.group0().zxy() * other.group3().yzx()) - (self.group0().yzx() * other.group3().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(self[e45]) * other.group3().xyz()))
                .with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group1().xyz())
                + ((other.group3().zx() * self.group2().yz()) - (other.group3().yz() * self.group2().zx())).with_z((other[e4315] * self[e15]) - (other[e4235] * self[e25])))
            .with_w(
                (other[e41] * self[e15])
                    + (other[e42] * self[e25])
                    + (self[e43] * other[e35])
                    + (other[e23] * self[e23])
                    + (other[e31] * self[e31])
                    + (other[e12] * self[e12])
                    + (other[e1234] * self[e3215])
                    - (other[e45] * self[e45])
                    - (other[e4235] * self[e4235]),
            ),
        )
    }
}
impl WeightExpansion<DualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       19        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345] * -1.0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * -1.0) * self.group3(),
        )
    }
}
impl WeightExpansion<FlatPoint> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        )
    }
}
impl WeightExpansion<Flector> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    // Totals...
    // yes simd        9       14        0      N/A
    //  no simd       16       26        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().zxy() * other.group1().yzx()) - (self.group0().yzx() * other.group1().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            ((Simd32x3::from(other[e3215]) * self.group1().xyz())
                + ((self.group2().yz() * other.group1().zx()) - (self.group2().zx() * other.group1().yz())).with_z((self[e15] * other[e4315]) - (self[e25] * other[e4235])))
            .with_w((self[e41] * other[e15]) + (self[e42] * other[e25]) - (self[e45] * other[e45]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315])),
        )
    }
}
impl WeightExpansion<Line> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        8       12        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group0()) + (self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(0.0),
            // e1234
            -(self[e41] * other[e415]) - (self[e42] * other[e425]) - (self[e43] * other[e435]),
        )
    }
}
impl WeightExpansion<Motor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        3        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd       12       29        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e5] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g0_w * self[e1234]) - (right_anti_dual_g0_xyz[0] * self[e41]) - (right_anti_dual_g0_xyz[1] * self[e42]) - (right_anti_dual_g0_xyz[2] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())
                + (self.group0().yzx() * right_anti_dual_g1.zxy())
                - (self.group0().zxy() * right_anti_dual_g1.yzx()))
            .with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       23        0        0
    //    simd2        0        1        0      N/A
    //    simd3       12       20        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       32       45        0      N/A
    //  no simd       56       89        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (other[e41] * self[e15])
                    + (other[e42] * self[e25])
                    + (other[e43] * self[e35])
                    + (other[e23] * self[e23])
                    + (other[e31] * self[e31])
                    + (other[e12] * self[e12])
                    + (self[e1234] * other[e3215])
                    + (self[e3215] * other[e1234])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (self[e45] * other[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0[0]) * self.group2().xyz().with_w(self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0[0]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz(),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e45])))
                .with_w(0.0),
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group1().xyz()) + (right_anti_dual_g1_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group2().yzx()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g5 * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g0[0]) * self.group3().xyz())
                + (self.group0().yzx() * other.group8().zxy())
                + (other.group7().yzx() * self.group2().zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (self.group0().zxy() * other.group8().yzx())
                - (other.group7().zxy() * self.group2().yzx()))
            .with_w(right_anti_dual_g0[0] * self[e3215]),
            // e1234
            (right_anti_dual_g0[0] * self[e1234])
                - (right_anti_dual_g5[0] * self[e41])
                - (right_anti_dual_g5[1] * self[e42])
                - (right_anti_dual_g5[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<Plane> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       16       21        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0().zxy() * other.group0().yzx()) - (self.group0().yzx() * other.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + ((self.group2().yz() * other.group0().zx()) - (self.group2().zx() * other.group0().yz())).with_z(0.0))
                    .with_w(0.0),
        )
    }
}
impl WeightExpansion<Sphere> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        5        9        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        6       11        0      N/A
    //  no simd       19       32        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group2().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e45])))
                .with_w(0.0),
            // e235, e315, e125, e12345
            (Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[e1234]))
                + ((right_anti_dual_g0_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())).with_w(right_anti_dual_g0_xyz[0] * self[e4235]),
        )
    }
}
impl WeightExpansion<VersorEven> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       24       41        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0_w) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz()).with_w(
                (right_anti_dual_g0_w * self[e1234])
                    - (right_anti_dual_g0_xyz[0] * self[e23])
                    - (right_anti_dual_g0_xyz[1] * self[e31])
                    - (right_anti_dual_g0_xyz[2] * self[e12])
                    - (right_anti_dual_g1_xyz[0] * self[e41])
                    - (right_anti_dual_g1_xyz[1] * self[e42])
                    - (right_anti_dual_g1_xyz[2] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())
                + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy())
                + (self.group0().yzx() * right_anti_dual_g2.zxy())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx())
                - (self.group0().zxy() * right_anti_dual_g2.yzx()))
            .with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<VersorOdd> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        8        0        0
    //    simd3        5       10        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       13       19        0      N/A
    //  no simd       26       42        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e1234]) * self.group1().xyz()) + (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g3_xyz.yzx() * self.group0().zxy()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e45])))
                .with_w(0.0),
            // e235, e315, e125, e12345
            (self.group1().xyzx() * Simd32x3::from(other[e3215]).with_w(other[e23]))
                + ((right_anti_dual_g3_xyz.yzx() * self.group2().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group2().yzx())).with_w(
                    (right_anti_dual_g3_xyz[0] * self[e4235]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) + (self[e1234] * other[e3215])
                        - (right_anti_dual_g2_xyz[0] * self[e41])
                        - (right_anti_dual_g2_xyz[1] * self[e42])
                        - (right_anti_dual_g2_xyz[2] * self[e43])
                        - (self[e45] * other[e45]),
                ),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for DualNum {
    type Output = WeightExpansionInfixPartial<DualNum>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       15        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Circle> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e5]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * other.group1().xyz(),
        )
    }
}
impl WeightExpansion<CircleRotor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0() * Simd32x2::from(self[e5]).with_z(self[e5])).with_w(right_anti_dual_g2_w * self[e12345]),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * other.group1().xyz().with_w(right_anti_dual_g2_w),
        )
    }
}
impl WeightExpansion<Dipole> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<FlatPoint> for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([self[e5] * other[e45] * -1.0, 0.0]))
    }
}
impl WeightExpansion<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).with_w(self[e5] * other[e45] * -1.0),
        )
    }
}
impl WeightExpansion<Line> for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group0() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[e12345] * right_anti_dual_g0[3]),
            // e235, e315, e125, e5
            right_anti_dual_g0 * Simd32x4::from(self[e5]),
        )
    }
}
impl WeightExpansion<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        2        0      N/A
    //    simd4        0        5        0      N/A
    // Totals...
    // yes simd        1       11        0      N/A
    //  no simd        1       31        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (right_anti_dual_g0[0] * self[e12345]) - (self[e5] * other[e4])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group9().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group7() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[e5]) * other.group6().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group4().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, self[e5], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[e5]).with_z(1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<RoundPoint> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4] * -1.0)
    }
}
impl WeightExpansion<Sphere> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<VersorEven> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        1       10        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0().xyz() * Simd32x2::from(self[e5]).with_z(self[e5])).with_w((right_anti_dual_g0_w * self[e12345]) - (self[e5] * other[e4])),
            // e235, e315, e125, e5
            Simd32x4::from(self[e5]) * other.group1().xyz().with_w(right_anti_dual_g0_w),
        )
    }
}
impl WeightExpansion<VersorOdd> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[e5]) * other.group0().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for FlatPoint {
    type Output = WeightExpansionInfixPartial<FlatPoint>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Circle> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       14        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Dipole> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd       10       14        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz())).with_w(other[e45] * self[e45] * -1.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
                + ((other.group3().zx() * self.group0().yz()) - (other.group3().yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<FlatPoint> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e45] * self[e45] * -1.0)
    }
}
impl WeightExpansion<Flector> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        7       14        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[e45]) * other.group1().xyz().with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
                + ((self.group0().yz() * other.group1().zx()) - (self.group0().zx() * other.group1().yz())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<Line> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(self[e45]).xyz()).with_w(-(other[e415] * self[e15]) - (other[e425] * self[e25]) - (other[e435] * self[e35])),
        )
    }
}
impl WeightExpansion<Motor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       11        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g0_xyz * Simd32x4::from(self[e45]).xyz())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e15]) - (right_anti_dual_g0_xyz[1] * self[e25]) - (right_anti_dual_g0_xyz[2] * self[e35])),
        )
    }
}
impl WeightExpansion<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        4        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       15       33        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (self[e45] * other[e45])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group6().xyz()) + (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx())).with_w(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3        9        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[e45]) * other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from([
                (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
                (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
                (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
            ]),
        )
    }
}
impl WeightExpansion<Sphere> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        5        0      N/A
    // no simd        6       15        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e45])),
            // e235, e315, e125
            (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()),
        )
    }
}
impl WeightExpansion<VersorEven> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        3        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       14        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       17        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e45]))).with_w(self[e45] * other[e45] * -1.0),
            // e235, e315, e125, e5
            ((right_anti_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Flector {
    type Output = WeightExpansionInfixPartial<Flector>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        3        0      N/A
    // no simd        6        9        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       18        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g2_w) * self.group0(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group1().xyz()) + (other.group0().yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group0().yzx()))
            .with_w(right_anti_dual_g2_w * self[e3215]),
        )
    }
}
impl WeightExpansion<Dipole> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        2        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       14       17        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz()))
                .with_w((other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e1234] * self[e3215]) - (other[e45] * self[e45])),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
                + ((other.group3().zx() * self.group0().yz()) - (other.group3().yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl WeightExpansion<FlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e45] * self[e45] * -1.0)
    }
}
impl WeightExpansion<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd       10       13        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e45]).xyz() * other.group1().xyz())
                .with_w(-(other[e45] * self[e45]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125])),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, (other[e4315] * self[e15]) - (other[e4235] * self[e25]), 0.0])
                + ((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(self[e45]).xyz()).with_w(-(other[e415] * self[e15]) - (other[e425] * self[e25]) - (other[e435] * self[e35])),
        )
    }
}
impl WeightExpansion<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        5        0      N/A
    //  no simd        3       12        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group0().xyz())).with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       22       42        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (other[e41] * self[e15])
                    + (other[e42] * self[e25])
                    + (other[e43] * self[e35])
                    + (self[e3215] * other[e1234])
                    - (self[e45] * other[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0[0]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e45]) * other.group6().xyz()) + (other.group7().yzx() * self.group0().zxy())
                - (other.group7().zxy() * self.group0().yzx()))
            .with_w(right_anti_dual_g0[0] * self[e3215]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        9       12        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e45]).xyz() * other.group0().xyz()).with_w(-(self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125])),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, (self[e15] * other[e4315]) - (self[e25] * other[e4235]), 0.0])
                + ((self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<Sphere> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        6       16        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e45]))).with_w(right_anti_dual_g0_xyz[0] * self[e4235]),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorEven> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        3        4        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        9       18        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * other.group1().xyz())
                + (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()))
            .with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<VersorOdd> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        7       17        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e45])))
                .with_w((right_anti_dual_g3_xyz[0] * self[e4235]) - (self[e45] * other[e45])),
            // e235, e315, e125, e5
            ((right_anti_dual_g3_xyz.yzx() * self.group0().zxy()) - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Line {
    type Output = WeightExpansionInfixPartial<Line>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiDipoleInversion> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            -(right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(right_anti_dual_g0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        5        6        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        AntiScalar::from_groups(
            // e12345
            -(right_anti_dual_g1_xyz[0] * self[e415])
                - (right_anti_dual_g1_xyz[1] * self[e425])
                - (right_anti_dual_g1_xyz[2] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125]),
        )
    }
}
impl WeightExpansion<CircleRotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       13        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(right_anti_dual_g2_w) * self.group0()).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
            // e235, e315, e125, e5
            (Simd32x3::from(right_anti_dual_g2_w) * self.group1()).with_w(0.0),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        3        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       11        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((self.group0().yzx() * other.group3().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (self.group0().zxy() * other.group3().yzx()))
                .with_w(self[e235] * other[e4235] * -1.0),
        )
    }
}
impl WeightExpansion<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other[e12345] * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl WeightExpansion<Flector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3        8        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((self.group0().yzx() * other.group1().zxy()) - (self.group0().zxy() * other.group1().yzx())).with_w(self[e235] * other[e4235] * -1.0),
        )
    }
}
impl WeightExpansion<Line> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]))
    }
}
impl WeightExpansion<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        4        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        2       10        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e415]) - (right_anti_dual_g0_xyz[1] * self[e425]) - (right_anti_dual_g0_xyz[2] * self[e435])),
            // e235, e315, e125, e5
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1()).with_w(0.0),
        )
    }
}
impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       15        0      N/A
    //  no simd       11       31        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (self[e235] * other[e423])
                    - (self[e315] * other[e431])
                    - (self[e125] * other[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, right_anti_dual_g0[0], 0.0]) * (self.group0() * Simd32x2::from(right_anti_dual_g0[0]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0[0]) * self.group1(),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()))
                .with_w(right_anti_dual_g1_xyz[0] * self[e235]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        2        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        3        8        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((self.group0().yzx() * other.group0().zxy()) - (self.group0().zxy() * other.group0().yzx())).with_w(self[e235] * other[e4235] * -1.0),
        )
    }
}
impl WeightExpansion<Sphere> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       13        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()))
                .with_w(right_anti_dual_g0_xyz[0] * self[e235]),
        )
    }
}
impl WeightExpansion<VersorEven> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        9        0      N/A
    //  no simd        5       13        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0()).with_w(
                -(right_anti_dual_g0_xyz[0] * self[e235])
                    - (right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435]),
            ),
            // e235, e315, e125, e5
            (Simd32x3::from(right_anti_dual_g0_w) * self.group1()).with_w(0.0),
        )
    }
}
impl WeightExpansion<VersorOdd> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       13        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g3_xyz.yzx() * self.group0().zxy()) - (Simd32x3::from(other[e1234]) * self.group1()) - (right_anti_dual_g3_xyz.zxy() * self.group0().yzx()))
                .with_w(right_anti_dual_g3_xyz[0] * self[e235]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Motor {
    type Output = WeightExpansionInfixPartial<Motor>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(self[e5]).xyz()).with_w(other[e45] * self[e5] * -1.0),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        7        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        6        9        0      N/A
    //  no simd        6       13        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0() * Simd32x4::from(self[e5]).xyz()).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e4] * self[e5]),
            ),
            // e235, e315, e125, e5
            (right_anti_dual_g1_xyz * Simd32x4::from(self[e5]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
        )
    }
}
impl WeightExpansion<Circle> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd        5       12        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (other.group0() * Simd32x4::from(self[e5]).xyz()).with_w(
                -(right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ),
            // e235, e315, e125, e5
            (right_anti_dual_g1_xyz * Simd32x4::from(self[e5]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       15        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(right_anti_dual_g2_w * self[e12345]),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(right_anti_dual_g2_w * self[e5]),
        )
    }
}
impl WeightExpansion<Dipole> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(self[e5]).xyz()).with_w(other[e45] * self[e5] * -1.0),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        2        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd        9       22        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[e5]) * other.group3().xyz().with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (other[e4125] * self[e425]) - (other[e4315] * self[e435]),
                (other[e4235] * self[e435]) - (other[e4125] * self[e415]),
                (other[e4315] * self[e415]) - (other[e4235] * self[e425]),
            ]) + (Simd32x3::from(self[e5]) * other.group0())
                - (Simd32x3::from(other[e1234]) * self.group1().xyz()))
            .with_w(other[e4235] * self[e235] * -1.0),
        )
    }
}
impl WeightExpansion<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       10        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
        )
    }
}
impl WeightExpansion<FlatPoint> for Motor {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([other[e45] * self[e5] * -1.0, 0.0]))
    }
}
impl WeightExpansion<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        7        9        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self[e5]).xyz() * other.group1().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e4315] * self[e415]) - (other[e4235] * self[e425]), 0.0])
                + ((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_zw(0.0, 0.0),
        )
    }
}
impl WeightExpansion<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        4        0      N/A
    //  no simd        2        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(-(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435])),
            // e235, e315, e125, e5
            (other.group0() * Simd32x4::from(self[e5]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        6       15        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz()).with_w(
                (right_anti_dual_g0_w * self[e12345])
                    - (right_anti_dual_g0_xyz[0] * self[e415])
                    - (right_anti_dual_g0_xyz[1] * self[e425])
                    - (right_anti_dual_g0_xyz[2] * self[e435]),
            ),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())).with_w(right_anti_dual_g0_w * self[e5]),
        )
    }
}
impl WeightExpansion<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       13        0        0
    //    simd2        1        3        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       24        0      N/A
    //  no simd       22       44        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g5 = other.group6().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g0[0] * self[e12345])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (self[e5] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            right_anti_dual_g1 * Simd32x4::from(self[e5] * -1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group7())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (right_anti_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e5]) * other.group4())
                + ((right_anti_dual_g1.yz() * self.group0().zx()) - (right_anti_dual_g1.zx() * self.group0().yz()))
                    .with_z((right_anti_dual_g1[0] * self[e425]) - (right_anti_dual_g1[1] * self[e415]))
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()))
            .with_w(right_anti_dual_g1[0] * self[e235]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self[e5]).xyz() * other.group0().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            ((self.group0().yz() * other.group0().zx()) - (self.group0().zx() * other.group0().yz()))
                .with_zw((self[e415] * other[e4315]) - (self[e425] * other[e4235]), self[e235] * other[e4235] * -1.0),
        )
    }
}
impl WeightExpansion<RoundPoint> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4] * -1.0)
    }
}
impl WeightExpansion<Sphere> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd3        1        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       11        0      N/A
    //  no simd        6       18        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[e5] * -1.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g0[1] * self[e435]) - (right_anti_dual_g0[2] * self[e425]),
                (right_anti_dual_g0[2] * self[e415]) - (right_anti_dual_g0[0] * self[e435]),
                (right_anti_dual_g0[0] * self[e425]) - (right_anti_dual_g0[1] * self[e415]),
            ]) - (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()))
            .with_w(right_anti_dual_g0[0] * self[e235]),
        )
    }
}
impl WeightExpansion<VersorEven> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        1        3        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        2        6        0      N/A
    //  no simd        7       15        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x3::from(self[e5]).with_w(self[e12345])) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz()).with_w(0.0),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(right_anti_dual_g0[3] * self[e5]),
        )
    }
}
impl WeightExpansion<VersorOdd> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd       14       22        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g3 * Simd32x4::from(self[e5] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, right_anti_dual_g3[1] * self[e415] * -1.0, 0.0])
                + (right_anti_dual_g3.yzxx() * self.group0().zxy().with_w(self[e235]))
                + ((Simd32x3::from(self[e5]) * other.group0().xyz()) + -(right_anti_dual_g3.zx() * self.group0().yz()).with_z(0.0)
                    - (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz()))
                .with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for MultiVector {
    type Output = WeightExpansionInfixPartial<MultiVector>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       18        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       18       27        0      N/A
    //  no simd       26       45        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[scalar] * other[scalar]) + (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (self[e23] * right_anti_dual_g1[0])
                    - (self[e31] * right_anti_dual_g1[1])
                    - (self[e12] * right_anti_dual_g1[2])
                    - (right_anti_dual_g1[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group2().xyz(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (right_anti_dual_g1[0] * self[e2]) - (right_anti_dual_g1[1] * self[e1]), 0.0])
                + (((right_anti_dual_g1.yz() * self.group1().zx()) - (right_anti_dual_g1.zx() * self.group1().yz())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g1[3] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       20       26        0        0
    //    simd3       12       18        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       32       45        0      N/A
    //  no simd       56       84        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g2_w * self[e5])
                    + (right_anti_dual_g3_w * self[e4])
                    + (right_anti_dual_g3_xyz[0] * self[e1])
                    + (right_anti_dual_g3_xyz[1] * self[e2])
                    + (right_anti_dual_g3_xyz[2] * self[e3])
                    - (right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            right_anti_dual_g1_xyz * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()))
                .with_w(0.0),
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz.zxy() * self.group1().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (right_anti_dual_g3_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group5())
                + (right_anti_dual_g2_xyz.zxy() * self.group4().yzx())
                + (other.group0().yzx() * self.group3().zxy())
                - (right_anti_dual_g2_xyz.yzx() * self.group4().zxy())
                - (other.group0().zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g3_w * self[scalar]),
            // e1234
            (right_anti_dual_g2_w * self[scalar])
                - (right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1        8        0      N/A
    //  no simd        1       21        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e3215] * self[e1234]) + (other[scalar] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e3215] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, other[e3215], 0.0]) * (self.group4() * Simd32x2::from(other[e3215]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other[e3215]) * self.group5(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e3215]) * self.group7().with_w(self[e321]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        3        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       15       30        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(self[e423] * right_anti_dual_g0[0]) - (self[e431] * right_anti_dual_g0[1]) - (self[e412] * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * right_anti_dual_g0.xyz()) - (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g0[2] * self[e2]) - (right_anti_dual_g0[1] * self[e3]),
                (right_anti_dual_g0[0] * self[e3]) - (right_anti_dual_g0[2] * self[e1]),
                (right_anti_dual_g0[1] * self[e1]) - (right_anti_dual_g0[0] * self[e2]),
            ]),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group5()) + (self.group4().yzx() * right_anti_dual_g0.zxy()) - (self.group4().zxy() * right_anti_dual_g0.yzx()))
                .with_w(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       14       24        0      N/A
    //  no simd       22       39        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1_w * self[e4]) + (right_anti_dual_g1_xyz[0] * self[e1]) + (right_anti_dual_g1_xyz[1] * self[e2]) + (right_anti_dual_g1_xyz[2] * self[e3])
                    - (self[e423] * right_anti_dual_g0[0])
                    - (self[e431] * right_anti_dual_g0[1])
                    - (self[e412] * right_anti_dual_g0[2])
                    - (right_anti_dual_g0[3] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * right_anti_dual_g0.xyz()) - (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g0[2] * self[e2]) - (right_anti_dual_g0[1] * self[e3]),
                (right_anti_dual_g0[0] * self[e3]) - (right_anti_dual_g0[2] * self[e1]),
                (right_anti_dual_g0[1] * self[e1]) - (right_anti_dual_g0[0] * self[e2]),
            ]),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group5()) + (self.group4().yzx() * right_anti_dual_g0.zxy())
                - (self.group4().zxy() * right_anti_dual_g0.yzx()))
            .with_w(right_anti_dual_g1_w * self[scalar]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        2        7        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       11       31        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_anti_dual_g0[0] * self[e23])
                    - (right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (right_anti_dual_g0 * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            right_anti_dual_g1 * Simd32x3::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1 * Simd32x3::from(self[e4])) + (right_anti_dual_g0.yzx() * self.group1().zxy()) - (right_anti_dual_g0.zxy() * self.group1().yzx())).with_w(0.0),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd3        4        6        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       22       38        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[scalar] * other[scalar])
                    + (self[e41] * other[e15])
                    + (self[e42] * other[e25])
                    + (self[e43] * other[e35])
                    + (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (other[e3215] * self[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group4()) - (Simd32x3::from(self[scalar]) * other.group0().xyz())).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(other[e3215]) * self.group5()) - (Simd32x3::from(self[scalar]) * other.group1().xyz()),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (other[e12] * self[e2]) - (other[e31] * self[e3]),
                (other[e23] * self[e3]) - (other[e12] * self[e1]),
                (other[e31] * self[e1]) - (other[e23] * self[e2]),
            ]) + (Simd32x3::from(other[e3215]) * self.group7())
                - (Simd32x3::from(self[e4]) * other.group1().xyz()))
            .with_w(other[e3215] * self[e321]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        6        0      N/A
    //  no simd        3        9        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g0[3] * self[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0       12        0      N/A
    //  no simd        0       33        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(right_anti_dual_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e5
            right_anti_dual_g0 * self[e5],
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual_g0) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(right_anti_dual_g0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(right_anti_dual_g0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(right_anti_dual_g0) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group9(),
            // e1234
            right_anti_dual_g0 * self[e1234],
        )
    }
}
impl WeightExpansion<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       14       17        0        0
    //    simd3       11       17        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       25       35        0      N/A
    //  no simd       47       72        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group2().with_w(right_anti_dual_g1_w),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            right_anti_dual_g1_xyz * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz())).with_w(0.0),
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group1().zxy()) - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group1().yzx()) - (other.group2().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group5())
                + (other.group0().yzx() * self.group3().zxy())
                + (other.group2().zxy() * self.group4().yzx())
                - (other.group0().zxy() * self.group3().yzx())
                - (other.group2().yzx() * self.group4().zxy()))
            .with_w(0.0),
            // e1234
            -(right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       17       26        0        0
    //    simd3       18       25        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       35       52        0      N/A
    //  no simd       71      105        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g2_w * self[scalar],
                (right_anti_dual_g2_w * self[e12345])
                    - (right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g2_w) * self.group1(),
            // e5
            right_anti_dual_g2_w * self[e5],
            // e15, e25, e35, e45
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group3().xyz()))
                .with_w((right_anti_dual_g1_w * self[scalar]) + (right_anti_dual_g2_w * self[e45])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g2_w) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            (right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group5()),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group6().xyz()) + (Simd32x3::from(self[e5]) * other.group0())
                - (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()))
            .with_w(right_anti_dual_g2_w * self[e321]),
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group7()) + (other.group0().yzx() * self.group1().zxy())
                - (other.group0().zxy() * self.group1().yzx()),
            // e235, e315, e125
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group8()) + (right_anti_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g1_w) * self.group5())
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group9().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group4().yzx())
                + (other.group0().yzx() * self.group3().zxy())
                - (right_anti_dual_g2_xyz.yzx() * self.group4().zxy())
                - (other.group0().zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g2_w * self[e3215]),
            // e1234
            (right_anti_dual_g2_w * self[e1234])
                - (right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12]),
        )
    }
}
impl WeightExpansion<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       16        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        6        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       17       26        0      N/A
    //  no simd       25       46        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g2[0] * self[e41])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43])
                    - (self[e23] * right_anti_dual_g1[0])
                    - (self[e31] * right_anti_dual_g1[1])
                    - (self[e12] * right_anti_dual_g1[2])
                    - (right_anti_dual_g1[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e235, e315, e125
            right_anti_dual_g2 * Simd32x3::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (right_anti_dual_g1[0] * self[e2]) - (right_anti_dual_g1[1] * self[e1]), 0.0])
                + ((right_anti_dual_g2 * Simd32x3::from(self[e4])) + ((right_anti_dual_g1.yz() * self.group1().zx()) - (right_anti_dual_g1.zx() * self.group1().yz())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5])))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (right_anti_dual_g1[3] * self[e4]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       34       46        0        0
    //    simd2        2        3        0      N/A
    //    simd3       13       19        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       51       71        0      N/A
    //  no simd       85      121        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e41] * other[e15])
                    + (self[e42] * other[e25])
                    + (self[e43] * other[e35])
                    + (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (other[e1234] * self[e3215])
                    + (other[e3215] * self[e1234])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (other[e45] * self[e45])
                    - (other[e4235] * self[e4235])
                    - (other[e4315] * self[e4315])
                    - (other[e4125] * self[e4125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            (other.group3() * Simd32x3::from(self[e5]).with_w(self[e4])) + (Simd32x3::from(other[e3215]) * self.group1().xyz()).with_w(0.0),
            // e41, e42, e43
            -(Simd32x3::from(other[e1234]) * self.group1().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e4315] * self[e3]) - (other[e4125] * self[e2]),
                (other[e4125] * self[e1]) - (other[e4235] * self[e3]),
                (other[e4235] * self[e2]) - (other[e4315] * self[e1]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4()) + (Simd32x3::from(self[e45]) * other.group3().xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(self[scalar] * other[e45]),
            // e423, e431, e412
            (right_anti_dual_g0 * Simd32x3::from(self[scalar])) + (Simd32x3::from(other[e1234]) * self.group5()) + (self.group4().zxy() * other.group3().yzx())
                - (self.group4().yzx() * other.group3().zxy()),
            // e235, e315, e125
            Simd32x3::from([
                (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
                (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
                (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
            ]) + (Simd32x3::from(other[e3215]) * self.group5())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()),
            // e4235, e4315, e4125, e3215
            (self.group1().yzxy() * other.group1().zxy().with_w(other[e25]))
                + ((Simd32x3::from(other[e3215]) * self.group7())
                    + ((other.group3().zx() * self.group6().yz()) - (other.group1().yz() * self.group1().zx()) - (other.group3().yz() * self.group6().zx()))
                        .with_z((other[e4315] * self[e415]) - (other[e23] * self[e2]) - (other[e4235] * self[e425]))
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(other[e1234]) * self.group8())
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(
                    (other[e15] * self[e1]) + (other[e35] * self[e3])
                        - (self[e235] * other[e4235])
                        - (self[e315] * other[e4315])
                        - (self[e125] * other[e4125])
                        - (other[e45] * self[e5]),
                ),
            // e1234
            (right_anti_dual_g0[0] * self[e1])
                + (right_anti_dual_g0[1] * self[e2])
                + (right_anti_dual_g0[2] * self[e3])
                + (self[e423] * other[e4235])
                + (self[e431] * other[e4315])
                + (self[e412] * other[e4125])
                + (other[e45] * self[e4])
                - (other[e1234] * self[e321]),
        )
    }
}
impl WeightExpansion<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2       18        0        0
    //    simd3        0        6        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        2       27        0      N/A
    //  no simd        2       48        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e12345] * self[scalar] * -1.0, -(other[e5] * self[e4]) - (other[e12345] * self[e12345])]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e5
            other[e12345] * self[e5] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[e12345] * -1.0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[e12345] * -1.0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[e12345] * -1.0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[e12345] * -1.0) * self.group8(),
            // e4235, e4315, e4125, e3215
            (self.group9().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e5] * self[scalar]) - (other[e12345] * self[e3215])),
            // e1234
            other[e12345] * self[e1234] * -1.0,
        )
    }
}
impl WeightExpansion<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       11        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        6       14        0      N/A
    //  no simd        6       20        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (other[e45] * self[e45])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e45]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5])),
            // e1234
            other[e45] * self[e4],
        )
    }
}
impl WeightExpansion<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       37        0        0
    //    simd2        0        1        0      N/A
    //    simd3        6       12        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       30       53        0      N/A
    //  no simd       48       87        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (other[e3215] * self[e1234])
                    - (other[e45] * self[e45])
                    - (other[e4235] * self[e4235])
                    - (other[e4315] * self[e4315])
                    - (other[e4125] * self[e4125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group1().xyz() * Simd32x2::from(self[scalar] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(other[e3215] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from([
                (other[e4315] * self[e3]) - (other[e4125] * self[e2]),
                (other[e4125] * self[e1]) - (other[e4235] * self[e3]),
                (other[e4235] * self[e2]) - (other[e4315] * self[e1]),
            ]),
            // e415, e425, e435, e321
            (other.group1().xyzx() * Simd32x3::from(self[e45]).with_w(self[e23]))
                + (Simd32x3::from(other[e3215]) * self.group4()).with_w((self[scalar] * other[e45]) + (self[e31] * other[e4315]) + (self[e12] * other[e4125])),
            // e423, e431, e412
            (self.group4().zxy() * other.group1().yzx()) - (self.group4().yzx() * other.group1().zxy()),
            // e235, e315, e125
            Simd32x3::from([
                (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
                (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
                (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
            ]) + (Simd32x3::from(other[e3215]) * self.group5())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group7()) + (other.group1().zx() * self.group6().yz()).with_z(other[e4315] * self[e415])
                - (Simd32x3::from(self[e4]) * other.group0().xyz()))
            .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) - (self[e235] * other[e4235]) - (self[e125] * other[e4125]) - (other[e45] * self[e5]))
                - (other.group1().yzxy() * self.group6().zxy().with_w(self[e315])),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       12        0        0
    //    simd3        4       10        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       13       23        0      N/A
    //  no simd       21       46        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(other[e415] * self[e415])
                    - (other[e425] * self[e425])
                    - (other[e435] * self[e435])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group1() * Simd32x2::from(self[scalar]).with_z(1.0)).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[e415] * self[e1]) - (other[e425] * self[e2]) - (other[e435] * self[e3])),
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group1().yzx()) - (other.group1().yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group0()) + (other.group1().zxy() * self.group4().yzx()) - (other.group1().yzx() * self.group4().zxy())).with_w(0.0),
            // e1234
            -(other[e415] * self[e41]) - (other[e425] * self[e42]) - (other[e435] * self[e43]),
        )
    }
}
impl WeightExpansion<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       19        0        0
    //    simd3        9       16        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       20       37        0      N/A
    //  no simd       41       75        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self[scalar] * right_anti_dual_g0[3],
                (right_anti_dual_g1_w * self[e4]) + (self[e12345] * right_anti_dual_g0[3])
                    - (right_anti_dual_g1_xyz[0] * self[e423])
                    - (right_anti_dual_g1_xyz[1] * self[e431])
                    - (right_anti_dual_g1_xyz[2] * self[e412])
                    - (right_anti_dual_g0[0] * self[e415])
                    - (right_anti_dual_g0[1] * self[e425])
                    - (right_anti_dual_g0[2] * self[e435]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[3]) * self.group1(),
            // e5
            right_anti_dual_g0[3] * self[e5],
            // e15, e25, e35, e45
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group3().xyz())).with_w(right_anti_dual_g0[3] * self[e45]),
            // e41, e42, e43
            Simd32x3::from(right_anti_dual_g0[3]) * self.group4(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * right_anti_dual_g0.xyz()) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group5()),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group6().xyz())).with_w(right_anti_dual_g0[3] * self[e321]),
            // e423, e431, e412
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group7()) + (Simd32x3::from(self[e4]) * right_anti_dual_g0.xyz()),
            // e235, e315, e125
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group8()) + (Simd32x3::from(self[e5]) * right_anti_dual_g0.xyz()) + (right_anti_dual_g1_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g1_xyz.yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g0 * Simd32x3::from(self[e45]).with_w(self[e3215]))
                + ((Simd32x3::from(right_anti_dual_g0[3]) * self.group9().xyz()) + (right_anti_dual_g1_xyz.zxy() * self.group4().yzx())
                    - (right_anti_dual_g1_xyz.yzx() * self.group4().zxy()))
                .with_w(right_anti_dual_g1_w * self[scalar]),
            // e1234
            (right_anti_dual_g0[3] * self[e1234]) - (self[e41] * right_anti_dual_g0[0]) - (self[e42] * right_anti_dual_g0[1]) - (self[e43] * right_anti_dual_g0[2]),
        )
    }
}
impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       65       80        0        0
    //    simd2        3        5        0      N/A
    //    simd3       31       38        0      N/A
    //    simd4        8       10        0      N/A
    // Totals...
    // yes simd      107      133        0      N/A
    //  no simd      196      244        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6 = (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9 = other.group1().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g10 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g0[0] * self[scalar],
                (right_anti_dual_g10 * self[e5])
                    + (right_anti_dual_g0[0] * self[e12345])
                    + (right_anti_dual_g0[1] * self[scalar])
                    + (right_anti_dual_g1[0] * self[e4235])
                    + (right_anti_dual_g1[1] * self[e4315])
                    + (right_anti_dual_g1[2] * self[e4125])
                    + (right_anti_dual_g1[3] * self[e3215])
                    + (right_anti_dual_g9[0] * self[e1])
                    + (right_anti_dual_g9[1] * self[e2])
                    + (right_anti_dual_g9[2] * self[e3])
                    + (right_anti_dual_g9[3] * self[e4])
                    + (other[e3215] * self[e1234])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (self[e23] * right_anti_dual_g6[0])
                    - (self[e31] * right_anti_dual_g6[1])
                    - (self[e12] * right_anti_dual_g6[2])
                    - (self[e423] * right_anti_dual_g3[0])
                    - (self[e431] * right_anti_dual_g3[1])
                    - (self[e412] * right_anti_dual_g3[2])
                    - (right_anti_dual_g3[3] * self[e321])
                    - (right_anti_dual_g6[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            (right_anti_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0[0]) * self.group1()),
            // e5
            (right_anti_dual_g0[0] * self[e5]) + (self[scalar] * other[e3215]),
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0[0]) * self.group3()) + (Simd32x4::from(other[e3215]) * self.group1())
                - (right_anti_dual_g1 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g0[0]) * self.group4()) + (Simd32x3::from(self[scalar]) * other.group7()) + (Simd32x3::from(self[e4]) * right_anti_dual_g1.xyz())
                - (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g1[2] * self[e2]) - (right_anti_dual_g1[1] * self[e3]),
                (right_anti_dual_g1[0] * self[e3]) - (right_anti_dual_g1[2] * self[e1]),
                (right_anti_dual_g1[1] * self[e1]) - (right_anti_dual_g1[0] * self[e2]),
            ]) + (right_anti_dual_g5 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g0[0]) * self.group5()),
            // e415, e425, e435, e321
            (right_anti_dual_g6 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(right_anti_dual_g0[0]) * self.group6())
                + ((Simd32x3::from(right_anti_dual_g1[3]) * self.group3().xyz())
                    + (Simd32x3::from(other[e3215]) * self.group4())
                    + (Simd32x3::from(self[e4]) * right_anti_dual_g3.xyz())
                    + (Simd32x3::from(self[e5]) * other.group7())
                    - (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                    - (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz()))
                .with_w(0.0),
            // e423, e431, e412
            (right_anti_dual_g5 * Simd32x3::from(self[e4]))
                + (right_anti_dual_g7 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g0[0]) * self.group7())
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group5())
                + (other.group7().yzx() * self.group1().zxy())
                + (self.group4().yzx() * right_anti_dual_g1.zxy())
                - (other.group7().zxy() * self.group1().yzx())
                - (self.group4().zxy() * right_anti_dual_g1.yzx()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g1[1] * self[e35]) + (right_anti_dual_g3[2] * self[e2]) - (right_anti_dual_g1[2] * self[e25]) - (right_anti_dual_g3[1] * self[e3]),
                (right_anti_dual_g1[2] * self[e15]) + (right_anti_dual_g3[0] * self[e3]) - (right_anti_dual_g1[0] * self[e35]) - (right_anti_dual_g3[2] * self[e1]),
                (right_anti_dual_g1[0] * self[e25]) + (right_anti_dual_g3[1] * self[e1]) - (right_anti_dual_g1[1] * self[e15]) - (right_anti_dual_g3[0] * self[e2]),
            ]) + (right_anti_dual_g5 * Simd32x3::from(self[e5]))
                + (right_anti_dual_g8 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g0[0]) * self.group8())
                + (Simd32x3::from(other[e3215]) * self.group5()),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g9 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(right_anti_dual_g0[0]) * self.group9())
                + ((right_anti_dual_g5 * Simd32x3::from(self[e45]))
                    + (right_anti_dual_g8 * Simd32x3::from(self[e4]))
                    + (Simd32x3::from(right_anti_dual_g3[3]) * self.group5())
                    + (Simd32x3::from(other[e3215]) * self.group7())
                    + (other.group7().yzx() * self.group3().zxy())
                    + (self.group4().yzx() * right_anti_dual_g3.zxy())
                    + ((right_anti_dual_g1.yz() * self.group6().zx()) + (right_anti_dual_g6.yz() * self.group1().zx())
                        - (right_anti_dual_g1.zx() * self.group6().yz())
                        - (right_anti_dual_g6.zx() * self.group1().yz()))
                    .with_z((right_anti_dual_g1[0] * self[e425]) + (right_anti_dual_g6[0] * self[e2]) - (right_anti_dual_g1[1] * self[e415]) - (right_anti_dual_g6[1] * self[e1]))
                    - (right_anti_dual_g7 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(right_anti_dual_g1[3]) * self.group8())
                    - (other.group7().zxy() * self.group3().yzx())
                    - (self.group4().zxy() * right_anti_dual_g3.yzx()))
                .with_w((self[e235] * right_anti_dual_g1[0]) + (self[e315] * right_anti_dual_g1[1]) + (self[e125] * right_anti_dual_g1[2]) + (other[e3215] * self[e321])),
            // e1234
            (right_anti_dual_g10 * self[scalar])
                + (right_anti_dual_g0[0] * self[e1234])
                + (right_anti_dual_g7[0] * self[e1])
                + (right_anti_dual_g7[1] * self[e2])
                + (right_anti_dual_g7[2] * self[e3])
                + (right_anti_dual_g6[3] * self[e4])
                - (right_anti_dual_g5[0] * self[e41])
                - (right_anti_dual_g5[1] * self[e42])
                - (right_anti_dual_g5[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12])
                - (self[e423] * right_anti_dual_g1[0])
                - (self[e431] * right_anti_dual_g1[1])
                - (self[e412] * right_anti_dual_g1[2])
                - (right_anti_dual_g1[3] * self[e321]),
        )
    }
}
impl WeightExpansion<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       25        0        0
    //    simd2        1        2        0      N/A
    //    simd3        5       11        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       19       39        0      N/A
    //  no simd       33       66        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other[e3215] * self[e1234]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, self[scalar], 0.0]) * (other.group0().xyz() * Simd32x2::from(self[scalar] * -1.0).with_z(1.0) * Simd32x3::from([1.0, 1.0, -1.0])).with_w(0.0),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz())).with_w(self[e4] * other[e3215]),
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group0().xyz(),
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e4315]) - (self[e2] * other[e4125]),
                (self[e1] * other[e4125]) - (self[e3] * other[e4235]),
                (self[e2] * other[e4235]) - (self[e1] * other[e4315]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(self[e45]) * other.group0().xyz()) + (Simd32x3::from(other[e3215]) * self.group4())).with_w(0.0),
            // e423, e431, e412
            (self.group4().zxy() * other.group0().yzx()) - (self.group4().yzx() * other.group0().zxy()),
            // e235, e315, e125
            Simd32x3::from([
                (self[e25] * other[e4125]) - (self[e35] * other[e4315]),
                (self[e35] * other[e4235]) - (self[e15] * other[e4125]),
                (self[e15] * other[e4315]) - (self[e25] * other[e4235]),
            ]) + (Simd32x3::from(other[e3215]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e415] * other[e4315]) - (self[e425] * other[e4235]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group7()) + ((self.group6().yz() * other.group0().zx()) - (self.group6().zx() * other.group0().yz())).with_z(0.0))
                    .with_w(0.0),
            // e1234
            (self[e423] * other[e4235]) + (self[e431] * other[e4315]) + (self[e412] * other[e4125]),
        )
    }
}
impl WeightExpansion<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       12        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e5] * -1.0);
        let right_anti_dual_g1 = other[e4] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1 * self[e5])
                    + (right_anti_dual_g0[0] * self[e1])
                    + (right_anti_dual_g0[1] * self[e2])
                    + (right_anti_dual_g0[2] * self[e3])
                    + (right_anti_dual_g0[3] * self[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e1234
            right_anti_dual_g1 * self[scalar],
        )
    }
}
impl WeightExpansion<Scalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar])
    }
}
impl WeightExpansion<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       13       25        0        0
    //    simd2        0        1        0      N/A
    //    simd3        8       12        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       23       42        0      N/A
    //  no simd       45       79        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g0[0] * self[e4235])
                    + (right_anti_dual_g0[1] * self[e4315])
                    + (right_anti_dual_g0[2] * self[e4125])
                    + (right_anti_dual_g0[3] * self[e3215])
                    + (other[e3215] * self[e1234]),
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]) * self.group1()) - (right_anti_dual_g0 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_anti_dual_g0.xyz()) - (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g0[2] * self[e2]) - (right_anti_dual_g0[1] * self[e3]),
                (right_anti_dual_g0[0] * self[e3]) - (right_anti_dual_g0[2] * self[e1]),
                (right_anti_dual_g0[1] * self[e1]) - (right_anti_dual_g0[0] * self[e2]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g0.xyz()))
            .with_w(0.0),
            // e423, e431, e412
            (Simd32x3::from(right_anti_dual_g0[3]) * self.group5()) + (self.group4().yzx() * right_anti_dual_g0.zxy()) - (self.group4().zxy() * right_anti_dual_g0.yzx()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g0[1] * self[e35]) - (right_anti_dual_g0[2] * self[e25]),
                (right_anti_dual_g0[2] * self[e15]) - (right_anti_dual_g0[0] * self[e35]),
                (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group5()),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g0.yzxy() * self.group6().zxy().with_w(self[e315]))
                + ((Simd32x3::from(other[e3215]) * self.group7()) + -(right_anti_dual_g0.zx() * self.group6().yz()).with_z(right_anti_dual_g0[1] * self[e415] * -1.0)
                    - (Simd32x3::from(right_anti_dual_g0[3]) * self.group8()))
                .with_w(self[e235] * right_anti_dual_g0[0]),
            // e1234
            -(self[e423] * right_anti_dual_g0[0]) - (self[e431] * right_anti_dual_g0[1]) - (self[e412] * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * self[e321]),
        )
    }
}
impl WeightExpansion<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       22       32        0        0
    //    simd3       17       23        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       41       59        0      N/A
    //  no simd       81      117        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e4] * -1.0;
        let right_anti_dual_g3_xyz = other.group3().xyz();
        let right_anti_dual_g3_w = other[e5] * -1.0;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g0_w * self[scalar],
                (right_anti_dual_g0_w * self[e12345])
                    + (right_anti_dual_g2_w * self[e5])
                    + (right_anti_dual_g3_w * self[e4])
                    + (right_anti_dual_g3_xyz[0] * self[e1])
                    + (right_anti_dual_g3_xyz[1] * self[e2])
                    + (right_anti_dual_g3_xyz[2] * self[e3])
                    - (right_anti_dual_g1_w * self[e321])
                    - (right_anti_dual_g0_xyz[0] * self[e235])
                    - (right_anti_dual_g0_xyz[1] * self[e315])
                    - (right_anti_dual_g0_xyz[2] * self[e125])
                    - (right_anti_dual_g1_xyz[0] * self[e415])
                    - (right_anti_dual_g1_xyz[1] * self[e425])
                    - (right_anti_dual_g1_xyz[2] * self[e435])
                    - (right_anti_dual_g2_xyz[0] * self[e423])
                    - (right_anti_dual_g2_xyz[1] * self[e431])
                    - (right_anti_dual_g2_xyz[2] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group1(),
            // e5
            right_anti_dual_g0_w * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_anti_dual_g0_w) * self.group3()) + (Simd32x4::from(self[scalar]) * right_anti_dual_g2_xyz.with_w(right_anti_dual_g1_w)),
            // e41, e42, e43
            (right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group4()),
            // e23, e31, e12
            (right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group5()),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e4]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group6().xyz())
                - (Simd32x3::from(right_anti_dual_g1_w) * self.group1().xyz()))
            .with_w(right_anti_dual_g0_w * self[e321]),
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group7()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())
                - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()),
            // e235, e315, e125
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group8()) + (right_anti_dual_g2_xyz.zxy() * self.group1().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group1().zxy()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(right_anti_dual_g0_w) * self.group9())
                + ((right_anti_dual_g1_xyz * Simd32x3::from(self[e45]))
                    + (right_anti_dual_g3_xyz * Simd32x3::from(self[scalar]))
                    + (Simd32x3::from(right_anti_dual_g1_w) * self.group5())
                    + (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())
                    + (right_anti_dual_g2_xyz.zxy() * self.group4().yzx())
                    - (right_anti_dual_g0_xyz.zxy() * self.group3().yzx())
                    - (right_anti_dual_g2_xyz.yzx() * self.group4().zxy()))
                .with_w(right_anti_dual_g3_w * self[scalar]),
            // e1234
            (right_anti_dual_g0_w * self[e1234]) + (right_anti_dual_g2_w * self[scalar])
                - (right_anti_dual_g0_xyz[0] * self[e23])
                - (right_anti_dual_g0_xyz[1] * self[e31])
                - (right_anti_dual_g0_xyz[2] * self[e12])
                - (right_anti_dual_g1_xyz[0] * self[e41])
                - (right_anti_dual_g1_xyz[1] * self[e42])
                - (right_anti_dual_g1_xyz[2] * self[e43]),
        )
    }
}
impl WeightExpansion<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       33       45        0        0
    //    simd2        2        3        0      N/A
    //    simd3       13       18        0      N/A
    //    simd4        2        4        0      N/A
    // Totals...
    // yes simd       50       70        0      N/A
    //  no simd       84      121        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self[scalar] * other[scalar])
                    + (self[e23] * other[e23])
                    + (self[e31] * other[e31])
                    + (self[e12] * other[e12])
                    + (right_anti_dual_g3[0] * self[e4235])
                    + (right_anti_dual_g3[1] * self[e4315])
                    + (right_anti_dual_g3[2] * self[e4125])
                    + (right_anti_dual_g3[3] * self[e3215])
                    + (self[e15] * other[e41])
                    + (self[e25] * other[e42])
                    + (self[e35] * other[e43])
                    + (other[e3215] * self[e1234])
                    - (right_anti_dual_g2_xyz[0] * self[e41])
                    - (right_anti_dual_g2_xyz[1] * self[e42])
                    - (right_anti_dual_g2_xyz[2] * self[e43])
                    - (self[e45] * other[e45]),
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g3 * Simd32x4::from(self[scalar]),
            // e5
            self[scalar] * other[e3215],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]) * self.group1()) - (right_anti_dual_g3 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_anti_dual_g3.xyz()) - (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g3[2] * self[e2]) - (right_anti_dual_g3[1] * self[e3]),
                (right_anti_dual_g3[0] * self[e3]) - (right_anti_dual_g3[2] * self[e1]),
                (right_anti_dual_g3[1] * self[e1]) - (right_anti_dual_g3[0] * self[e2]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group3().xyz()) + (Simd32x3::from(other[e3215]) * self.group4())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g3.xyz()))
            .with_w(self[scalar] * other[e45]),
            // e423, e431, e412
            (Simd32x3::from(right_anti_dual_g3[3]) * self.group5()) + (self.group4().yzx() * right_anti_dual_g3.zxy())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz())
                - (self.group4().zxy() * right_anti_dual_g3.yzx()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g3[1] * self[e35]) - (right_anti_dual_g3[2] * self[e25]),
                (right_anti_dual_g3[2] * self[e15]) - (right_anti_dual_g3[0] * self[e35]),
                (right_anti_dual_g3[0] * self[e25]) - (right_anti_dual_g3[1] * self[e15]),
            ]) + (right_anti_dual_g2_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(other[e3215]) * self.group5()),
            // e4235, e4315, e4125, e3215
            (right_anti_dual_g3.yzxz() * self.group6().zxy().with_w(self[e125]))
                + ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4]))
                    + (Simd32x3::from(other[e3215]) * self.group7())
                    + (Simd32x3::from(self[e5]) * other.group0().xyz())
                    + ((self.group1().yz() * other.group1().zx()) - (right_anti_dual_g3.zx() * self.group6().yz()) - (self.group1().zx() * other.group1().yz()))
                        .with_z((self[e1] * other[e31]) - (right_anti_dual_g3[1] * self[e415]) - (self[e2] * other[e23]))
                    - (Simd32x3::from(right_anti_dual_g3[3]) * self.group8()))
                .with_w((self[e235] * right_anti_dual_g3[0]) + (self[e315] * right_anti_dual_g3[1]) + (self[e321] * other[e3215]) - (other[e45] * self[e5])),
            // e1234
            (self[e4] * other[e45])
                - (self[e423] * right_anti_dual_g3[0])
                - (self[e431] * right_anti_dual_g3[1])
                - (self[e412] * right_anti_dual_g3[2])
                - (right_anti_dual_g3[3] * self[e321])
                - (self[e1] * other[e41])
                - (self[e2] * other[e42])
                - (self[e3] * other[e43]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Plane {
    type Output = WeightExpansionInfixPartial<Plane>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<CircleRotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<DipoleInversion> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightExpansion<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<Flector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]))
    }
}
impl WeightExpansion<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       12        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1_xyz[0] * self[e4235]) + (right_anti_dual_g1_xyz[1] * self[e4315]) + (right_anti_dual_g1_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e1234
            0.0,
        )
    }
}
impl WeightExpansion<Plane> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]))
    }
}
impl WeightExpansion<Sphere> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e4235]) + (right_anti_dual_g0_xyz[1] * self[e4315]) + (right_anti_dual_g0_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]),
        )
    }
}
impl WeightExpansion<VersorEven> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        5        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[e12345] * -1.0) * self.group0())
    }
}
impl WeightExpansion<VersorOdd> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        3        5        0      N/A
    //  no simd        3        7        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g3_xyz[0] * self[e4235]) + (right_anti_dual_g3_xyz[1] * self[e4315]) + (right_anti_dual_g3_xyz[2] * self[e4125]) + (self[e3215] * other[e1234]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for RoundPoint {
    type Output = WeightExpansionInfixPartial<RoundPoint>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for RoundPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        6        9        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd       18       28        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0),
            // e235, e315, e125, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
                .with_w(other[e1] * self[e1]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for RoundPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        1        0      N/A
    // no simd        0        4        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[e3215]) * self.group0())
    }
}
impl WeightExpansion<AntiFlatPoint> for RoundPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Line::from_groups(
            // e415, e425, e435
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()),
            // e235, e315, e125
            (right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy()),
        )
    }
}
impl WeightExpansion<AntiFlector> for RoundPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        5        0      N/A
    //  no simd        6       13        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz())).with_w(other[e1] * self[e1]),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for RoundPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0.yzx() * self.group0().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_anti_dual_g0.zxy() * self.group0().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for RoundPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd       10       13        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group0().zx() * self.group0().yz()) - (other.group0().yz() * self.group0().zx())).with_z(0.0) - (Simd32x3::from(self[e4]) * other.group1().xyz()))
                    .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiPlane> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (other[e5] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e5
            right_anti_dual_g0 * self[e5],
        )
    }
}
impl WeightExpansion<Circle> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6        9        0      N/A
    // no simd       18       27        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        Circle::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group2()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0),
            // e235, e315, e125
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group0().yzx()) - (other.group2().yzx() * self.group0().zxy()),
        )
    }
}
impl WeightExpansion<CircleRotor> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       18       33        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group0().zxy()) - (other.group0().zxy() * self.group0().yzx()),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0())).with_w(0.0),
            // e235, e315, e125, e4
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
                .with_w(right_anti_dual_g2_w * self[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(right_anti_dual_g2_w) * self.group0().xyz().with_w(self[e5]),
        )
    }
}
impl WeightExpansion<Dipole> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        7        0        0
    //    simd2        1        3        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       18        0      N/A
    //  no simd       23       38        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            -(Simd32x3::from(other[e1234]) * self.group0().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz()),
            // e23, e31, e12, e45
            (other.group3().yzxw() * self.group0().zxyw()) + -(other.group3().zx() * self.group0().yz()).with_zw(other[e4315] * self[e1] * -1.0, other[e1234] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group3().xyz())).with_w(right_anti_dual_g0[0] * self[e1]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group0().yz()) - (other.group1().yz() * self.group0().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<DualNum> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0        9        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(other[e5] * self[e4] * -1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(other[e12345] * self[e5] * -1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
        )
    }
}
impl WeightExpansion<FlatPoint> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5])),
            // e1234
            other[e45] * self[e4],
        )
    }
}
impl WeightExpansion<Flector> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        9        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        5        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        9       28        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((other.group1().yz() * self.group0().zx()) - (other.group1().zx() * self.group0().yz()))
                .with_zw((other[e4235] * self[e2]) - (other[e4315] * self[e1]), other[e3215] * self[e4]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group1().xyz())).with_w(other[e45] * self[e4]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5])),
        )
    }
}
impl WeightExpansion<Line> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[e415] * self[e1]) - (other[e425] * self[e2]) - (other[e435] * self[e3])),
            // e235, e315, e125
            (Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy()),
        )
    }
}
impl WeightExpansion<Motor> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        7        0        0
    //    simd3        2        5        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4       13        0      N/A
    //  no simd        8       26        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_anti_dual_g0_xyz * Simd32x4::from(self[e4]).xyz()).with_w(other[e5] * self[e4] * -1.0),
            // e415, e425, e435, e321
            (right_anti_dual_g1_xyz * Simd32x4::from(self[e4]).xyz())
                .with_w(-(right_anti_dual_g0_xyz[0] * self[e1]) - (right_anti_dual_g0_xyz[1] * self[e2]) - (right_anti_dual_g0_xyz[2] * self[e3])),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()))
                .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
        )
    }
}
impl WeightExpansion<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       16        0        0
    //    simd2        0        1        0      N/A
    //    simd3       10       18        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       21       38        0      N/A
    //  no simd       44       84        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3])
                    - (other[e4] * self[e5])
                    - (self[e4] * other[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[0]) * self.group0(),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]) * self.group0()) - (right_anti_dual_g1 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_anti_dual_g1.xyz()) - (Simd32x3::from(right_anti_dual_g1[3]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g1[2] * self[e2]) - (right_anti_dual_g1[1] * self[e3]),
                (right_anti_dual_g1[0] * self[e3]) - (right_anti_dual_g1[2] * self[e1]),
                (right_anti_dual_g1[1] * self[e1]) - (right_anti_dual_g1[0] * self[e2]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) + (Simd32x3::from(self[e4]) * other.group8()) + (Simd32x3::from(self[e5]) * other.group7())).with_w(0.0),
            // e423, e431, e412
            (right_anti_dual_g5 * Simd32x3::from(self[e4])) + (other.group7().yzx() * self.group0().zxy()) - (other.group7().zxy() * self.group0().yzx()),
            // e235, e315, e125
            (right_anti_dual_g5 * Simd32x3::from(self[e5])) + (other.group8().zxy() * self.group0().yzx()) - (other.group8().yzx() * self.group0().zxy()),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g6_xyz.yzx() * self.group0().zxy())
                - (right_anti_dual_g7 * Simd32x3::from(self[e5]))
                - (Simd32x3::from(self[e4]) * other.group3().xyz())
                - (right_anti_dual_g6_xyz.zxy() * self.group0().yzx()))
            .with_w(0.0),
            // e1234
            (right_anti_dual_g7[0] * self[e1]) + (right_anti_dual_g7[1] * self[e2]) + (right_anti_dual_g7[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<Plane> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        4        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        3        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd        6       17        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((other.group0().yz() * self.group0().zx()) - (other.group0().zx() * self.group0().yz()))
                .with_zw((other[e4235] * self[e2]) - (other[e4315] * self[e1]), other[e3215] * self[e4]),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz()),
        )
    }
}
impl WeightExpansion<RoundPoint> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3])
                - (other[e4] * self[e5])
                - (self[e4] * other[e5]),
        )
    }
}
impl WeightExpansion<Sphere> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        2        6        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        3        9        0      N/A
    //  no simd       10       24        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        Dipole::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (self.group0().yzxw() * right_anti_dual_g0_xyz.zxy().with_w(other[e3215]))
                + -(right_anti_dual_g0_xyz.yzx() * self.group0().zxy()).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35
            (Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e5])),
        )
    }
}
impl WeightExpansion<VersorEven> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        4        7        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd       20       35        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g3_xyz = other.group3().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().zxyx() * right_anti_dual_g0_xyz.yzx().with_w(right_anti_dual_g3_xyz[0]))
                + (self.group0().wwwy() * right_anti_dual_g1_xyz.with_w(right_anti_dual_g3_xyz[1]))
                + -(right_anti_dual_g0_xyz.zxy() * self.group0().yzx()).with_w(0.0),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(other[e321]) * self.group0().xyz()))
                .with_w(0.0),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy()))
                .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
        )
    }
}
impl WeightExpansion<VersorOdd> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        4        8        0      N/A
    //    simd4        2        1        0      N/A
    // Totals...
    // yes simd        8       17        0      N/A
    //  no simd       23       38        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e23, e31, e12, e45
            (self.group0().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(other[e3215]))
                + -(right_anti_dual_g3_xyz.yzx() * self.group0().zxy()).with_w(other[e1234] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e5]))).with_w(self[e1] * other[e41] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (self[e1] * other[e31]) - (self[e2] * other[e23]), 0.0])
                + ((Simd32x3::from(self[e5]) * other.group0().xyz()) + ((self.group0().yz() * other.group1().zx()) - (self.group0().zx() * other.group1().yz())).with_z(0.0)
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Scalar {
    type Output = WeightExpansionInfixPartial<Scalar>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for Scalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       20        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for Scalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        7        0      N/A
    //  no simd        0       27        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiDualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        0      N/A
    // no simd        0        2        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(self[scalar]) * other.group0())
    }
}
impl WeightExpansion<AntiFlatPoint> for Scalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]))
    }
}
impl WeightExpansion<AntiFlector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiLine> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group1(),
        )
    }
}
impl WeightExpansion<AntiMotor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<AntiPlane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        2        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ other[e12345] * self[scalar] * -1.0)
    }
}
impl WeightExpansion<Circle> for Scalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0       14        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group2(),
        )
    }
}
impl WeightExpansion<CircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       19        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<Dipole> for Scalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        0        6        0      N/A
    //  no simd        0       16        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group2(),
        )
    }
}
impl WeightExpansion<DipoleInversion> for Scalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       28        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            Simd32x4::from(self[scalar]) * other.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<DualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        0      N/A
    //  no simd        0        3        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(self[scalar] * -1.0) * other.group0())
    }
}
impl WeightExpansion<FlatPoint> for Scalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<Flector> for Scalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl WeightExpansion<Line> for Scalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        0        2        0      N/A
    // no simd        0        6        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group1(),
        )
    }
}
impl WeightExpansion<Motor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        4        0      N/A
    // no simd        0       16        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        9        0        0
    //    simd2        0        2        0      N/A
    //    simd3        0        7        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        0       21        0      N/A
    //  no simd        0       46        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(self[scalar]) * other.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(self[scalar] * -1.0) * other.group9().xyz()).with_w(other[e1234] * self[scalar]),
            // e5
            other[e3215] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group8().with_w(other[e321]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group7(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group6().xyz(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * (other.group5() * Simd32x3::from(-1.0)).with_w(other[e45]),
            // e423, e431, e412
            Simd32x3::from(self[scalar] * -1.0) * other.group4(),
            // e235, e315, e125
            Simd32x3::from(self[scalar] * -1.0) * other.group3().xyz(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[scalar]) * other.group1().xyz()).with_w(other[e5] * self[scalar] * -1.0),
            // e1234
            other[e4] * self[scalar] * -1.0,
        )
    }
}
impl WeightExpansion<Plane> for Scalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))
    }
}
impl WeightExpansion<RoundPoint> for Scalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(other[e5] * self[scalar] * -1.0),
            // e1234
            other[e4] * self[scalar] * -1.0,
        )
    }
}
impl WeightExpansion<Scalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar])
    }
}
impl WeightExpansion<Sphere> for Scalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        8        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
            // e5
            other[e3215] * self[scalar],
        )
    }
}
impl WeightExpansion<VersorEven> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        8        0      N/A
    // no simd        0       32        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2().xyz().with_w(other[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3().xyz().with_w(other[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<VersorOdd> for Scalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //    simd3        0        2        0      N/A
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        0        8        0      N/A
    //  no simd        0       30        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(other[e3215]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for Sphere {
    type Output = WeightExpansionInfixPartial<Sphere>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiDualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e3215] * self[e1234])
    }
}
impl WeightExpansion<AntiMotor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ other[e3215] * self[e1234])
    }
}
impl WeightExpansion<AntiScalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e1234
            right_anti_dual_g0 * self[e1234],
        )
    }
}
impl WeightExpansion<CircleRotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g2_w) * self.group0(),
            // e1234
            right_anti_dual_g2_w * self[e1234],
        )
    }
}
impl WeightExpansion<DipoleInversion> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e1234] * self[e3215]) + (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightExpansion<DualNum> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        4        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e1234
            other[e12345] * self[e1234] * -1.0,
        )
    }
}
impl WeightExpansion<Flector> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightExpansion<Motor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
            // e1234
            right_anti_dual_g0_w * self[e1234],
        )
    }
}
impl WeightExpansion<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       15        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1_xyz = other.group9().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g1_xyz[0] * self[e4235])
                    + (right_anti_dual_g1_xyz[1] * self[e4315])
                    + (right_anti_dual_g1_xyz[2] * self[e4125])
                    + (other[e3215] * self[e1234])
                    + (self[e3215] * other[e1234]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0[0]) * self.group0(),
            // e1234
            right_anti_dual_g0[0] * self[e1234],
        )
    }
}
impl WeightExpansion<Plane> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e12345
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        )
    }
}
impl WeightExpansion<Sphere> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        4        8        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e4235])
                + (right_anti_dual_g0_xyz[1] * self[e4315])
                + (right_anti_dual_g0_xyz[2] * self[e4125])
                + (other[e3215] * self[e1234])
                + (self[e3215] * other[e1234]),
        )
    }
}
impl WeightExpansion<VersorEven> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        0      N/A
    //  no simd        0        6        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
            // e1234
            right_anti_dual_g0_w * self[e1234],
        )
    }
}
impl WeightExpansion<VersorOdd> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        5        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        4        6        0      N/A
    //  no simd        4        8        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g3_xyz[0] * self[e4235])
                + (right_anti_dual_g3_xyz[1] * self[e4315])
                + (right_anti_dual_g3_xyz[2] * self[e4125])
                + (self[e3215] * other[e1234])
                + (other[e3215] * self[e1234]),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for VersorEven {
    type Output = WeightExpansionInfixPartial<VersorEven>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       13        0        0
    //    simd3        5        8        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd       16       22        0      N/A
    //  no simd       29       41        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g1_w = other[e321] * -1.0;
        let right_anti_dual_g2_xyz = other.group2().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(self[e5]) * other.group0()) - (Simd32x3::from(right_anti_dual_g1_w) * self.group3().xyz()))
                .with_w(0.0),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[e5]) * right_anti_dual_g1_xyz.with_w(other[e4] * -1.0))
                + ((right_anti_dual_g2_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy())).with_w(
                    (other[e1] * self[e1])
                        - (right_anti_dual_g1_w * self[e321])
                        - (right_anti_dual_g1_xyz[0] * self[e415])
                        - (right_anti_dual_g1_xyz[1] * self[e425])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (other[e423] * self[e235])
                        - (other[e431] * self[e315])
                        - (other[e412] * self[e125]),
                ),
        )
    }
}
impl WeightExpansion<AntiDualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        3        0        0
    //    simd3        2        4        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       15        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e321] * -1.0;
        Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())).with_w(right_anti_dual_g0_w * self[e321] * -1.0),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd3        1        3        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        5        8        0      N/A
    //  no simd       10       17        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group3().xyzx() * Simd32x3::from(other[e321]).with_w(other[e1]))
                + (right_anti_dual_g0_xyz * Simd32x3::from(self[e4])).with_w(
                    (other[e321] * self[e321]) - (right_anti_dual_g0_xyz[0] * self[e423]) - (right_anti_dual_g0_xyz[1] * self[e431]) - (right_anti_dual_g0_xyz[2] * self[e412]),
                ),
            // e235, e315, e125, e5
            ((right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiLine> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        2        4        0      N/A
    // no simd        6       12        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0.yzx() * self.group3().zxy()) - (Simd32x3::from(self[e4]) * other.group1()) - (right_anti_dual_g0.zxy() * self.group3().yzx())).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        2        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        9       17        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other[e3215]) * self.group3(),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (other[e12] * self[e2]) - (other[e31] * self[e3]),
                (other[e23] * self[e3]) - (other[e12] * self[e1]),
                (other[e31] * self[e1]) - (other[e23] * self[e2]),
            ]) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[e4]) * other.group1().xyz()))
            .with_w(other[e3215] * self[e321]),
        )
    }
}
impl WeightExpansion<AntiPlane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3]) - (other[e5] * self[e4]),
        )
    }
}
impl WeightExpansion<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       17        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0) * self.group3(),
        )
    }
}
impl WeightExpansion<Circle> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd3        6        9        0      N/A
    // no simd       18       27        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        CircleRotor::from_groups(
            // e423, e431, e412
            (right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (other.group0().yzx() * self.group3().zxy()) - (other.group0().zxy() * self.group3().yzx()),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e321]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0()) + (Simd32x3::from(self[e4]) * other.group2())).with_w(0.0),
            // e235, e315, e125, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5])) + (other.group2().zxy() * self.group3().yzx()) - (other.group2().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        9       12        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        9       17        0      N/A
    //  no simd       27       44        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        let right_anti_dual_g2_w = other[e12345] * -1.0;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g2_w) * self.group0().xyz()) + (other.group0().yzx() * self.group3().zxy())
                - (other.group0().zxy() * self.group3().yzx()))
            .with_w(right_anti_dual_g2_w * self[e12345]),
            // e415, e425, e435, e321
            ((right_anti_dual_g2_xyz * Simd32x3::from(self[e4]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group1().xyz())
                + (Simd32x3::from(other[e321]) * self.group3().xyz())
                + (Simd32x3::from(self[e5]) * other.group0()))
            .with_w(right_anti_dual_g2_w * self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g2_w) * self.group2().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g2_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g2_w) * self.group3(),
        )
    }
}
impl WeightExpansion<Dipole> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        3        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        8       11        0      N/A
    //  no simd       16       19        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e31] * self[e1]) - (other[e23] * self[e2]), 0.0])
                + (((other.group1().zx() * self.group3().yz()) - (other.group1().yz() * self.group3().zx())).with_z(0.0)
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2()))
                .with_w(0.0),
            // e1234
            (right_anti_dual_g0[0] * self[e1]) + (right_anti_dual_g0[1] * self[e2]) + (right_anti_dual_g0[2] * self[e3]) + (other[e45] * self[e4]),
        )
    }
}
impl WeightExpansion<DipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       16        0        0
    //    simd2        2        4        0      N/A
    //    simd3        4        7        0      N/A
    //    simd4        4        4        0      N/A
    // Totals...
    // yes simd       19       31        0      N/A
    //  no simd       41       61        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            -(Simd32x3::from(other[e1234]) * self.group3().xyz()) - (Simd32x3::from(self[e4]) * other.group3().xyz()),
            // e23, e31, e12, e45
            (other.group3().yzxw() * self.group3().zxyw()) + -(other.group3().zx() * self.group3().yz()).with_zw(other[e4315] * self[e1] * -1.0, other[e1234] * self[e5] * -1.0),
            // e15, e25, e35, e1234
            (other.group3().wwwx() * self.group3().xyz().with_w(self[e423]))
                + (Simd32x3::from(self[e5]) * other.group3().xyz())
                    .with_w((right_anti_dual_g0[0] * self[e1]) + (other[e4315] * self[e431]) + (other[e4125] * self[e412]) - (other[e1234] * self[e321])),
            // e4235, e4315, e4125, e3215
            (self.group3().yzxx() * other.group1().zxy().with_w(other[e15]))
                + ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                    + ((other.group3().zx() * self.group1().yz()) - (other.group1().yz() * self.group3().zx()) - (other.group3().yz() * self.group1().zx()))
                        .with_z((other[e4315] * self[e415]) - (other[e23] * self[e2]) - (other[e4235] * self[e425]))
                    - (right_anti_dual_g0 * Simd32x3::from(self[e5]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz()))
                .with_w((other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5]) - (other[e4235] * self[e235]) - (other[e4315] * self[e315]))
                - (self.group2().xyzz() * Simd32x3::from(other[e1234]).with_w(other[e4125])),
        )
    }
}
impl WeightExpansion<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1       11        0      N/A
    //  no simd        1       24        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e5] * self[e4]) - (other[e12345] * self[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[e12345] * -1.0) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e12345] * -1.0) * self.group3(),
        )
    }
}
impl WeightExpansion<FlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e4]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) + (other[e35] * self[e3]) - (other[e45] * self[e5])),
            // e1234
            other[e45] * self[e4],
        )
    }
}
impl WeightExpansion<Flector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd2        1        3        0      N/A
    //    simd3        1        3        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       23       39        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group1().xyz(),
            // e23, e31, e12, e45
            ((other.group1().yz() * self.group3().zx()) - (other.group1().zx() * self.group3().yz()))
                .with_zw((other[e4235] * self[e2]) - (other[e4315] * self[e1]), other[e3215] * self[e4]),
            // e15, e25, e35, e1234
            (other.group1().wwwx() * self.group3().xyz().with_w(self[e423]))
                + (Simd32x3::from(self[e5]) * other.group1().xyz()).with_w((other[e45] * self[e4]) + (other[e4315] * self[e431]) + (other[e4125] * self[e412])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (other.group1().zx() * self.group1().yz()).with_z(other[e4315] * self[e415]))
                .with_w((other[e15] * self[e1]) + (other[e25] * self[e2]) - (other[e4315] * self[e315]) - (other[e4125] * self[e125]))
                - (other.group0() * Simd32x3::from(self[e4]).with_w(self[e5]))
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e235])),
        )
    }
}
impl WeightExpansion<Line> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[e4]) * other.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(self[e4]).xyz()).with_w(-(other[e415] * self[e1]) - (other[e425] * self[e2]) - (other[e435] * self[e3])),
            // e235, e315, e125, e12345
            ((Simd32x3::from(self[e5]) * other.group0()) + (other.group1().zxy() * self.group3().yzx()) - (other.group1().yzx() * self.group3().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        7        0        0
    //    simd3        4        7        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd        8       16        0      N/A
    //  no simd       19       36        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e12345] * -1.0);
        let right_anti_dual_g1_xyz = other.group1().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_anti_dual_g0 * Simd32x3::from(self[e4]).with_w(self[e12345]))
                + (Simd32x3::from(right_anti_dual_g0[3]) * self.group0().xyz())
                    .with_w(-(right_anti_dual_g0[0] * self[e415]) - (right_anti_dual_g0[1] * self[e425]) - (right_anti_dual_g0[2] * self[e435]) - (other[e5] * self[e4])),
            // e415, e425, e435, e321
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())).with_w(right_anti_dual_g0[3] * self[e321]),
            // e235, e315, e125, e5
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz())
                + (Simd32x3::from(self[e5]) * right_anti_dual_g0.xyz())
                + (right_anti_dual_g1_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g1_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g0[3] * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[3]) * self.group3(),
        )
    }
}
impl WeightExpansion<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       28       39        0        0
    //    simd2        0        2        0      N/A
    //    simd3       13       21        0      N/A
    //    simd4        5        7        0      N/A
    // Totals...
    // yes simd       46       69        0      N/A
    //  no simd       87      134        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3_w = other[e321] * -1.0;
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g9_xyz = other.group1().xyz();
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual_g0[0] * self[e12345]) + (right_anti_dual_g9_xyz[0] * self[e1]) + (right_anti_dual_g9_xyz[1] * self[e2]) + (right_anti_dual_g9_xyz[2] * self[e3])
                    - (right_anti_dual_g3_w * self[e321])
                    - (right_anti_dual_g5[0] * self[e415])
                    - (right_anti_dual_g5[1] * self[e425])
                    - (right_anti_dual_g5[2] * self[e435])
                    - (other[e423] * self[e235])
                    - (other[e431] * self[e315])
                    - (other[e412] * self[e125])
                    - (other[e235] * self[e423])
                    - (other[e315] * self[e431])
                    - (other[e125] * self[e412])
                    - (other[e4] * self[e5])
                    - (self[e4] * other[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0[0]) * self.group3(),
            // e5
            right_anti_dual_g0[0] * self[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e3215]) * self.group3()) - (right_anti_dual_g1 * Simd32x4::from(self[e5])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_anti_dual_g1.xyz()) - (Simd32x3::from(right_anti_dual_g1[3]) * self.group3().xyz()),
            // e23, e31, e12
            Simd32x3::from([
                (right_anti_dual_g1[2] * self[e2]) - (right_anti_dual_g1[1] * self[e3]),
                (right_anti_dual_g1[0] * self[e3]) - (right_anti_dual_g1[2] * self[e1]),
                (right_anti_dual_g1[1] * self[e1]) - (right_anti_dual_g1[0] * self[e2]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()) + (Simd32x3::from(self[e5]) * other.group7()) + (Simd32x3::from(self[e4]) * other.group8())
                - (Simd32x3::from(right_anti_dual_g3_w) * self.group3().xyz()))
            .with_w(right_anti_dual_g0[0] * self[e321]),
            // e423, e431, e412
            (right_anti_dual_g5 * Simd32x3::from(self[e4])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz()) + (other.group7().yzx() * self.group3().zxy())
                - (other.group7().zxy() * self.group3().yzx()),
            // e235, e315, e125
            (right_anti_dual_g5 * Simd32x3::from(self[e5])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group2().xyz()) + (other.group8().zxy() * self.group3().yzx())
                - (other.group8().yzx() * self.group3().zxy()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other[e3215]) * self.group0().xyz().with_w(self[e321]))
                + (right_anti_dual_g1.yzxz() * self.group1().zxy().with_w(self[e125]))
                + ((right_anti_dual_g8 * Simd32x3::from(self[e4]))
                    + (right_anti_dual_g6_xyz.yzx() * self.group3().zxy())
                    + -(right_anti_dual_g1.zx() * self.group1().yz()).with_z(right_anti_dual_g1[1] * self[e415] * -1.0)
                    - (right_anti_dual_g7 * Simd32x3::from(self[e5])))
                .with_w((right_anti_dual_g1[0] * self[e235]) + (right_anti_dual_g1[1] * self[e315]) - (right_anti_dual_g8[1] * self[e2]) - (right_anti_dual_g8[2] * self[e3]))
                - (self.group2() * Simd32x3::from(right_anti_dual_g1[3]).with_w(other[e45]))
                - (self.group3().yzxx() * right_anti_dual_g6_xyz.zxy().with_w(right_anti_dual_g8[0])),
            // e1234
            (right_anti_dual_g7[0] * self[e1]) + (right_anti_dual_g7[1] * self[e2]) + (right_anti_dual_g7[2] * self[e3]) + (other[e45] * self[e4])
                - (right_anti_dual_g1[0] * self[e423])
                - (right_anti_dual_g1[1] * self[e431])
                - (right_anti_dual_g1[2] * self[e412])
                - (right_anti_dual_g1[3] * self[e321]),
        )
    }
}
impl WeightExpansion<Plane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd2        2        4        0      N/A
    //    simd3        2        4        0      N/A
    //    simd4        1        0        0      N/A
    // Totals...
    // yes simd        7       14        0      N/A
    //  no simd       16       26        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4] * -1.0) * other.group0().xyz(),
            // e23, e31, e12, e45
            ((other.group0().yz() * self.group3().zx()) - (other.group0().zx() * self.group3().yz()))
                .with_zw((other[e4235] * self[e2]) - (other[e4315] * self[e1]), other[e3215] * self[e4]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group3().xyz()) + (Simd32x3::from(self[e5]) * other.group0().xyz())).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (other[e4315] * self[e415]) - (other[e4235] * self[e425]), 0.0])
                + ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + ((other.group0().zx() * self.group1().yz()) - (other.group0().yz() * self.group1().zx())).with_z(0.0))
                    .with_w(0.0),
        )
    }
}
impl WeightExpansion<RoundPoint> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        AntiScalar::from_groups(
            // e12345
            (right_anti_dual_g0_xyz[0] * self[e1]) + (right_anti_dual_g0_xyz[1] * self[e2]) + (right_anti_dual_g0_xyz[2] * self[e3])
                - (other[e4] * self[e5])
                - (self[e4] * other[e5]),
        )
    }
}
impl WeightExpansion<Sphere> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        6        0        0
    //    simd3        6       11        0      N/A
    // Totals...
    // yes simd        8       17        0      N/A
    //  no simd       20       39        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g0_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e23, e31, e12, e45
            ((right_anti_dual_g0_xyz.zxy() * self.group3().yzx()) - (right_anti_dual_g0_xyz.yzx() * self.group3().zxy()))
                .with_w((other[e3215] * self[e4]) - (self[e5] * other[e1234])),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))).with_w(self[e321] * other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (right_anti_dual_g0_xyz.yzx() * self.group1().zxy())
                - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                - (right_anti_dual_g0_xyz.zxy() * self.group1().yzx()))
            .with_w((right_anti_dual_g0_xyz[0] * self[e235]) + (right_anti_dual_g0_xyz[1] * self[e315])),
        )
    }
}
impl WeightExpansion<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        9        0        0
    //    simd3        8       11        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       14       22        0      N/A
    //  no simd       33       50        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        let right_anti_dual_g2_xyz = other.group2().xyz();
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(right_anti_dual_g0_w) * self.group0())
                + ((right_anti_dual_g1_xyz * Simd32x3::from(self[e4])) + (right_anti_dual_g0_xyz.yzx() * self.group3().zxy())
                    - (right_anti_dual_g0_xyz.zxy() * self.group3().yzx()))
                .with_w(
                    (other[e1] * self[e1])
                        - (right_anti_dual_g1_xyz[2] * self[e435])
                        - (right_anti_dual_g2_xyz[0] * self[e423])
                        - (right_anti_dual_g2_xyz[1] * self[e431])
                        - (right_anti_dual_g2_xyz[2] * self[e412])
                        - (other[e4] * self[e5]),
                ),
            // e415, e425, e435, e321
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e5]))
                + (right_anti_dual_g2_xyz * Simd32x3::from(self[e4]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())
                + (Simd32x3::from(other[e321]) * self.group3().xyz()))
            .with_w(right_anti_dual_g0_w * self[e321]),
            // e235, e315, e125, e5
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[e5]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group3().yzx())
                - (right_anti_dual_g2_xyz.yzx() * self.group3().zxy()))
            .with_w(right_anti_dual_g0_w * self[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual_g0_w) * self.group3(),
        )
    }
}
impl WeightExpansion<VersorOdd> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        8        0        0
    //    simd2        1        2        0      N/A
    //    simd3        7       11        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       14       23        0      N/A
    //  no simd       35       53        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g3_xyz = other.group3().xyz() * Simd32x3::from(-1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            (right_anti_dual_g3_xyz * Simd32x3::from(self[e4])) - (Simd32x3::from(other[e1234]) * self.group3().xyz()),
            // e23, e31, e12, e45
            (self.group3().yzxw() * right_anti_dual_g3_xyz.zxy().with_w(other[e3215]))
                + -(right_anti_dual_g3_xyz.yzx() * self.group3().zxy()).with_w(self[e5] * other[e1234] * -1.0),
            // e15, e25, e35, e1234
            ((Simd32x3::from(other[e3215]) * self.group3().xyz()) - (right_anti_dual_g3_xyz * Simd32x3::from(self[e5]))).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (self.group2().wwwz() * other.group0().xyz().with_w(right_anti_dual_g3_xyz[2]))
                + ((Simd32x3::from(other[e3215]) * self.group0().xyz())
                    + (right_anti_dual_g3_xyz.yzx() * self.group1().zxy())
                    + ((self.group3().yz() * other.group1().zx()) - (self.group3().zx() * other.group1().yz())).with_z((self[e1] * other[e31]) - (self[e2] * other[e23]))
                    - (Simd32x3::from(self[e4]) * other.group2().xyz())
                    - (Simd32x3::from(other[e1234]) * self.group2().xyz())
                    - (right_anti_dual_g3_xyz.zxy() * self.group1().yzx()))
                .with_w((right_anti_dual_g3_xyz[0] * self[e235]) + (right_anti_dual_g3_xyz[1] * self[e315]) + (self[e321] * other[e3215]) - (self[e5] * other[e45])),
        )
    }
}
impl std::ops::Div<WeightExpansionInfix> for VersorOdd {
    type Output = WeightExpansionInfixPartial<VersorOdd>;
    fn div(self, _rhs: WeightExpansionInfix) -> Self::Output {
        WeightExpansionInfixPartial(self)
    }
}
impl WeightExpansion<AntiCircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       10       11        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd       10       17        0      N/A
    //  no simd       10       31        0        0
    fn weight_expansion(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group2().xyz() * Simd32x3::from(-1.0)).with_w(
                (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[scalar] * self[scalar])
                    - (right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
            ),
        )
    }
}
impl WeightExpansion<AntiDipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       10        0        0
    //    simd3        6        9        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       12       20        0      N/A
    //  no simd       24       41        0        0
    fn weight_expansion(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2_xyz = other.group2().xyz();
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (right_anti_dual_g2_xyz * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43])
                    - (other[e4] * self[scalar]),
            ),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group3().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (right_anti_dual_g2_xyz.zxy() * self.group0().yzx())
                + (other.group0().yzx() * self.group2().zxy())
                - (right_anti_dual_g2_xyz.yzx() * self.group0().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(other[e5] * self[scalar] * -1.0),
        )
    }
}
impl WeightExpansion<AntiDualNum> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        2        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        1        4        0      N/A
    //  no simd        1        9        0        0
    fn weight_expansion(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0().xyz() * Simd32x2::from(other[e3215]).with_z(other[e3215])).with_w((other[e3215] * self[e1234]) + (other[scalar] * self[scalar])),
            // e235, e315, e125, e5
            Simd32x4::from(other[e3215]) * self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl WeightExpansion<AntiFlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        3        0        0
    //    simd2        1        2        0      N/A
    //    simd3        1        1        0      N/A
    //    simd4        1        1        0      N/A
    // Totals...
    // yes simd        4        7        0      N/A
    //  no simd       10       14        0        0
    fn weight_expansion(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, (right_anti_dual_g0[1] * self[e41]) - (right_anti_dual_g0[0] * self[e42]), 0.0])
                + ((Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                    + ((right_anti_dual_g0.zx() * self.group0().yz()) - (right_anti_dual_g0.yz() * self.group0().zx())).with_z(0.0))
                .with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiFlector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        2        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       12        0      N/A
    //  no simd        9       19        0        0
    fn weight_expansion(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().xyz().with_w(other[e321] * -1.0);
        Flector::from_groups(
            // e15, e25, e35, e45
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g0[2] * self[e42]) - (right_anti_dual_g0[1] * self[e43]),
                (right_anti_dual_g0[0] * self[e43]) - (right_anti_dual_g0[2] * self[e41]),
                (right_anti_dual_g0[1] * self[e41]) - (right_anti_dual_g0[0] * self[e42]),
            ]) + (Simd32x3::from(right_anti_dual_g0[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(other[e5] * self[scalar] * -1.0),
        )
    }
}
impl WeightExpansion<AntiLine> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        6        0        0
    //    simd3        0        4        0      N/A
    // Totals...
    // yes simd        5       10        0      N/A
    //  no simd        5       18        0        0
    fn weight_expansion(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x3::from(-1.0);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(right_anti_dual_g0[0] * self[e23])
                    - (right_anti_dual_g0[1] * self[e31])
                    - (right_anti_dual_g0[2] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ),
            // e235, e315, e125, e5
            (right_anti_dual_g1 * Simd32x4::from(self[scalar]).xyz()).with_w(0.0),
        )
    }
}
impl WeightExpansion<AntiMotor> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        1        2        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        8       12        0      N/A
    //  no simd       16       25        0        0
    fn weight_expansion(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        Motor::from_groups(
            // e415, e425, e435, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]))
                + (other.group1().wwwx() * self.group0().xyzx())
                + Simd32x3::from(0.0).with_w(
                    (other[e25] * self[e42]) + (other[e35] * self[e43]) + (other[e3215] * self[e1234])
                        - (right_anti_dual_g0[0] * self[e23])
                        - (right_anti_dual_g0[1] * self[e31])
                        - (right_anti_dual_g0[2] * self[e12]),
                ),
            // e235, e315, e125, e5
            ((Simd32x3::from(other[e3215]) * self.group1().xyz()) - (Simd32x3::from(self[scalar]) * other.group1().xyz())).with_w(other[e3215] * self[scalar]),
        )
    }
}
impl WeightExpansion<AntiPlane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd4        0        2        0      N/A
    // no simd        0        8        0        0
    fn weight_expansion(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        )
    }
}
impl WeightExpansion<AntiScalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0       17        0        0
    fn weight_expansion(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other[e12345] * -1.0;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_anti_dual_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(right_anti_dual_g0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(right_anti_dual_g0) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_anti_dual_g0) * self.group3(),
        )
    }
}
impl WeightExpansion<Circle> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5        7        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd       10       16        0      N/A
    //  no simd       20       35        0        0
    fn weight_expansion(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (right_anti_dual_g1[0] * self[e41])
                    - (right_anti_dual_g1[1] * self[e42])
                    - (right_anti_dual_g1[2] * self[e43]),
            ),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (other.group0().yzx() * self.group2().zxy())
                + (other.group2().zxy() * self.group0().yzx())
                - (other.group0().zxy() * self.group2().yzx())
                - (other.group2().yzx() * self.group0().zxy()))
            .with_w(0.0),
        )
    }
}
impl WeightExpansion<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3       10        0        0
    //    simd3        6        8        0      N/A
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd       11       21        0      N/A
    //  no simd       29       46        0        0
    fn weight_expansion(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e12345] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x3::from(right_anti_dual_g2[3]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group0())).with_w(right_anti_dual_g2[3] * self[scalar]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g2[3]) * self.group1()),
            // e15, e25, e35, e1234
            (right_anti_dual_g2 * Simd32x3::from(self[scalar]).with_w(self[e1234])) + (Simd32x3::from(right_anti_dual_g2[3]) * self.group2().xyz()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g2[2] * self[e42]) - (right_anti_dual_g2[1] * self[e43]),
                (right_anti_dual_g2[0] * self[e43]) - (right_anti_dual_g2[2] * self[e41]),
                (right_anti_dual_g2[1] * self[e41]) - (right_anti_dual_g2[0] * self[e42]),
            ]) + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(right_anti_dual_g2[3]) * self.group3().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (other.group0().yzx() * self.group2().zxy())
                - (other.group0().zxy() * self.group2().yzx()))
            .with_w(right_anti_dual_g2[3] * self[e3215]),
        )
    }
}
impl WeightExpansion<Dipole> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       10        0        0
    //    simd3        0        4        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        9       16        0      N/A
    //  no simd        9       30        0        0
    fn weight_expansion(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        let right_anti_dual_g1 = other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2 = other.group2() * Simd32x3::from(-1.0);
        CircleRotor::from_groups(
            // e423, e431, e412
            right_anti_dual_g0 * Simd32x3::from(self[scalar]),
            // e415, e425, e435, e321
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e235, e315, e125, e12345
            (right_anti_dual_g2 * Simd32x4::from(self[scalar]).xyz()).with_w(
                -(right_anti_dual_g0[0] * self[e15])
                    - (right_anti_dual_g0[1] * self[e25])
                    - (right_anti_dual_g0[2] * self[e35])
                    - (right_anti_dual_g2[0] * self[e41])
                    - (right_anti_dual_g2[1] * self[e42])
                    - (right_anti_dual_g2[2] * self[e43])
                    - (right_anti_dual_g1[0] * self[e23])
                    - (right_anti_dual_g1[1] * self[e31])
                    - (right_anti_dual_g1[2] * self[e12])
                    - (right_anti_dual_g1[3] * self[e45]),
            ),
        )
    }
}
impl WeightExpansion<DipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       12       19        0        0
    //    simd2        1        2        0      N/A
    //    simd3        6        9        0      N/A
    //    simd4        1        2        0      N/A
    // Totals...
    // yes simd       20       32        0      N/A
    //  no simd       36       58        0        0
    fn weight_expansion(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x3::from(-1.0);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group1().xyzy() * Simd32x3::from(other[e1234]).with_w(other[e31]))
                + ((right_anti_dual_g0 * Simd32x3::from(self[scalar]))
                    + ((other.group3().yz() * self.group0().zx()) - (other.group3().zx() * self.group0().yz())).with_z((other[e4235] * self[e42]) - (other[e4315] * self[e41])))
                .with_w(
                    (other[e23] * self[e23]) + (other[e12] * self[e12]) + (other[e1234] * self[e3215])
                        - (right_anti_dual_g0[0] * self[e15])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35])
                        - (other[e45] * self[e45])
                        - (other[e4235] * self[e4235])
                        - (other[e4315] * self[e4315]),
                ),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e1234]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group3().xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz()))
            .with_w(other[e45] * self[scalar]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
                (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
                (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                - (Simd32x3::from(self[scalar]) * other.group2().xyz()))
            .with_w(other[e3215] * self[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]),
        )
    }
}
impl WeightExpansion<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        1        6        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        1       11        0      N/A
    //  no simd        1       24        0        0
    fn weight_expansion(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other[e12345] * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[e12345] * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[e12345] * -1.0) * self.group2(),
            // e4235, e4315, e4125, e3215
            (self.group3().xyz() * Simd32x2::from(other[e12345] * -1.0).with_z(other[e12345]) * Simd32x3::from([1.0, 1.0, -1.0]))
                .with_w(-(other[e5] * self[scalar]) - (other[e12345] * self[e3215])),
        )
    }
}
impl WeightExpansion<FlatPoint> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       11        0        0
    fn weight_expansion(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e45] * self[scalar]),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0))
                .with_w((other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) - (other[e45] * self[e45])),
        )
    }
}
impl WeightExpansion<Flector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6       14        0        0
    //    simd2        0        1        0      N/A
    //    simd3        2        5        0      N/A
    //    simd4        2        2        0      N/A
    // Totals...
    // yes simd       10       22        0      N/A
    //  no simd       20       39        0        0
    fn weight_expansion(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0().zxyx() * other.group1().yzx().with_w(other[e15]))
                + -(other.group1().zx() * self.group0().yz()).with_zw(other[e4315] * self[e41] * -1.0, -(other[e45] * self[e45]) - (other[e4235] * self[e4235])),
            // e415, e425, e435, e321
            (other.group1().wwwx() * self.group0().xyz().with_w(self[e23]))
                + (Simd32x3::from(self[e45]) * other.group1().xyz()).with_w((other[e45] * self[scalar]) + (other[e4315] * self[e31]) + (other[e4125] * self[e12])),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
                (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
                (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz())
                - (Simd32x3::from(self[scalar]) * other.group0().xyz()))
            .with_w(other[e3215] * self[scalar]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]).xyz() * other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<Line> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        3        0        0
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        8       18        0        0
    fn weight_expansion(self, other: Line) -> Self::Output {
        use crate::elements::*;
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            (other.group0() * Simd32x4::from(self[scalar]).xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(self[scalar]).xyz()).with_w(-(other[e415] * self[e41]) - (other[e425] * self[e42]) - (other[e435] * self[e43])),
            // e4235, e4315, e4125, e3215
            ((Simd32x3::from(self[e45]) * other.group0()) + (other.group1().zxy() * self.group0().yzx()) - (other.group1().yzx() * self.group0().zxy())).with_w(0.0),
        )
    }
}
impl WeightExpansion<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        5        8        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        5       13        0      N/A
    //  no simd       15       32        0        0
    fn weight_expansion(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1_xyz = other.group1().xyz();
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(right_anti_dual_g0_w) * self.group0(),
            // e23, e31, e12, e45
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group1().xyz())).with_w(right_anti_dual_g0_w * self[e45]),
            // e15, e25, e35, e1234
            ((right_anti_dual_g1_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group2().xyz())).with_w(right_anti_dual_g0_w * self[e1234]),
            // e4235, e4315, e4125, e3215
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())
                + (right_anti_dual_g1_xyz.zxy() * self.group0().yzx())
                - (right_anti_dual_g1_xyz.yzx() * self.group0().zxy()))
            .with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       31       47        0        0
    //    simd2        0        1        0      N/A
    //    simd3       15       22        0      N/A
    //    simd4        1        3        0      N/A
    // Totals...
    // yes simd       47       73        0      N/A
    //  no simd       80      127        0        0
    fn weight_expansion(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0().yx() * Simd32x2::from([-1.0, 1.0]);
        let right_anti_dual_g1 = (other.group9().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        let right_anti_dual_g3 = other.group8().with_w(other[e321] * -1.0);
        let right_anti_dual_g5 = other.group6().xyz();
        let right_anti_dual_g6_xyz = other.group5() * Simd32x3::from(-1.0);
        let right_anti_dual_g7 = other.group4() * Simd32x3::from(-1.0);
        let right_anti_dual_g8 = other.group3().xyz() * Simd32x3::from(-1.0);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                right_anti_dual_g0[0] * self[scalar],
                (right_anti_dual_g0[1] * self[scalar])
                    + (right_anti_dual_g1[0] * self[e4235])
                    + (right_anti_dual_g1[1] * self[e4315])
                    + (right_anti_dual_g1[2] * self[e4125])
                    + (right_anti_dual_g1[3] * self[e3215])
                    + (other[e3215] * self[e1234])
                    - (right_anti_dual_g6_xyz[0] * self[e23])
                    - (right_anti_dual_g6_xyz[1] * self[e31])
                    - (right_anti_dual_g6_xyz[2] * self[e12])
                    - (right_anti_dual_g7[0] * self[e15])
                    - (right_anti_dual_g7[1] * self[e25])
                    - (right_anti_dual_g7[2] * self[e35])
                    - (right_anti_dual_g8[0] * self[e41])
                    - (right_anti_dual_g8[1] * self[e42])
                    - (right_anti_dual_g8[2] * self[e43])
                    - (other[e45] * self[e45]),
            ]),
            // e1, e2, e3, e4
            right_anti_dual_g1 * Simd32x4::from(self[scalar]),
            // e5
            other[e3215] * self[scalar],
            // e15, e25, e35, e45
            (right_anti_dual_g3 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0[0]) * self.group2().xyz().with_w(self[e45])),
            // e41, e42, e43
            (Simd32x3::from(right_anti_dual_g0[0]) * self.group0().xyz()) + (Simd32x3::from(self[scalar]) * other.group7()),
            // e23, e31, e12
            (right_anti_dual_g5 * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0[0]) * self.group1().xyz()),
            // e415, e425, e435, e321
            ((right_anti_dual_g6_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group2().xyz())
                + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz()))
            .with_w(other[e45] * self[scalar]),
            // e423, e431, e412
            Simd32x3::from([
                (right_anti_dual_g1[2] * self[e42]) - (right_anti_dual_g1[1] * self[e43]),
                (right_anti_dual_g1[0] * self[e43]) - (right_anti_dual_g1[2] * self[e41]),
                (right_anti_dual_g1[1] * self[e41]) - (right_anti_dual_g1[0] * self[e42]),
            ]) + (right_anti_dual_g7 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz()),
            // e235, e315, e125
            Simd32x3::from([
                (right_anti_dual_g1[1] * self[e35]) - (right_anti_dual_g1[2] * self[e25]),
                (right_anti_dual_g1[2] * self[e15]) - (right_anti_dual_g1[0] * self[e35]),
                (right_anti_dual_g1[0] * self[e25]) - (right_anti_dual_g1[1] * self[e15]),
            ]) + (right_anti_dual_g8 * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g3[2] * self[e42]) - (right_anti_dual_g3[1] * self[e43]),
                (right_anti_dual_g3[0] * self[e43]) - (right_anti_dual_g3[2] * self[e41]),
                (right_anti_dual_g3[1] * self[e41]) - (right_anti_dual_g3[0] * self[e42]),
            ]) + (right_anti_dual_g5 * Simd32x3::from(self[e45]))
                + (Simd32x3::from(right_anti_dual_g0[0]) * self.group3().xyz())
                + (Simd32x3::from(right_anti_dual_g3[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group1().xyz())
                + (other.group7().yzx() * self.group2().zxy())
                - (other.group7().zxy() * self.group2().yzx()))
            .with_w(right_anti_dual_g0[0] * self[e3215]),
            // e1234
            (right_anti_dual_g0[0] * self[e1234])
                - (right_anti_dual_g5[0] * self[e41])
                - (right_anti_dual_g5[1] * self[e42])
                - (right_anti_dual_g5[2] * self[e43])
                - (other[e423] * self[e23])
                - (other[e431] * self[e31])
                - (other[e412] * self[e12])
                - (other[e4] * self[scalar]),
        )
    }
}
impl WeightExpansion<Plane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4       11        0        0
    //    simd2        1        2        0      N/A
    //    simd3        2        5        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd       12       30        0        0
    fn weight_expansion(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((other.group0().yz() * self.group0().zx()) - (other.group0().zx() * self.group0().yz()))
                .with_zw((other[e4235] * self[e42]) - (other[e4315] * self[e41]), other[e4235] * self[e4235] * -1.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(other[e3215]) * self.group0().xyz()) + (Simd32x3::from(self[e45]) * other.group0().xyz())).with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (other[e4125] * self[e25]) - (other[e4315] * self[e35]),
                (other[e4235] * self[e35]) - (other[e4125] * self[e15]),
                (other[e4315] * self[e15]) - (other[e4235] * self[e25]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz()))
            .with_w(other[e3215] * self[scalar]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]).xyz() * other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        )
    }
}
impl WeightExpansion<RoundPoint> for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        4        0        0
    //    simd3        0        1        0      N/A
    // Totals...
    // yes simd        0        5        0      N/A
    //  no simd        0        7        0        0
    fn weight_expansion(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]).xyz() * other.group0().xyz()).with_w(self[scalar] * other[e5] * -1.0),
            // e1234
            other[e4] * self[scalar] * -1.0,
        )
    }
}
impl WeightExpansion<Scalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn weight_expansion(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar])
    }
}
impl WeightExpansion<Sphere> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd2        0        1        0      N/A
    //    simd3        3        5        0      N/A
    //    simd4        3        3        0      N/A
    // Totals...
    // yes simd        9       18        0      N/A
    //  no simd       24       38        0        0
    fn weight_expansion(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, right_anti_dual_g0[0] * self[e42] * -1.0, 0.0])
                + (right_anti_dual_g0.zxyx() * self.group0().yzx().with_w(self[e4235]))
                + (right_anti_dual_g0.wwwy() * self.group1().xyz().with_w(self[e4315]))
                + -(right_anti_dual_g0.yz() * self.group0().zx()).with_zw(0.0, 0.0),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g0[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g0.xyz()))
            .with_w(0.0),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (right_anti_dual_g0[1] * self[e35]) - (right_anti_dual_g0[2] * self[e25]),
                (right_anti_dual_g0[2] * self[e15]) - (right_anti_dual_g0[0] * self[e35]),
                (right_anti_dual_g0[0] * self[e25]) - (right_anti_dual_g0[1] * self[e15]),
            ]) + (Simd32x3::from(other[e3215]) * self.group1().xyz()))
            .with_w(other[e3215] * self[scalar]),
            // e1, e2, e3, e4
            right_anti_dual_g0 * Simd32x4::from(self[scalar]),
        )
    }
}
impl WeightExpansion<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        8       17        0        0
    //    simd3        7        8        0      N/A
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd       18       29        0      N/A
    //  no simd       41       57        0        0
    fn weight_expansion(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0_xyz = other.group0().xyz();
        let right_anti_dual_g0_w = other[e12345] * -1.0;
        let right_anti_dual_g1 = other.group1().xyz().with_w(other[e321] * -1.0);
        let right_anti_dual_g2 = other.group2().xyz().with_w(other[e4] * -1.0);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((right_anti_dual_g0_xyz * Simd32x3::from(self[scalar])) + (Simd32x3::from(right_anti_dual_g0_w) * self.group0().xyz())).with_w(right_anti_dual_g0_w * self[scalar]),
            // e23, e31, e12, e45
            (right_anti_dual_g1 * Simd32x4::from(self[scalar])) + (Simd32x4::from(right_anti_dual_g0_w) * self.group1()),
            // e15, e25, e35, e1234
            (right_anti_dual_g2 * Simd32x4::from(self[scalar]))
                + (Simd32x4::from(right_anti_dual_g0_w) * self.group2())
                + Simd32x3::from(0.0).with_w(
                    -(right_anti_dual_g0_xyz[0] * self[e23])
                        - (right_anti_dual_g0_xyz[1] * self[e31])
                        - (right_anti_dual_g0_xyz[2] * self[e12])
                        - (right_anti_dual_g1[0] * self[e41])
                        - (right_anti_dual_g1[1] * self[e42])
                        - (right_anti_dual_g1[2] * self[e43]),
                ),
            // e4235, e4315, e4125, e3215
            (Simd32x3::from([
                (right_anti_dual_g2[2] * self[e42]) - (right_anti_dual_g2[1] * self[e43]),
                (right_anti_dual_g2[0] * self[e43]) - (right_anti_dual_g2[2] * self[e41]),
                (right_anti_dual_g2[1] * self[e41]) - (right_anti_dual_g2[0] * self[e42]),
            ]) + (Simd32x3::from(right_anti_dual_g0_w) * self.group3().xyz())
                + (Simd32x3::from(right_anti_dual_g1[3]) * self.group1().xyz())
                + (Simd32x3::from(self[scalar]) * other.group3().xyz())
                + (Simd32x3::from(self[e45]) * right_anti_dual_g1.xyz())
                + (right_anti_dual_g0_xyz.yzx() * self.group2().zxy())
                - (right_anti_dual_g0_xyz.zxy() * self.group2().yzx()))
            .with_w(right_anti_dual_g0_w * self[e3215]),
        )
    }
}
impl WeightExpansion<VersorOdd> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        9       17        0        0
    //    simd2        0        1        0      N/A
    //    simd3        5        8        0      N/A
    //    simd4        3        5        0      N/A
    // Totals...
    // yes simd       17       31        0      N/A
    //  no simd       36       63        0        0
    fn weight_expansion(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual_g0 = other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]);
        let right_anti_dual_g2_xyz = other.group2().xyz() * Simd32x3::from(-1.0);
        let right_anti_dual_g3 = (other.group3().xyz() * Simd32x3::from(-1.0)).with_w(other[e1234]);
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            (right_anti_dual_g0 * Simd32x4::from(self[scalar]))
                + (right_anti_dual_g3.zxyx() * self.group0().yzx().with_w(self[e4235]))
                + (self.group1().xyzx() * Simd32x3::from(right_anti_dual_g3[3]).with_w(other[e23]))
                + -(right_anti_dual_g3.yz() * self.group0().zx()).with_zw(
                    right_anti_dual_g3[0] * self[e42] * -1.0,
                    -(right_anti_dual_g2_xyz[0] * self[e41])
                        - (right_anti_dual_g2_xyz[1] * self[e42])
                        - (right_anti_dual_g2_xyz[2] * self[e43])
                        - (right_anti_dual_g0[0] * self[e15])
                        - (right_anti_dual_g0[1] * self[e25])
                        - (right_anti_dual_g0[2] * self[e35])
                        - (other[e45] * self[e45]),
                ),
            // e415, e425, e435, e321
            ((Simd32x3::from(right_anti_dual_g3[3]) * self.group2().xyz()) + (Simd32x3::from(other[e3215]) * self.group0().xyz())
                - (Simd32x3::from(self[scalar]) * other.group1().xyz())
                - (Simd32x3::from(self[e45]) * right_anti_dual_g3.xyz()))
            .with_w(other[e45] * self[scalar]),
            // e235, e315, e125, e5
            (Simd32x3::from([
                (right_anti_dual_g3[1] * self[e35]) - (right_anti_dual_g3[2] * self[e25]),
                (right_anti_dual_g3[2] * self[e15]) - (right_anti_dual_g3[0] * self[e35]),
                (right_anti_dual_g3[0] * self[e25]) - (right_anti_dual_g3[1] * self[e15]),
            ]) + (right_anti_dual_g2_xyz * Simd32x3::from(self[scalar]))
                + (Simd32x3::from(other[e3215]) * self.group1().xyz()))
            .with_w(other[e3215] * self[scalar]),
            // e1, e2, e3, e4
            right_anti_dual_g3 * Simd32x4::from(self[scalar]),
        )
    }
}
